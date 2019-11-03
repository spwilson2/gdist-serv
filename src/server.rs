extern crate tokio;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::runtime::Runtime;

use std::mem::size_of;

extern crate futures_util;

extern crate gdist_common;
use gdist_common::*;

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

struct Server {
    addr: SocketAddr,
}

fn create_message() {

}

impl Server {
    fn handle_connection_request(&mut self, msg: &ConnectionMessage) {
        assert!(msg.contents.header.id == ConnectionMessageType::RegisterRequest);
        // Verify that data is empty
        if msg.contents.header.size != 0 {
            println!("{} dropped packet, size != 0", line!());
            return;
        }
        // Verify that the ID is 0
        if msg.contents.header.client_id == ConnectionClientID::INVALID {
            println!("{}: dropped packet", line!());
            return;
        }

        // Create a new client ID, respond with it

        let _new_id = self.generate_id();

        // self.respond(&msg, )
        // let 
    }

    fn generate_id(&mut self) {
        unimplemented!();
    }
    async fn respond(&mut self, buf: &[u8]) {
        unimplemented!();
    }

    async fn runloop(&mut self) {
        let mut socket = UdpSocket::bind(&self.addr).await.unwrap();
        let mut buf = vec![0; 1024];

        loop {
            // Wait for receipt of packets.
            let response = socket.recv_from(&mut buf).await;
            let (size, _addr) = response.unwrap();

            // IF packet not of OK size/fmt ignore it
            if size < MIN_PACKET_SIZE {
                continue;
            }

            // WARN: This shadows ownership of buf.
            let msg: ConnectionMessage = unsafe { std::ptr::read(buf.as_ptr() as *const _) };
            if msg.header.magic != MAGIC {
                println!("Dropped message due to bad magic");
            } else if msg.header.version != VERSION {
                println!("Dropped message due to bad version");
                continue;
            } else if msg.contents.header.size != (size - size_of::<ConnectionMessageHeader>()) {
                continue;
            }

            // NOTE: The compiler could still do something funky here if it's "smart"
            // enough to think that the id could never be greater than Invalid.
            if msg.contents.header.id.is_invalid() {
                println!("Dropped message due to bad type");
                continue;
            }

            match msg.contents.header.id {
                ConnectionMessageType::RegisterRequest => self.handle_connection_request(&msg),
                ConnectionMessageType::RegisterResponse => {}
                ConnectionMessageType::RegisterResponseAck => {}
                ConnectionMessageType::Heartbeat => {}
                ConnectionMessageType::Datagram => {}
                ConnectionMessageType::DatagramAck => {}
                ConnectionMessageType::Disconnect => {}
                ConnectionMessageType::DisconnectAck => {}
                _ => unreachable!(
                "Message with invalid ID received. It should have been dropped before this point."
            ),
            }
        }
    }
}

pub fn main(addr: SocketAddr) {
    let rt = Runtime::new().unwrap();
    let mut server = Server {addr};
    rt.block_on(server.runloop());
}
