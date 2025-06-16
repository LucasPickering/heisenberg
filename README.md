# heisenberg

[![Package Version](https://img.shields.io/hexpm/v/heisenberg)](https://hex.pm/packages/heisenberg)
[![Hex Docs](https://img.shields.io/badge/hex-docs-ffaff3)](https://hexdocs.pm/heisenberg/)

```sh
gleam add heisenberg@1
```

```gleam
import heisenberg

pub fn main() -> Nil {
  // TODO: An example of the project in use
}
```

Further documentation can be found at <https://hexdocs.pm/heisenberg>.

## Development

```sh
gleam run   # Run the project
gleam test  # Run the tests
```

## Pi Setup

### Install Erlang VM

There's no OTP 28 in the Debian package registry so you have to build it from source on the Pi (it takes a while):

```sh
# Build erlang from source
# https://www.erlang.org/downloads
./configure && make && sudo make install
```

Then copy `heisenberg_ui.desktop` to `/home/pi/.config/autostart/heisenberg_ui.desktop`
