-- Level select: scroll horizontally through a pack's levels.

local consts = require("consts")
local save = require("save")

local level_select = {}

function level_select.enter(args)
  local pack_key = (args and args.pack_key) or Packs.order[1]
  local pack = Packs[pack_key]
  assert(pack, "level_select: unknown pack '" .. tostring(pack_key) .. "'")

  local focused = 1
  for i, lvl in ipairs(pack.levels) do
    if not save.is_complete(State.save, pack.slug, lvl.title) then
      focused = i
      break
    end
  end

  State.level_select = {
    pack_key = pack_key,
    pack = pack,
    focused_level_index = focused,
    move_held_delay = 0,
  }
end

function level_select.leave() end

local function step(ls, delta)
  ls.focused_level_index = ((ls.focused_level_index - 1 + delta) % #ls.pack.levels) + 1
end

function level_select.update(dt)
  local ls = State.level_select
  if ls.move_held_delay > 0 then
    ls.move_held_delay -= dt
  end

  if input.pressed(input.LEFT) or
      (input.held(input.LEFT) and ls.move_held_delay <= 0) then
    ls.move_held_delay = consts.MOVE_HELD_DELAY
    sfx.play(consts.SFX.MENU_MOVE)
    step(ls, -1)
  elseif input.pressed(input.RIGHT) or
      (input.held(input.RIGHT) and ls.move_held_delay <= 0) then
    ls.move_held_delay = consts.MOVE_HELD_DELAY
    sfx.play(consts.SFX.MENU_MOVE)
    step(ls, 1)
  elseif input.pressed(input.UP) or
      (input.held(input.UP) and ls.move_held_delay <= 0) then
    ls.move_held_delay = consts.MOVE_HELD_DELAY
    sfx.play(consts.SFX.MENU_MOVE)
    step(ls, -10)
  elseif input.pressed(input.DOWN) or
      (input.held(input.DOWN) and ls.move_held_delay <= 0) then
    ls.move_held_delay = consts.MOVE_HELD_DELAY
    sfx.play(consts.SFX.MENU_MOVE)
    step(ls, 10)
  end

  if input.pressed(input.BTN1) then
    sfx.play(consts.SFX.MENU_SELECT)
    SwitchScene("gameplay", {
      pack_key = ls.pack_key,
      level_index = ls.focused_level_index,
    })
  end

  if input.pressed(input.BTN2) then
    sfx.play(consts.SFX.MENU_CANCEL)
    SwitchScene("main_menu")
  end
end

function level_select.draw(_dt)
  local ls = State.level_select
  local pack = ls.pack
  gfx.clear(gfx.COLOR_DARK_PURPLE)

  gfx.text(pack.title, 4, 4, gfx.COLOR_WHITE)
  gfx.text("by " .. pack.author, 4, 14, gfx.COLOR_LIGHT_GRAY)
  gfx.text("Select a Level", 4, 30, gfx.COLOR_WHITE)

  -- show a horizontal strip of level names around the focused index
  local cy = math.floor(usagi.GAME_H / 2) - 6
  local cx = math.floor(usagi.GAME_W / 2)
  local span = 4
  for offset = -span, span do
    local i = ls.focused_level_index + offset
    if i >= 1 and i <= #pack.levels then
      local lvl = pack.levels[i]
      local complete = save.is_complete(State.save, pack.slug, lvl.title)
      local color
      if offset == 0 then
        color = gfx.COLOR_PEACH
      elseif complete then
        color = gfx.COLOR_BLUE
      else
        color = gfx.COLOR_WHITE
      end
      local w, _ = usagi.measure_text(lvl.title)
      local x = cx + offset * 80 - math.floor(w / 2)
      gfx.text(lvl.title, x, cy, color)
      if complete then
        local cw, _ = usagi.measure_text("complete")
        gfx.text("complete", cx + offset * 80 - math.floor(cw / 2), cy + 12, color)
      end
    end
  end

  local hint = input.mapping_for(input.BTN1) .. ":start  " ..
      input.mapping_for(input.BTN2) .. ":back"
  local _, hh = usagi.measure_text(hint)
  gfx.text(hint, 4, usagi.GAME_H - hh - 2, gfx.COLOR_PINK)
end

return level_select
