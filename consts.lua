-- Shared constants for SokoWorld.

local consts = {}

consts.TILE_SIZE = usagi.SPRITE_SIZE

consts.MOVE_HELD_DELAY = 0.2

-- Seconds for the player / crates to slide between adjacent tiles.
-- Should stay below MOVE_HELD_DELAY so an auto-repeat never starts a
-- new slide while the previous one is still finishing.
consts.MOVE_ANIM_DURATION = 0.08

-- Sprite indices in sprites.png (1-based, matches gfx.spr).
consts.SPR = {
  CRATE = 1,
  CRATE_ON_STORAGE = 2,
  STORAGE = 3,
  PLAYER = 4,
  GROUND = 5,
  WALL = 6,
}

-- Sound effect names (file stems under usagi/sfx/).
consts.SFX = {
  PUSH = "dialogBlip2",
  FOOTSTEP = "footstep_4",
  LEVEL_COMPLETE = "get_item",
  CRATE_ON_STORAGE = "pieceSelect",
  CANT_MOVE = "pieceCantPlace",
  RESET = "save",
  MENU_MOVE = "menuMove",
  MENU_SELECT = "menuSelect",
  MENU_CANCEL = "menuCancel",
}

consts.VERSION = "0.3-dev"

return consts
