local menu = {}
local function render()

end

local function update()
    print("test")
    return 0, uth.menu.loopState.waitAll
end

menu.render = render
menu.update = update

return menu