// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.22.

// Section: imports

use super::*;
use crate::libxmtp_api::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate_io!();

#[no_mangle]
pub extern "C" fn frbgen_xmtp_bindings_flutter_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiError(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<ApiError>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_xmtp_bindings_flutter_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiError(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<ApiError>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_xmtp_bindings_flutter_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcInnerClient(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Arc < InnerClient >>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_xmtp_bindings_flutter_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcInnerClient(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Arc < InnerClient >>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_xmtp_bindings_flutter_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Client>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_xmtp_bindings_flutter_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Client>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_xmtp_bindings_flutter_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderError(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<ClientBuilderError>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_xmtp_bindings_flutter_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderError(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<ClientBuilderError>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_xmtp_bindings_flutter_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageError(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<StorageError>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_xmtp_bindings_flutter_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageError(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<StorageError>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_xmtp_bindings_flutter_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowError(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<anyhow :: Error>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_xmtp_bindings_flutter_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowError(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<anyhow :: Error>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_xmtp_bindings_flutter_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientError(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<xmtp_mls :: client :: ClientError>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_xmtp_bindings_flutter_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientError(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<xmtp_mls :: client :: ClientError>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_xmtp_bindings_flutter_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupError(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<xmtp_mls :: groups :: GroupError>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_xmtp_bindings_flutter_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupError(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<xmtp_mls :: groups :: GroupError>>::decrement_strong_count(ptr as _);
}
