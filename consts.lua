-- Shared constants for SokoWorld.

local consts = {}

consts.TILE_SIZE = usagi.SPRITE_SIZE

consts.MOVE_HELD_DELAY = 0.2

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
