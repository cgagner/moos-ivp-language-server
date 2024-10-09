# Neovim

Neovim has an LSP client that works with the `moos-ivp-language-server`.
At the moment, the LSP client needs to be configured to automatically start
the language server when a MOOS file is opened. 

## Install `moos-ivp-language-server`

### Source Code

Building the `moos-ivp-lanugage-server` is the preferred method for
developers that want to take advantage of the latest features. It requires
Rust to be installed on the system. See 
[Getting Started](https://www.rust-lang.org/learn/get-started) for instructions
on installing Rust for your system.

Once Rust is installed, the `moos-ivp-language-server` can be installed using:

```bash
git clone git@github.com:cgagner/moos-ivp-language-server.git
cd moos-ivp-language-server
cargo install --path .
```

Verify `moos-ivp-language-server` is installed using:

```bash
moos-ivp-language-server --help
```

### Binary Install

The `moos-ivp-language-server` automatically generates binary installation
as part of the release process. Navigate to the
[Releases](https://github.com/cgagner/moos-ivp-language-server/releases)
page and select the binary file from the `Assets` section that matches your
operating system and architecture. 

Once downloaded, extract the executable into a directory in your PATH. For
example, on Linux and Mac OS systems `~/.local/bin`.

Example:

```bash
mkdir -p ~/.local/bin
cp ~/Downloads/moos-ivp-language-server ~/.local/bin
chmod 755 ~/.local/bin/moos-ivp-language-server
echo "export PATH=${PATH}:~/.local/bin" >> ~/.bashrc
source ~/.bashrc
```

Verify `moos-ivp-language-server` is installed using:

```bash
moos-ivp-language-server --help
```

## Neovim configuration

### New Neovim users - Kickstart

I have forked the [nvim-lua/kickstart.nvim](https://github.com/nvim-lua/kickstart.nvim)
repository and created the `moos-ivp` branch to provide a simple way to get
kickstarted with using Neovim and the `moos-ivp-language-server`. Use the
commands below to get started.

#### Linux and Mac

```bash
git clone -b moos-ivp https://github.com/cgagner/kickstart.nvim.git "${XDG_CONFIG_HOME:-$HOME/.config}"/nvim
```

#### Windows

Powershell

```bash
git clone -b moos-ivp https://github.com/cgagner/kickstart.nvim.git $env:USERPROFILE\AppData\Local\nvim\
```

### Manually modify `init.lua`

For more experience Neovim users, you can manually edit your 
`~/.config/nvim/init.lua` file to have the following:

```lua
      -- Associate file extensions for MOOS
      vim.filetype.add {
        extension = {
          moos = 'moos',
          bhv = 'moos',
          plug = 'moos',
          def = 'moos',
        },
      }

      -- Enable inlay hints
      vim.lsp.inlay_hint.enable(true)

      -- Create an event handler for the FileType autocommand
      vim.api.nvim_create_autocmd('FileType', {
        -- This handler will fire when the buffer's 'filetype' is "python"
        pattern = 'moos',
        callback = function(args)
          vim.lsp.start {
            name = 'moos-ivp-language-server',
            -- Make sure moos-ivp-language-server is in your path
            cmd = { 'moos-ivp-language-server' },
          }
        end,
      })
```
