# SokoWorld

Free and open source Sokoban client

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

Run a specific level with:

```console
cargo run -- -l=4
```

## Dev Notes

- the JS shims in web are explicitly checked in in case they disappear & to have versions match

## Assets

- Retro: Brett Chalupa - CC0
- Kenney Tileset: [Kenney](https://kenney.nl/assets/sokoban) - CC0

## Deployment

The game is currently hosted on itch at https://brettchalupa.itch.io/sokoworld

WASM builds can be built and pushed by running:

```console
./release_wasm.sh
```

Ideally in the future this would push builds for desktop operating systems, create tags, etc., but this works for development.

## Credits

- Programming: Brett Chalupa
- Sprites
  - Kenney set: [kenney.nl](https://kenney.nl) (CC0)
  - Retro set: Brett Chalupa
- Sound Effects
  - melos: Melos Han-Tani of Analgesic Productions (non-commercial usage)
