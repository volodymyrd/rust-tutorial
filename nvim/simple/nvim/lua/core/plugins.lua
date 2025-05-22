-- core/plugins.lua
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
  vim.fn.system({
    "git", "clone", "--filter=blob:none",
    "https://github.com/folke/lazy.nvim", lazypath
  })
end
vim.opt.rtp:prepend(lazypath)

require("lazy").setup({
  -- Plugin list
  "nvim-lua/plenary.nvim",
  "nvim-telescope/telescope.nvim",

  -- Colorscheme
  "folke/tokyonight.nvim",

  -- LSP Support
  "neovim/nvim-lspconfig",
  "williamboman/mason.nvim",
  "williamboman/mason-lspconfig.nvim",

  -- Autocompletion
  "hrsh7th/nvim-cmp",
  "hrsh7th/cmp-nvim-lsp",
  "L3MON4D3/LuaSnip",
  "saadparwaiz1/cmp_luasnip",

  -- Treesitter for syntax highlighting
  "nvim-treesitter/nvim-treesitter",

  -- Debugger
  "mfussenegger/nvim-dap",
  "rcarriga/nvim-dap-ui",

  -- Rust support
  "simrat39/rust-tools.nvim",
})

require'nvim-treesitter.configs'.setup {
  ensure_installed = { "lua", "rust", "toml" },
  highlight = { enable = true },
}

