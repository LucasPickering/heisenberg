use crate::{
    config::Config,
    state::{Message, Tx},
    util::http,
};
use chrono::{DateTime, Local, NaiveDate, TimeDelta, Utc};
use itertools::Itertools;
use serde::Deserialize;
use std::{thread, time::Duration};

/// https://github.com/jasonlttl/gameday-api-docs/blob/master/team-information.md
const MLB_ID_MAP: &[(u32, &str)] = &[
    (108, "LAA"),
    (109, "ARI"),
    (110, "BAL"),
    (111, "BOS"),
    (112, "CHC"),
    (113, "CIN"),
    (114, "CLE"),
    (115, "COL"),
    (116, "DET"),
    (117, "HOU"),
    (118, "KC"),
    (119, "LAD"),
    (120, "WSH"),
    (121, "NYM"),
    (133, "OAK"),
    (134, "PIT"),
    (135, "SD"),
    (136, "SEA"),
    (137, "SF"),
    (138, "STL"),
    (139, "TB"),
    (140, "TEX"),
    (141, "TOR"),
    (142, "MIN"),
    (143, "PHI"),
    (144, "ATL"),
    (145, "CWS"),
    (146, "MIA"),
    (147, "NYY"),
    (158, "MIL"),
];

/// Time between requests
const DATA_TTL: Duration = Duration::from_secs(60 * 60);
const URL: &str = "https://statsapi.mlb.com/api/v1/schedule";
/// Number of days to show at a time, starting from today
const DAY_RANGE: i64 = 2;

/// Fetch sports schedule in a loop. When we get a new schedule, send a message
/// to update the state
pub fn sports_loop(config: Config, tx: Tx) {
    let team_ids = config
        .sports_teams
        .iter()
        .filter_map(|abbr| abbr_to_id(abbr))
        .format(",")
        .to_string();
    loop {
        let start_date = Local::now().date_naive();
        // The end date is inclusive, so -1 to account for the start date
        let end_date = start_date + TimeDelta::days(DAY_RANGE - 1);
        let request = ureq::get(URL).query_pairs([
            ("sportId", "1"),
            ("startDate", &start_date.to_string()),
            ("endDate", &end_date.to_string()),
            ("teamId", &team_ids),
        ]);
        if let Ok(mlb) = http::<ApiMlbSchedule>(request) {
            let schedule = SportsSchedule::new(mlb);
            tx.send(Message::Sports(schedule));
        }
        thread::sleep(DATA_TTL);
    }
}

/// Upcoming SPORTS games
#[derive(Debug, Default)]
pub struct SportsSchedule {
    /// Games grouped by their date
    pub games_by_date: Vec<(NaiveDate, Vec<SportsGame>)>,
}

/// Sports game ready for display in the schedule
#[derive(Debug)]
pub struct SportsGame {
    pub time: DateTime<Local>,
    pub home: String,
    pub away: String,
}

impl SportsSchedule {
    fn new(mlb: ApiMlbSchedule) -> Self {
        // The HTTP request applies the date filter, so we shouldn't have to do
        // any filtering here
        let games_by_date = mlb
            .dates
            .into_iter()
            .map(|date| {
                let games = date
                    .games
                    .into_iter()
                    .map(|game| SportsGame {
                        time: game.game_date.with_timezone(&Local),
                        home: game.teams.home.team.abbr().to_owned(),
                        away: game.teams.away.team.abbr().to_owned(),
                    })
                    .collect();
                (date.date, games)
            })
            .collect();

        Self { games_by_date }
    }
}

/// MLB schedule
/// https://statsapi.mlb.com/api/v1/schedule?hydrate=team
#[derive(Debug, Deserialize)]
struct ApiMlbSchedule {
    dates: Vec<ApiMlbDate>,
}

/// Games on a particular date
#[derive(Debug, Deserialize)]
struct ApiMlbDate {
    date: NaiveDate,
    games: Vec<ApiMlbGame>,
}

/// A single game
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiMlbGame {
    game_date: DateTime<Utc>,
    teams: ApiMlbTeams,
}

/// Teams in a game
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiMlbTeams {
    away: ApiMlbGameTeam,
    home: ApiMlbGameTeam,
}

/// A team's result in a game
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiMlbGameTeam {
    team: ApiMlbTeam,
}

/// An MLB team
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiMlbTeam {
    id: u32,
}

impl ApiMlbTeam {
    /// Get the 3-letter abbreviation for this team
    fn abbr(&self) -> &'static str {
        id_to_abbr(self.id).unwrap_or("???")
    }
}

/// Convert MLB team API id to its abbreviation
fn id_to_abbr(id: u32) -> Option<&'static str> {
    MLB_ID_MAP
        .iter()
        .find(|(i, _)| *i == id)
        .map(|(_, abbr)| *abbr)
}

/// Convert MLB team abbreviation to its API id
fn abbr_to_id(abbr: &str) -> Option<u32> {
    MLB_ID_MAP
        .iter()
        .find(|(_, a)| *a == abbr)
        .map(|(id, _)| *id)
}
