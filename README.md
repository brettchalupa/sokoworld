# SokoWorld

Free and open source Sokoban client made with Rust and Macroquad.

[Play the game!](https://brettchalupa.itch.io/sokoworld)

Alternate install instructions:

- [Download the latest release from GitHub](https://github.com/brettchalupa/sokoworld/releases)

## OS Notes

If you're playing the game on Linux, it defaults to Wayland and falls back to X11.

If you want to have the game's assets live separately from the binary, you can specify either of the following:

- `--assets` arg: `./sokoworld --assets /some/path/assets`
- set the `SOKOWORLD_ASSETS` environment variable: `SOKOWORLD_ASSETS=/some/path/assets ./sokoworld`

## Defining Levels

Levels are collected in Packs, which are
[TOML](https://toml.io/en/) files that describe the pack and
the data for each level within it.

The benefit of using TOML is that it's much more human readable
than XML but provides a bit of structure and mark-up. It
strikes a nice balance of easy to map to data structures while
also being human editable.

See [level-format.txt](./level-format.txt) for the meaning of
characters in the level files.

## Dev Tools

In debug builds, <kbd>Shift</kbd> + <kbd>Esc</kbd> quits quickly.

### Play a Specific Level

Run a specific pack and level with:

```console
cargo run -- -p=assets/yoshio-murase-automatic.toml -l=2
```

where `-p=` is the path to the level pack and `-l=` is the number of level in the levels table.

### Reload the Current Level from Disk

Press the <kbd>9</kbd> key to reload the current level from disk. This is useful for iterating on the design of a level.

## Developing

1. Install Rust (version 1.80.1 used for initial development)
2. Install ancillary dependencies to assist with development: `./deps.sh`
3. Run the project: `cargo run`

There's also `./serve_wasm.sh` to boot up a simple web server to test WASM builds, which can be rebuilt with `./build_wasm.sh`

## Dev Notes

- the JS shims in web are explicitly checked in in case they disappear & to have versions match

## Deployment

The game is currently hosted on itch at https://brettchalupa.itch.io/sokoworld

### Web

WASM builds can be built and pushed by running:

```console
./release_wasm.sh
```

Ideally in the future this would push builds for desktop operating systems, create tags, etc., but this works for development.

### macOS

macOS uses [cargo bundle](https://github.com/burtonageo/cargo-bundle):

1. Install cargo bundle: `cargo install cargo-bundle`
2. Build the bundle: `cargo bundle --release`

There's a `release_macos.sh` script to build and upload a Universal app for macOS (works on both Intel and Apple Silicon Macs).

### Windows

Not ideal but functioning

1. `cargo run --release`
2. `mkdir win`
3. `cp .\target\release\sokoworld.exe .\win\`
4. copy the assets folder into the `win` dir
5. zip it up
6. upload it to itch.io manually

### Linux

Run the script:

```console
./release_linux.sh
```

## Credits

- Programming: Brett Chalupa
- Sprites
  - Kenney set: [kenney.nl](https://kenney.nl) (CC0)
  - Retro set: Brett Chalupa
  - Marble set: [Vellidragon](https://opengameart.org/content/sokoban-clone-tiles) (CC0)
  - Doggo set: [Chrysalis](https://opengameart.org/content/tic-80-sokoban-tileset-8x8) (CC-BY 4.0)
- Sound Effects
  - melos: Melos Han-Tani of Analgesic Productions (non-commercial usage)

## License

The course code is released under the Unlicense, see [LICENSE](./LICENSE) for full details.

The license does not apply to the other assets, they fall under their own terms (see above). Assets created by me, Brett Chalupa, for the game are CC0 (Public Domain).
