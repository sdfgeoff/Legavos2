use std::io::Error;
use std::net::UdpSocket;
use std::time::Duration;

mod message;
pub use message::Message;

const TIMEOUT: Duration = Duration::new(1, 0); // 1 second

pub struct Network {
    pub hostname: String,
    pub socket: UdpSocket,
    pub recv_buf: [u8; 2048],
}

impl Network {
    pub fn new(hostname: String) -> Result<Self, Error> {
        let socket = UdpSocket::bind("[::]:0")?;
        socket.set_write_timeout(Some(TIMEOUT))?;
        socket.set_read_timeout(Some(TIMEOUT))?;

        Ok(Self {
            hostname,
            socket,
            recv_buf: [0; 2048],
        })
    }

    pub fn wait_for_message(&mut self) -> Result<Message, Error> {
        let (amt, _src) = self.socket.recv_from(&mut self.recv_buf)?;
        let new_message = Message::decode(&self.recv_buf[..amt])?;
        Ok(new_message)
    }

    pub fn send(&self, message: &Message) -> Result<(), Error> {
        self.socket.send_to(&message.encode(), &self.hostname)?;
        Ok(())
    }
}
