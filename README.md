# memcache_test
Rust test project for memcache

This project is a simple introduction to running memcached on a local MacOS and
spinning up a simple client.  It is not intended to be comprehensive.

##.  Install memcached on local machine

_brew install memcached_

##.  Locate and start the memcached

_/usr/local/opt/memcached/bin/memcached -l localhost_

##.  Create a new cargo project (clone this repo or start from scratch)
You can clone this repo for the contents, but steps will be listed here anyway.

_cargo new memcache_test_

##.  Add dependency on memcache to the Cargo.toml [dependencies]
memcache = "*"

