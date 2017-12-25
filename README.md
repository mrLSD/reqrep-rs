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
* change Cargo.toml:
```toml
[dependencies]
nanomsg = "0.6.2"
```

## Status
Development stage

**Licence: MIT**