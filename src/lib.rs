#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate ws;
extern crate lovesense;
extern crate serde_json;
extern crate bytes;
extern crate mio;
#[macro_use] extern crate log;
extern crate env_logger;

use config::{Config};

mod local_server;
pub mod buttplug_server;
pub mod messages;
pub mod config;

pub fn start_server(config: Config,
                    local_server_loop: Option<mio::EventLoop<local_server::LocalServer>>)
{
    // Before doing anything, let us register a logger. The mio library has really good logging
    // at the _trace_ and _debug_ levels. Having a logger setup is invaluable when trying to
    // figure out why something is not working correctly.
    env_logger::init().ok().expect("Failed to init logger");
    buttplug_server::start_server(config, local_server_loop);
}

#[cfg(test)]
mod tests {
}
