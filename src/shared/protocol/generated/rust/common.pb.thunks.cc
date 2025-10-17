#include "common.pb.h"
#include "google/protobuf/map.h"
#include "google/protobuf/repeated_field.h"
#include "google/protobuf/repeated_ptr_field.h"
#include "rust/cpp_kernel/serialized_data.h"
#include "rust/cpp_kernel/strings.h"
// nearclip.common.ErrorMessage
extern "C" {
void* proto2_rust_thunk_Message_nearclip_common_ErrorMessage_new() { return new ::nearclip::common::ErrorMessage(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_common_ErrorMessage_default_instance() {
  return &::nearclip::common::ErrorMessage::default_instance();
}
::nearclip::common::ErrorCode proto2_rust_thunk_nearclip_common_ErrorMessage_code_get(::nearclip::common::ErrorMessage* msg) {
  return msg->code();
}
void proto2_rust_thunk_nearclip_common_ErrorMessage_code_set(::nearclip::common::ErrorMessage* msg, ::nearclip::common::ErrorCode val) {
  msg->set_code(val);
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_common_ErrorMessage_message_get(::nearclip::common::ErrorMessage* msg) {
  absl::string_view val = msg->message();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_common_ErrorMessage_message_set(::nearclip::common::ErrorMessage* msg, std::string* s) {
  msg->set_message(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_common_ErrorMessage_details_get(::nearclip::common::ErrorMessage* msg) {
  absl::string_view val = msg->details();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_common_ErrorMessage_details_set(::nearclip::common::ErrorMessage* msg, std::string* s) {
  msg->set_details(std::move(*s));
  delete s;
}

::uint64_t proto2_rust_thunk_nearclip_common_ErrorMessage_timestamp_get(::nearclip::common::ErrorMessage* msg) {
  return msg->timestamp();
}
void proto2_rust_thunk_nearclip_common_ErrorMessage_timestamp_set(::nearclip::common::ErrorMessage* msg, ::uint64_t val) {
  msg->set_timestamp(val);
}

}  //extern "C"

// nearclip.common.Heartbeat
extern "C" {
void* proto2_rust_thunk_Message_nearclip_common_Heartbeat_new() { return new ::nearclip::common::Heartbeat(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_common_Heartbeat_default_instance() {
  return &::nearclip::common::Heartbeat::default_instance();
}
::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_common_Heartbeat_device_id_get(::nearclip::common::Heartbeat* msg) {
  absl::string_view val = msg->device_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_common_Heartbeat_device_id_set(::nearclip::common::Heartbeat* msg, std::string* s) {
  msg->set_device_id(std::move(*s));
  delete s;
}

::uint64_t proto2_rust_thunk_nearclip_common_Heartbeat_timestamp_get(::nearclip::common::Heartbeat* msg) {
  return msg->timestamp();
}
void proto2_rust_thunk_nearclip_common_Heartbeat_timestamp_set(::nearclip::common::Heartbeat* msg, ::uint64_t val) {
  msg->set_timestamp(val);
}

::uint32_t proto2_rust_thunk_nearclip_common_Heartbeat_sequence_number_get(::nearclip::common::Heartbeat* msg) {
  return msg->sequence_number();
}
void proto2_rust_thunk_nearclip_common_Heartbeat_sequence_number_set(::nearclip::common::Heartbeat* msg, ::uint32_t val) {
  msg->set_sequence_number(val);
}

}  //extern "C"

// nearclip.common.HeartbeatAck
extern "C" {
void* proto2_rust_thunk_Message_nearclip_common_HeartbeatAck_new() { return new ::nearclip::common::HeartbeatAck(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_common_HeartbeatAck_default_instance() {
  return &::nearclip::common::HeartbeatAck::default_instance();
}
::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_common_HeartbeatAck_device_id_get(::nearclip::common::HeartbeatAck* msg) {
  absl::string_view val = msg->device_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_common_HeartbeatAck_device_id_set(::nearclip::common::HeartbeatAck* msg, std::string* s) {
  msg->set_device_id(std::move(*s));
  delete s;
}

::uint64_t proto2_rust_thunk_nearclip_common_HeartbeatAck_received_timestamp_get(::nearclip::common::HeartbeatAck* msg) {
  return msg->received_timestamp();
}
void proto2_rust_thunk_nearclip_common_HeartbeatAck_received_timestamp_set(::nearclip::common::HeartbeatAck* msg, ::uint64_t val) {
  msg->set_received_timestamp(val);
}

::uint32_t proto2_rust_thunk_nearclip_common_HeartbeatAck_sequence_number_get(::nearclip::common::HeartbeatAck* msg) {
  return msg->sequence_number();
}
void proto2_rust_thunk_nearclip_common_HeartbeatAck_sequence_number_set(::nearclip::common::HeartbeatAck* msg, ::uint32_t val) {
  msg->set_sequence_number(val);
}

}  //extern "C"

// nearclip.common.ProtocolVersion
extern "C" {
void* proto2_rust_thunk_Message_nearclip_common_ProtocolVersion_new() { return new ::nearclip::common::ProtocolVersion(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_common_ProtocolVersion_default_instance() {
  return &::nearclip::common::ProtocolVersion::default_instance();
}
::uint32_t proto2_rust_thunk_nearclip_common_ProtocolVersion_major_get(::nearclip::common::ProtocolVersion* msg) {
  return msg->major();
}
void proto2_rust_thunk_nearclip_common_ProtocolVersion_major_set(::nearclip::common::ProtocolVersion* msg, ::uint32_t val) {
  msg->set_major(val);
}

::uint32_t proto2_rust_thunk_nearclip_common_ProtocolVersion_minor_get(::nearclip::common::ProtocolVersion* msg) {
  return msg->minor();
}
void proto2_rust_thunk_nearclip_common_ProtocolVersion_minor_set(::nearclip::common::ProtocolVersion* msg, ::uint32_t val) {
  msg->set_minor(val);
}

::uint32_t proto2_rust_thunk_nearclip_common_ProtocolVersion_patch_get(::nearclip::common::ProtocolVersion* msg) {
  return msg->patch();
}
void proto2_rust_thunk_nearclip_common_ProtocolVersion_patch_set(::nearclip::common::ProtocolVersion* msg, ::uint32_t val) {
  msg->set_patch(val);
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_common_ProtocolVersion_build_info_get(::nearclip::common::ProtocolVersion* msg) {
  absl::string_view val = msg->build_info();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_common_ProtocolVersion_build_info_set(::nearclip::common::ProtocolVersion* msg, std::string* s) {
  msg->set_build_info(std::move(*s));
  delete s;
}

}  //extern "C"

// nearclip.common.CapabilityNegotiation
extern "C" {
void* proto2_rust_thunk_Message_nearclip_common_CapabilityNegotiation_new() { return new ::nearclip::common::CapabilityNegotiation(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_common_CapabilityNegotiation_default_instance() {
  return &::nearclip::common::CapabilityNegotiation::default_instance();
}
bool proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_has(::nearclip::common::CapabilityNegotiation* msg) {
  return msg->has_min_version();
}
void proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_clear(::nearclip::common::CapabilityNegotiation* msg) { msg->clear_min_version(); }
const void* proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_get(::nearclip::common::CapabilityNegotiation* msg) {
  return static_cast<const void*>(&msg->min_version());
}
void* proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_get_mut(::nearclip::common::CapabilityNegotiation* msg) {
  return static_cast<void*>(msg->mutable_min_version());
}
void proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_set(::nearclip::common::CapabilityNegotiation* msg, ::nearclip::common::ProtocolVersion* sub_msg) {
  msg->set_allocated_min_version(sub_msg);
}

bool proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_has(::nearclip::common::CapabilityNegotiation* msg) {
  return msg->has_max_version();
}
void proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_clear(::nearclip::common::CapabilityNegotiation* msg) { msg->clear_max_version(); }
const void* proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_get(::nearclip::common::CapabilityNegotiation* msg) {
  return static_cast<const void*>(&msg->max_version());
}
void* proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_get_mut(::nearclip::common::CapabilityNegotiation* msg) {
  return static_cast<void*>(msg->mutable_max_version());
}
void proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_set(::nearclip::common::CapabilityNegotiation* msg, ::nearclip::common::ProtocolVersion* sub_msg) {
  msg->set_allocated_max_version(sub_msg);
}

google::protobuf::RepeatedPtrField<std::string>* proto2_rust_thunk_nearclip_common_CapabilityNegotiation_supported_features_get_mut(
    ::nearclip::common::CapabilityNegotiation* msg) {
  return msg->mutable_supported_features();
}
const google::protobuf::RepeatedPtrField<std::string>* proto2_rust_thunk_nearclip_common_CapabilityNegotiation_supported_features_get(
    const ::nearclip::common::CapabilityNegotiation* msg) {
  return &msg->supported_features();
}
void proto2_rust_thunk_nearclip_common_CapabilityNegotiation_supported_features_move_set(
    ::nearclip::common::CapabilityNegotiation* msg,
    google::protobuf::RepeatedPtrField<std::string>* value) {
  *msg->mutable_supported_features() = std::move(*value);
  delete value;
}

google::protobuf::RepeatedPtrField<std::string>* proto2_rust_thunk_nearclip_common_CapabilityNegotiation_required_features_get_mut(
    ::nearclip::common::CapabilityNegotiation* msg) {
  return msg->mutable_required_features();
}
const google::protobuf::RepeatedPtrField<std::string>* proto2_rust_thunk_nearclip_common_CapabilityNegotiation_required_features_get(
    const ::nearclip::common::CapabilityNegotiation* msg) {
  return &msg->required_features();
}
void proto2_rust_thunk_nearclip_common_CapabilityNegotiation_required_features_move_set(
    ::nearclip::common::CapabilityNegotiation* msg,
    google::protobuf::RepeatedPtrField<std::string>* value) {
  *msg->mutable_required_features() = std::move(*value);
  delete value;
}

}  //extern "C"

// nearclip.common.CapabilityNegotiationResponse
extern "C" {
void* proto2_rust_thunk_Message_nearclip_common_CapabilityNegotiationResponse_new() { return new ::nearclip::common::CapabilityNegotiationResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_common_CapabilityNegotiationResponse_default_instance() {
  return &::nearclip::common::CapabilityNegotiationResponse::default_instance();
}
bool proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_has(::nearclip::common::CapabilityNegotiationResponse* msg) {
  return msg->has_selected_version();
}
void proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_clear(::nearclip::common::CapabilityNegotiationResponse* msg) { msg->clear_selected_version(); }
const void* proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_get(::nearclip::common::CapabilityNegotiationResponse* msg) {
  return static_cast<const void*>(&msg->selected_version());
}
void* proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_get_mut(::nearclip::common::CapabilityNegotiationResponse* msg) {
  return static_cast<void*>(msg->mutable_selected_version());
}
void proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_set(::nearclip::common::CapabilityNegotiationResponse* msg, ::nearclip::common::ProtocolVersion* sub_msg) {
  msg->set_allocated_selected_version(sub_msg);
}

google::protobuf::RepeatedPtrField<std::string>* proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_supported_features_get_mut(
    ::nearclip::common::CapabilityNegotiationResponse* msg) {
  return msg->mutable_supported_features();
}
const google::protobuf::RepeatedPtrField<std::string>* proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_supported_features_get(
    const ::nearclip::common::CapabilityNegotiationResponse* msg) {
  return &msg->supported_features();
}
void proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_supported_features_move_set(
    ::nearclip::common::CapabilityNegotiationResponse* msg,
    google::protobuf::RepeatedPtrField<std::string>* value) {
  *msg->mutable_supported_features() = std::move(*value);
  delete value;
}

google::protobuf::RepeatedPtrField<std::string>* proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_unsupported_features_get_mut(
    ::nearclip::common::CapabilityNegotiationResponse* msg) {
  return msg->mutable_unsupported_features();
}
const google::protobuf::RepeatedPtrField<std::string>* proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_unsupported_features_get(
    const ::nearclip::common::CapabilityNegotiationResponse* msg) {
  return &msg->unsupported_features();
}
void proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_unsupported_features_move_set(
    ::nearclip::common::CapabilityNegotiationResponse* msg,
    google::protobuf::RepeatedPtrField<std::string>* value) {
  *msg->mutable_unsupported_features() = std::move(*value);
  delete value;
}

bool proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_compatibility_get(::nearclip::common::CapabilityNegotiationResponse* msg) {
  return msg->compatibility();
}
void proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_compatibility_set(::nearclip::common::CapabilityNegotiationResponse* msg, bool val) {
  msg->set_compatibility(val);
}

}  //extern "C"

// nearclip.common.ErrorCode

