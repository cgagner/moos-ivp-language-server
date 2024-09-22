# Neovim

Neovim has an LSP client that works with the `moos-ivp-language-server`.
At the moment, the LSP client needs to be configured to automatically start
the language server when a MOOS file is opened. 

## Installing `moos-ivp-language-server`

### Pre-built binary

TODO: Use GitHub Actions to publish the binary artifacts.

### Install from Cargo

TODO: Need to publish `moos-ivp-language-server` to allow `cargo install`
      to work for Rust users.

```bash
cargo install moos-ivp-language-server
```

### Install from source

TODO: This cannot be completed until the `moos-rs` refactor has been
completed because we need to be able to find the `moos-parsers` crate.

```bash
cd moos-ivp-language-server
cargo install --path .
```

## Neovim configuration

### New Neovim users

TODO: Fork the Neovim kickstart repository and include instructions here
for cloning and setup.

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