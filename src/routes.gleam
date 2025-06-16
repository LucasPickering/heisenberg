import gleam/bytes_tree
import gleam/http/response.{type Response}
import lustre/element
import lustre/element/html.{html}
import lustre/vdom/vnode
import mist.{type ResponseData}

pub fn greet(name: String) -> Response(ResponseData) {
  let html =
    html([], [
      html.head([], [html.title([], "Greetings!")]),
      html.body([], [html.h1([], [html.text("Hey there, " <> name <> "!")])]),
    ])
  respond(200, html)
}

pub fn not_found() -> Response(ResponseData) {
  let html =
    html([], [
      html.head([], [html.title([], "Not Found")]),
      html.body([], [html.h1([], [html.text("Not Found")])]),
    ])
  respond(404, html)
}

fn respond(status_code: Int, html: vnode.Element(a)) -> Response(ResponseData) {
  let res = response.new(status_code)
  response.set_body(
    res,
    html
      |> element.to_document_string
      |> bytes_tree.from_string
      |> mist.Bytes,
  )
}
