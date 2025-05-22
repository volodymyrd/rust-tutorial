-- ~/.config/nvim/lua/core/lsp.lua

-- Setup Mason
require("mason").setup()

-- Setup Mason LSP config
require("mason-lspconfig").setup({
  ensure_installed = { "rust_analyzer" },
})

-- Set up LSPConfig manually
local lspconfig = require("lspconfig")
local capabilities = require("cmp_nvim_lsp").default_capabilities()

-- Setup rust_analyzer
lspconfig.rust_analyzer.setup({
  capabilities = capabilities,
  settings = {
    ["rust-analyzer"] = {
      cargo = { allFeatures = true },
      checkOnSave = {
        command = "clippy",
      },
    },
  },
})

