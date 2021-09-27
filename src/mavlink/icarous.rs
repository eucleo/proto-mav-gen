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
impl crate::proto::icarous::IcarousHeartbeat {
    pub const ENCODED_LEN: usize = 1usize;
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
            let tmp = buf.get_u8();
            _struct.status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "IcarousFmsState".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.status as u8);
        _tmp
    }
}
impl crate::proto::icarous::IcarousKinematicBands {
    pub const ENCODED_LEN: usize = 46usize;
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
            _struct.min1 = buf.get_f32_le();
            _struct.max1 = buf.get_f32_le();
            _struct.min2 = buf.get_f32_le();
            _struct.max2 = buf.get_f32_le();
            _struct.min3 = buf.get_f32_le();
            _struct.max3 = buf.get_f32_le();
            _struct.min4 = buf.get_f32_le();
            _struct.max4 = buf.get_f32_le();
            _struct.min5 = buf.get_f32_le();
            _struct.max5 = buf.get_f32_le();
            _struct.num_bands = buf.get_i8() as i32;
            let tmp = buf.get_u8();
            _struct.type1 = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "IcarousTrackBandTypes".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.type2 = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "IcarousTrackBandTypes".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.type3 = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "IcarousTrackBandTypes".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.type4 = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "IcarousTrackBandTypes".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.type5 = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "IcarousTrackBandTypes".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.min1 as f32);
        _tmp.put_f32_le(self.max1 as f32);
        _tmp.put_f32_le(self.min2 as f32);
        _tmp.put_f32_le(self.max2 as f32);
        _tmp.put_f32_le(self.min3 as f32);
        _tmp.put_f32_le(self.max3 as f32);
        _tmp.put_f32_le(self.min4 as f32);
        _tmp.put_f32_le(self.max4 as f32);
        _tmp.put_f32_le(self.min5 as f32);
        _tmp.put_f32_le(self.max5 as f32);
        _tmp.put_i8(self.num_bands as i8);
        _tmp.put_u8(self.type1 as u8);
        _tmp.put_u8(self.type2 as u8);
        _tmp.put_u8(self.type3 as u8);
        _tmp.put_u8(self.type4 as u8);
        _tmp.put_u8(self.type5 as u8);
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum MavMessage {
    IcarousHeartbeat(crate::proto::icarous::IcarousHeartbeat),
    IcarousKinematicBands(crate::proto::icarous::IcarousKinematicBands),
}
impl Message for MavMessage {
    fn parse(version: MavlinkVersion, id: u32, payload: &[u8]) -> Result<MavMessage, ParserError> {
        match id {
            42000 => crate::proto::icarous::IcarousHeartbeat::mavlink_deser(version, payload)
                .map(MavMessage::IcarousHeartbeat),
            42001 => crate::proto::icarous::IcarousKinematicBands::mavlink_deser(version, payload)
                .map(MavMessage::IcarousKinematicBands),
            _ => Err(ParserError::UnknownMessage { id }),
        }
    }
    fn message_name(&self) -> &'static str {
        match self {
            MavMessage::IcarousHeartbeat(..) => "IcarousHeartbeat",
            MavMessage::IcarousKinematicBands(..) => "IcarousKinematicBands",
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::IcarousHeartbeat(..) => 42000,
            MavMessage::IcarousKinematicBands(..) => 42001,
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "IcarousHeartbeat" => Ok(42000),
            "IcarousKinematicBands" => Ok(42001),
            _ => Err("Invalid message name."),
        }
    }
    fn default_message_from_id(id: u32) -> Result<MavMessage, &'static str> {
        match id {
            42000 => Ok(MavMessage::IcarousHeartbeat(
                crate::proto::icarous::IcarousHeartbeat::default(),
            )),
            42001 => Ok(MavMessage::IcarousKinematicBands(
                crate::proto::icarous::IcarousKinematicBands::default(),
            )),
            _ => Err("Invalid message id."),
        }
    }
    fn mavlink_ser(&self) -> Vec<u8> {
        match *self {
            MavMessage::IcarousHeartbeat(ref body) => body.mavlink_ser(),
            MavMessage::IcarousKinematicBands(ref body) => body.mavlink_ser(),
        }
    }
    fn proto_encode(&self) -> Vec<u8> {
        match *self {
            MavMessage::IcarousHeartbeat(ref body) => body.encode_to_vec(),
            MavMessage::IcarousKinematicBands(ref body) => body.encode_to_vec(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            42000 => 227,
            42001 => 239,
            _ => 0,
        }
    }
}
