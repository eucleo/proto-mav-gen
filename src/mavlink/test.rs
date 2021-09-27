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
impl crate::proto::test::TestTypes {
    pub const ENCODED_LEN: usize = 179usize;
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
            _struct.u64 = buf.get_u64_le();
            _struct.s64 = buf.get_i64_le();
            _struct.d = buf.get_f64_le();
            for _ in 0..3usize {
                let val = buf.get_u64_le();
                #[allow(clippy::useless_conversion)]
                _struct.u64_array.push(val.into());
            }
            for _ in 0..3usize {
                let val = buf.get_i64_le();
                #[allow(clippy::useless_conversion)]
                _struct.s64_array.push(val.into());
            }
            for _ in 0..3usize {
                let val = buf.get_f64_le();
                #[allow(clippy::useless_conversion)]
                _struct.d_array.push(val.into());
            }
            _struct.u32 = buf.get_u32_le();
            _struct.s32 = buf.get_i32_le();
            _struct.f = buf.get_f32_le();
            for _ in 0..3usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.u32_array.push(val.into());
            }
            for _ in 0..3usize {
                let val = buf.get_i32_le();
                #[allow(clippy::useless_conversion)]
                _struct.s32_array.push(val.into());
            }
            for _ in 0..3usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.f_array.push(val.into());
            }
            _struct.u16 = buf.get_u16_le() as u32;
            _struct.s16 = buf.get_i16_le() as i32;
            for _ in 0..3usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.u16_array.push(val.into());
            }
            for _ in 0..3usize {
                let val = buf.get_i16_le() as i32;
                #[allow(clippy::useless_conversion)]
                _struct.s16_array.push(val.into());
            }
            _struct.c = buf.get_u8() as u32;
            let mut s = Vec::with_capacity(10usize);
            for _ in 0..10usize {
                s.push(buf.get_u8());
            }
            _struct.s = String::from_utf8_lossy(&s).into();
            _struct.u8 = buf.get_u8() as u32;
            _struct.s8 = buf.get_i8() as i32;
            for _ in 0..3usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.u8_array.push(val.into());
            }
            for _ in 0..3usize {
                let val = buf.get_i8() as i32;
                #[allow(clippy::useless_conversion)]
                _struct.s8_array.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.u64 as u64);
        _tmp.put_i64_le(self.s64 as i64);
        _tmp.put_f64_le(self.d as f64);
        for val in &self.u64_array {
            _tmp.put_u64_le(*val as u64);
        }
        for val in &self.s64_array {
            _tmp.put_i64_le(*val as i64);
        }
        for val in &self.d_array {
            _tmp.put_f64_le(*val as f64);
        }
        _tmp.put_u32_le(self.u32 as u32);
        _tmp.put_i32_le(self.s32 as i32);
        _tmp.put_f32_le(self.f as f32);
        for val in &self.u32_array {
            _tmp.put_u32_le(*val as u32);
        }
        for val in &self.s32_array {
            _tmp.put_i32_le(*val as i32);
        }
        for val in &self.f_array {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_u16_le(self.u16 as u16);
        _tmp.put_i16_le(self.s16 as i16);
        for val in &self.u16_array {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.s16_array {
            _tmp.put_i16_le(*val as i16);
        }
        _tmp.put_u8(self.c as u8);
        for val in self.s.bytes() {
            _tmp.put_u8(val);
        }
        _tmp.put_u8(self.u8 as u8);
        _tmp.put_i8(self.s8 as i8);
        for val in &self.u8_array {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.s8_array {
            _tmp.put_i8(*val as i8);
        }
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum MavMessage {
    TestTypes(crate::proto::test::TestTypes),
}
impl Message for MavMessage {
    fn parse(version: MavlinkVersion, id: u32, payload: &[u8]) -> Result<MavMessage, ParserError> {
        match id {
            0 => crate::proto::test::TestTypes::mavlink_deser(version, payload)
                .map(MavMessage::TestTypes),
            _ => Err(ParserError::UnknownMessage { id }),
        }
    }
    fn message_name(&self) -> &'static str {
        match self {
            MavMessage::TestTypes(..) => "TestTypes",
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::TestTypes(..) => 0,
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "TestTypes" => Ok(0),
            _ => Err("Invalid message name."),
        }
    }
    fn default_message_from_id(id: u32) -> Result<MavMessage, &'static str> {
        match id {
            0 => Ok(MavMessage::TestTypes(
                crate::proto::test::TestTypes::default(),
            )),
            _ => Err("Invalid message id."),
        }
    }
    fn mavlink_ser(&self) -> Vec<u8> {
        match *self {
            MavMessage::TestTypes(ref body) => body.mavlink_ser(),
        }
    }
    fn proto_encode(&self) -> Vec<u8> {
        match *self {
            MavMessage::TestTypes(ref body) => body.encode_to_vec(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            0 => 103,
            _ => 0,
        }
    }
}
