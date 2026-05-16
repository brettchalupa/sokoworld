-- Playable level state: tracks player position, crate positions,
-- step/push counters, and move history for rewind.

local consts = require("consts")
local level = require("level")
local save = require("save")

local playable_level = {}

local DIR_UP = "up"
local DIR_DOWN = "down"
local DIR_LEFT = "left"
local DIR_RIGHT = "right"

local function pos_eq(a, b)
  return a.x == b.x and a.y == b.y
end

local function find_index(list, pos)
  for i, p in ipairs(list) do
    if pos_eq(p, pos) then return i end
  end
  return nil
end

local function is_on_any_storage(storage_locations, pos)
  for _, sl in ipairs(storage_locations) do
    if pos_eq(sl, pos) then return true end
  end
  return false
end

function playable_level.new(pack_slug, pack_level)
  local lvl = level.parse(pack_level)
  local ts = consts.TILE_SIZE
  local crates = {}
  for _, pos in ipairs(lvl.crates) do
    table.insert(crates, {
      x = pos.x,
      y = pos.y,
      vx = pos.x * ts,
      vy = pos.y * ts,
      on_storage = is_on_any_storage(lvl.storage_locations, pos),
    })
  end

  return {
    pack_slug = pack_slug,
    level = lvl,
    player = {
      x = lvl.player.x,
      y = lvl.player.y,
      vx = lvl.player.x * ts,
      vy = lvl.player.y * ts,
    },
    crates = crates,
    steps = 0,
    pushes = 0,
    move_held_delay = 0,
    moves = {},
    complete = false,
    request_next_level = false,
  }
end

function playable_level.reset(pl)
  local ts = consts.TILE_SIZE
  pl.player.x = pl.level.player.x
  pl.player.y = pl.level.player.y
  pl.player.vx = pl.player.x * ts
  pl.player.vy = pl.player.y * ts
  for i, c in ipairs(pl.crates) do
    local src = pl.level.crates[i]
    c.x = src.x
    c.y = src.y
    c.vx = c.x * ts
    c.vy = c.y * ts
    c.on_storage = is_on_any_storage(pl.level.storage_locations, c)
  end
  pl.steps = 0
  pl.pushes = 0
  pl.moves = {}
  pl.complete = false
end

local function direction_of(dx, dy)
  if dx > 0 then return DIR_RIGHT end
  if dx < 0 then return DIR_LEFT end
  if dy < 0 then return DIR_UP end
  return DIR_DOWN
end

local function check_crate_on_storage(pl, c)
  if is_on_any_storage(pl.level.storage_locations, c) then
    if not c.on_storage then
      sfx.play(consts.SFX.CRATE_ON_STORAGE)
    end
    c.on_storage = true
  else
    c.on_storage = false
  end
end

local function crate_at(pl, x, y)
  for i, c in ipairs(pl.crates) do
    if c.x == x and c.y == y then return i, c end
  end
  return nil, nil
end

local function wall_at(pl, x, y)
  for _, w in ipairs(pl.level.walls) do
    if w.x == x and w.y == y then return true end
  end
  return false
end

local function record_move(pl, direction, crate_moved_index)
  table.insert(pl.moves, {
    direction = direction,
    crate_moved_index = crate_moved_index,
  })
end

local function handle_movement(pl)
  local dx, dy = 0, 0
  local moved_action = nil

  if input.pressed(input.LEFT) or (input.held(input.LEFT) and pl.move_held_delay <= 0) then
    dx = -1; moved_action = input.LEFT
  elseif input.pressed(input.RIGHT) or (input.held(input.RIGHT) and pl.move_held_delay <= 0) then
    dx = 1; moved_action = input.RIGHT
  elseif input.pressed(input.UP) or (input.held(input.UP) and pl.move_held_delay <= 0) then
    dy = -1; moved_action = input.UP
  elseif input.pressed(input.DOWN) or (input.held(input.DOWN) and pl.move_held_delay <= 0) then
    dy = 1; moved_action = input.DOWN
  end

  if not moved_action then return end

  pl.move_held_delay = consts.MOVE_HELD_DELAY

  local new_px = pl.player.x + dx
  local new_py = pl.player.y + dy
  local crate_i, _ = crate_at(pl, new_px, new_py)

  if crate_i then
    local c = pl.crates[crate_i]
    local new_cx = c.x + dx
    local new_cy = c.y + dy
    if wall_at(pl, new_cx, new_cy) or crate_at(pl, new_cx, new_cy) then
      sfx.play(consts.SFX.CANT_MOVE)
      return
    end
    -- move player
    sfx.play(consts.SFX.FOOTSTEP)
    record_move(pl, direction_of(dx, dy), crate_i)
    pl.player.x = new_px
    pl.player.y = new_py
    pl.steps += 1
    -- move crate
    c.x = new_cx
    c.y = new_cy
    pl.pushes += 1
    sfx.play(consts.SFX.PUSH)
    check_crate_on_storage(pl, c)
  else
    if wall_at(pl, new_px, new_py) then
      sfx.play(consts.SFX.CANT_MOVE)
      return
    end
    sfx.play(consts.SFX.FOOTSTEP)
    record_move(pl, direction_of(dx, dy), nil)
    pl.player.x = new_px
    pl.player.y = new_py
    pl.steps += 1
  end

  -- check win
  local all_on = true
  for _, c in ipairs(pl.crates) do
    if not is_on_any_storage(pl.level.storage_locations, c) then
      all_on = false; break
    end
  end
  if all_on then
    sfx.play(consts.SFX.LEVEL_COMPLETE)
    pl.complete = true
    save.mark_complete(State.save, pl.pack_slug, pl.level.title, pl.steps, pl.pushes)
  end
