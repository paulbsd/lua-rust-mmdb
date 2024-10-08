## lua-rust-mmdb

### Products that can use

* HAProxy (with mlua's lua5x features)
* dnsdist (with mlua's luajit feature)

### Examples

see examples:
* with [haproxy](examples/haproxy)
* with [dnsdist](examples/dnsdist)

### Build

* for lua5x: `cargo b -r -p lua5x_rust_mmdb`
* for luajit: `cargo b -r -p luajit_rust_mmdb`

## License

```
           DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
                   Version 2, December 2004
 
Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>

Everyone is permitted to copy and distribute verbatim or modified
copies of this license document, and changing it is allowed as long
as the name is changed.
 
           DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
  TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

 0. You just DO WHAT THE FUCK YOU WANT TO.
```