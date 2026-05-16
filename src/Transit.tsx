import { ComponentChildren } from "preact";
import { TransitPredictions } from "./state.ts";

function Transit(
  { transit }: { transit: TransitPredictions },
): ComponentChildren {
  return (
    <table>
      <tbody>
        {transit.lines.map((line) => (
          <>
            <tr>
              <th>{line.name}</th>
            </tr>

            {line.stops.map(stop => (
              <tr>
                <td>{stop.name}</td>
                <td>
                  {stop.predictions.map(minutes => `${minutes}m`).join(", ")}
                </td>
              </tr>
            ))}
          </>
        ))}
      </tbody>
    </table>
  );
}

export default Transit;