end

local function handle_rewind(pl)
  if #pl.moves == 0 then
    sfx.play(consts.SFX.CANT_MOVE)
    return
  end
  local m = table.remove(pl.moves)
  sfx.play(consts.SFX.FOOTSTEP)
  pl.steps -= 1
  if m.crate_moved_index then
    pl.pushes -= 1
  end

  local dx, dy = 0, 0
  if m.direction == DIR_UP then
    dy = 1
  elseif m.direction == DIR_DOWN then
    dy = -1
  elseif m.direction == DIR_LEFT then
    dx = 1
  elseif m.direction == DIR_RIGHT then
    dx = -1
  end

  pl.player.x += dx
  pl.player.y += dy

  if m.crate_moved_index then
    local c = pl.crates[m.crate_moved_index]
    c.x += dx
    c.y += dy
    check_crate_on_storage(pl, c)
  end
end

local function animate_visuals(pl, dt)
  local ts = consts.TILE_SIZE
  local step = ts / consts.MOVE_ANIM_DURATION * dt
  local p = pl.player
  p.vx = util.approach(p.vx, p.x * ts, step)
  p.vy = util.approach(p.vy, p.y * ts, step)
  for _, c in ipairs(pl.crates) do
    c.vx = util.approach(c.vx, c.x * ts, step)
    c.vy = util.approach(c.vy, c.y * ts, step)
  end
end

function playable_level.update(pl, dt)
  animate_visuals(pl, dt)

  if input.pressed(input.BTN3) then
    playable_level.reset(pl)
    sfx.play(consts.SFX.RESET)
    return
  end

  if pl.complete then
    if input.pressed(input.BTN1) then
      pl.request_next_level = true
    end
    return
  end

  if pl.move_held_delay > 0 then
    pl.move_held_delay -= dt
  end

  if input.pressed(input.BTN2) then
    handle_rewind(pl)
  else
    handle_movement(pl)
  end
end

function playable_level.draw(pl)
  local ts = consts.TILE_SIZE
  local offset = {
    x = math.floor((usagi.GAME_W - pl.level.width * ts) / 2),
    y = math.floor((usagi.GAME_H - pl.level.height * ts) / 2),
  }
  -- nudge the playfield down a bit so the HUD title sits above it
  offset.y = math.max(offset.y, usagi.SPRITE_SIZE)

  level.draw(pl.level, offset)
  gfx.spr(consts.SPR.PLAYER, pl.player.vx + offset.x, pl.player.vy + offset.y)
  for _, c in ipairs(pl.crates) do
    local idx = c.on_storage and consts.SPR.CRATE_ON_STORAGE or consts.SPR.CRATE
    gfx.spr(idx, c.vx + offset.x, c.vy + offset.y)
  end

  gfx.text(pl.level.title, 4, 2, gfx.COLOR_WHITE)
  gfx.text("Steps: " .. pl.steps .. "  Pushes: " .. pl.pushes,
    4, 10, gfx.COLOR_LIGHT_GRAY)

  local hint = "Rewind:" .. input.mapping_for(input.BTN2) ..
      "  Reset:" .. input.mapping_for(input.BTN3)
  local _, hint_h = usagi.measure_text(hint)
  local hint_y = usagi.GAME_H - hint_h - 2
  gfx.text(hint, 4, hint_y, gfx.COLOR_PEACH)

  if pl.complete then
    local msg = "Nice job! Press " .. input.mapping_for(input.BTN1) .. " for next level."
    local mw, mh = usagi.measure_text(msg)
    gfx.text(msg, math.floor((usagi.GAME_W - mw) / 2), hint_y - mh - 2, gfx.COLOR_BLUE)
  end
end

return playable_level
