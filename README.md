# astrum
a desktop shell that depends on libcosmic

---
to run it, run `nix-shell` first then `cargo run`

that should make a configuration file `~/.config/astrum/config.lua`

within the lua configuration it depends on [`neoconf`](https://github.com/folke/neoconf.nvim.git) to automatically add `lua_ls` bindings
you probably can do it within vscode manually too, i dont use vscode so i dont know how hard it is

