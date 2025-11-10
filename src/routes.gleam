import gleam/bytes_tree
import gleam/erlang/process
import gleam/http/request.{type Request}
import gleam/http/response.{type Response}
import gleam/option.{None}
import gleam/otp/actor
import gleam/result
import gleam/string
import lustre/element
import lustre/element/html.{html}
import lustre/vdom/vnode.{type Element}
import mist.{type Connection, type ResponseData}
import model
import page.{type Page}
import repeatedly
import view

pub fn routes(
  model_actor: model.ModelActor,
  req: Request(Connection),
) -> Response(ResponseData) {
  case request.path_segments(req) {
    // TODO redirect to /weather
    [] -> serve_file(["index.html"])
    ["static", ..path] -> serve_file(path)
    // All main tabs use the same HTML. index.html has some JS that
    // hits the correct content endpoint based on the URL
    [page] ->
      case page.parse(page) {
        Ok(_) -> serve_file(["index.html"])
        Error(Nil) -> not_found()
      }
    // SSE endpoints page HTML as the state changes
    ["content", page] -> {
      case page.parse(page) {
        Ok(page) -> sse_connection(model_actor, req, page)
        Error(Nil) -> not_found()
      }
    }
    _ -> not_found()
  }
}

/// Generate a 404 page
fn not_found() -> Response(ResponseData) {
  let html =
    html([], [
      html.head([], [html.title([], "Not Found")]),
      html.body([], [html.h1([], [html.text("Not Found")])]),
    ])
  html_response(404, html)
}

/// Generate an HTTP response from HTML content
fn html_response(status_code: Int, html: Element(a)) -> Response(ResponseData) {
  let resp = response.new(status_code)
  response.set_body(
    resp,
    html
      |> element.to_document_string
      |> bytes_tree.from_string
      |> mist.Bytes,
  )
}

/// Start an SSE connection that pushes HTML content to the client. The client
/// should replace its main content with the returned HTML
fn sse_connection(
  model_actor: model.ModelActor,
  req: Request(Connection),
  page: Page,
) -> Response(ResponseData) {
  mist.server_sent_events(
    req,
    response.new(200),
    init: fn(subj) {
      // Get the current global state from the model actor
      let global_model = model.get_model(model_actor)
      let repeater =
        repeatedly.call(1000, Nil, fn(_state, _count) {
          // Every second, ask for the new global model
          let global_model = model.get_model(model_actor)
          process.send(subj, Refresh(global_model))
        })
      let connection_model =
        ConnectionModel(model: global_model, repeater: repeater)
      Ok(actor.initialised(connection_model))
    },
    loop: fn(model: ConnectionModel, message: ConnectionMessage, conn) {
      // Check if the global model has changed. If so, send a message to the
      // client with the new HTML
      let Refresh(global_model) = message
      // If data hasn't changed, don't send an event to the client
      // TODO
      // use <- bool.guard(
      //   when: global_model == model.model,
      //   return: actor.continue(model),
      // )

      // Generate the new HTML content
      let event =
        view.view(page, model.model)
        |> element.to_string_tree
        |> mist.event
      // Send it to the client
      case mist.send_event(conn, event) {
        Ok(_) -> {
          actor.continue(ConnectionModel(..model, model: global_model))
        }
        Error(_) -> {
          // TODO do we need to stop repeater here?
          repeatedly.stop(model.repeater)
          actor.stop()
        }
      }
    },
  )
}

/// Generate an HTTP response from a file
fn serve_file(path: List(String)) -> Response(ResponseData) {
  // TODO don't allow .. traversal
  let file_path = "static/" <> string.join(path, "/")

  mist.send_file(file_path, offset: 0, limit: None)
  |> result.map(fn(file) {
    let content_type = guess_content_type(file_path)
    response.new(200)
    |> response.prepend_header("content-type", content_type)
    |> response.set_body(file)
  })
  |> result.lazy_unwrap(not_found)
}

fn guess_content_type(path: String) -> String {
  "TODO"
}

/// State for a single SSE connection
type ConnectionModel {
  ConnectionModel(model: model.GlobalModel, repeater: repeatedly.Repeater(Nil))
}

/// TODO
type ConnectionMessage {
  Refresh(model: model.GlobalModel)
}
