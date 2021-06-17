# 🔥 Hades

A GBA emulator,

This project is under heavy development and can't properly run any game yet.

![](https://imgur.com/d0lAJJD.png)

## Running

You need a game ROM and a legitimate GameBoy Advance BIOS or a replacement BIOS.

Place your BIOS in the same folder than the executable and name it `bios.bin`.

You can then run:

```bash
hades <path/to/game.gba>
```

## Building

The build dependencies are:

  - `meson`
  - `ninja`
  - `SDL2`
  - `GCC`
  - `readline`
  - `capstone`

On Ubuntu, you can install all those dependencies with:

```bash
$ apt install meson ninja-build gcc libsdl2-dev libreadline-dev libcapstone-dev
```

To build Hades, run:

```bash
meson build --buildtype=release
cd build
ninja
```

## Thanks

Special thanks to some insanely good reading/projects that act as a documentation when writing Hades:

  - [GBATEK](https://problemkaputt.de/gbatek.htm)
  - [Tonc](https://www.coranac.com/tonc/text/toc.htm)
  - [gba-tests](https://github.com/jsmolka/gba-tests)
  - [NanoBoyAdvance](https://github.com/fleroviux/NanoBoyAdvance/)
  - [mGBA](https://mgba.io/)
  - [gdkGBA](https://github.com/gdkchan/gdkGBA/)
