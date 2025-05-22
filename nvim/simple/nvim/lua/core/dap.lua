-- core/dap.lua
local rt = require("rust-tools")
rt.setup({
  dap = {
    adapter = rt.dap.get_codelldb_adapter(
      "/path/to/codelldb",
      "/path/to/lldb/lib/liblldb.so"
    ),
  },
})

