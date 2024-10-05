use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;

use lazy_static::lazy_static;
use maxminddb::geoip2::*;
use maxminddb::*;
use mlua::prelude::*;

lazy_static! {
    static ref database: Arc<Mutex<Option<Reader<Mmap>>>> = Arc::new(Mutex::new(None));
}

pub fn load(_lua: &Lua, filename: String) -> LuaResult<Option<String>> {
    let dbarc = Arc::clone(&database);
    let mut db = dbarc.lock().unwrap();
    *db = Some(Reader::open_mmap(filename).unwrap());
    Ok(None)
}

pub fn get_city(_lua: &Lua, ipstr: String) -> LuaResult<String> {
    let ipaddress: IpAddr = FromStr::from_str(ipstr.as_str()).unwrap();
    let dbarc = Arc::clone(&database);
    let db = dbarc.lock().unwrap();
    let c: City = db.as_ref().unwrap().lookup(ipaddress).unwrap();
    let city: &str = match c.city {
        Some(o) => match o.names.unwrap().get(&"en") {
            Some(oo) => oo,
            None => "",
        },
        None => "",
    };
    Ok(String::from(city))
}

pub fn get_country(_lua: &Lua, ipstr: String) -> LuaResult<String> {
    let ipaddress: IpAddr = FromStr::from_str(ipstr.as_str()).unwrap();
    let dbarc = Arc::clone(&database);
    let db = dbarc.lock().unwrap();
    let c: Country = db.as_ref().unwrap().lookup(ipaddress).unwrap();
    let country = match c.country {
        Some(o) => o.iso_code.unwrap(),
        None => "",
    };

    Ok(String::from(country))
}

#[mlua::lua_module]
fn libmmdb(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("load", lua.create_function(load)?)?;
    exports.set("get_city", lua.create_function(get_city)?)?;
    exports.set("get_country", lua.create_function(get_country)?)?;
    Ok(exports)
}
