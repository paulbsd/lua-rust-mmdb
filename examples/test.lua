#!/usr/bin/env luajit

package.path = package.path .. ";./target/release/?.lua"
package.cpath = package.cpath .. ";./target/release/?.so"

local mmdb = require("libmmdb")

mmdb.load("./GeoLite2-City.mmdb")

local ip = "73.95.4.91"
local ci = mmdb.get_city(ip)
local co = mmdb.get_country(ip)
print(string.format("ip: %s, city: %s, country: %s",ip,ci,co))