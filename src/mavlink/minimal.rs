// This file was automatically generated, do not edit
#[allow(unused_imports)]
use crate::{};
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
impl crate::proto::minimal::Heartbeat {
    pub const ENCODED_LEN: usize = 9usize;
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
            _struct.custom_mode = buf.get_u32_le();
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
        _tmp.put_u32_le(self.custom_mode as u32);
        _tmp.put_u8(self.r#type as u8);
        _tmp.put_u8(self.autopilot as u8);
        _tmp.put_u8(self.base_mode as u8);
        _tmp.put_u8(self.system_status as u8);
        _tmp.put_u8(self.mavlink_version as u8);
        _tmp
    }
}
impl crate::proto::minimal::ProtocolVersion {
    pub const ENCODED_LEN: usize = 22usize;
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
            _struct.version = buf.get_u16_le() as u32;
            _struct.min_version = buf.get_u16_le() as u32;
            _struct.max_version = buf.get_u16_le() as u32;
            for _ in 0..8usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.spec_version_hash.push(val.into());
            }
            for _ in 0..8usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.library_version_hash.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.version as u16);
        _tmp.put_u16_le(self.min_version as u16);
        _tmp.put_u16_le(self.max_version as u16);
        for val in &self.spec_version_hash {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.library_version_hash {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum MavMessage {
    Heartbeat(crate::proto::minimal::Heartbeat),
    ProtocolVersion(crate::proto::minimal::ProtocolVersion),
}
impl Message for MavMessage {
    fn parse(version: MavlinkVersion, id: u32, payload: &[u8]) -> Result<MavMessage, ParserError> {
        match id {
            0 => crate::proto::minimal::Heartbeat::mavlink_deser(version, payload)
                .map(MavMessage::Heartbeat),
            300 => crate::proto::minimal::ProtocolVersion::mavlink_deser(version, payload)
                .map(MavMessage::ProtocolVersion),
            _ => Err(ParserError::UnknownMessage { id }),
        }
    }
    fn message_name(&self) -> &'static str {
        match self {
            MavMessage::Heartbeat(..) => "Heartbeat",
            MavMessage::ProtocolVersion(..) => "ProtocolVersion",
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::Heartbeat(..) => 0,
            MavMessage::ProtocolVersion(..) => 300,
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "Heartbeat" => Ok(0),
            "ProtocolVersion" => Ok(300),
            _ => Err("Invalid message name."),
        }
    }
    fn default_message_from_id(id: u32) -> Result<MavMessage, &'static str> {
        match id {
            0 => Ok(MavMessage::Heartbeat(
                crate::proto::minimal::Heartbeat::default(),
            )),
            300 => Ok(MavMessage::ProtocolVersion(
                crate::proto::minimal::ProtocolVersion::default(),
            )),
            _ => Err("Invalid message id."),
        }
    }
    fn mavlink_ser(&self) -> Vec<u8> {
        match *self {
            MavMessage::Heartbeat(ref body) => body.mavlink_ser(),
            MavMessage::ProtocolVersion(ref body) => body.mavlink_ser(),
        }
    }
    fn proto_encode(&self) -> Vec<u8> {
        match *self {
            MavMessage::Heartbeat(ref body) => body.encode_to_vec(),
            MavMessage::ProtocolVersion(ref body) => body.encode_to_vec(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            0 => 50,
            300 => 217,
            _ => 0,
        }
    }
}
