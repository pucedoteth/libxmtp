// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishClientEnvelopesRequest {
    #[prost(message, repeated, tag="1")]
    pub envelopes: ::prost::alloc::vec::Vec<super::envelopes::ClientEnvelope>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishClientEnvelopesResponse {
    #[prost(message, repeated, tag="1")]
    pub originator_envelopes: ::prost::alloc::vec::Vec<super::envelopes::OriginatorEnvelope>,
}
/// Encoded file descriptor set for the `xmtp.xmtpv4.payer_api` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xfa, 0x09, 0x0a, 0x20, 0x78, 0x6d, 0x74, 0x70, 0x76, 0x34, 0x2f, 0x70, 0x61, 0x79, 0x65,
    0x72, 0x5f, 0x61, 0x70, 0x69, 0x2f, 0x70, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x61, 0x70, 0x69, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x15, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x78, 0x6d, 0x74, 0x70,
    0x76, 0x34, 0x2e, 0x70, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x61, 0x70, 0x69, 0x1a, 0x1c, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x78, 0x6d, 0x74, 0x70,
    0x76, 0x34, 0x2f, 0x65, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2f, 0x65, 0x6e, 0x76,
    0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x64, 0x0a, 0x1d,
    0x50, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x45, 0x6e, 0x76,
    0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x43, 0x0a,
    0x09, 0x65, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x25, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x76, 0x34, 0x2e, 0x65,
    0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x45,
    0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x52, 0x09, 0x65, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70,
    0x65, 0x73, 0x22, 0x7e, 0x0a, 0x1e, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x43, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x5c, 0x0a, 0x14, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x74,
    0x6f, 0x72, 0x5f, 0x65, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x29, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x76, 0x34,
    0x2e, 0x65, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x4f, 0x72, 0x69, 0x67, 0x69,
    0x6e, 0x61, 0x74, 0x6f, 0x72, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x52, 0x13, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x74, 0x6f, 0x72, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70,
    0x65, 0x73, 0x32, 0xc5, 0x01, 0x0a, 0x08, 0x50, 0x61, 0x79, 0x65, 0x72, 0x41, 0x70, 0x69, 0x12,
    0xb8, 0x01, 0x0a, 0x16, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x12, 0x34, 0x2e, 0x78, 0x6d, 0x74,
    0x70, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x76, 0x34, 0x2e, 0x70, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x61,
    0x70, 0x69, 0x2e, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x1a, 0x35, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x76, 0x34, 0x2e, 0x70,
    0x61, 0x79, 0x65, 0x72, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68,
    0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x31, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x2b, 0x3a,
    0x01, 0x2a, 0x22, 0x26, 0x2f, 0x6d, 0x6c, 0x73, 0x2f, 0x76, 0x32, 0x2f, 0x70, 0x61, 0x79, 0x65,
    0x72, 0x2f, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x2d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x2d, 0x65, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x42, 0xca, 0x01, 0x0a, 0x19, 0x63,
    0x6f, 0x6d, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x76, 0x34, 0x2e, 0x70,
    0x61, 0x79, 0x65, 0x72, 0x5f, 0x61, 0x70, 0x69, 0x42, 0x0d, 0x50, 0x61, 0x79, 0x65, 0x72, 0x41,
    0x70, 0x69, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x2c, 0x67, 0x69, 0x74, 0x68, 0x75,
    0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x78, 0x6d, 0x74, 0x70, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2f, 0x76, 0x33, 0x2f, 0x67, 0x6f, 0x2f, 0x78, 0x6d, 0x74, 0x70, 0x76, 0x34, 0x2f, 0x70, 0x61,
    0x79, 0x65, 0x72, 0x5f, 0x61, 0x70, 0x69, 0xa2, 0x02, 0x03, 0x58, 0x58, 0x50, 0xaa, 0x02, 0x14,
    0x58, 0x6d, 0x74, 0x70, 0x2e, 0x58, 0x6d, 0x74, 0x70, 0x76, 0x34, 0x2e, 0x50, 0x61, 0x79, 0x65,
    0x72, 0x41, 0x70, 0x69, 0xca, 0x02, 0x14, 0x58, 0x6d, 0x74, 0x70, 0x5c, 0x58, 0x6d, 0x74, 0x70,
    0x76, 0x34, 0x5c, 0x50, 0x61, 0x79, 0x65, 0x72, 0x41, 0x70, 0x69, 0xe2, 0x02, 0x20, 0x58, 0x6d,
    0x74, 0x70, 0x5c, 0x58, 0x6d, 0x74, 0x70, 0x76, 0x34, 0x5c, 0x50, 0x61, 0x79, 0x65, 0x72, 0x41,
    0x70, 0x69, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02,
    0x16, 0x58, 0x6d, 0x74, 0x70, 0x3a, 0x3a, 0x58, 0x6d, 0x74, 0x70, 0x76, 0x34, 0x3a, 0x3a, 0x50,
    0x61, 0x79, 0x65, 0x72, 0x41, 0x70, 0x69, 0x4a, 0xfb, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x01, 0x00,
    0x1b, 0x01, 0x0a, 0x15, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x01, 0x00, 0x12, 0x1a, 0x0b, 0x20, 0x50,
    0x61, 0x79, 0x65, 0x72, 0x20, 0x41, 0x50, 0x49, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x03, 0x00, 0x1e, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x05, 0x00, 0x26, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x00, 0x2a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x08, 0x00, 0x43, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x08, 0x00, 0x43, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0a, 0x00, 0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x0a, 0x08, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x0b, 0x02, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0b, 0x0b, 0x2f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x30, 0x39, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x3c, 0x3d, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x0e, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x0e, 0x08, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x02, 0x4d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0f, 0x0b, 0x33, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x34, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x4b, 0x4c, 0x0a, 0x4b, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04,
    0x13, 0x00, 0x1b, 0x01, 0x1a, 0x3f, 0x20, 0x41, 0x20, 0x6e, 0x61, 0x72, 0x72, 0x6f, 0x77, 0x6c,
    0x79, 0x20, 0x73, 0x63, 0x6f, 0x70, 0x65, 0x64, 0x20, 0x41, 0x50, 0x49, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x20, 0x74, 0x68, 0x72, 0x6f, 0x75, 0x67, 0x68, 0x20, 0x61, 0x20, 0x70,
    0x61, 0x79, 0x65, 0x72, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x13, 0x08,
    0x10, 0x0a, 0x20, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x04, 0x15, 0x02, 0x1a, 0x03, 0x1a,
    0x12, 0x20, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x20, 0x65, 0x6e, 0x76, 0x65, 0x6c, 0x6f,
    0x70, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x06,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x15, 0x1d, 0x3a, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x15, 0x45, 0x63, 0x0a, 0x0d, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x16, 0x04, 0x19, 0x06, 0x0a, 0x11, 0x0a, 0x09,
    0x06, 0x00, 0x02, 0x00, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04, 0x16, 0x04, 0x19, 0x06, 0x0a,
    0x11, 0x0a, 0x0a, 0x06, 0x00, 0x02, 0x00, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x04, 0x12, 0x03, 0x17,
    0x06, 0x34, 0x0a, 0x11, 0x0a, 0x0a, 0x06, 0x00, 0x02, 0x00, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x07,
    0x12, 0x03, 0x18, 0x06, 0x0f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("xmtp.xmtpv4.payer_api.serde.rs");
include!("xmtp.xmtpv4.payer_api.tonic.rs");
// @@protoc_insertion_point(module)