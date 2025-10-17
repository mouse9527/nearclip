#include "sync.pb.h"
#include "google/protobuf/map.h"
#include "google/protobuf/repeated_field.h"
#include "google/protobuf/repeated_ptr_field.h"
#include "rust/cpp_kernel/serialized_data.h"
#include "rust/cpp_kernel/strings.h"
// nearclip.sync.ClipboardData
extern "C" {
void* proto2_rust_thunk_Message_nearclip_sync_ClipboardData_new() { return new ::nearclip::sync::ClipboardData(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_sync_ClipboardData_default_instance() {
  return &::nearclip::sync::ClipboardData::default_instance();
}
::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_sync_ClipboardData_data_id_get(::nearclip::sync::ClipboardData* msg) {
  absl::string_view val = msg->data_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_sync_ClipboardData_data_id_set(::nearclip::sync::ClipboardData* msg, std::string* s) {
  msg->set_data_id(std::move(*s));
  delete s;
}

::nearclip::sync::DataType proto2_rust_thunk_nearclip_sync_ClipboardData_type_get(::nearclip::sync::ClipboardData* msg) {
  return msg->type();
}
void proto2_rust_thunk_nearclip_sync_ClipboardData_type_set(::nearclip::sync::ClipboardData* msg, ::nearclip::sync::DataType val) {
  msg->set_type(val);
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_sync_ClipboardData_content_get(::nearclip::sync::ClipboardData* msg) {
  absl::string_view val = msg->content();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_sync_ClipboardData_content_set(::nearclip::sync::ClipboardData* msg, std::string* s) {
  msg->set_content(std::move(*s));
  delete s;
}

const void* proto2_rust_thunk_nearclip_sync_ClipboardData_metadata_get(const ::nearclip::sync::ClipboardData* msg) {
  return &msg->metadata();
}
void* proto2_rust_thunk_nearclip_sync_ClipboardData_metadata_get_mut(::nearclip::sync::ClipboardData* msg) { return msg->mutable_metadata(); }
void proto2_rust_thunk_nearclip_sync_ClipboardData_metadata_set(::nearclip::sync::ClipboardData* msg,
                         google::protobuf::Map<std::string, std::string>* value) {
  *msg->mutable_metadata() = std::move(*value);
  delete value;
}

::uint64_t proto2_rust_thunk_nearclip_sync_ClipboardData_created_at_get(::nearclip::sync::ClipboardData* msg) {
  return msg->created_at();
}
void proto2_rust_thunk_nearclip_sync_ClipboardData_created_at_set(::nearclip::sync::ClipboardData* msg, ::uint64_t val) {
  msg->set_created_at(val);
}

::uint64_t proto2_rust_thunk_nearclip_sync_ClipboardData_expires_at_get(::nearclip::sync::ClipboardData* msg) {
  return msg->expires_at();
}
void proto2_rust_thunk_nearclip_sync_ClipboardData_expires_at_set(::nearclip::sync::ClipboardData* msg, ::uint64_t val) {
  msg->set_expires_at(val);
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_sync_ClipboardData_source_app_get(::nearclip::sync::ClipboardData* msg) {
  absl::string_view val = msg->source_app();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_sync_ClipboardData_source_app_set(::nearclip::sync::ClipboardData* msg, std::string* s) {
  msg->set_source_app(std::move(*s));
  delete s;
}

}  //extern "C"

// nearclip.sync.DataChunk
extern "C" {
void* proto2_rust_thunk_Message_nearclip_sync_DataChunk_new() { return new ::nearclip::sync::DataChunk(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_sync_DataChunk_default_instance() {
  return &::nearclip::sync::DataChunk::default_instance();
}
::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_sync_DataChunk_data_id_get(::nearclip::sync::DataChunk* msg) {
  absl::string_view val = msg->data_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_sync_DataChunk_data_id_set(::nearclip::sync::DataChunk* msg, std::string* s) {
  msg->set_data_id(std::move(*s));
  delete s;
}

::uint32_t proto2_rust_thunk_nearclip_sync_DataChunk_chunk_index_get(::nearclip::sync::DataChunk* msg) {
  return msg->chunk_index();
}
void proto2_rust_thunk_nearclip_sync_DataChunk_chunk_index_set(::nearclip::sync::DataChunk* msg, ::uint32_t val) {
  msg->set_chunk_index(val);
}

::uint32_t proto2_rust_thunk_nearclip_sync_DataChunk_total_chunks_get(::nearclip::sync::DataChunk* msg) {
  return msg->total_chunks();
}
void proto2_rust_thunk_nearclip_sync_DataChunk_total_chunks_set(::nearclip::sync::DataChunk* msg, ::uint32_t val) {
  msg->set_total_chunks(val);
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_sync_DataChunk_chunk_data_get(::nearclip::sync::DataChunk* msg) {
  absl::string_view val = msg->chunk_data();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_sync_DataChunk_chunk_data_set(::nearclip::sync::DataChunk* msg, std::string* s) {
  msg->set_chunk_data(std::move(*s));
  delete s;
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_sync_DataChunk_checksum_get(::nearclip::sync::DataChunk* msg) {
  absl::string_view val = msg->checksum();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_sync_DataChunk_checksum_set(::nearclip::sync::DataChunk* msg, std::string* s) {
  msg->set_checksum(std::move(*s));
  delete s;
}

}  //extern "C"

// nearclip.sync.SyncMessage
extern "C" {
void* proto2_rust_thunk_Message_nearclip_sync_SyncMessage_new() { return new ::nearclip::sync::SyncMessage(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_sync_SyncMessage_default_instance() {
  return &::nearclip::sync::SyncMessage::default_instance();
}
::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_sync_SyncMessage_device_id_get(::nearclip::sync::SyncMessage* msg) {
  absl::string_view val = msg->device_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_sync_SyncMessage_device_id_set(::nearclip::sync::SyncMessage* msg, std::string* s) {
  msg->set_device_id(std::move(*s));
  delete s;
}

::nearclip::sync::SyncOperation proto2_rust_thunk_nearclip_sync_SyncMessage_operation_get(::nearclip::sync::SyncMessage* msg) {
  return msg->operation();
}
void proto2_rust_thunk_nearclip_sync_SyncMessage_operation_set(::nearclip::sync::SyncMessage* msg, ::nearclip::sync::SyncOperation val) {
  msg->set_operation(val);
}

bool proto2_rust_thunk_nearclip_sync_SyncMessage_data_has(::nearclip::sync::SyncMessage* msg) {
  return msg->has_data();
}
void proto2_rust_thunk_nearclip_sync_SyncMessage_data_clear(::nearclip::sync::SyncMessage* msg) { msg->clear_data(); }
const void* proto2_rust_thunk_nearclip_sync_SyncMessage_data_get(::nearclip::sync::SyncMessage* msg) {
  return static_cast<const void*>(&msg->data());
}
void* proto2_rust_thunk_nearclip_sync_SyncMessage_data_get_mut(::nearclip::sync::SyncMessage* msg) {
  return static_cast<void*>(msg->mutable_data());
}
void proto2_rust_thunk_nearclip_sync_SyncMessage_data_set(::nearclip::sync::SyncMessage* msg, ::nearclip::sync::ClipboardData* sub_msg) {
  msg->set_allocated_data(sub_msg);
}

google::protobuf::RepeatedPtrField<::nearclip::sync::DataChunk>* proto2_rust_thunk_nearclip_sync_SyncMessage_chunks_get_mut(
    ::nearclip::sync::SyncMessage* msg) {
  return msg->mutable_chunks();
}
const google::protobuf::RepeatedPtrField<::nearclip::sync::DataChunk>* proto2_rust_thunk_nearclip_sync_SyncMessage_chunks_get(
    const ::nearclip::sync::SyncMessage* msg) {
  return &msg->chunks();
}
void proto2_rust_thunk_nearclip_sync_SyncMessage_chunks_move_set(
    ::nearclip::sync::SyncMessage* msg,
    google::protobuf::RepeatedPtrField<::nearclip::sync::DataChunk>* value) {
  *msg->mutable_chunks() = std::move(*value);
  delete value;
}

::uint64_t proto2_rust_thunk_nearclip_sync_SyncMessage_timestamp_get(::nearclip::sync::SyncMessage* msg) {
  return msg->timestamp();
}
void proto2_rust_thunk_nearclip_sync_SyncMessage_timestamp_set(::nearclip::sync::SyncMessage* msg, ::uint64_t val) {
  msg->set_timestamp(val);
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_sync_SyncMessage_signature_get(::nearclip::sync::SyncMessage* msg) {
  absl::string_view val = msg->signature();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_sync_SyncMessage_signature_set(::nearclip::sync::SyncMessage* msg, std::string* s) {
  msg->set_signature(std::move(*s));
  delete s;
}

}  //extern "C"

// nearclip.sync.SyncAck
extern "C" {
void* proto2_rust_thunk_Message_nearclip_sync_SyncAck_new() { return new ::nearclip::sync::SyncAck(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_sync_SyncAck_default_instance() {
  return &::nearclip::sync::SyncAck::default_instance();
}
::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_sync_SyncAck_data_id_get(::nearclip::sync::SyncAck* msg) {
  absl::string_view val = msg->data_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_sync_SyncAck_data_id_set(::nearclip::sync::SyncAck* msg, std::string* s) {
  msg->set_data_id(std::move(*s));
  delete s;
}

bool proto2_rust_thunk_nearclip_sync_SyncAck_success_get(::nearclip::sync::SyncAck* msg) {
  return msg->success();
}
void proto2_rust_thunk_nearclip_sync_SyncAck_success_set(::nearclip::sync::SyncAck* msg, bool val) {
  msg->set_success(val);
}

::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_sync_SyncAck_error_message_get(::nearclip::sync::SyncAck* msg) {
  absl::string_view val = msg->error_message();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_sync_SyncAck_error_message_set(::nearclip::sync::SyncAck* msg, std::string* s) {
  msg->set_error_message(std::move(*s));
  delete s;
}

::uint64_t proto2_rust_thunk_nearclip_sync_SyncAck_timestamp_get(::nearclip::sync::SyncAck* msg) {
  return msg->timestamp();
}
void proto2_rust_thunk_nearclip_sync_SyncAck_timestamp_set(::nearclip::sync::SyncAck* msg, ::uint64_t val) {
  msg->set_timestamp(val);
}

}  //extern "C"

// nearclip.sync.SyncStatusQuery
extern "C" {
void* proto2_rust_thunk_Message_nearclip_sync_SyncStatusQuery_new() { return new ::nearclip::sync::SyncStatusQuery(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_sync_SyncStatusQuery_default_instance() {
  return &::nearclip::sync::SyncStatusQuery::default_instance();
}
::google::protobuf::rust::PtrAndLen proto2_rust_thunk_nearclip_sync_SyncStatusQuery_device_id_get(::nearclip::sync::SyncStatusQuery* msg) {
  absl::string_view val = msg->device_id();
  return ::google::protobuf::rust::PtrAndLen{val.data(), val.size()};
}
void proto2_rust_thunk_nearclip_sync_SyncStatusQuery_device_id_set(::nearclip::sync::SyncStatusQuery* msg, std::string* s) {
  msg->set_device_id(std::move(*s));
  delete s;
}

::uint64_t proto2_rust_thunk_nearclip_sync_SyncStatusQuery_since_timestamp_get(::nearclip::sync::SyncStatusQuery* msg) {
  return msg->since_timestamp();
}
void proto2_rust_thunk_nearclip_sync_SyncStatusQuery_since_timestamp_set(::nearclip::sync::SyncStatusQuery* msg, ::uint64_t val) {
  msg->set_since_timestamp(val);
}

}  //extern "C"

// nearclip.sync.SyncStatusResponse
extern "C" {
void* proto2_rust_thunk_Message_nearclip_sync_SyncStatusResponse_new() { return new ::nearclip::sync::SyncStatusResponse(); }

const google::protobuf::MessageLite* proto2_rust_thunk_Message_nearclip_sync_SyncStatusResponse_default_instance() {
  return &::nearclip::sync::SyncStatusResponse::default_instance();
}
google::protobuf::RepeatedPtrField<std::string>* proto2_rust_thunk_nearclip_sync_SyncStatusResponse_pending_data_ids_get_mut(
    ::nearclip::sync::SyncStatusResponse* msg) {
  return msg->mutable_pending_data_ids();
}
const google::protobuf::RepeatedPtrField<std::string>* proto2_rust_thunk_nearclip_sync_SyncStatusResponse_pending_data_ids_get(
    const ::nearclip::sync::SyncStatusResponse* msg) {
  return &msg->pending_data_ids();
}
void proto2_rust_thunk_nearclip_sync_SyncStatusResponse_pending_data_ids_move_set(
    ::nearclip::sync::SyncStatusResponse* msg,
    google::protobuf::RepeatedPtrField<std::string>* value) {
  *msg->mutable_pending_data_ids() = std::move(*value);
  delete value;
}

google::protobuf::RepeatedPtrField<std::string>* proto2_rust_thunk_nearclip_sync_SyncStatusResponse_completed_data_ids_get_mut(
    ::nearclip::sync::SyncStatusResponse* msg) {
  return msg->mutable_completed_data_ids();
}
const google::protobuf::RepeatedPtrField<std::string>* proto2_rust_thunk_nearclip_sync_SyncStatusResponse_completed_data_ids_get(
    const ::nearclip::sync::SyncStatusResponse* msg) {
  return &msg->completed_data_ids();
}
void proto2_rust_thunk_nearclip_sync_SyncStatusResponse_completed_data_ids_move_set(
    ::nearclip::sync::SyncStatusResponse* msg,
    google::protobuf::RepeatedPtrField<std::string>* value) {
  *msg->mutable_completed_data_ids() = std::move(*value);
  delete value;
}

::uint64_t proto2_rust_thunk_nearclip_sync_SyncStatusResponse_last_sync_timestamp_get(::nearclip::sync::SyncStatusResponse* msg) {
  return msg->last_sync_timestamp();
}
void proto2_rust_thunk_nearclip_sync_SyncStatusResponse_last_sync_timestamp_set(::nearclip::sync::SyncStatusResponse* msg, ::uint64_t val) {
  msg->set_last_sync_timestamp(val);
}

}  //extern "C"

// nearclip.sync.DataType

// nearclip.sync.SyncOperation

