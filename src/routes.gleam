import gleam/bytes_tree
import gleam/http/request.{type Request}
import gleam/http/response.{type Response}
import lustre/attribute as attr
import lustre/element
import lustre/element/html.{html}
import lustre/vdom/vnode.{type Element}
import mist.{type Connection, type ResponseData}

pub fn routes(req: Request(Connection)) -> Response(ResponseData) {
  case request.path_segments(req) {
    ["weather"] -> weather()
    _ -> not_found()
  }
}

fn weather() -> Response(ResponseData) {
  let create_tab = fn(label: String) -> Element(_) {
    html.button(
      [
        attr.class("nav-tab"),
        // attr.class(case model.active_tab == tab {
      //   True -> "active"
      //   False -> ""
      // }),
      // event.on_click(SelectTab(tab)),
      ],
      [element.text(label)],
    )
  }

  respond(200, create_tab("Weather"))
}

pub fn not_found() -> Response(ResponseData) {
  let html =
    html([], [
      html.head([], [html.title([], "Not Found")]),
      html.body([], [html.h1([], [html.text("Not Found")])]),
    ])
  respond(404, html)
}

fn respond(status_code: Int, html: Element(a)) -> Response(ResponseData) {
  let res = response.new(status_code)
  response.set_body(
    res,
    html
      |> element.to_document_string
      |> bytes_tree.from_string
      |> mist.Bytes,
  )
}
