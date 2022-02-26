--- @param text string the text to render
--- @param font string the font id
--- @param x number the x to render
--- @param y number the y to render
--- @param bx number the bounding x
--- @param by number the bounding y
--- @param align string the align [LT MT RT .. ]
function drawText(text, font, x, y, bx, by, align)
end

function createProgress()
end

function loadTexture(id, path, progress)
end

uth = {}
uth.drawText = drawText
uth.createProgress = createProgress
uth.loadTexture = loadTexture

uth = {
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