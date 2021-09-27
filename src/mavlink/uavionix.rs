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
impl crate::proto::uavionix::UavionixAdsbOutCfg {
    pub const ENCODED_LEN: usize = 20usize;
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
            _struct.icao = buf.get_u32_le();
            _struct.stall_speed = buf.get_u16_le() as u32;
            let mut s = Vec::with_capacity(9usize);
            for _ in 0..9usize {
                s.push(buf.get_u8());
            }
            _struct.callsign = String::from_utf8_lossy(&s).into();
            let tmp = buf.get_u8();
            _struct.emitter_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "AdsbEmitterType".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.aircraft_size =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "UavionixAdsbOutCfgAircraftSize".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u8();
            _struct.gps_offset_lat =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "UavionixAdsbOutCfgGpsOffsetLat".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u8();
            _struct.gps_offset_lon =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "UavionixAdsbOutCfgGpsOffsetLon".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u8();
            _struct.rf_select = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "UavionixAdsbOutRfSelect".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.icao as u32);
        _tmp.put_u16_le(self.stall_speed as u16);
        for val in self.callsign.bytes() {
            _tmp.put_u8(val);
        }
        _tmp.put_u8(self.emitter_type as u8);
        _tmp.put_u8(self.aircraft_size as u8);
        _tmp.put_u8(self.gps_offset_lat as u8);
        _tmp.put_u8(self.gps_offset_lon as u8);
        _tmp.put_u8(self.rf_select as u8);
        _tmp
    }
}
impl crate::proto::uavionix::UavionixAdsbOutDynamic {
    pub const ENCODED_LEN: usize = 41usize;
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
            _struct.utc_time = buf.get_u32_le();
            _struct.gps_lat = buf.get_i32_le();
            _struct.gps_lon = buf.get_i32_le();
            _struct.gps_alt = buf.get_i32_le();
            _struct.baro_alt_msl = buf.get_i32_le();
            _struct.accuracy_hor = buf.get_u32_le();
            _struct.accuracy_vert = buf.get_u16_le() as u32;
            _struct.accuracy_vel = buf.get_u16_le() as u32;
            _struct.vel_vert = buf.get_i16_le() as i32;
            _struct.vel_ns = buf.get_i16_le() as i32;
            _struct.vel_ew = buf.get_i16_le() as i32;
            let tmp = buf.get_u16_le();
            _struct.state = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "UavionixAdsbOutDynamicState".to_string(),
                value: tmp as u32,
            })?;
            _struct.squawk = buf.get_u16_le() as u32;
            let tmp = buf.get_u8();
            _struct.gps_fix = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "UavionixAdsbOutDynamicGpsFix".to_string(),
                value: tmp as u32,
            })?;
            _struct.num_sats = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.emergency_status =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "UavionixAdsbEmergencyStatus".to_string(),
                    value: tmp as u32,
                })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.utc_time as u32);
        _tmp.put_i32_le(self.gps_lat as i32);
        _tmp.put_i32_le(self.gps_lon as i32);
        _tmp.put_i32_le(self.gps_alt as i32);
        _tmp.put_i32_le(self.baro_alt_msl as i32);
        _tmp.put_u32_le(self.accuracy_hor as u32);
        _tmp.put_u16_le(self.accuracy_vert as u16);
        _tmp.put_u16_le(self.accuracy_vel as u16);
        _tmp.put_i16_le(self.vel_vert as i16);
        _tmp.put_i16_le(self.vel_ns as i16);
        _tmp.put_i16_le(self.vel_ew as i16);
        _tmp.put_u16_le(self.state as u16);
        _tmp.put_u16_le(self.squawk as u16);
        _tmp.put_u8(self.gps_fix as u8);
        _tmp.put_u8(self.num_sats as u8);
        _tmp.put_u8(self.emergency_status as u8);
        _tmp
    }
}
impl crate::proto::uavionix::UavionixAdsbTransceiverHealthReport {
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
            _struct.rf_health = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "UavionixAdsbRfHealth".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.rf_health as u8);
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum MavMessage {
    UavionixAdsbOutCfg(crate::proto::uavionix::UavionixAdsbOutCfg),
    UavionixAdsbOutDynamic(crate::proto::uavionix::UavionixAdsbOutDynamic),
    UavionixAdsbTransceiverHealthReport(
        crate::proto::uavionix::UavionixAdsbTransceiverHealthReport,
    ),
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
            10001 => crate::proto::uavionix::UavionixAdsbOutCfg::mavlink_deser(version, payload)
                .map(MavMessage::UavionixAdsbOutCfg),
            10002 => {
                crate::proto::uavionix::UavionixAdsbOutDynamic::mavlink_deser(version, payload)
                    .map(MavMessage::UavionixAdsbOutDynamic)
            }
            10003 => crate::proto::uavionix::UavionixAdsbTransceiverHealthReport::mavlink_deser(
                version, payload,
            )
            .map(MavMessage::UavionixAdsbTransceiverHealthReport),
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
            MavMessage::UavionixAdsbOutCfg(..) => "UavionixAdsbOutCfg",
            MavMessage::UavionixAdsbOutDynamic(..) => "UavionixAdsbOutDynamic",
            MavMessage::UavionixAdsbTransceiverHealthReport(..) => {
                "UavionixAdsbTransceiverHealthReport"
            }
            MavMessage::Common(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::UavionixAdsbOutCfg(..) => 10001,
            MavMessage::UavionixAdsbOutDynamic(..) => 10002,
            MavMessage::UavionixAdsbTransceiverHealthReport(..) => 10003,
            MavMessage::Common(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "UavionixAdsbOutCfg" => Ok(10001),
            "UavionixAdsbOutDynamic" => Ok(10002),
            "UavionixAdsbTransceiverHealthReport" => Ok(10003),
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
            10001 => Ok(MavMessage::UavionixAdsbOutCfg(
                crate::proto::uavionix::UavionixAdsbOutCfg::default(),
            )),
            10002 => Ok(MavMessage::UavionixAdsbOutDynamic(
                crate::proto::uavionix::UavionixAdsbOutDynamic::default(),
            )),
            10003 => Ok(MavMessage::UavionixAdsbTransceiverHealthReport(
                crate::proto::uavionix::UavionixAdsbTransceiverHealthReport::default(),
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
            MavMessage::UavionixAdsbOutCfg(ref body) => body.mavlink_ser(),
            MavMessage::UavionixAdsbOutDynamic(ref body) => body.mavlink_ser(),
            MavMessage::UavionixAdsbTransceiverHealthReport(ref body) => body.mavlink_ser(),
            MavMessage::Common(ref msg) => msg.mavlink_ser(),
        }
    }
    fn proto_encode(&self) -> Vec<u8> {
        match *self {
            MavMessage::UavionixAdsbOutCfg(ref body) => body.encode_to_vec(),
            MavMessage::UavionixAdsbOutDynamic(ref body) => body.encode_to_vec(),
            MavMessage::UavionixAdsbTransceiverHealthReport(ref body) => body.encode_to_vec(),
            MavMessage::Common(ref msg) => msg.proto_encode(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            10001 => 209,
            10002 => 186,
            10003 => 4,
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
