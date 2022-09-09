use mlua::prelude::{
    Lua,
    LuaTable,
    LuaResult
};

fn greet_people(lua: &Lua, names: Vec<String>) -> LuaResult<LuaTable> {
    let strings = lua.create_table()?;
    for (i, name) in names.into_iter().enumerate() {
        strings.raw_insert((i + 1).try_into().unwrap(), format!("Hello {}!", name))?;
    }
    Ok(strings)
}

fn my_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("greet_people", lua.create_function(greet_people)?)?;
    Ok(exports)
}
