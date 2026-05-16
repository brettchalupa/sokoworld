-- Static credits screen. BTN1 or BTN2 returns to the main menu.

local consts = require("consts")

local credits = {}

function credits.enter() end

function credits.leave() end

function credits.update(_dt)
  if input.pressed(input.BTN1) or input.pressed(input.BTN2) then
    sfx.play(consts.SFX.MENU_CANCEL)
    SwitchScene("main_menu")
  end
end

local LINES = {
  "Art by Chrysalis and Brett Chalupa",
  "Sound effects by Melos Han-Tani",
  "Level design by Brett Chalupa and Yoshio Murase",
  "Programming by Brett Chalupa",
}

function credits.draw(_dt)
  gfx.clear(gfx.COLOR_BLACK)
  gfx.text("Credits", 8, 8, gfx.COLOR_WHITE)
  for i, line in ipairs(LINES) do
    gfx.text(line, 8, 28 + (i - 1) * 12, gfx.COLOR_LIGHT_GRAY)
  end
  local back = "Press " .. input.mapping_for(input.BTN1) ..
      " or " .. input.mapping_for(input.BTN2) .. " to return"
  local _, bh = usagi.measure_text(back)
  gfx.text(back, 8, usagi.GAME_H - bh - 2, gfx.COLOR_RED)
end

return credits
