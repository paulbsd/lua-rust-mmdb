use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::RwLock;

use lazy_static::lazy_static;
use maxminddb::geoip2::*;
use maxminddb::*;
use mlua::prelude::*;

lazy_static! {
    static ref database: Arc<RwLock<Option<Reader<Mmap>>>> = Arc::new(RwLock::new(None));
}

pub fn load(_lua: &Lua, filename: String) -> LuaResult<Option<String>> {
    let dbarc = Arc::clone(&database);
    let mut db = dbarc.write().unwrap();
    *db = Some(Reader::open_mmap(filename).unwrap());
    Ok(None)
}

pub fn get_postal(_lua: &Lua, ipstr: Option<String>) -> LuaResult<Option<String>> {
    let ipaddress: IpAddr = match parse_ip(ipstr) {
        Ok(o) => o,
        Err(_) => return Ok(None),
    };
    let mut postal: Option<String> = None;

    let dbarc = Arc::clone(&database);
    let db = dbarc.read().unwrap();
    match db.as_ref().unwrap().lookup::<City>(ipaddress) {
        Ok(citylookup) => {
            postal = match citylookup.postal {
                Some(o) => match o.code {
                    Some(oo) => Some(oo.to_string()),
                    None => None,
                },
                None => None,
            }
        }
        Err(_) => {}
    };
    Ok(postal)
}

pub fn get_continent(_lua: &Lua, ipstr: Option<String>) -> LuaResult<Option<String>> {
    let ipaddress: IpAddr = match parse_ip(ipstr) {
        Ok(o) => o,
        Err(_) => return Ok(None),
    };
    let mut continent: Option<String> = None;

    let dbarc = Arc::clone(&database);
    let db = dbarc.read().unwrap();
    match db.as_ref().unwrap().lookup::<City>(ipaddress) {
        Ok(citylookup) => {
            continent = match citylookup.continent {
                Some(o) => match o.names.unwrap().get(&"en") {
                    Some(oo) => Some(oo.to_string()),
                    None => None,
                },
                None => None,
            }
        }
        Err(_) => {}
    };
    Ok(continent)
}

pub fn get_city(_lua: &Lua, ipstr: Option<String>) -> LuaResult<Option<String>> {
    let ipaddress: IpAddr = match parse_ip(ipstr) {
        Ok(o) => o,
        Err(_) => return Ok(None),
    };
    let mut city: Option<String> = None;

    let dbarc = Arc::clone(&database);
    let db = dbarc.read().unwrap();
    match db.as_ref().unwrap().lookup::<City>(ipaddress) {
        Ok(citylookup) => {
            city = match citylookup.city {
                Some(o) => match o.names.unwrap().get(&"en") {
                    Some(oo) => Some(oo.to_string()),
                    None => None,
                },
                None => None,
            }
        }
        Err(_) => {}
    };
    Ok(city)
}

pub fn get_subdivisions(_lua: &Lua, ipstr: Option<String>) -> LuaResult<Option<Vec<String>>> {
    let ipaddress: IpAddr = match parse_ip(ipstr) {
        Ok(o) => o,
        Err(_) => return Ok(None),
    };
    let mut subdivisions: Option<Vec<String>> = None;

    let dbarc = Arc::clone(&database);
    let db = dbarc.read().unwrap();
    match db.as_ref().unwrap().lookup::<City>(ipaddress) {
        Ok(citylookup) => {
            subdivisions = match citylookup.subdivisions {
                Some(o) => {
                    let mut s: Vec<String> = vec![];
                    for i in o {
                        s.push(i.iso_code.unwrap().into());
                    }
                    Some(s)
                }
                None => None,
            }
        }
        Err(_) => {}
    };

    Ok(subdivisions)
}

pub fn get_country(_lua: &Lua, ipstr: Option<String>) -> LuaResult<Option<String>> {
    let ipaddress: IpAddr = match parse_ip(ipstr) {
        Ok(o) => o,
        Err(_) => return Ok(None),
    };
    let mut countrycode: Option<String> = None;

    let dbarc = Arc::clone(&database);
    let db = dbarc.read().unwrap();
    match db.as_ref().unwrap().lookup::<Country>(ipaddress) {
        Ok(citylookup) => {
            countrycode = match citylookup.country {
                Some(o) => Some(o.iso_code.unwrap().to_string()),
                None => None,
            }
        }
        Err(_) => {}
    };

    Ok(countrycode)
}

#[derive(Debug, Clone)]
struct ParseError;

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "unable to parse ip address")
    }
}

fn parse_ip(ipstr: Option<String>) -> Result<IpAddr, ParseError> {
    let ipaddress: IpAddr;
    if let None = ipstr {
        return Err(ParseError);
    }

    match IpAddr::from_str(ipstr.unwrap_or(String::new()).as_str()) {
        Ok(o) => ipaddress = o,
        Err(_) => return Err(ParseError),
    };

    Ok(ipaddress)
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
            exports.set("get_subdivisions", lua.create_function(get_subdivisions)?)?;
            exports.set("get_continent", lua.create_function(get_continent)?)?;
            exports.set("get_postal", lua.create_function(get_postal)?)?;
            Ok(exports)
        }
    };
}
