local mmdb = require("libmmdblua5x")

local args = table.pack(...)
mmdb.load(args[1])

local function country(txn, var1)
  local src = txn:get_var(var1)
  return mmdb.get_country(src)
end

local function city(txn, var1)
  local src = txn:get_var(var1)
  return mmdb.get_city(src)
end

core.register_fetches("country", country)
core.register_fetches("city", city)
