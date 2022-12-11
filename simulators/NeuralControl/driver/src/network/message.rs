use std::io::{Error, ErrorKind};

const MESSAGE_PING_CHAR: u8 = 0;
const MESSAGE_PONG_CHAR: u8 = 1;
const MESSAGE_STATE_CHAR: u8 = 2;
const MESSAGE_ACTION_CHAR: u8 = 3;

pub enum Message {
    Action(Vec<f32>),
    State(Vec<f32>),
    Ping,
    Pong,
}

impl Message {
    fn from_type_and_data(m_type: u8, data: Vec<f32>) -> Result<Self, Error> {
        match m_type {
            MESSAGE_PING_CHAR => Ok(Self::Ping),
            MESSAGE_PONG_CHAR => Ok(Self::Pong),
            MESSAGE_ACTION_CHAR => Ok(Self::Action(data)),
            MESSAGE_STATE_CHAR => Ok(Self::State(data)),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Unknown message type character",
            )),
        }
    }

    fn to_type_int(&self) -> u8 {
        match self {
            Self::Ping => MESSAGE_PING_CHAR,
            Self::Pong => MESSAGE_PONG_CHAR,
            Self::Action(_d) => MESSAGE_ACTION_CHAR,
            Self::State(_d) => MESSAGE_STATE_CHAR,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut outp = vec![];

        let dummy = vec![];
        let data: &Vec<f32> = match self {
            Self::Ping => &dummy,
            Self::Pong => &dummy,
            Self::Action(d) => d,
            Self::State(d) => d,
        };
        outp.push(self.to_type_int());
        outp.push(data.len().try_into().unwrap());
        outp.push(0);
        outp.push(0);

        for item in data.iter() {
            outp.extend(item.to_le_bytes());
        }
        outp
    }

    pub fn decode(data: &[u8]) -> Result<Message, Error> {
        if data.len() < 4 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Missing Header".to_string(),
            ));
        }
        let m_type = data[0];
        let len: usize = data[1].try_into().unwrap();

        if data.len() != len * 4 + 4 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Message Length Incorrect".to_string(),
            ));
        }

        let mut float_array: Vec<f32> = Vec::with_capacity(len);
        for i in 0..len {
            let offset: usize = 4 + 4 * i;
            let slice: [u8; 4] = data[offset..offset + 4].try_into().unwrap();
            float_array.push(f32::from_le_bytes(slice));
        }

        Self::from_type_and_data(m_type, float_array)
    }
}
