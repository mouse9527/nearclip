#include "pairing.pb.h"
#include "google/protobuf/map.h"
#include "google/protobuf/repeated_field.h"
#include "google/protobuf/repeated_ptr_field.h"
#include "rust/cpp_kernel/serialized_data.h"
#include "rust/cpp_kernel/strings.h"
// nearclip.pairing.PairingRequest
extern "C" {
void* proto2_rust_thunk_Message_nearclip_pairing_PairingRequest_new() { return new ::nearclip::pairing::PairingRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_pairing_PairingRequest_default_instance() {
  return &::nearclip::pairing::PairingRequest::default_instance();
}
::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_pairing_PairingRequest_initiator_id_get(::nearclip::pairing::PairingRequest* msg) {
  absl::string_view val = msg->initiator_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_pairing_PairingRequest_initiator_id_set(::nearclip::pairing::PairingRequest* msg, std::string* s) {
  msg->set_initiator_id(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_pairing_PairingRequest_target_id_get(::nearclip::pairing::PairingRequest* msg) {
  absl::string_view val = msg->target_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_pairing_PairingRequest_target_id_set(::nearclip::pairing::PairingRequest* msg, std::string* s) {
  msg->set_target_id(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_pairing_PairingRequest_public_key_get(::nearclip::pairing::PairingRequest* msg) {
  absl::string_view val = msg->public_key();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_pairing_PairingRequest_public_key_set(::nearclip::pairing::PairingRequest* msg, std::string* s) {
  msg->set_public_key(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_pairing_PairingRequest_device_name_get(::nearclip::pairing::PairingRequest* msg) {
  absl::string_view val = msg->device_name();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_pairing_PairingRequest_device_name_set(::nearclip::pairing::PairingRequest* msg, std::string* s) {
  msg->set_device_name(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_pairing_PairingRequest_nonce_get(::nearclip::pairing::PairingRequest* msg) {
  absl::string_view val = msg->nonce();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_pairing_PairingRequest_nonce_set(::nearclip::pairing::PairingRequest* msg, std::string* s) {
  msg->set_nonce(std::move(*s));
  delete s;
}

::uint64_t proto2_rust_thunk_nearclip_pairing_PairingRequest_timestamp_get(::nearclip::pairing::PairingRequest* msg) {
  return msg->timestamp();
}
void proto2_rust_thunk_nearclip_pairing_PairingRequest_timestamp_set(::nearclip::pairing::PairingRequest* msg, ::uint64_t val) {
  msg->set_timestamp(val);
}

}  //extern "C"

// nearclip.pairing.PairingResponse
extern "C" {
void* proto2_rust_thunk_Message_nearclip_pairing_PairingResponse_new() { return new ::nearclip::pairing::PairingResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_pairing_PairingResponse_default_instance() {
  return &::nearclip::pairing::PairingResponse::default_instance();
}
::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_pairing_PairingResponse_responder_id_get(::nearclip::pairing::PairingResponse* msg) {
  absl::string_view val = msg->responder_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_pairing_PairingResponse_responder_id_set(::nearclip::pairing::PairingResponse* msg, std::string* s) {
  msg->set_responder_id(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_pairing_PairingResponse_initiator_id_get(::nearclip::pairing::PairingResponse* msg) {
  absl::string_view val = msg->initiator_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_pairing_PairingResponse_initiator_id_set(::nearclip::pairing::PairingResponse* msg, std::string* s) {
  msg->set_initiator_id(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_pairing_PairingResponse_public_key_get(::nearclip::pairing::PairingResponse* msg) {
  absl::string_view val = msg->public_key();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_pairing_PairingResponse_public_key_set(::nearclip::pairing::PairingResponse* msg, std::string* s) {
  msg->set_public_key(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_pairing_PairingResponse_signed_nonce_get(::nearclip::pairing::PairingResponse* msg) {
  absl::string_view val = msg->signed_nonce();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_pairing_PairingResponse_signed_nonce_set(::nearclip::pairing::PairingResponse* msg, std::string* s) {
  msg->set_signed_nonce(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_pairing_PairingResponse_shared_secret_get(::nearclip::pairing::PairingResponse* msg) {
  absl::string_view val = msg->shared_secret();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_pairing_PairingResponse_shared_secret_set(::nearclip::pairing::PairingResponse* msg, std::string* s) {
  msg->set_shared_secret(std::move(*s));
  delete s;
}

::uint64_t proto2_rust_thunk_nearclip_pairing_PairingResponse_timestamp_get(::nearclip::pairing::PairingResponse* msg) {
  return msg->timestamp();
}
void proto2_rust_thunk_nearclip_pairing_PairingResponse_timestamp_set(::nearclip::pairing::PairingResponse* msg, ::uint64_t val) {
  msg->set_timestamp(val);
}

}  //extern "C"

// nearclip.pairing.PairingConfirmation
extern "C" {
void* proto2_rust_thunk_Message_nearclip_pairing_PairingConfirmation_new() { return new ::nearclip::pairing::PairingConfirmation(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_pairing_PairingConfirmation_default_instance() {
  return &::nearclip::pairing::PairingConfirmation::default_instance();
}
::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_pairing_PairingConfirmation_session_id_get(::nearclip::pairing::PairingConfirmation* msg) {
  absl::string_view val = msg->session_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_pairing_PairingConfirmation_session_id_set(::nearclip::pairing::PairingConfirmation* msg, std::string* s) {
  msg->set_session_id(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_pairing_PairingConfirmation_confirmation_hash_get(::nearclip::pairing::PairingConfirmation* msg) {
  absl::string_view val = msg->confirmation_hash();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_pairing_PairingConfirmation_confirmation_hash_set(::nearclip::pairing::PairingConfirmation* msg, std::string* s) {
  msg->set_confirmation_hash(std::move(*s));
  delete s;
}

::uint64_t proto2_rust_thunk_nearclip_pairing_PairingConfirmation_timestamp_get(::nearclip::pairing::PairingConfirmation* msg) {
  return msg->timestamp();
}
void proto2_rust_thunk_nearclip_pairing_PairingConfirmation_timestamp_set(::nearclip::pairing::PairingConfirmation* msg, ::uint64_t val) {
  msg->set_timestamp(val);
}

}  //extern "C"

// nearclip.pairing.PairingStatusUpdate
extern "C" {
void* proto2_rust_thunk_Message_nearclip_pairing_PairingStatusUpdate_new() { return new ::nearclip::pairing::PairingStatusUpdate(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_pairing_PairingStatusUpdate_default_instance() {
  return &::nearclip::pairing::PairingStatusUpdate::default_instance();
}
::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_session_id_get(::nearclip::pairing::PairingStatusUpdate* msg) {
  absl::string_view val = msg->session_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_session_id_set(::nearclip::pairing::PairingStatusUpdate* msg, std::string* s) {
  msg->set_session_id(std::move(*s));
  delete s;
}

::nearclip::pairing::PairingStatus proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_status_get(::nearclip::pairing::PairingStatusUpdate* msg) {
  return msg->status();
}
void proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_status_set(::nearclip::pairing::PairingStatusUpdate* msg, ::nearclip::pairing::PairingStatus val) {
  msg->set_status(val);
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_error_message_get(::nearclip::pairing::PairingStatusUpdate* msg) {
  absl::string_view val = msg->error_message();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_error_message_set(::nearclip::pairing::PairingStatusUpdate* msg, std::string* s) {
  msg->set_error_message(std::move(*s));
  delete s;
}

::uint64_t proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_timestamp_get(::nearclip::pairing::PairingStatusUpdate* msg) {
  return msg->timestamp();
}
void proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_timestamp_set(::nearclip::pairing::PairingStatusUpdate* msg, ::uint64_t val) {
  msg->set_timestamp(val);
}

}  //extern "C"

// nearclip.pairing.UnpairingRequest
extern "C" {
void* proto2_rust_thunk_Message_nearclip_pairing_UnpairingRequest_new() { return new ::nearclip::pairing::UnpairingRequest(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_pairing_UnpairingRequest_default_instance() {
  return &::nearclip::pairing::UnpairingRequest::default_instance();
}
::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_pairing_UnpairingRequest_device_id_get(::nearclip::pairing::UnpairingRequest* msg) {
  absl::string_view val = msg->device_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_pairing_UnpairingRequest_device_id_set(::nearclip::pairing::UnpairingRequest* msg, std::string* s) {
  msg->set_device_id(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_pairing_UnpairingRequest_reason_get(::nearclip::pairing::UnpairingRequest* msg) {
  absl::string_view val = msg->reason();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_pairing_UnpairingRequest_reason_set(::nearclip::pairing::UnpairingRequest* msg, std::string* s) {
  msg->set_reason(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_pairing_UnpairingRequest_signature_get(::nearclip::pairing::UnpairingRequest* msg) {
  absl::string_view val = msg->signature();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_pairing_UnpairingRequest_signature_set(::nearclip::pairing::UnpairingRequest* msg, std::string* s) {
  msg->set_signature(std::move(*s));
  delete s;
}

}  //extern "C"

// nearclip.pairing.PairingStatus

