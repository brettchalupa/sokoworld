-- Ordered, key-addressable collection of packs.
--
-- Adding a pack: write `packs/<key>.lua`, then add a `<key> = require(...)`
-- line and append the key to `order`. Removing a pack: delete both lines.
--
-- Looking up: `Packs.pack_a` or `Packs[key]` returns the pack table.
-- Iterating in display order: `for _, key in ipairs(Packs.order) do ... end`.

local pack_a = require("packs.pack_a")
local pack_b = require("packs.pack_b")
local ym_auto = require("packs.ym_auto")

return {
  pack_a = pack_a,
  pack_b = pack_b,
  ym_auto = ym_auto,
  order = { "pack_a", "pack_b", "ym_auto" },
}
