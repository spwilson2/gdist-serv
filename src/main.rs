extern crate clap;
use clap::{App, SubCommand};

extern crate tokio;
use std::net::SocketAddr;
use std::str::FromStr;

extern crate futures_util;
// use futures_util::try_future::TryFutureExt;

mod server;
mod client;


fn main() {
  let matches = App::new("gdist")
    .version("0.1.0")
    .subcommand(
      SubCommand::with_name("client")
        .arg_from_usage("-s, --server <server>")
        .arg_from_usage("-c, --client [client]"),
    )
    .subcommand(SubCommand::with_name("server").arg_from_usage("-s, --server <server>"))
    .get_matches();

  // TODO - Connection Layer

  // TODO: Parse cli args - Decide if server or client

  match matches.subcommand() {
    ("client", Some(_sub_m)) => client::main(),
    ("server", Some(sub_m)) => {
     server::main(SocketAddr::from_str(sub_m.value_of("server").unwrap()).unwrap());
    }
    _ => {}
  };
}
