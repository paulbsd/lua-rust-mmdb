#!/usr/bin/env luajit

local res = {
  city = {},
  country = {},
}

function stats(res, city, country)
  res.city[city] = (res.city[city] or 0) + 1
  res.country[country] = (res.country[country] or 0) + 1
  return res
end

function main()
  local filename = "test/ip.csv"
  local mmdb = require("include")
  mmdb.load("data/GeoIP-City.mmdb")

  local i = 0
  for ip in io.lines(filename) do
    local city = mmdb.get_city(ip) or "not found"
    local country = mmdb.get_country(ip) or "not found"
    res = stats(res, city, country)
    --local s = string.format("%s: city:%s country:%s", ip, city, country)
    --print(s)
    i=i+1
    --if i > 100 then break end
  end
  for code, count in pairs(res.country) do
    print(string.format("code:%s count:%s", code, count))
  end
end

main()
