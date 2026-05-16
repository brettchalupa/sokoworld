# SokoWorld

Free and open source Sokoban client made with
[Usagi Engine](https://usagiengine.com).

[Play the game!](https://brettchalupa.itch.io/sokoworld)

Alternate install instructions:

- [Download the latest release from GitHub](https://github.com/brettchalupa/sokoworld/releases)

## Defining Levels

Levels are collected in Packs, which are Lua files [TOML](https://toml.io/en/)
files that describe the pack and the data for each level within it.

The benefit of using TOML is that it's much more human readable than XML but
provides a bit of structure and mark-up. It strikes a nice balance of easy to
map to data structures while also being human editable.

See [level-format.txt](./level-format.txt) for the meaning of characters in the
level files.

## Dev Tools

In debug builds, <kbd>Shift</kbd> + <kbd>Esc</kbd> quits quickly.

## Developing

1. Install Usagi Engine (version 0.8.0 used for development)
2. Run `usagi dev`

## Deployment

The game is currently hosted on itch at https://brettchalupa.itch.io/sokoworld

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
