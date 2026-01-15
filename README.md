# heisenberg

## Pi Setup

Raspberry Pi OS Lite (TODO get version)

- Set up hyperpixel (TODO more detail)
- Install xorg, xterm, unclutter-xfixes
- startx in .bash_profile
- start xterm w/ heisenberg in .xinitrc
- configure xterm: .Xresources

## Development

```sh
mise dev # Run locally
mise deploy # Deploy to RPi
mise watch -- deploy # Deploy and watch files
```
