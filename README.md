# Rust REQREP

Rust REQREP message queue wrapper based on **Nanomsg**

## Why?
Nanomsg is faster brokerless Message Queue.
Currenly implemented `nanomsg.rs` wrapper for C librarry.

Easy to use common events handler for message queue.
It's possible serve events, send events, and handle events.
We find the way do it beauty, faster and efficiently.

## How to use
* `make deps` - build and install nanomsg lib
* `make build` - build debug version
* `make releast` - build debug version
* To run:
  * `make build` - build app
  * `target/debug/reqrep server tcp://0.0.0.0:3030` - run server
  * `target/debug/reqrep client tcp://0.0.0.0:3030` - run client  
* change Cargo.toml:
```toml
[dependencies]
nanomsg = "0.6.2"
```

## Status
Development stage

**Licence: MIT**