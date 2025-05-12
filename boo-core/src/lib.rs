use nvim_oxi::Object;

pub fn setup(opts: Option<Object>) {}

pub fn hi(opts: Option<Object>) {
    use mlua::prelude::LuaFunction;
    use nvim_oxi::{mlua, print};

    print!("Hello from nvim-oxi..");
    let lua = mlua::lua();
    let print: LuaFunction = lua.globals().get("print").unwrap();
    print.call::<_, ()>("..and goodbye from mlua!").unwrap();
}
