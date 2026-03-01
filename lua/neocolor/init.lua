local M = {}

local state = {
  initialized = false,
}

local function init_native()
  if state.initialized then
    return
  end

  local ok, lib = pcall(require, "neocolor_lib")
  if not ok then
    vim.schedule(function()
      vim.notify("neocolor: failed to load neocolor_lib: " .. tostring(lib), vim.log.levels.ERROR)
    end)
    return
  end

  lib.setup()
  lib.update()
  state.initialized = true
end

M.setup = function(opts)
  opts = opts or {}

  if opts.defer_setup == false then
    init_native()
    return
  end

  vim.schedule(init_native)
end

return M
