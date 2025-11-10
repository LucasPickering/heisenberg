import gleam/io
import heisenberg
import radiate

pub fn main() {
  // Hot reloading
  let assert Ok(_) =
    radiate.new()
    // radiate silently fails if you pass any relative path other than `.` on
    // macOS. And no one knows how to get the cwd in gleam!! crazy
    |> radiate.add_dir(".")
    |> radiate.on_reload(fn(_state, path) {
      io.println("Change in " <> path <> ", reloading")
    })
    |> radiate.start()

  heisenberg.main()
}
