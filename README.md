# SokoWorld

Free and open source Sokoban client made with
[Usagi Engine](https://usagiengine.com).

[Play the game!](https://brettchalupa.itch.io/sokoworld)

Alternate install instructions:

- [Download the latest release from GitHub](https://github.com/brettchalupa/sokoworld/releases)

## Defining Levels

Levels are collected in Packs, which are Lua files in ./packs dir.

See [level-format.txt](./level-format.txt) for the meaning of characters in the
level files.

## Dev Tools

In debug builds, <kbd>Shift</kbd> + <kbd>Esc</kbd> quits quickly.

## Developing

1. Install Usagi Engine (version 0.8.0 used for development)
2. Run `usagi dev`

## Designing Levels

In `./packs` are Lua tables of levels. Add new levels. Press <kbd>9</kbd> to
reload the current level tiles from the pack after updating them.

## Deployment

The game is currently hosted on itch at https://brettchalupa.itch.io/sokoworld

Deploy it with `just deploy`

## Rust Version

SokoWorld was originally written in Rust. If you want to view the Rust code,
[browse at commit 9d34a04e5fd8d121ec820de1a0239c48c0886f61](https://github.com/brettchalupa/sokoworld/tree/9d34a04e5fd8d121ec820de1a0239c48c0886f61).

## Credits

- Programming: Brett Chalupa
- Sprites:
  [Chrysalis](https://opengameart.org/content/tic-80-sokoban-tileset-8x8) (CC-BY
  4.0)
- Sound Effects: Melos Han-Tani of Analgesic Productions (non-commercial usage)
- Level design:
  - Brett Chalupa - pack-a
  - Yoshio Murase - ym-auto

## License

The cosource code is released under the Unlicense, see [LICENSE](./LICENSE) for
full details.

The license does not apply to some of the games assets and levels, they fall
under their own terms (see above). Assets created by me, Brett Chalupa, for the
game are CC0 (Public Domain).
