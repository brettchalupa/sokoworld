-- Gameplay: wraps playable_level, handles next-level transition,
-- registers pause-menu shortcuts via usagi.menu_item.

local consts = require("consts")
local playable_level = require("playable_level")

local gameplay = {}

local function build_playable(pack, level_index)
  return playable_level.new(pack.slug, pack.levels[level_index])
end

function gameplay.enter(args)
  local pack_key = args.pack_key
  local level_index = args.level_index
  local pack = Packs[pack_key]
  assert(pack, "gameplay: unknown pack '" .. tostring(pack_key) .. "'")

  State.gameplay = {
    pack_key = pack_key,
    pack = pack,
    level_index = level_index,
    pl = build_playable(pack, level_index),
  }

  usagi.clear_menu_items()
  usagi.menu_item("Back to Level Select", function()
    SwitchScene("level_select", { pack_key = State.gameplay.pack_key })
    return false
  end)
  usagi.menu_item("Back to Main Menu", function()
    SwitchScene("main_menu")
    return false
  end)
end

function gameplay.leave()
  usagi.clear_menu_items()
end

function gameplay.update(dt)
  local g = State.gameplay

  -- Re-parse current level from Packs (which auto-refreshes on save).
  -- Useful while designing: edit the pack file, save, hit 9 to apply.
  if input.key_pressed(input.KEY_9) then
    sfx.play(consts.SFX.RESET)
    g.pack = Packs[g.pack_key]
    g.pl = build_playable(g.pack, g.level_index)
    return
  end

  playable_level.update(g.pl, dt)

  if g.pl.request_next_level then
    g.pl.request_next_level = false
    g.level_index = g.level_index + 1
    if g.level_index > #g.pack.levels then
      SwitchScene("level_select", { pack_key = g.pack_key })
    else
      g.pl = build_playable(g.pack, g.level_index)
    end
  end
end

function gameplay.draw(_dt)
  gfx.clear(gfx.COLOR_DARK_BLUE)
  playable_level.draw(State.gameplay.pl)
end

return gameplay
