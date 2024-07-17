use rglua::{lua::{lua_getfield, lua_getmetatable, lua_pop, lua_rawequal, lua_toboolean, LuaString, Userdata, REGISTRYINDEX}, prelude::{lua_newuserdata, lua_touserdata, LuaState}};

pub fn write_userdata<T>(l: LuaState, data: T) {
    let data_ptr = lua_newuserdata(l, std::mem::size_of::<T>()) as *mut T;
    unsafe {
        std::ptr::write(data_ptr, data);
    }
}

pub fn read_userdata<T: Clone>(l: LuaState) -> Result<T, String> {
    let data_ptr = lua_touserdata(l, 1) as *mut T;
    if data_ptr.is_null() {
        Err("Invalid userdata.".to_string())
    } else {
        Ok(unsafe { (*data_ptr).clone() })
    }
}

pub fn check_userdata<T: Clone>(l: LuaState, idx: i32, key: LuaString) -> Result<T, String> {
    let udata = lua_touserdata(l, idx);
    if udata.is_null() {
        return Err("Invalid userdata".to_string());
    }

    if lua_getmetatable(l, idx) == 0 {
        return Err("Userdata without metatable".to_string());
    }

    lua_getfield(l, REGISTRYINDEX, key);

    if lua_rawequal(l, -1, -2) == 1 {
        lua_pop(l, 2);
        Ok(unsafe { (*( (*(udata as *mut Userdata)).data as *mut T)).clone() })
    } else {
        lua_pop(l, 2);
        Err("Not rawequal".to_string())
    }
}

pub fn read_boolean(l: LuaState, idx: i32) -> bool { 
    return lua_toboolean(l, idx) == 1;
}