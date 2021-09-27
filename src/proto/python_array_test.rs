/// Array test #0.
///
/// MavLink id: 150
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ArrayTest0 {
    /// Value array
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub ar_u32: ::prost::alloc::vec::Vec<u32>,
    /// Value array
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub ar_u16: ::prost::alloc::vec::Vec<u32>,
    /// Stub field
    #[prost(uint32, tag = "3")]
    pub v1: u32,
    /// Value array
    #[prost(int32, repeated, packed = "false", tag = "4")]
    pub ar_i8: ::prost::alloc::vec::Vec<i32>,
    /// Value array
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub ar_u8: ::prost::alloc::vec::Vec<u32>,
}
/// Array test #1.
///
/// MavLink id: 151
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ArrayTest1 {
    /// Value array
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub ar_u32: ::prost::alloc::vec::Vec<u32>,
}
/// Array test #3.
///
/// MavLink id: 153
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ArrayTest3 {
    /// Value array
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub ar_u32: ::prost::alloc::vec::Vec<u32>,
    /// Stub field
    #[prost(uint32, tag = "2")]
    pub v: u32,
}
/// Array test #4.
///
/// MavLink id: 154
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ArrayTest4 {
    /// Value array
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub ar_u32: ::prost::alloc::vec::Vec<u32>,
    /// Stub field
    #[prost(uint32, tag = "2")]
    pub v: u32,
}
/// Array test #5.
///
/// MavLink id: 155
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ArrayTest5 {
    /// Value array
    #[prost(string, tag = "1")]
    pub c1: ::prost::alloc::string::String,
    /// Value array
    #[prost(string, tag = "2")]
    pub c2: ::prost::alloc::string::String,
}
/// Array test #6.
///
/// MavLink id: 156
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ArrayTest6 {
    /// Value array
    #[prost(double, repeated, packed = "false", tag = "1")]
    pub ar_d: ::prost::alloc::vec::Vec<f64>,
    /// Stub field
    #[prost(uint32, tag = "2")]
    pub v3: u32,
    /// Value array
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub ar_u32: ::prost::alloc::vec::Vec<u32>,
    /// Value array
    #[prost(int32, repeated, packed = "false", tag = "4")]
    pub ar_i32: ::prost::alloc::vec::Vec<i32>,
    /// Value array
    #[prost(float, repeated, packed = "false", tag = "5")]
    pub ar_f: ::prost::alloc::vec::Vec<f32>,
    /// Stub field
    #[prost(uint32, tag = "6")]
    pub v2: u32,
    /// Value array
    #[prost(uint32, repeated, packed = "false", tag = "7")]
    pub ar_u16: ::prost::alloc::vec::Vec<u32>,
    /// Value array
    #[prost(int32, repeated, packed = "false", tag = "8")]
    pub ar_i16: ::prost::alloc::vec::Vec<i32>,
    /// Stub field
    #[prost(uint32, tag = "9")]
    pub v1: u32,
    /// Value array
    #[prost(uint32, repeated, packed = "false", tag = "10")]
    pub ar_u8: ::prost::alloc::vec::Vec<u32>,
    /// Value array
    #[prost(int32, repeated, packed = "false", tag = "11")]
    pub ar_i8: ::prost::alloc::vec::Vec<i32>,
    /// Value array
    #[prost(string, tag = "12")]
    pub ar_c: ::prost::alloc::string::String,
}
/// Array test #7.
///
/// MavLink id: 157
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ArrayTest7 {
    /// Value array
    #[prost(double, repeated, packed = "false", tag = "1")]
    pub ar_d: ::prost::alloc::vec::Vec<f64>,
    /// Value array
    #[prost(float, repeated, packed = "false", tag = "2")]
    pub ar_f: ::prost::alloc::vec::Vec<f32>,
    /// Value array
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub ar_u32: ::prost::alloc::vec::Vec<u32>,
    /// Value array
    #[prost(int32, repeated, packed = "false", tag = "4")]
    pub ar_i32: ::prost::alloc::vec::Vec<i32>,
    /// Value array
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub ar_u16: ::prost::alloc::vec::Vec<u32>,
    /// Value array
    #[prost(int32, repeated, packed = "false", tag = "6")]
    pub ar_i16: ::prost::alloc::vec::Vec<i32>,
    /// Value array
    #[prost(uint32, repeated, packed = "false", tag = "7")]
    pub ar_u8: ::prost::alloc::vec::Vec<u32>,
    /// Value array
    #[prost(int32, repeated, packed = "false", tag = "8")]
    pub ar_i8: ::prost::alloc::vec::Vec<i32>,
    /// Value array
    #[prost(string, tag = "9")]
    pub ar_c: ::prost::alloc::string::String,
}
/// Array test #8.
///
/// MavLink id: 158
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ArrayTest8 {
    /// Value array
    #[prost(double, repeated, packed = "false", tag = "1")]
    pub ar_d: ::prost::alloc::vec::Vec<f64>,
    /// Stub field
    #[prost(uint32, tag = "2")]
    pub v3: u32,
    /// Value array
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub ar_u16: ::prost::alloc::vec::Vec<u32>,
}
