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

pub fn get_city(_lua: &Lua, ipstr: Option<String>) -> LuaResult<Option<String>> {
    let ipaddress: IpAddr;
    let city: Option<String>;
    if let None = ipstr {
        return Ok(None);
    }
    match IpAddr::from_str(ipstr.unwrap().as_str()) {
        Ok(o) => ipaddress = o,
        Err(_) => return Ok(None),
    };
    let dbarc = Arc::clone(&database);
    let db = dbarc.lock().unwrap();
    let c: City = db.as_ref().unwrap().lookup(ipaddress).unwrap();
    city = match c.city {
        Some(o) => match o.names.unwrap().get(&"en") {
            Some(oo) => Some(oo.to_string()),
            None => None,
        },
        None => None,
    };
    Ok(city)
}

pub fn get_country(_lua: &Lua, ipstr: Option<String>) -> LuaResult<Option<String>> {
    let ipaddress: IpAddr;
    let countrycode: Option<String>;
    if let None = ipstr {
        return Ok(None);
    }

    match IpAddr::from_str(ipstr.unwrap().as_str()) {
        Ok(o) => ipaddress = o,
        Err(_) => return Ok(None),
    };
    let dbarc = Arc::clone(&database);
    let db = dbarc.lock().unwrap();
    let c: Country = db.as_ref().unwrap().lookup(ipaddress).unwrap();
    countrycode = match c.country {
        Some(o) => Some(o.iso_code.unwrap().to_string()),
        None => None,
    };

    Ok(countrycode)
}

#[macro_export]
macro_rules! create_functions {
    ($fn_name: ident) => {
        #[mlua::lua_module]
        fn $fn_name(lua: &Lua) -> LuaResult<LuaTable> {
            let exports = lua.create_table()?;
            exports.set("load", lua.create_function(load)?)?;
            exports.set("get_city", lua.create_function(get_city)?)?;
            exports.set("get_country", lua.create_function(get_country)?)?;
            Ok(exports)
        }
    };
}
