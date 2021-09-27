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
impl crate::proto::python_array_test::ArrayTest0 {
    pub const ENCODED_LEN: usize = 33usize;
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
            for _ in 0..4usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.ar_u32.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.ar_u16.push(val.into());
            }
            _struct.v1 = buf.get_u8() as u32;
            for _ in 0..4usize {
                let val = buf.get_i8() as i32;
                #[allow(clippy::useless_conversion)]
                _struct.ar_i8.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.ar_u8.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.ar_u32 {
            _tmp.put_u32_le(*val as u32);
        }
        for val in &self.ar_u16 {
            _tmp.put_u16_le(*val as u16);
        }
        _tmp.put_u8(self.v1 as u8);
        for val in &self.ar_i8 {
            _tmp.put_i8(*val as i8);
        }
        for val in &self.ar_u8 {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::python_array_test::ArrayTest1 {
    pub const ENCODED_LEN: usize = 16usize;
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
            for _ in 0..4usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.ar_u32.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.ar_u32 {
            _tmp.put_u32_le(*val as u32);
        }
        _tmp
    }
}
impl crate::proto::python_array_test::ArrayTest3 {
    pub const ENCODED_LEN: usize = 17usize;
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
            for _ in 0..4usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.ar_u32.push(val.into());
            }
            _struct.v = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.ar_u32 {
            _tmp.put_u32_le(*val as u32);
        }
        _tmp.put_u8(self.v as u8);
        _tmp
    }
}
impl crate::proto::python_array_test::ArrayTest4 {
    pub const ENCODED_LEN: usize = 17usize;
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
            for _ in 0..4usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.ar_u32.push(val.into());
            }
            _struct.v = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.ar_u32 {
            _tmp.put_u32_le(*val as u32);
        }
        _tmp.put_u8(self.v as u8);
        _tmp
    }
}
impl crate::proto::python_array_test::ArrayTest5 {
    pub const ENCODED_LEN: usize = 10usize;
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
            let mut s = Vec::with_capacity(5usize);
            for _ in 0..5usize {
                s.push(buf.get_u8());
            }
            _struct.c1 = String::from_utf8_lossy(&s).into();
            let mut s = Vec::with_capacity(5usize);
            for _ in 0..5usize {
                s.push(buf.get_u8());
            }
            _struct.c2 = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in self.c1.bytes() {
            _tmp.put_u8(val);
        }
        for val in self.c2.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::python_array_test::ArrayTest6 {
    pub const ENCODED_LEN: usize = 91usize;
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
            for _ in 0..2usize {
                let val = buf.get_f64_le();
                #[allow(clippy::useless_conversion)]
                _struct.ar_d.push(val.into());
            }
            _struct.v3 = buf.get_u32_le();
            for _ in 0..2usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.ar_u32.push(val.into());
            }
            for _ in 0..2usize {
                let val = buf.get_i32_le();
                #[allow(clippy::useless_conversion)]
                _struct.ar_i32.push(val.into());
            }
            for _ in 0..2usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.ar_f.push(val.into());
            }
            _struct.v2 = buf.get_u16_le() as u32;
            for _ in 0..2usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.ar_u16.push(val.into());
            }
            for _ in 0..2usize {
                let val = buf.get_i16_le() as i32;
                #[allow(clippy::useless_conversion)]
                _struct.ar_i16.push(val.into());
            }
            _struct.v1 = buf.get_u8() as u32;
            for _ in 0..2usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.ar_u8.push(val.into());
            }
            for _ in 0..2usize {
                let val = buf.get_i8() as i32;
                #[allow(clippy::useless_conversion)]
                _struct.ar_i8.push(val.into());
            }
            let mut s = Vec::with_capacity(32usize);
            for _ in 0..32usize {
                s.push(buf.get_u8());
            }
            _struct.ar_c = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.ar_d {
            _tmp.put_f64_le(*val as f64);
        }
        _tmp.put_u32_le(self.v3 as u32);
        for val in &self.ar_u32 {
            _tmp.put_u32_le(*val as u32);
        }
        for val in &self.ar_i32 {
            _tmp.put_i32_le(*val as i32);
        }
        for val in &self.ar_f {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_u16_le(self.v2 as u16);
        for val in &self.ar_u16 {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.ar_i16 {
            _tmp.put_i16_le(*val as i16);
        }
        _tmp.put_u8(self.v1 as u8);
        for val in &self.ar_u8 {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.ar_i8 {
            _tmp.put_i8(*val as i8);
        }
        for val in self.ar_c.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::python_array_test::ArrayTest7 {
    pub const ENCODED_LEN: usize = 84usize;
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
            for _ in 0..2usize {
                let val = buf.get_f64_le();
                #[allow(clippy::useless_conversion)]
                _struct.ar_d.push(val.into());
            }
            for _ in 0..2usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.ar_f.push(val.into());
            }
            for _ in 0..2usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.ar_u32.push(val.into());
            }
            for _ in 0..2usize {
                let val = buf.get_i32_le();
                #[allow(clippy::useless_conversion)]
                _struct.ar_i32.push(val.into());
            }
            for _ in 0..2usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.ar_u16.push(val.into());
            }
            for _ in 0..2usize {
                let val = buf.get_i16_le() as i32;
                #[allow(clippy::useless_conversion)]
                _struct.ar_i16.push(val.into());
            }
            for _ in 0..2usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.ar_u8.push(val.into());
            }
            for _ in 0..2usize {
                let val = buf.get_i8() as i32;
                #[allow(clippy::useless_conversion)]
                _struct.ar_i8.push(val.into());
            }
            let mut s = Vec::with_capacity(32usize);
            for _ in 0..32usize {
                s.push(buf.get_u8());
            }
            _struct.ar_c = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.ar_d {
            _tmp.put_f64_le(*val as f64);
        }
        for val in &self.ar_f {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.ar_u32 {
            _tmp.put_u32_le(*val as u32);
        }
        for val in &self.ar_i32 {
            _tmp.put_i32_le(*val as i32);
        }
        for val in &self.ar_u16 {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.ar_i16 {
            _tmp.put_i16_le(*val as i16);
        }
        for val in &self.ar_u8 {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.ar_i8 {
            _tmp.put_i8(*val as i8);
        }
        for val in self.ar_c.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::python_array_test::ArrayTest8 {
    pub const ENCODED_LEN: usize = 24usize;
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
            for _ in 0..2usize {
                let val = buf.get_f64_le();
                #[allow(clippy::useless_conversion)]
                _struct.ar_d.push(val.into());
            }
            _struct.v3 = buf.get_u32_le();
            for _ in 0..2usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.ar_u16.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.ar_d {
            _tmp.put_f64_le(*val as f64);
        }
        _tmp.put_u32_le(self.v3 as u32);
        for val in &self.ar_u16 {
            _tmp.put_u16_le(*val as u16);
        }
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum MavMessage {
    ArrayTest0(crate::proto::python_array_test::ArrayTest0),
    ArrayTest1(crate::proto::python_array_test::ArrayTest1),
    ArrayTest3(crate::proto::python_array_test::ArrayTest3),
    ArrayTest4(crate::proto::python_array_test::ArrayTest4),
    ArrayTest5(crate::proto::python_array_test::ArrayTest5),
    ArrayTest6(crate::proto::python_array_test::ArrayTest6),
    ArrayTest7(crate::proto::python_array_test::ArrayTest7),
    ArrayTest8(crate::proto::python_array_test::ArrayTest8),
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
            150 => crate::proto::python_array_test::ArrayTest0::mavlink_deser(version, payload)
                .map(MavMessage::ArrayTest0),
            151 => crate::proto::python_array_test::ArrayTest1::mavlink_deser(version, payload)
                .map(MavMessage::ArrayTest1),
            153 => crate::proto::python_array_test::ArrayTest3::mavlink_deser(version, payload)
                .map(MavMessage::ArrayTest3),
            154 => crate::proto::python_array_test::ArrayTest4::mavlink_deser(version, payload)
                .map(MavMessage::ArrayTest4),
            155 => crate::proto::python_array_test::ArrayTest5::mavlink_deser(version, payload)
                .map(MavMessage::ArrayTest5),
            156 => crate::proto::python_array_test::ArrayTest6::mavlink_deser(version, payload)
                .map(MavMessage::ArrayTest6),
            157 => crate::proto::python_array_test::ArrayTest7::mavlink_deser(version, payload)
                .map(MavMessage::ArrayTest7),
            158 => crate::proto::python_array_test::ArrayTest8::mavlink_deser(version, payload)
                .map(MavMessage::ArrayTest8),
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
            MavMessage::ArrayTest0(..) => "ArrayTest0",
            MavMessage::ArrayTest1(..) => "ArrayTest1",
            MavMessage::ArrayTest3(..) => "ArrayTest3",
            MavMessage::ArrayTest4(..) => "ArrayTest4",
            MavMessage::ArrayTest5(..) => "ArrayTest5",
            MavMessage::ArrayTest6(..) => "ArrayTest6",
            MavMessage::ArrayTest7(..) => "ArrayTest7",
            MavMessage::ArrayTest8(..) => "ArrayTest8",
            MavMessage::Common(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::ArrayTest0(..) => 150,
            MavMessage::ArrayTest1(..) => 151,
            MavMessage::ArrayTest3(..) => 153,
            MavMessage::ArrayTest4(..) => 154,
            MavMessage::ArrayTest5(..) => 155,
            MavMessage::ArrayTest6(..) => 156,
            MavMessage::ArrayTest7(..) => 157,
            MavMessage::ArrayTest8(..) => 158,
            MavMessage::Common(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "ArrayTest0" => Ok(150),
            "ArrayTest1" => Ok(151),
            "ArrayTest3" => Ok(153),
            "ArrayTest4" => Ok(154),
            "ArrayTest5" => Ok(155),
            "ArrayTest6" => Ok(156),
            "ArrayTest7" => Ok(157),
            "ArrayTest8" => Ok(158),
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
            150 => Ok(MavMessage::ArrayTest0(
                crate::proto::python_array_test::ArrayTest0::default(),
            )),
            151 => Ok(MavMessage::ArrayTest1(
                crate::proto::python_array_test::ArrayTest1::default(),
            )),
            153 => Ok(MavMessage::ArrayTest3(
                crate::proto::python_array_test::ArrayTest3::default(),
            )),
            154 => Ok(MavMessage::ArrayTest4(
                crate::proto::python_array_test::ArrayTest4::default(),
            )),
            155 => Ok(MavMessage::ArrayTest5(
                crate::proto::python_array_test::ArrayTest5::default(),
            )),
            156 => Ok(MavMessage::ArrayTest6(
                crate::proto::python_array_test::ArrayTest6::default(),
            )),
            157 => Ok(MavMessage::ArrayTest7(
                crate::proto::python_array_test::ArrayTest7::default(),
            )),
            158 => Ok(MavMessage::ArrayTest8(
                crate::proto::python_array_test::ArrayTest8::default(),
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
            MavMessage::ArrayTest0(ref body) => body.mavlink_ser(),
            MavMessage::ArrayTest1(ref body) => body.mavlink_ser(),
            MavMessage::ArrayTest3(ref body) => body.mavlink_ser(),
            MavMessage::ArrayTest4(ref body) => body.mavlink_ser(),
            MavMessage::ArrayTest5(ref body) => body.mavlink_ser(),
            MavMessage::ArrayTest6(ref body) => body.mavlink_ser(),
            MavMessage::ArrayTest7(ref body) => body.mavlink_ser(),
            MavMessage::ArrayTest8(ref body) => body.mavlink_ser(),
            MavMessage::Common(ref msg) => msg.mavlink_ser(),
        }
    }
    fn proto_encode(&self) -> Vec<u8> {
        match *self {
            MavMessage::ArrayTest0(ref body) => body.encode_to_vec(),
            MavMessage::ArrayTest1(ref body) => body.encode_to_vec(),
            MavMessage::ArrayTest3(ref body) => body.encode_to_vec(),
            MavMessage::ArrayTest4(ref body) => body.encode_to_vec(),
            MavMessage::ArrayTest5(ref body) => body.encode_to_vec(),
            MavMessage::ArrayTest6(ref body) => body.encode_to_vec(),
            MavMessage::ArrayTest7(ref body) => body.encode_to_vec(),
            MavMessage::ArrayTest8(ref body) => body.encode_to_vec(),
            MavMessage::Common(ref msg) => msg.proto_encode(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            150 => 26,
            151 => 72,
            153 => 19,
            154 => 89,
            155 => 27,
            156 => 14,
            157 => 187,
            158 => 106,
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
