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
impl crate::proto::autoquad::AqTelemetryF {
    pub const ENCODED_LEN: usize = 82usize;
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
            _struct.value1 = buf.get_f32_le();
            _struct.value2 = buf.get_f32_le();
            _struct.value3 = buf.get_f32_le();
            _struct.value4 = buf.get_f32_le();
            _struct.value5 = buf.get_f32_le();
            _struct.value6 = buf.get_f32_le();
            _struct.value7 = buf.get_f32_le();
            _struct.value8 = buf.get_f32_le();
            _struct.value9 = buf.get_f32_le();
            _struct.value10 = buf.get_f32_le();
            _struct.value11 = buf.get_f32_le();
            _struct.value12 = buf.get_f32_le();
            _struct.value13 = buf.get_f32_le();
            _struct.value14 = buf.get_f32_le();
            _struct.value15 = buf.get_f32_le();
            _struct.value16 = buf.get_f32_le();
            _struct.value17 = buf.get_f32_le();
            _struct.value18 = buf.get_f32_le();
            _struct.value19 = buf.get_f32_le();
            _struct.value20 = buf.get_f32_le();
            _struct.index = buf.get_u16_le() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.value1 as f32);
        _tmp.put_f32_le(self.value2 as f32);
        _tmp.put_f32_le(self.value3 as f32);
        _tmp.put_f32_le(self.value4 as f32);
        _tmp.put_f32_le(self.value5 as f32);
        _tmp.put_f32_le(self.value6 as f32);
        _tmp.put_f32_le(self.value7 as f32);
        _tmp.put_f32_le(self.value8 as f32);
        _tmp.put_f32_le(self.value9 as f32);
        _tmp.put_f32_le(self.value10 as f32);
        _tmp.put_f32_le(self.value11 as f32);
        _tmp.put_f32_le(self.value12 as f32);
        _tmp.put_f32_le(self.value13 as f32);
        _tmp.put_f32_le(self.value14 as f32);
        _tmp.put_f32_le(self.value15 as f32);
        _tmp.put_f32_le(self.value16 as f32);
        _tmp.put_f32_le(self.value17 as f32);
        _tmp.put_f32_le(self.value18 as f32);
        _tmp.put_f32_le(self.value19 as f32);
        _tmp.put_f32_le(self.value20 as f32);
        _tmp.put_u16_le(self.index as u16);
        _tmp
    }
}
impl crate::proto::autoquad::AqEscTelemetry {
    pub const ENCODED_LEN: usize = 55usize;
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
            _struct.time_boot_ms = buf.get_u32_le();
            for _ in 0..4usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.data0.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.data1.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.status_age.push(val.into());
            }
            _struct.seq = buf.get_u8() as u32;
            _struct.num_motors = buf.get_u8() as u32;
            _struct.num_in_seq = buf.get_u8() as u32;
            for _ in 0..4usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.escid.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.data_version.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        for val in &self.data0 {
            _tmp.put_u32_le(*val as u32);
        }
        for val in &self.data1 {
            _tmp.put_u32_le(*val as u32);
        }
        for val in &self.status_age {
            _tmp.put_u16_le(*val as u16);
        }
        _tmp.put_u8(self.seq as u8);
        _tmp.put_u8(self.num_motors as u8);
        _tmp.put_u8(self.num_in_seq as u8);
        for val in &self.escid {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.data_version {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum MavMessage {
    AqTelemetryF(crate::proto::autoquad::AqTelemetryF),
    AqEscTelemetry(crate::proto::autoquad::AqEscTelemetry),
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
            150 => crate::proto::autoquad::AqTelemetryF::mavlink_deser(version, payload)
                .map(MavMessage::AqTelemetryF),
            152 => crate::proto::autoquad::AqEscTelemetry::mavlink_deser(version, payload)
                .map(MavMessage::AqEscTelemetry),
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
            MavMessage::AqTelemetryF(..) => "AqTelemetryF",
            MavMessage::AqEscTelemetry(..) => "AqEscTelemetry",
            MavMessage::Common(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::AqTelemetryF(..) => 150,
            MavMessage::AqEscTelemetry(..) => 152,
            MavMessage::Common(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "AqTelemetryF" => Ok(150),
            "AqEscTelemetry" => Ok(152),
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
            150 => Ok(MavMessage::AqTelemetryF(
                crate::proto::autoquad::AqTelemetryF::default(),
            )),
            152 => Ok(MavMessage::AqEscTelemetry(
                crate::proto::autoquad::AqEscTelemetry::default(),
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
            MavMessage::AqTelemetryF(ref body) => body.mavlink_ser(),
            MavMessage::AqEscTelemetry(ref body) => body.mavlink_ser(),
            MavMessage::Common(ref msg) => msg.mavlink_ser(),
        }
    }
    fn proto_encode(&self) -> Vec<u8> {
        match *self {
            MavMessage::AqTelemetryF(ref body) => body.encode_to_vec(),
            MavMessage::AqEscTelemetry(ref body) => body.encode_to_vec(),
            MavMessage::Common(ref msg) => msg.proto_encode(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            150 => 241,
            152 => 115,
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
