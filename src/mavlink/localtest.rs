// This file was automatically generated, do not edit
#[allow(unused_imports)]
use crate::mavlink::common::*;
#[allow(unused_imports)]
use bitflags::bitflags;
#[allow(unused_imports)]
use bytes::{Buf, BufMut, Bytes, BytesMut};
#[allow(unused_imports)]
use num_derive::FromPrimitive;
#[allow(unused_imports)]
use num_derive::ToPrimitive;
#[allow(unused_imports)]
use num_traits::FromPrimitive;
#[allow(unused_imports)]
use num_traits::ToPrimitive;
#[allow(unused_imports)]
use prost::Message as ProstMessage;
use proto_mav_comm::MavlinkVersion;
use proto_mav_comm::{error::*, Message};
impl crate::proto::localtest::Heartbeat2 {
    pub const ENCODED_LEN: usize = 1541usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            for _ in 0..256usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.custom_mode.push(val.into());
            }
            let tmp = buf.get_u8();
            _struct.r#type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavType".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.autopilot = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavAutopilot".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.base_mode = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavModeFlag".to_string(),
                value: tmp as u32,
            })?;
            let mut s = Vec::with_capacity(256usize);
            for _ in 0..256usize {
                s.push(buf.get_u8());
            }
            _struct.string_test = String::from_utf8_lossy(&s).into();
            for _ in 0..256usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.bytes_test.push(val.into());
            }
            let tmp = buf.get_u8();
            _struct.system_status =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavState".to_string(),
                    value: tmp as u32,
                })?;
            _struct.mavlink_version = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.custom_mode {
            _tmp.put_u32_le(*val as u32);
        }
        _tmp.put_u8(self.r#type as u8);
        _tmp.put_u8(self.autopilot as u8);
        _tmp.put_u8(self.base_mode as u8);
        for val in self.string_test.bytes() {
            _tmp.put_u8(val);
        }
        for val in &self.bytes_test {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.system_status as u8);
        _tmp.put_u8(self.mavlink_version as u8);
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum MavMessage {
    Heartbeat2(crate::proto::localtest::Heartbeat2),
    Common(crate::mavlink::common::MavMessage),
}
impl From<crate::mavlink::common::MavMessage> for MavMessage {
    fn from(message: crate::mavlink::common::MavMessage) -> Self {
        MavMessage::Common(message)
    }
}
impl Message for MavMessage {
    fn parse(version: MavlinkVersion, id: u32, payload: &[u8]) -> Result<MavMessage, ParserError> {
        match id {
            55000 => crate::proto::localtest::Heartbeat2::mavlink_deser(version, payload)
                .map(MavMessage::Heartbeat2),
            _ => {
                if let Ok(msg) = crate::mavlink::common::MavMessage::parse(version, id, payload) {
                    return Ok(MavMessage::Common(msg));
                }
                Err(ParserError::UnknownMessage { id })
            }
        }
    }
    fn proto_parse(id: u32, payload: &[u8]) -> Result<MavMessage, ParserError> {
        match id {
            55000 => crate::proto::localtest::Heartbeat2::decode(payload)
                .map(MavMessage::Heartbeat2)
                .map_err(|error| ParserError::ProstDecode { error }),
            _ => {
                if let Ok(msg) = crate::mavlink::common::MavMessage::proto_parse(id, payload) {
                    return Ok(MavMessage::Common(msg));
                }
                Err(ParserError::UnknownMessage { id })
            }
        }
    }
    fn message_name(&self) -> &'static str {
        match self {
            MavMessage::Heartbeat2(..) => "Heartbeat2",
            MavMessage::Common(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::Heartbeat2(..) => 55000,
            MavMessage::Common(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "Heartbeat2" => Ok(55000),
            _ => {
                match crate::mavlink::common::MavMessage::message_id_from_name(name) {
                    Ok(name) => return Ok(name),
                    Err(..) => {}
                }
                Err("Invalid message name.")
            }
        }
    }
    fn default_message_from_id(id: u32) -> Result<MavMessage, &'static str> {
        match id {
            55000 => Ok(MavMessage::Heartbeat2(
                crate::proto::localtest::Heartbeat2::default(),
            )),
            _ => {
                if let Ok(msg) = crate::mavlink::common::MavMessage::default_message_from_id(id) {
                    return Ok(MavMessage::Common(msg));
                }
                Err("Invalid message id.")
            }
        }
    }
    fn mavlink_ser(&self) -> Vec<u8> {
        match *self {
            MavMessage::Heartbeat2(ref body) => body.mavlink_ser(),
            MavMessage::Common(ref msg) => msg.mavlink_ser(),
        }
    }
    fn proto_encode(&self) -> Vec<u8> {
        match *self {
            MavMessage::Heartbeat2(ref body) => body.encode_to_vec(),
            MavMessage::Common(ref msg) => msg.proto_encode(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            55000 => 44,
            _ => {
                match crate::mavlink::common::MavMessage::extra_crc(id) {
                    0 => {}
                    any => return any,
                }
                0
            }
        }
    }
}
