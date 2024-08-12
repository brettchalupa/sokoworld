# Sokoworld

Free and open source Sokoban client

## Level Format

+-----------------------+-----------+------------+
| Puzzle element        | Character | ASCII Code |
+-----------------------+-----------+------------+
| Wall                  | #         | 0x23       |
+-----------------------+-----------+------------+
| Player                | @         | 0x40       |
+-----------------------+-----------+------------+
| Player on goal square | +         | 0x2b       |
+-----------------------+-----------+------------+
| Box                   | $         | 0x24       |
+-----------------------+-----------+------------+
| Box on goal square    | *         | 0x2a       |
+-----------------------+-----------+------------+
| Goal square           | .         | 0x2e       |
+-----------------------+-----------+------------+
| Floor                 | (Space)   | 0x20       |
+-----------------------+-----------+------------+

## Dev Tools

Run a specific level with:

``` console
cargo run -- -l=4
```

## Dev Notes

- the JS shims in web are explicitly checked in in case they disappear & to have versions match
