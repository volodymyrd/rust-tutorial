# Most Popular Neovim (nvim) Commands

Neovim is a powerful, extensible text editor. Below is a list of popular and essential commands frequently used by both
new and experienced users.

---

## 🗂 File Operations

| Command             | Description                 |
|---------------------|-----------------------------|
| `:e <filename>`     | Edit/Open a file            |
| `:w`                | Save (write) file           |
| `:w <filename>`     | Save file as                |
| `:q`                | Quit                        |
| `:q!`               | Quit without saving         |
| `:wq` or `ZZ`       | Save and quit               |
| `:x`                | Save and quit (if modified) |
| `:Ex` or `:Explore` | Open file explorer          |   
| `%`                 | New file                    |  
| `d`                 | New directory               | 
| `t`                 | Open file in new tab        | 
| `gt`                | Switch tabs                 | 
| `e .`               | Sinc with current directory | 

---

## 🧭 Navigation

| Command             | Description                    |
|---------------------|--------------------------------|
| `h`, `j`, `k`, `l`  | Move left, down, up, right     |
| `gg`                | Go to beginning of file        |
| `G`                 | Go to end of file              |
| `:n`                | Go to line `n` (e.g., `:25`)   |
| `H`, `M`, `L`       | Top, Middle, Bottom of screen  |
| `Ctrl-d` / `Ctrl-u` | Scroll half-page down/up       |
| `%`                 | Jump to matching bracket/brace |

---

## ✍️ Editing

| Command                             | Description                                                         |
|-------------------------------------|---------------------------------------------------------------------|
| `i`, `a`, `o`                       | Insert modes (insert before, append after, open new line)           |
| `x`                                 | Delete character under cursor                                       |
| `dd`                                | Delete (cut) a line                                                 |
| `yy`                                | Yank (copy) a line                                                  |
| `p`, `P`                            | Paste after / before cursor                                         |
| `u`                                 | Undo                                                                |
| `Ctrl-r`                            | Redo                                                                |
| `cw`, `c$`, `ciw`                   | Change word / to end of line / inside word                          |
| `v`, `V`, `Ctrl-v`                  | Visual Mode (character / line / block selection)                    |
| `d` (in visual mode)                | Delete selected text or lines                                       |
| `y` (in visual mode)                | Yank selected text                                                  |
| `$` (in visual block)               | Extend block selection to end of each line                          |
| `Shift-i` (in block)                | Insert before selected block columns                                |
| `r<char>`                           | Replace character under cursor                                      |
| `c` (in visual mode)                | Change selected text                                                |
| `:` after visual block              | Use ex command on selected block/lines (e.g., `:d` to delete lines) |
| `V` + `d`                           | Delete full lines                                                   |
| `Ctrl-v` + `$` + `d`                | Delete from block column to end of line on each selected line       |
| `:10,20d`                           | Delete lines 10 through 20                                          |
| `Ctrl-v` + `Shift-i` + text + `Esc` | Insert text at beginning of all selected lines                      |

---

## 🔍 Search and Replace

| Command         | Description                 |
|-----------------|-----------------------------|
| `/pattern`      | Search forward              |
| `?pattern`      | Search backward             |
| `n` / `N`       | Repeat search next/previous |
| `:%s/old/new/g` | Replace all in file         |
| `:s/old/new/g`  | Replace all in current line |

---

## 🧠 Buffers, Windows & Tabs

| Command               | Description                          |
|-----------------------|--------------------------------------|
| `:ls` or `:buffers`   | List open buffers                    |
| `:b <num>`            | Switch to buffer                     |
| `:split` or `:vsplit` | Split window horizontally/vertically |
| `Ctrl-w h/j/k/l`      | Move between splits                  |
| `:tabnew`             | Open new tab                         |
| `gt`, `gT`            | Next/previous tab                    |

---

## ⚙️ Useful Misc

| Command               | Description                         |
|-----------------------|-------------------------------------|
| `:help <topic>`       | Open help for a topic               |
| `:noh`                | Remove search highlight             |
| `:set number`         | Show line numbers                   |
| `:set relativenumber` | Show relative line numbers          |
| `:syntax on`          | Enable syntax highlighting          |
| `:checkhealth`        | Check nvim configuration and health |

---

## 🧩 Plugin Shortcuts (Assuming Common Plugins)

| Command                 | Description                         |
|-------------------------|-------------------------------------|
| `:PackerSync`           | Sync plugins with packer.nvim       |
| `:Telescope find_files` | Search files (with Telescope)       |
| `:NvimTreeToggle`       | Toggle file tree (with NvimTree)    |
| `:LspInfo`              | Show active LSP servers             |
| `gd`, `gr`              | Go to definition / references (LSP) |
| `[d`, `]d`              | Previous/next diagnostic (LSP)      |

---

## 🏁 Exiting Insert Mode

| Key      | Description         |
|----------|---------------------|
| `Esc`    | Exit insert mode    |
| `Ctrl-[` | Equivalent to `Esc` |

---

## 🖥 Running Terminal Commands in Neovim

### 🔸 Method 1: `:!` — Run One-off Shell Commands

```vim
:!ls
```

### 🔸 Method 2: `:terminal` — Open Interactive Terminal

```vim
:terminal
```

* Opens a full terminal buffer inside Neovim.
* Use i to enter terminal mode.
* Use Ctrl-\ Ctrl-n to return to normal mode.

### 🔸 Method 3: Terminal in a Split

```vim
:split | terminal
:vsplit | terminal
:belowright 20split | terminal
```

Happy editing with Neovim! 🚀
