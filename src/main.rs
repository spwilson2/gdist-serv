
extern crate clap;
use clap::{Arg, App, SubCommand};

// extern crate tokio;
// use tokio::net::UdpSocket;
use std::net::SocketAddr;
// use tokio::prelude::*;
// use tokio::runtime::Runtime;

use std::str::FromStr;

fn client_main() {

}

// async fn wait(mut s: UdpSocket) -> ! {
   
//     loop {
//         //let mut addr : SocketAddr;
//         let mut buf = vec![0; 1024];
//         let rs = s.recv_from(buf.as_mut_slice()).await;
//         let (_size, addr) = rs.unwrap();
//         s.send_to(buf.as_slice(),  &addr).await.unwrap();
//         println!("Echo'd response {:?}", &buf);
//     }
// }

fn server_main(addr :&SocketAddr) {

    // let addr: SocketAddr = "127.0.0.1:6667".parse().unwrap();
    // let listener = UdpSocket::bind(&addr).then(|socket| {
    //     wait(socket.unwrap())
    // });

    // let rt = Runtime::new().unwrap();
    // rt.block_on(listener)

}

fn main() {

  let matches = App::new("gdist")
    .version("0.1.0")
    .subcommand(SubCommand::with_name("client")
      .arg_from_usage("-s, --server <server>")
      .arg_from_usage("-c, --client [client]"))
    .subcommand(SubCommand::with_name("server")
      .arg_from_usage("-s, --server <server>")).get_matches();

  // TODO - Connection Layer

  // TODO: Parse cli args - Decide if server or client

  match matches.subcommand() {
    ("client", Some(sub_m)) => {client_main()},
    ("server", Some(sub_m)) => {
      server_main(&SocketAddr::from_str(sub_m.value_of("server").unwrap()).unwrap());
      },
    _ => {}
  };
}