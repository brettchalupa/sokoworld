-- Parses sokoban level data strings and draws static tiles.
--
-- Character legend:
--   #   wall
--   @   player
--   +   player on storage location
--   $   crate
--   *   crate on storage location
--   .   storage location
--   space / -  / _   ground (only after the first wall on the row)

local consts = require("consts")

local level = {}

local function clone_pos(p) return { x = p.x, y = p.y } end

-- Parses pack_level.data into a level table.
-- Returns: { title, walls, crates, storage_locations, grounds, player, width, height }
function level.parse(pack_level)
  local rows = {}
  for row in string.gmatch(pack_level.data, "([^\n]*)\n?") do
    table.insert(rows, row)
  end
  -- gmatch with that pattern emits a trailing empty match; drop trailing empties only
  while #rows > 0 and rows[#rows] == "" do
    table.remove(rows)
  end

  local walls = {}
  local crates = {}
  local storage_locations = {}
  local grounds = {}
  local player = { x = 0, y = 0 }
  local width = 0
  local height = #rows

  for y_idx, row in ipairs(rows) do
    local y = y_idx - 1
    local row_width = #row
    if row_width > width then width = row_width end

    local found_first_wall = false
    for x_idx = 1, #row do
      local x = x_idx - 1
      local c = string.sub(row, x_idx, x_idx)
      local pos = { x = x, y = y }

      if c == "#" then
        found_first_wall = true
        table.insert(walls, pos)
      elseif c == "@" then
        player = pos
        table.insert(grounds, clone_pos(pos))
      elseif c == "+" then
        table.insert(storage_locations, pos)
        player = clone_pos(pos)
      elseif c == "$" then
        table.insert(crates, pos)
        table.insert(grounds, clone_pos(pos))
      elseif c == "*" then
        table.insert(storage_locations, pos)
        table.insert(crates, clone_pos(pos))
      elseif c == "." then
        table.insert(storage_locations, pos)
      elseif c == " " or c == "-" or c == "_" then
        -- first row and first column never become ground; only after a wall has been seen on this row
        if found_first_wall and y ~= 0 and x ~= 0 then
          table.insert(grounds, pos)
        end
      else
        error("unexpected char '" .. c .. "' in level at " .. x .. ", " .. y)
      end
    end
  end

  return {
    title = pack_level.title,
    walls = walls,
    crates = crates,
    storage_locations = storage_locations,
    grounds = grounds,
    player = player,
    width = width,
    height = height,
  }
end

-- Draws the static elements of a level (walls, storage, ground).
-- offset is in pixels; tile positions are in grid cells.
function level.draw(lvl, offset)
  local ts = consts.TILE_SIZE
  for _, w in ipairs(lvl.walls) do
    gfx.spr(consts.SPR.WALL, w.x * ts + offset.x, w.y * ts + offset.y)
  end
  for _, s in ipairs(lvl.storage_locations) do
    gfx.spr(consts.SPR.STORAGE, s.x * ts + offset.x, s.y * ts + offset.y)
  end
  for _, g in ipairs(lvl.grounds) do
    gfx.spr(consts.SPR.GROUND, g.x * ts + offset.x, g.y * ts + offset.y)
  end
end

return level
