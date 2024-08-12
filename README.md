# Sokoworld

Free and open source Sokoban client

See [level-format.txt](./level-format.txt) for the meaning of characters in the
level files.

## Dev Tools

Run a specific level with:

```console
cargo run -- -l=4
```

## Dev Notes

- the JS shims in web are explicitly checked in in case they disappear & to have versions match

## Assets

- Sprites: [Kenney](https://kenney.nl/assets/sokoban) - CC0

## Deployment

The game is currently hosted on itch at https://brettchalupa.itch.io/sokoworld

WASM builds can be built and pushed by running:

```console
./release_wasm.sh
```

Ideally in the future this would push builds for desktop operating systems, create tags, etc., but this works for development.
