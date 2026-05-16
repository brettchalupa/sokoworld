-- Main menu: pack carousel + menu rows (Play / Credits / Quit).

local consts = require("consts")
local save = require("save")

local main_menu = {}

local OPT_PACK_SELECT = 1
local OPT_CREDITS = 2
local OPT_QUIT = 3
local OPT_LABELS = {
  [OPT_CREDITS] = "Credits",
  [OPT_QUIT] = "Quit",
}

local function build_options()
  local opts = { OPT_PACK_SELECT, OPT_CREDITS }
  if usagi.PLATFORM ~= "web" then
    table.insert(opts, OPT_QUIT)
  end
  return opts
end

function main_menu.enter()
  if not State.menu then State.menu = {} end
  local m = State.menu
  m.options = build_options()
  m.menu_index = m.menu_index or 1
  m.focused_pack_index = m.focused_pack_index or 1
  -- Guard against index drift if pack list was edited between sessions.
  if m.focused_pack_index > #Packs.order then m.focused_pack_index = 1 end
  m.move_held_delay = 0
end

function main_menu.leave() end

local function step_menu(m, delta)
  m.menu_index = ((m.menu_index - 1 + delta) % #m.options) + 1
end

local function step_pack(m, delta)
  m.focused_pack_index = ((m.focused_pack_index - 1 + delta) % #Packs.order) + 1
end

local function focused_pack(m)
  return Packs[Packs.order[m.focused_pack_index]]
end

function main_menu.update(dt)
  local m = State.menu
  if m.move_held_delay > 0 then
    m.move_held_delay -= dt
  end

  local current = m.options[m.menu_index]

  if current == OPT_PACK_SELECT then
    if input.pressed(input.LEFT) or
        (input.held(input.LEFT) and m.move_held_delay <= 0) then
      m.move_held_delay = consts.MOVE_HELD_DELAY
      sfx.play(consts.SFX.MENU_MOVE)
      step_pack(m, -1)
    elseif input.pressed(input.RIGHT) or
        (input.held(input.RIGHT) and m.move_held_delay <= 0) then
      m.move_held_delay = consts.MOVE_HELD_DELAY
      sfx.play(consts.SFX.MENU_MOVE)
      step_pack(m, 1)
    end
  end

  if input.pressed(input.UP) or
      (input.held(input.UP) and m.move_held_delay <= 0) then
    m.move_held_delay = consts.MOVE_HELD_DELAY
    sfx.play(consts.SFX.MENU_MOVE)
    step_menu(m, -1)
  elseif input.pressed(input.DOWN) or
      (input.held(input.DOWN) and m.move_held_delay <= 0) then
    m.move_held_delay = consts.MOVE_HELD_DELAY
    sfx.play(consts.SFX.MENU_MOVE)
    step_menu(m, 1)
  end

  if input.pressed(input.BTN1) then
    sfx.play(consts.SFX.MENU_SELECT)
    if current == OPT_PACK_SELECT then
      SwitchScene("level_select", { pack_key = Packs.order[m.focused_pack_index] })
    elseif current == OPT_CREDITS then
      SwitchScene("credits")
    elseif current == OPT_QUIT then
      usagi.quit()
    end
  end
end

function main_menu.draw(_dt)
  local m = State.menu
  gfx.clear(gfx.COLOR_DARK_GREEN)

  gfx.text_ex("SokoWorld", 8, 6, 2, 0, gfx.COLOR_WHITE)
  gfx.spr(1, 120, 12)
  gfx.spr(4, 140, 12)

  local current = m.options[m.menu_index]
  local pack = focused_pack(m)
  local pack_focused = current == OPT_PACK_SELECT
  local pack_color = pack_focused and gfx.COLOR_PEACH or gfx.COLOR_WHITE

  local center_x = math.floor(usagi.GAME_W / 2)
  local cy = 50
  local function center_text(txt, y, color)
    local w, _ = usagi.measure_text(txt)
    gfx.text(txt, center_x - math.floor(w / 2), y, color)
  end

  center_text(pack.title, cy, pack_color)
  center_text(pack.author .. " - " .. pack.difficulty, cy + 12, pack_color)
  local count = save.count_complete(State.save, pack)
  center_text(#pack.levels .. " levels (" .. count .. " complete)",
    cy + 24, pack_color)

  if #Packs.order > 1 then
    local arrow_color = pack_focused and gfx.COLOR_PEACH or gfx.COLOR_WHITE
    gfx.text("<", 8, cy + 12, arrow_color)
    gfx.text(">", usagi.GAME_W - 12, cy + 12, arrow_color)
  end

  local row_y = 110
  for i, opt in ipairs(m.options) do
    if opt ~= OPT_PACK_SELECT then
      local color = (i == m.menu_index) and gfx.COLOR_PEACH or gfx.COLOR_WHITE
      center_text(OPT_LABELS[opt], row_y, color)
      row_y = row_y + 12
    end
  end

  local version = "v" .. consts.VERSION
  local _, vh = usagi.measure_text(version)
  local bottom_y = usagi.GAME_H - vh - 2
  gfx.text(version, 4, bottom_y, gfx.COLOR_PINK)
  local hint = input.mapping_for(input.BTN1) .. ":select"
  local hw, _ = usagi.measure_text(hint)
  gfx.text(hint, usagi.GAME_W - hw - 4, bottom_y, gfx.COLOR_PINK)
end

return main_menu
