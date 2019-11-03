pub const MIN_PACKET_SIZE: usize = std::mem::size_of::<ConnectionMessage>();
pub const MAX_PACKET_SIZE: usize = 0x1000;

pub const VERSION: u64 = 0x00001000;
pub const MAGIC: u64 = 0x00001000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ConnectionMessageType {
  RegisterRequest,
  RegisterResponse,
  RegisterResponseAck,

  Heartbeat,
  Datagram,
  DatagramAck,
  Disconnect,
  DisconnectAck,

  Invalid,
}

#[repr(C)]
pub struct ConnectionMessageHeader {
  pub version: u64,
  pub magic: u64,
}

impl Default for ConnectionMessageHeader {
  fn default() -> ConnectionMessageHeader {
    ConnectionMessageHeader {
      version: VERSION,
      magic: MAGIC,
    }
  }
}

#[repr(C)]
pub struct ConnectionMessage {
  pub header: ConnectionMessageHeader,
  pub contents: ConnectionMessageContents,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct ConnectionClientID {
  id : usize,
}

impl ConnectionClientID {
  pub const INVALID :ConnectionClientID =  ConnectionClientID { id : 0 };
}
impl ConnectionMessageType {
  pub fn is_invalid(&self) -> bool {

    return unsafe { *(&self as *const _ as *const i32) >= ConnectionMessageType::Invalid as i32};
  }
}

pub struct ConnectionMessageContentsHeader {
  pub size: usize,
  pub id: ConnectionMessageType,
  pub client_id: ConnectionClientID,
}

#[repr(C)]
pub struct ConnectionMessageContents {
  pub header: ConnectionMessageContentsHeader,
  pub data: [u8; MAX_PACKET_SIZE],
}

pub struct ConnectionMessageCtorArgs {
  client_id: ConnectionClientID, 
  message_type: ConnectionMessageType,  
  data: [u8]
}

impl ConnectionMessage {

  /// Construct a ConnectionMessage.
  /// Note: If the intention is to then write the message to a buffer,
  /// it would be less memory intensive to use the write_buf method instead.
  pub fn new(args: &ConnectionMessageCtorArgs) -> ConnectionMessage {
    assert!(args.data.len() <= MAX_PACKET_SIZE);
    
    let mut msg = ConnectionMessage {
      header: ConnectionMessageHeader::default(),
      contents: ConnectionMessageContents {
        header: ConnectionMessageContentsHeader {
              size: args.data.len(),
              client_id: args.client_id,
              id: args.message_type,
        },
        data: [0u8; MAX_PACKET_SIZE],
      }
    };

    unsafe {
      std::ptr::copy_nonoverlapping(args.data.as_ptr(), 
        msg.contents.data.as_mut_ptr(), args.data.len());
    }
    return msg;
  }

  pub fn buf_size(args: &ConnectionMessageCtorArgs) -> usize {
    return args.data.len() + std::mem::size_of::<ConnectionMessageContentsHeader>();
  }

  /// Construct a ConnectionMessage in the given mutable buffer
  pub fn write_buf(args: &ConnectionMessageCtorArgs, buf: &mut [u8]) -> Result<(), usize> {
    let size = ConnectionMessage::buf_size(&args);
    if buf.len() < size {
      return Err(size);
    }

    unsafe {
      let mut msg : &mut ConnectionMessage = &mut *(buf.as_mut_ptr() as usize as *mut ConnectionMessage);
      msg.header = ConnectionMessageHeader::default();
      msg.contents.header = ConnectionMessageContentsHeader {
              size: args.data.len(),
              client_id: args.client_id,
              id: args.message_type,
        };

      std::ptr::copy_nonoverlapping(args.data.as_ptr(), 
        msg.contents.data.as_mut_ptr(), args.data.len());
    }
    return Ok(());
  }
}


#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn check_dumb_compiler() {
    let buf = vec![0; 1024];

    let msg: ConnectionMessage = unsafe { std::ptr::read(buf.as_ptr() as *const _) };

    unsafe {
      // offset_of
      assert_eq!(std::mem::size_of::<ConnectionMessageType>(), std::mem::size_of::<u32>());
      *(&msg.contents.id  as *const _ as usize as *mut u32)  = ConnectionMessageType::Invalid as u32 + 1;
    }

    // NOTE: The compiler could still do something funky here if it's "smart"
    // enough to think that the id could never be greater than Invalid.
    if msg.contents.id.is_invalid() {
      println!("Dropped message due to bad type");
      return;
    }

    panic!("Message should have been dropped with an enum ID >= invalid")
  }

  #[test]
  fn invalid_id_equal() {
    assert_eq!(ConnectionClientID::INVALID, ConnectionClientID {id : 0})
  }
}
