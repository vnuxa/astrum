since this project is still a work in progress,
you have to build it from source.

---

if you use nix you can add this as a nix flake input
```nix
inputs = {
    astrum.url = "github:vnuxa/astrum"
}
```
and somewhere in your package list add the following
```nix
    astrum.defaultPackage.${pkgs.system}
```


---
I also recommend having the [lua language server](https://github.com/LuaLS/lua-language-server) installed in your text editor, since it contains astrum types and descriptions
