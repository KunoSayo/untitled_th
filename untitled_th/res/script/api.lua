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
    },
    textAlign = {
        left = {},
        right = {},
        top = {},
        bottom = {},
        hCenter = {},
        vCenter = {},
    }
}
--- @class UserDataHandle
local handle = {}

--- @class Progress
local progress = {}

--- 获得正在加载的数
function progress:loading()
end
--- 获得错误数
function progress:err()
end

--- @param text string the text to render
--- @param font UserDataHandle the font
--- @param x number the x to render
--- @param y number the y to render
--- @param bx number the bounding x
--- @param by number the bounding y
--- @param align UserDataHandle the align [LT MT RT .. ]
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