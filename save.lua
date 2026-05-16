-- Tracks per-level completions, persisted via usagi.save / usagi.load.
--
-- Save shape:
--   {
--     completions = {
--       ["<pack_slug>:<level_title>"] = { steps = N, pushes = N },
--       ...
--     }
--   }

local save = {}

local function key(pack_slug, level_title)
  return pack_slug .. ":" .. level_title
end

function save.load()
  local data = usagi.load() or {}
  if type(data.completions) ~= "table" then
    data.completions = {}
  end
  return data
end

function save.is_complete(data, pack_slug, level_title)
  return data.completions[key(pack_slug, level_title)] ~= nil
end

function save.mark_complete(data, pack_slug, level_title, steps, pushes)
  data.completions[key(pack_slug, level_title)] = {
    steps = steps,
    pushes = pushes,
  }
  usagi.save(data)
end

function save.count_complete(data, pack)
  local n = 0
  for _, lvl in ipairs(pack.levels) do
    if save.is_complete(data, pack.slug, lvl.title) then
      n = n + 1
    end
  end
  return n
end

return save
