-- SokoWorld - Sokoban built with Usagi.

function _config()
  return {
    name = "SokoWorld",
    game_id = "com.brettmakesgames.sokoworld",
    icon = 4,
  }
end

-- Modules to rebuild on every save. `main.lua`'s chunk re-runs whenever
-- any project file changes, so clearing package.loaded here and then
-- re-requiring gives us a fresh table each save. `_init` is NOT called
-- on save reload, so State persists; only the code/data modules refresh.
local PROJECT_MODULES = {
  "consts", "save", "level", "playable_level",
  "packs", "packs.pack_a", "packs.ym_auto",
  "scenes.main_menu", "scenes.level_select", "scenes.gameplay", "scenes.credits",
}

for _, name in ipairs(PROJECT_MODULES) do
  package.loaded[name] = nil
end

Consts = require("consts")
Save = require("save")
Packs = require("packs")
Scenes = {
  main_menu = require("scenes.main_menu"),
  level_select = require("scenes.level_select"),
  gameplay = require("scenes.gameplay"),
  credits = require("scenes.credits"),
}

function _init()
  State = {
    save = Save.load(),
    current_scene = nil,
    pending_scene = nil,
  }
  SwitchScene("main_menu")
end

-- Queues a scene transition. The actual leave/enter runs at the start of
-- the next `_update` so the rest of the current frame finishes against a
-- stable scene (no mismatch between this frame's update and draw).
function SwitchScene(name, args)
  assert(Scenes[name], "SwitchScene: unknown scene '" .. tostring(name) .. "'")
  State.pending_scene = { name = name, args = args }
end

local function apply_pending_scene()
  if not State.pending_scene then return end
  local p = State.pending_scene
  State.pending_scene = nil
  local prev = State.current_scene
  if prev and Scenes[prev].leave then Scenes[prev].leave() end
  State.current_scene = p.name
  if Scenes[p.name].enter then Scenes[p.name].enter(p.args) end
end

function _update(dt)
  apply_pending_scene()
  if State and State.current_scene then
    Scenes[State.current_scene].update(dt)
  end
end

function _draw(dt)
  if State and State.current_scene then
    Scenes[State.current_scene].draw(dt)
  else
    gfx.clear(gfx.COLOR_BLACK)
  end
end
