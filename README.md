# Taste For Neovim Plugin With Rust

## Configuration

```lua
{
    "ritzier/boo",
    build = "bash ./install.sh",
    lazy = false,
    config = function()
        require("boo")
    end,
}
```
