#include "discovery.pb.h"
#include "google/protobuf/map.h"
#include "google/protobuf/repeated_field.h"
#include "google/protobuf/repeated_ptr_field.h"
#include "rust/cpp_kernel/serialized_data.h"
#include "rust/cpp_kernel/strings.h"
// nearclip.discovery.DeviceBroadcast
extern "C" {
void* proto2_rust_thunk_Message_nearclip_discovery_DeviceBroadcast_new() { return new ::nearclip::discovery::DeviceBroadcast(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_discovery_DeviceBroadcast_default_instance() {
  return &::nearclip::discovery::DeviceBroadcast::default_instance();
}
::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_id_get(::nearclip::discovery::DeviceBroadcast* msg) {
  absl::string_view val = msg->device_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_id_set(::nearclip::discovery::DeviceBroadcast* msg, std::string* s) {
  msg->set_device_id(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_name_get(::nearclip::discovery::DeviceBroadcast* msg) {
  absl::string_view val = msg->device_name();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_name_set(::nearclip::discovery::DeviceBroadcast* msg, std::string* s) {
  msg->set_device_name(std::move(*s));
  delete s;
}

::nearclip::discovery::DeviceType proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_type_get(::nearclip::discovery::DeviceBroadcast* msg) {
  return msg->device_type();
}
void proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_type_set(::nearclip::discovery::DeviceBroadcast* msg, ::nearclip::discovery::DeviceType val) {
  msg->set_device_type(val);
}

google::protobuf::RepeatedField<int>* proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_capabilities_get_mut(
    ::nearclip::discovery::DeviceBroadcast* msg) {
  return msg->mutable_capabilities();
}
const google::protobuf::RepeatedField<int>* proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_capabilities_get(
    const ::nearclip::discovery::DeviceBroadcast* msg) {
  return &msg->capabilities();
}
void proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_capabilities_move_set(
    ::nearclip::discovery::DeviceBroadcast* msg,
    google::protobuf::RepeatedField<int>* value) {
  *msg->mutable_capabilities() = std::move(*value);
  delete value;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_version_get(::nearclip::discovery::DeviceBroadcast* msg) {
  absl::string_view val = msg->version();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_version_set(::nearclip::discovery::DeviceBroadcast* msg, std::string* s) {
  msg->set_version(std::move(*s));
  delete s;
}

::uint64_t proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_timestamp_get(::nearclip::discovery::DeviceBroadcast* msg) {
  return msg->timestamp();
}
void proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_timestamp_set(::nearclip::discovery::DeviceBroadcast* msg, ::uint64_t val) {
  msg->set_timestamp(val);
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_public_key_get(::nearclip::discovery::DeviceBroadcast* msg) {
  absl::string_view val = msg->public_key();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_public_key_set(::nearclip::discovery::DeviceBroadcast* msg, std::string* s) {
  msg->set_public_key(std::move(*s));
  delete s;
}

const void* proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_metadata_get(const ::nearclip::discovery::DeviceBroadcast* msg) {
  return &msg->metadata();
}
void* proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_metadata_get_mut(::nearclip::discovery::DeviceBroadcast* msg) { return msg->mutable_metadata(); }
void proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_metadata_set(::nearclip::discovery::DeviceBroadcast* msg,
                         google::protobuf::Map<std::string, std::string>* value) {
  *msg->mutable_metadata() = std::move(*value);
  delete value;
}

}  //extern "C"

// nearclip.discovery.ScanRequest
extern "C" {
void* proto2_rust_thunk_Message_nearclip_discovery_ScanRequest_new() { return new ::nearclip::discovery::ScanRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_discovery_ScanRequest_default_instance() {
  return &::nearclip::discovery::ScanRequest::default_instance();
}
::uint32_t proto2_rust_thunk_nearclip_discovery_ScanRequest_timeout_seconds_get(::nearclip::discovery::ScanRequest* msg) {
  return msg->timeout_seconds();
}
void proto2_rust_thunk_nearclip_discovery_ScanRequest_timeout_seconds_set(::nearclip::discovery::ScanRequest* msg, ::uint32_t val) {
  msg->set_timeout_seconds(val);
}

google::protobuf::RepeatedField<int>* proto2_rust_thunk_nearclip_discovery_ScanRequest_filter_types_get_mut(
    ::nearclip::discovery::ScanRequest* msg) {
  return msg->mutable_filter_types();
}
const google::protobuf::RepeatedField<int>* proto2_rust_thunk_nearclip_discovery_ScanRequest_filter_types_get(
    const ::nearclip::discovery::ScanRequest* msg) {
  return &msg->filter_types();
}
void proto2_rust_thunk_nearclip_discovery_ScanRequest_filter_types_move_set(
    ::nearclip::discovery::ScanRequest* msg,
    google::protobuf::RepeatedField<int>* value) {
  *msg->mutable_filter_types() = std::move(*value);
  delete value;
}

google::protobuf::RepeatedField<int>* proto2_rust_thunk_nearclip_discovery_ScanRequest_required_capabilities_get_mut(
    ::nearclip::discovery::ScanRequest* msg) {
  return msg->mutable_required_capabilities();
}
const google::protobuf::RepeatedField<int>* proto2_rust_thunk_nearclip_discovery_ScanRequest_required_capabilities_get(
    const ::nearclip::discovery::ScanRequest* msg) {
  return &msg->required_capabilities();
}
void proto2_rust_thunk_nearclip_discovery_ScanRequest_required_capabilities_move_set(
    ::nearclip::discovery::ScanRequest* msg,
    google::protobuf::RepeatedField<int>* value) {
  *msg->mutable_required_capabilities() = std::move(*value);
  delete value;
}

}  //extern "C"

// nearclip.discovery.ScanResponse
extern "C" {
void* proto2_rust_thunk_Message_nearclip_discovery_ScanResponse_new() { return new ::nearclip::discovery::ScanResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_discovery_ScanResponse_default_instance() {
  return &::nearclip::discovery::ScanResponse::default_instance();
}
google::protobuf::RepeatedPtrField<::nearclip::discovery::DeviceBroadcast>* proto2_rust_thunk_nearclip_discovery_ScanResponse_devices_get_mut(
    ::nearclip::discovery::ScanResponse* msg) {
  return msg->mutable_devices();
}
const google::protobuf::RepeatedPtrField<::nearclip::discovery::DeviceBroadcast>* proto2_rust_thunk_nearclip_discovery_ScanResponse_devices_get(
    const ::nearclip::discovery::ScanResponse* msg) {
  return &msg->devices();
}
void proto2_rust_thunk_nearclip_discovery_ScanResponse_devices_move_set(
    ::nearclip::discovery::ScanResponse* msg,
    google::protobuf::RepeatedPtrField<::nearclip::discovery::DeviceBroadcast>* value) {
  *msg->mutable_devices() = std::move(*value);
  delete value;
}

::uint64_t proto2_rust_thunk_nearclip_discovery_ScanResponse_scan_duration_ms_get(::nearclip::discovery::ScanResponse* msg) {
  return msg->scan_duration_ms();
}
void proto2_rust_thunk_nearclip_discovery_ScanResponse_scan_duration_ms_set(::nearclip::discovery::ScanResponse* msg, ::uint64_t val) {
  msg->set_scan_duration_ms(val);
}

}  //extern "C"

// nearclip.discovery.DeviceQuery
extern "C" {
void* proto2_rust_thunk_Message_nearclip_discovery_DeviceQuery_new() { return new ::nearclip::discovery::DeviceQuery(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_discovery_DeviceQuery_default_instance() {
  return &::nearclip::discovery::DeviceQuery::default_instance();
}
::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_discovery_DeviceQuery_device_id_get(::nearclip::discovery::DeviceQuery* msg) {
  absl::string_view val = msg->device_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_discovery_DeviceQuery_device_id_set(::nearclip::discovery::DeviceQuery* msg, std::string* s) {
  msg->set_device_id(std::move(*s));
  delete s;
}

google::protobuf::RepeatedField<int>* proto2_rust_thunk_nearclip_discovery_DeviceQuery_capabilities_get_mut(
    ::nearclip::discovery::DeviceQuery* msg) {
  return msg->mutable_capabilities();
}
const google::protobuf::RepeatedField<int>* proto2_rust_thunk_nearclip_discovery_DeviceQuery_capabilities_get(
    const ::nearclip::discovery::DeviceQuery* msg) {
  return &msg->capabilities();
}
void proto2_rust_thunk_nearclip_discovery_DeviceQuery_capabilities_move_set(
    ::nearclip::discovery::DeviceQuery* msg,
    google::protobuf::RepeatedField<int>* value) {
  *msg->mutable_capabilities() = std::move(*value);
  delete value;
}

}  //extern "C"

// nearclip.discovery.DeviceQueryResponse
extern "C" {
void* proto2_rust_thunk_Message_nearclip_discovery_DeviceQueryResponse_new() { return new ::nearclip::discovery::DeviceQueryResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_discovery_DeviceQueryResponse_default_instance() {
  return &::nearclip::discovery::DeviceQueryResponse::default_instance();
}
bool proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_has(::nearclip::discovery::DeviceQueryResponse* msg) {
  return msg->has_device();
}
void proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_clear(::nearclip::discovery::DeviceQueryResponse* msg) { msg->clear_device(); }
const void* proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_get(::nearclip::discovery::DeviceQueryResponse* msg) {
  return static_cast<const void*>(&msg->device());
}
void* proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_get_mut(::nearclip::discovery::DeviceQueryResponse* msg) {
  return static_cast<void*>(msg->mutable_device());
}
void proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_set(::nearclip::discovery::DeviceQueryResponse* msg, ::nearclip::discovery::DeviceBroadcast* sub_msg) {
  msg->set_allocated_device(sub_msg);
}

bool proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_is_online_get(::nearclip::discovery::DeviceQueryResponse* msg) {
  return msg->is_online();
}
void proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_is_online_set(::nearclip::discovery::DeviceQueryResponse* msg, bool val) {
  msg->set_is_online(val);
}

::uint64_t proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_last_seen_get(::nearclip::discovery::DeviceQueryResponse* msg) {
  return msg->last_seen();
}
void proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_last_seen_set(::nearclip::discovery::DeviceQueryResponse* msg, ::uint64_t val) {
  msg->set_last_seen(val);
}

}  //extern "C"

// nearclip.discovery.DeviceCapability

// nearclip.discovery.DeviceType

