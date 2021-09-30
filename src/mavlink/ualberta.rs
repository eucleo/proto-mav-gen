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
impl crate::proto::ualberta::NavFilterBias {
    pub const ENCODED_LEN: usize = 32usize;
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
            _struct.usec = buf.get_u64_le();
            _struct.accel_0 = buf.get_f32_le();
            _struct.accel_1 = buf.get_f32_le();
            _struct.accel_2 = buf.get_f32_le();
            _struct.gyro_0 = buf.get_f32_le();
            _struct.gyro_1 = buf.get_f32_le();
            _struct.gyro_2 = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.usec as u64);
        _tmp.put_f32_le(self.accel_0 as f32);
        _tmp.put_f32_le(self.accel_1 as f32);
        _tmp.put_f32_le(self.accel_2 as f32);
        _tmp.put_f32_le(self.gyro_0 as f32);
        _tmp.put_f32_le(self.gyro_1 as f32);
        _tmp.put_f32_le(self.gyro_2 as f32);
        _tmp
    }
}
impl crate::proto::ualberta::RadioCalibration {
    pub const ENCODED_LEN: usize = 42usize;
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
            for _ in 0..3usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.aileron.push(val.into());
            }
            for _ in 0..3usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.elevator.push(val.into());
            }
            for _ in 0..3usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.rudder.push(val.into());
            }
            for _ in 0..2usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.gyro.push(val.into());
            }
            for _ in 0..5usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.pitch.push(val.into());
            }
            for _ in 0..5usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.throttle.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.aileron {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.elevator {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.rudder {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.gyro {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.pitch {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.throttle {
            _tmp.put_u16_le(*val as u16);
        }
        _tmp
    }
}
impl crate::proto::ualberta::UalbertaSysStatus {
    pub const ENCODED_LEN: usize = 3usize;
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
            _struct.mode = buf.get_u8() as u32;
            _struct.nav_mode = buf.get_u8() as u32;
            _struct.pilot = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.mode as u8);
        _tmp.put_u8(self.nav_mode as u8);
        _tmp.put_u8(self.pilot as u8);
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum MavMessage {
    NavFilterBias(crate::proto::ualberta::NavFilterBias),
    RadioCalibration(crate::proto::ualberta::RadioCalibration),
    UalbertaSysStatus(crate::proto::ualberta::UalbertaSysStatus),
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
            220 => crate::proto::ualberta::NavFilterBias::mavlink_deser(version, payload)
                .map(MavMessage::NavFilterBias),
            221 => crate::proto::ualberta::RadioCalibration::mavlink_deser(version, payload)
                .map(MavMessage::RadioCalibration),
            222 => crate::proto::ualberta::UalbertaSysStatus::mavlink_deser(version, payload)
                .map(MavMessage::UalbertaSysStatus),
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
            220 => crate::proto::ualberta::NavFilterBias::decode(payload)
                .map(MavMessage::NavFilterBias)
                .map_err(|error| ParserError::ProstDecode { error }),
            221 => crate::proto::ualberta::RadioCalibration::decode(payload)
                .map(MavMessage::RadioCalibration)
                .map_err(|error| ParserError::ProstDecode { error }),
            222 => crate::proto::ualberta::UalbertaSysStatus::decode(payload)
                .map(MavMessage::UalbertaSysStatus)
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
            MavMessage::NavFilterBias(..) => "NavFilterBias",
            MavMessage::RadioCalibration(..) => "RadioCalibration",
            MavMessage::UalbertaSysStatus(..) => "UalbertaSysStatus",
            MavMessage::Common(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::NavFilterBias(..) => 220,
            MavMessage::RadioCalibration(..) => 221,
            MavMessage::UalbertaSysStatus(..) => 222,
            MavMessage::Common(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "NavFilterBias" => Ok(220),
            "RadioCalibration" => Ok(221),
            "UalbertaSysStatus" => Ok(222),
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
            220 => Ok(MavMessage::NavFilterBias(
                crate::proto::ualberta::NavFilterBias::default(),
            )),
            221 => Ok(MavMessage::RadioCalibration(
                crate::proto::ualberta::RadioCalibration::default(),
            )),
            222 => Ok(MavMessage::UalbertaSysStatus(
                crate::proto::ualberta::UalbertaSysStatus::default(),
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
            MavMessage::NavFilterBias(ref body) => body.mavlink_ser(),
            MavMessage::RadioCalibration(ref body) => body.mavlink_ser(),
            MavMessage::UalbertaSysStatus(ref body) => body.mavlink_ser(),
            MavMessage::Common(ref msg) => msg.mavlink_ser(),
        }
    }
    fn proto_encode(&self) -> Vec<u8> {
        match *self {
            MavMessage::NavFilterBias(ref body) => body.encode_to_vec(),
            MavMessage::RadioCalibration(ref body) => body.encode_to_vec(),
            MavMessage::UalbertaSysStatus(ref body) => body.encode_to_vec(),
            MavMessage::Common(ref msg) => msg.proto_encode(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            220 => 34,
            221 => 71,
            222 => 15,
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
