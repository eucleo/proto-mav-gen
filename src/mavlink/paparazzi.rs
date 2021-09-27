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
impl crate::proto::paparazzi::ScriptItem {
    pub const ENCODED_LEN: usize = 54usize;
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
            _struct.seq = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            let mut s = Vec::with_capacity(50usize);
            for _ in 0..50usize {
                s.push(buf.get_u8());
            }
            _struct.name = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.seq as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in self.name.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::paparazzi::ScriptRequest {
    pub const ENCODED_LEN: usize = 4usize;
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
            _struct.seq = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.seq as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::paparazzi::ScriptRequestList {
    pub const ENCODED_LEN: usize = 2usize;
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
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::paparazzi::ScriptCount {
    pub const ENCODED_LEN: usize = 4usize;
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
            _struct.count = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.count as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::paparazzi::ScriptCurrent {
    pub const ENCODED_LEN: usize = 2usize;
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
            _struct.seq = buf.get_u16_le() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.seq as u16);
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum MavMessage {
    ScriptItem(crate::proto::paparazzi::ScriptItem),
    ScriptRequest(crate::proto::paparazzi::ScriptRequest),
    ScriptRequestList(crate::proto::paparazzi::ScriptRequestList),
    ScriptCount(crate::proto::paparazzi::ScriptCount),
    ScriptCurrent(crate::proto::paparazzi::ScriptCurrent),
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
            180 => crate::proto::paparazzi::ScriptItem::mavlink_deser(version, payload)
                .map(MavMessage::ScriptItem),
            181 => crate::proto::paparazzi::ScriptRequest::mavlink_deser(version, payload)
                .map(MavMessage::ScriptRequest),
            182 => crate::proto::paparazzi::ScriptRequestList::mavlink_deser(version, payload)
                .map(MavMessage::ScriptRequestList),
            183 => crate::proto::paparazzi::ScriptCount::mavlink_deser(version, payload)
                .map(MavMessage::ScriptCount),
            184 => crate::proto::paparazzi::ScriptCurrent::mavlink_deser(version, payload)
                .map(MavMessage::ScriptCurrent),
            _ => {
                if let Ok(msg) = crate::mavlink::common::MavMessage::parse(version, id, payload) {
                    return Ok(MavMessage::Common(msg));
                }
                Err(ParserError::UnknownMessage { id })
            }
        }
    }
    fn message_name(&self) -> &'static str {
        match self {
            MavMessage::ScriptItem(..) => "ScriptItem",
            MavMessage::ScriptRequest(..) => "ScriptRequest",
            MavMessage::ScriptRequestList(..) => "ScriptRequestList",
            MavMessage::ScriptCount(..) => "ScriptCount",
            MavMessage::ScriptCurrent(..) => "ScriptCurrent",
            MavMessage::Common(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::ScriptItem(..) => 180,
            MavMessage::ScriptRequest(..) => 181,
            MavMessage::ScriptRequestList(..) => 182,
            MavMessage::ScriptCount(..) => 183,
            MavMessage::ScriptCurrent(..) => 184,
            MavMessage::Common(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "ScriptItem" => Ok(180),
            "ScriptRequest" => Ok(181),
            "ScriptRequestList" => Ok(182),
            "ScriptCount" => Ok(183),
            "ScriptCurrent" => Ok(184),
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
            180 => Ok(MavMessage::ScriptItem(
                crate::proto::paparazzi::ScriptItem::default(),
            )),
            181 => Ok(MavMessage::ScriptRequest(
                crate::proto::paparazzi::ScriptRequest::default(),
            )),
            182 => Ok(MavMessage::ScriptRequestList(
                crate::proto::paparazzi::ScriptRequestList::default(),
            )),
            183 => Ok(MavMessage::ScriptCount(
                crate::proto::paparazzi::ScriptCount::default(),
            )),
            184 => Ok(MavMessage::ScriptCurrent(
                crate::proto::paparazzi::ScriptCurrent::default(),
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
            MavMessage::ScriptItem(ref body) => body.mavlink_ser(),
            MavMessage::ScriptRequest(ref body) => body.mavlink_ser(),
            MavMessage::ScriptRequestList(ref body) => body.mavlink_ser(),
            MavMessage::ScriptCount(ref body) => body.mavlink_ser(),
            MavMessage::ScriptCurrent(ref body) => body.mavlink_ser(),
            MavMessage::Common(ref msg) => msg.mavlink_ser(),
        }
    }
    fn proto_encode(&self) -> Vec<u8> {
        match *self {
            MavMessage::ScriptItem(ref body) => body.encode_to_vec(),
            MavMessage::ScriptRequest(ref body) => body.encode_to_vec(),
            MavMessage::ScriptRequestList(ref body) => body.encode_to_vec(),
            MavMessage::ScriptCount(ref body) => body.encode_to_vec(),
            MavMessage::ScriptCurrent(ref body) => body.encode_to_vec(),
            MavMessage::Common(ref msg) => msg.proto_encode(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            180 => 231,
            181 => 129,
            182 => 115,
            183 => 186,
            184 => 40,
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
