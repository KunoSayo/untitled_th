local root = {
    menu = {
        tran = {
            exit = -1
        },
        loopState = {
            waitAll = 0,
            wait = 0,
            poll = 0,
            pollNoRender = 0,
        }
    }
}
---@class UserDataHandle
local handle = {}

--- @param text string the text to render
--- @param font UserDataHandle the font
--- @param x number the x to render
--- @param y number the y to render
--- @param bx number the bounding x
--- @param by number the bounding y
--- @param align string the align [LT MT RT .. ]
function root.drawText(text, font, x, y, bx, by, align)
end

function root.createProgress()
end

function root.loadTexture(id, path, progress)
end

--- @param id string font id
function root.getFont(id)
end

_G.uth = root