const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.32.1-release");
#[allow(non_camel_case_types)]
pub struct ErrorMessage {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ErrorMessage>
}

impl ::protobuf::Message for ErrorMessage {}

impl ::std::default::Default for ErrorMessage {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for ErrorMessage {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for ErrorMessage {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ErrorMessage {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `ErrorMessage` is `Sync` because it does not implement interior mutability.
//    Neither does `ErrorMessageMut`.
unsafe impl Sync for ErrorMessage {}

// SAFETY:
// - `ErrorMessage` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ErrorMessage {}

impl ::protobuf::Proxied for ErrorMessage {
  type View<'msg> = ErrorMessageView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ErrorMessage {}

impl ::protobuf::MutProxied for ErrorMessage {
  type Mut<'msg> = ErrorMessageMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ErrorMessageView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ErrorMessage>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ErrorMessageView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ErrorMessageView<'msg> {
  type Message = ErrorMessage;
}

impl ::std::fmt::Debug for ErrorMessageView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ErrorMessageView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    let mut serialized_data = ::protobuf::__internal::runtime::SerializedData::new();
    let success = unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_serialize(self.raw_msg(), &mut serialized_data)
    };
    if success {
      Ok(serialized_data.into_vec())
    } else {
      Err(::protobuf::SerializeError)
    }
  }
}

impl ::std::default::Default for ErrorMessageView<'_> {
  fn default() -> ErrorMessageView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_common_ErrorMessage_default_instance()) };
    ErrorMessageView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> ErrorMessageView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ErrorMessage>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> ErrorMessage {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // code: optional enum nearclip.common.ErrorCode
  pub fn code(self) -> super::ErrorCode {
    unsafe { proto2_rust_thunk_nearclip_common_ErrorMessage_code_get(self.raw_msg()) }
  }

  // message: optional string
  pub fn message(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_common_ErrorMessage_message_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // details: optional string
  pub fn details(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_common_ErrorMessage_details_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // timestamp: optional uint64
  pub fn timestamp(self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_common_ErrorMessage_timestamp_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `ErrorMessageView` is `Sync` because it does not support mutation.
unsafe impl Sync for ErrorMessageView<'_> {}

// SAFETY:
// - `ErrorMessageView` is `Send` because while its alive a `ErrorMessageMut` cannot.
// - `ErrorMessageView` does not use thread-local data.
unsafe impl Send for ErrorMessageView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ErrorMessageView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for ErrorMessageView<'msg> {}

impl<'msg> ::protobuf::AsView for ErrorMessageView<'msg> {
  type Proxied = ErrorMessage;
  fn as_view(&self) -> ::protobuf::View<'msg, ErrorMessage> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ErrorMessageView<'msg> {
  fn into_view<'shorter>(self) -> ErrorMessageView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ErrorMessage> for ErrorMessageView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ErrorMessage {
    let dst = ErrorMessage::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ErrorMessage> for ErrorMessageMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ErrorMessage {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for ErrorMessage {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    // SAFETY:
    // - The thunk returns an unaliased and valid `RepeatedPtrField*`
    unsafe {
      ::protobuf::Repeated::from_inner(::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeated::from_raw(::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_new())
      )
    }
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    // SAFETY
    // - `f.raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_free(f.as_view().as_raw(::protobuf::__internal::Private)) }
  }

  fn repeated_len(f: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_size(f.as_raw(::protobuf::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
    v: impl ::protobuf::IntoProxied<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(
        ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get_mut(f.as_raw(::protobuf::__internal::Private), i),
        v.into_proxied(::protobuf::__internal::Private).raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::protobuf::View<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get(f.as_raw(::protobuf::__internal::Private), i) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(msg) };
    ::protobuf::View::<Self>::new(::protobuf::__internal::Private, inner)
  }

  unsafe fn repeated_get_mut_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::Mut<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get_mut(f.as_raw(::protobuf::__internal::Private), i) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(msg) };
    ::protobuf::Mut::<Self>::new(::protobuf::__internal::Private, inner)
  }

  fn repeated_clear(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_clear(f.as_raw(::protobuf::__internal::Private)) };
  }

  fn repeated_push(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>, v: impl ::protobuf::IntoProxied<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let prototype = <ErrorMessageView as ::std::default::Default>::default().raw_msg();
      let new_elem = ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_add(f.as_raw(::protobuf::__internal::Private), prototype);
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(new_elem, v.into_proxied(::protobuf::__internal::Private).raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::protobuf::View<::protobuf::Repeated<Self>>,
    mut dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_copy_from(dest.as_raw(::protobuf::__internal::Private), src.as_raw(::protobuf::__internal::Private));
    }
  }

  fn repeated_reserve(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    additional: usize,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_reserve(f.as_raw(::protobuf::__internal::Private), additional) }
  }
}
impl ::protobuf::__internal::runtime::CppMapTypeConversions for ErrorMessage {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<ErrorMessageView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> ErrorMessageView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { ErrorMessageView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> ErrorMessageMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        ErrorMessageMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ErrorMessageMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ErrorMessage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ErrorMessageMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ErrorMessageMut<'msg> {
  type Message = ErrorMessage;
}

impl ::std::fmt::Debug for ErrorMessageMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ErrorMessageMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> ErrorMessageMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ErrorMessage>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ErrorMessage> {
    self.inner
  }

  pub fn to_owned(&self) -> ErrorMessage {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // code: optional enum nearclip.common.ErrorCode
  pub fn code(&self) -> super::ErrorCode {
    unsafe { proto2_rust_thunk_nearclip_common_ErrorMessage_code_get(self.raw_msg()) }
  }
  pub fn set_code(&mut self, val: super::ErrorCode) {
    unsafe { proto2_rust_thunk_nearclip_common_ErrorMessage_code_set(self.raw_msg(), val) }
  }

  // message: optional string
  pub fn message(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_common_ErrorMessage_message_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_message(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_common_ErrorMessage_message_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // details: optional string
  pub fn details(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_common_ErrorMessage_details_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_details(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_common_ErrorMessage_details_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_common_ErrorMessage_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_common_ErrorMessage_timestamp_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `ErrorMessageMut` does not perform any shared mutation.
// - `ErrorMessageMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ErrorMessageMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ErrorMessageMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for ErrorMessageMut<'msg> {}

impl<'msg> ::protobuf::AsView for ErrorMessageMut<'msg> {
  type Proxied = ErrorMessage;
  fn as_view(&self) -> ::protobuf::View<'_, ErrorMessage> {
    ErrorMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ErrorMessageMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ErrorMessage>
  where
      'msg: 'shorter {
    ErrorMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for ErrorMessageMut<'msg> {
  type MutProxied = ErrorMessage;
  fn as_mut(&mut self) -> ErrorMessageMut<'msg> {
    ErrorMessageMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ErrorMessageMut<'msg> {
  fn into_mut<'shorter>(self) -> ErrorMessageMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ErrorMessage {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_common_ErrorMessage_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ErrorMessage> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }


  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> ErrorMessageView {
    ErrorMessageView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> ErrorMessageMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    ErrorMessageMut::new(::protobuf::__internal::Private, inner)
  }

  // code: optional enum nearclip.common.ErrorCode
  pub fn code(&self) -> super::ErrorCode {
    unsafe { proto2_rust_thunk_nearclip_common_ErrorMessage_code_get(self.raw_msg()) }
  }
  pub fn set_code(&mut self, val: super::ErrorCode) {
    unsafe { proto2_rust_thunk_nearclip_common_ErrorMessage_code_set(self.raw_msg(), val) }
  }

  // message: optional string
  pub fn message(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_common_ErrorMessage_message_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_message(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_common_ErrorMessage_message_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // details: optional string
  pub fn details(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_common_ErrorMessage_details_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_details(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_common_ErrorMessage_details_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_common_ErrorMessage_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_common_ErrorMessage_timestamp_set(self.raw_msg(), val) }
  }

}  // impl ErrorMessage

impl ::std::ops::Drop for ErrorMessage {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for ErrorMessage {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ErrorMessage {
  type Proxied = Self;
  fn as_view(&self) -> ErrorMessageView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ErrorMessage {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ErrorMessageMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for ErrorMessageMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for ErrorMessageView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_common_ErrorMessage_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_common_ErrorMessage_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_common_ErrorMessage_code_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> super::ErrorCode;
  fn proto2_rust_thunk_nearclip_common_ErrorMessage_code_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: super::ErrorCode);

  fn proto2_rust_thunk_nearclip_common_ErrorMessage_message_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_common_ErrorMessage_message_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_common_ErrorMessage_details_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_common_ErrorMessage_details_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_common_ErrorMessage_timestamp_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u64;
  fn proto2_rust_thunk_nearclip_common_ErrorMessage_timestamp_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u64);

}

impl<'a> ErrorMessageMut<'a> {
  pub unsafe fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ErrorMessageView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for ErrorMessage {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<ErrorMessage>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for ErrorMessageMut<'a> {
  unsafe fn __unstable_wrap_raw_message_mut(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  unsafe fn __unstable_wrap_raw_message_mut_unchecked_lifetime(
      msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  fn __unstable_as_raw_message_mut(&mut self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageViewInterop<'a> for ErrorMessageView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

#[allow(non_camel_case_types)]
pub struct Heartbeat {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Heartbeat>
}

impl ::protobuf::Message for Heartbeat {}

impl ::std::default::Default for Heartbeat {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for Heartbeat {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for Heartbeat {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for Heartbeat {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `Heartbeat` is `Sync` because it does not implement interior mutability.
//    Neither does `HeartbeatMut`.
unsafe impl Sync for Heartbeat {}

// SAFETY:
// - `Heartbeat` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Heartbeat {}

impl ::protobuf::Proxied for Heartbeat {
  type View<'msg> = HeartbeatView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Heartbeat {}

impl ::protobuf::MutProxied for Heartbeat {
  type Mut<'msg> = HeartbeatMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HeartbeatView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Heartbeat>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeartbeatView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HeartbeatView<'msg> {
  type Message = Heartbeat;
}

impl ::std::fmt::Debug for HeartbeatView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for HeartbeatView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    let mut serialized_data = ::protobuf::__internal::runtime::SerializedData::new();
    let success = unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_serialize(self.raw_msg(), &mut serialized_data)
    };
    if success {
      Ok(serialized_data.into_vec())
    } else {
      Err(::protobuf::SerializeError)
    }
  }
}

impl ::std::default::Default for HeartbeatView<'_> {
  fn default() -> HeartbeatView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_common_Heartbeat_default_instance()) };
    HeartbeatView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> HeartbeatView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Heartbeat>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> Heartbeat {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // device_id: optional string
  pub fn device_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_common_Heartbeat_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // timestamp: optional uint64
  pub fn timestamp(self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_common_Heartbeat_timestamp_get(self.raw_msg()) }
  }

  // sequence_number: optional uint32
  pub fn sequence_number(self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_common_Heartbeat_sequence_number_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `HeartbeatView` is `Sync` because it does not support mutation.
unsafe impl Sync for HeartbeatView<'_> {}

// SAFETY:
// - `HeartbeatView` is `Send` because while its alive a `HeartbeatMut` cannot.
// - `HeartbeatView` does not use thread-local data.
unsafe impl Send for HeartbeatView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for HeartbeatView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for HeartbeatView<'msg> {}

impl<'msg> ::protobuf::AsView for HeartbeatView<'msg> {
  type Proxied = Heartbeat;
  fn as_view(&self) -> ::protobuf::View<'msg, Heartbeat> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeartbeatView<'msg> {
  fn into_view<'shorter>(self) -> HeartbeatView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Heartbeat> for HeartbeatView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Heartbeat {
    let dst = Heartbeat::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Heartbeat> for HeartbeatMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Heartbeat {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for Heartbeat {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    // SAFETY:
    // - The thunk returns an unaliased and valid `RepeatedPtrField*`
    unsafe {
      ::protobuf::Repeated::from_inner(::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeated::from_raw(::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_new())
      )
    }
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    // SAFETY
    // - `f.raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_free(f.as_view().as_raw(::protobuf::__internal::Private)) }
  }

  fn repeated_len(f: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_size(f.as_raw(::protobuf::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
    v: impl ::protobuf::IntoProxied<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(
        ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get_mut(f.as_raw(::protobuf::__internal::Private), i),
        v.into_proxied(::protobuf::__internal::Private).raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::protobuf::View<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get(f.as_raw(::protobuf::__internal::Private), i) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(msg) };
    ::protobuf::View::<Self>::new(::protobuf::__internal::Private, inner)
  }

  unsafe fn repeated_get_mut_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::Mut<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get_mut(f.as_raw(::protobuf::__internal::Private), i) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(msg) };
    ::protobuf::Mut::<Self>::new(::protobuf::__internal::Private, inner)
  }

  fn repeated_clear(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_clear(f.as_raw(::protobuf::__internal::Private)) };
  }

  fn repeated_push(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>, v: impl ::protobuf::IntoProxied<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let prototype = <HeartbeatView as ::std::default::Default>::default().raw_msg();
      let new_elem = ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_add(f.as_raw(::protobuf::__internal::Private), prototype);
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(new_elem, v.into_proxied(::protobuf::__internal::Private).raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::protobuf::View<::protobuf::Repeated<Self>>,
    mut dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_copy_from(dest.as_raw(::protobuf::__internal::Private), src.as_raw(::protobuf::__internal::Private));
    }
  }

  fn repeated_reserve(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    additional: usize,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_reserve(f.as_raw(::protobuf::__internal::Private), additional) }
  }
}
impl ::protobuf::__internal::runtime::CppMapTypeConversions for Heartbeat {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<HeartbeatView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> HeartbeatView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { HeartbeatView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> HeartbeatMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        HeartbeatMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HeartbeatMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Heartbeat>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeartbeatMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HeartbeatMut<'msg> {
  type Message = Heartbeat;
}

impl ::std::fmt::Debug for HeartbeatMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for HeartbeatMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> HeartbeatMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Heartbeat>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Heartbeat> {
    self.inner
  }

  pub fn to_owned(&self) -> Heartbeat {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_common_Heartbeat_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_common_Heartbeat_device_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_common_Heartbeat_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_common_Heartbeat_timestamp_set(self.raw_msg(), val) }
  }

  // sequence_number: optional uint32
  pub fn sequence_number(&self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_common_Heartbeat_sequence_number_get(self.raw_msg()) }
  }
  pub fn set_sequence_number(&mut self, val: u32) {
    unsafe { proto2_rust_thunk_nearclip_common_Heartbeat_sequence_number_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `HeartbeatMut` does not perform any shared mutation.
// - `HeartbeatMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for HeartbeatMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for HeartbeatMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for HeartbeatMut<'msg> {}

impl<'msg> ::protobuf::AsView for HeartbeatMut<'msg> {
  type Proxied = Heartbeat;
  fn as_view(&self) -> ::protobuf::View<'_, Heartbeat> {
    HeartbeatView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeartbeatMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Heartbeat>
  where
      'msg: 'shorter {
    HeartbeatView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for HeartbeatMut<'msg> {
  type MutProxied = Heartbeat;
  fn as_mut(&mut self) -> HeartbeatMut<'msg> {
    HeartbeatMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HeartbeatMut<'msg> {
  fn into_mut<'shorter>(self) -> HeartbeatMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Heartbeat {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_common_Heartbeat_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Heartbeat> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }


  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> HeartbeatView {
    HeartbeatView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> HeartbeatMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    HeartbeatMut::new(::protobuf::__internal::Private, inner)
  }

  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_common_Heartbeat_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_common_Heartbeat_device_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_common_Heartbeat_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_common_Heartbeat_timestamp_set(self.raw_msg(), val) }
  }

  // sequence_number: optional uint32
  pub fn sequence_number(&self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_common_Heartbeat_sequence_number_get(self.raw_msg()) }
  }
  pub fn set_sequence_number(&mut self, val: u32) {
    unsafe { proto2_rust_thunk_nearclip_common_Heartbeat_sequence_number_set(self.raw_msg(), val) }
  }

}  // impl Heartbeat

impl ::std::ops::Drop for Heartbeat {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for Heartbeat {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Heartbeat {
  type Proxied = Self;
  fn as_view(&self) -> HeartbeatView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Heartbeat {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HeartbeatMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for HeartbeatMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for HeartbeatView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_common_Heartbeat_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_common_Heartbeat_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_common_Heartbeat_device_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_common_Heartbeat_device_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_common_Heartbeat_timestamp_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u64;
  fn proto2_rust_thunk_nearclip_common_Heartbeat_timestamp_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u64);

  fn proto2_rust_thunk_nearclip_common_Heartbeat_sequence_number_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u32;
  fn proto2_rust_thunk_nearclip_common_Heartbeat_sequence_number_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u32);

}

impl<'a> HeartbeatMut<'a> {
  pub unsafe fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> HeartbeatView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for Heartbeat {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Heartbeat>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for HeartbeatMut<'a> {
  unsafe fn __unstable_wrap_raw_message_mut(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  unsafe fn __unstable_wrap_raw_message_mut_unchecked_lifetime(
      msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  fn __unstable_as_raw_message_mut(&mut self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageViewInterop<'a> for HeartbeatView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

#[allow(non_camel_case_types)]
pub struct HeartbeatAck {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HeartbeatAck>
}

impl ::protobuf::Message for HeartbeatAck {}

impl ::std::default::Default for HeartbeatAck {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for HeartbeatAck {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for HeartbeatAck {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for HeartbeatAck {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `HeartbeatAck` is `Sync` because it does not implement interior mutability.
//    Neither does `HeartbeatAckMut`.
unsafe impl Sync for HeartbeatAck {}

// SAFETY:
// - `HeartbeatAck` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for HeartbeatAck {}

impl ::protobuf::Proxied for HeartbeatAck {
  type View<'msg> = HeartbeatAckView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HeartbeatAck {}

impl ::protobuf::MutProxied for HeartbeatAck {
  type Mut<'msg> = HeartbeatAckMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HeartbeatAckView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeartbeatAck>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeartbeatAckView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HeartbeatAckView<'msg> {
  type Message = HeartbeatAck;
}

impl ::std::fmt::Debug for HeartbeatAckView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for HeartbeatAckView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    let mut serialized_data = ::protobuf::__internal::runtime::SerializedData::new();
    let success = unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_serialize(self.raw_msg(), &mut serialized_data)
    };
    if success {
      Ok(serialized_data.into_vec())
    } else {
      Err(::protobuf::SerializeError)
    }
  }
}

impl ::std::default::Default for HeartbeatAckView<'_> {
  fn default() -> HeartbeatAckView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_common_HeartbeatAck_default_instance()) };
    HeartbeatAckView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> HeartbeatAckView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeartbeatAck>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> HeartbeatAck {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // device_id: optional string
  pub fn device_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_common_HeartbeatAck_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // received_timestamp: optional uint64
  pub fn received_timestamp(self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_common_HeartbeatAck_received_timestamp_get(self.raw_msg()) }
  }

  // sequence_number: optional uint32
  pub fn sequence_number(self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_common_HeartbeatAck_sequence_number_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `HeartbeatAckView` is `Sync` because it does not support mutation.
unsafe impl Sync for HeartbeatAckView<'_> {}

// SAFETY:
// - `HeartbeatAckView` is `Send` because while its alive a `HeartbeatAckMut` cannot.
// - `HeartbeatAckView` does not use thread-local data.
unsafe impl Send for HeartbeatAckView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for HeartbeatAckView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for HeartbeatAckView<'msg> {}

impl<'msg> ::protobuf::AsView for HeartbeatAckView<'msg> {
  type Proxied = HeartbeatAck;
  fn as_view(&self) -> ::protobuf::View<'msg, HeartbeatAck> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeartbeatAckView<'msg> {
  fn into_view<'shorter>(self) -> HeartbeatAckView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HeartbeatAck> for HeartbeatAckView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeartbeatAck {
    let dst = HeartbeatAck::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HeartbeatAck> for HeartbeatAckMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeartbeatAck {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for HeartbeatAck {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    // SAFETY:
    // - The thunk returns an unaliased and valid `RepeatedPtrField*`
    unsafe {
      ::protobuf::Repeated::from_inner(::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeated::from_raw(::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_new())
      )
    }
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    // SAFETY
    // - `f.raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_free(f.as_view().as_raw(::protobuf::__internal::Private)) }
  }

  fn repeated_len(f: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_size(f.as_raw(::protobuf::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
    v: impl ::protobuf::IntoProxied<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(
        ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get_mut(f.as_raw(::protobuf::__internal::Private), i),
        v.into_proxied(::protobuf::__internal::Private).raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::protobuf::View<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get(f.as_raw(::protobuf::__internal::Private), i) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(msg) };
    ::protobuf::View::<Self>::new(::protobuf::__internal::Private, inner)
  }

  unsafe fn repeated_get_mut_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::Mut<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get_mut(f.as_raw(::protobuf::__internal::Private), i) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(msg) };
    ::protobuf::Mut::<Self>::new(::protobuf::__internal::Private, inner)
  }

  fn repeated_clear(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_clear(f.as_raw(::protobuf::__internal::Private)) };
  }

  fn repeated_push(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>, v: impl ::protobuf::IntoProxied<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let prototype = <HeartbeatAckView as ::std::default::Default>::default().raw_msg();
      let new_elem = ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_add(f.as_raw(::protobuf::__internal::Private), prototype);
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(new_elem, v.into_proxied(::protobuf::__internal::Private).raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::protobuf::View<::protobuf::Repeated<Self>>,
    mut dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_copy_from(dest.as_raw(::protobuf::__internal::Private), src.as_raw(::protobuf::__internal::Private));
    }
  }

  fn repeated_reserve(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    additional: usize,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_reserve(f.as_raw(::protobuf::__internal::Private), additional) }
  }
}
impl ::protobuf::__internal::runtime::CppMapTypeConversions for HeartbeatAck {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<HeartbeatAckView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> HeartbeatAckView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { HeartbeatAckView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> HeartbeatAckMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        HeartbeatAckMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HeartbeatAckMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeartbeatAck>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeartbeatAckMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HeartbeatAckMut<'msg> {
  type Message = HeartbeatAck;
}

impl ::std::fmt::Debug for HeartbeatAckMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for HeartbeatAckMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> HeartbeatAckMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeartbeatAck>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HeartbeatAck> {
    self.inner
  }

  pub fn to_owned(&self) -> HeartbeatAck {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_common_HeartbeatAck_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_common_HeartbeatAck_device_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // received_timestamp: optional uint64
  pub fn received_timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_common_HeartbeatAck_received_timestamp_get(self.raw_msg()) }
  }
  pub fn set_received_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_common_HeartbeatAck_received_timestamp_set(self.raw_msg(), val) }
  }

  // sequence_number: optional uint32
  pub fn sequence_number(&self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_common_HeartbeatAck_sequence_number_get(self.raw_msg()) }
  }
  pub fn set_sequence_number(&mut self, val: u32) {
    unsafe { proto2_rust_thunk_nearclip_common_HeartbeatAck_sequence_number_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `HeartbeatAckMut` does not perform any shared mutation.
// - `HeartbeatAckMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for HeartbeatAckMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for HeartbeatAckMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for HeartbeatAckMut<'msg> {}

impl<'msg> ::protobuf::AsView for HeartbeatAckMut<'msg> {
  type Proxied = HeartbeatAck;
  fn as_view(&self) -> ::protobuf::View<'_, HeartbeatAck> {
    HeartbeatAckView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeartbeatAckMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HeartbeatAck>
  where
      'msg: 'shorter {
    HeartbeatAckView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for HeartbeatAckMut<'msg> {
  type MutProxied = HeartbeatAck;
  fn as_mut(&mut self) -> HeartbeatAckMut<'msg> {
    HeartbeatAckMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HeartbeatAckMut<'msg> {
  fn into_mut<'shorter>(self) -> HeartbeatAckMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HeartbeatAck {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_common_HeartbeatAck_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HeartbeatAck> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }


  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> HeartbeatAckView {
    HeartbeatAckView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> HeartbeatAckMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    HeartbeatAckMut::new(::protobuf::__internal::Private, inner)
  }

  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_common_HeartbeatAck_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_common_HeartbeatAck_device_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // received_timestamp: optional uint64
  pub fn received_timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_common_HeartbeatAck_received_timestamp_get(self.raw_msg()) }
  }
  pub fn set_received_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_common_HeartbeatAck_received_timestamp_set(self.raw_msg(), val) }
  }

  // sequence_number: optional uint32
  pub fn sequence_number(&self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_common_HeartbeatAck_sequence_number_get(self.raw_msg()) }
  }
  pub fn set_sequence_number(&mut self, val: u32) {
    unsafe { proto2_rust_thunk_nearclip_common_HeartbeatAck_sequence_number_set(self.raw_msg(), val) }
  }

}  // impl HeartbeatAck

impl ::std::ops::Drop for HeartbeatAck {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for HeartbeatAck {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HeartbeatAck {
  type Proxied = Self;
  fn as_view(&self) -> HeartbeatAckView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HeartbeatAck {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HeartbeatAckMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for HeartbeatAckMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for HeartbeatAckView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_common_HeartbeatAck_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_common_HeartbeatAck_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_common_HeartbeatAck_device_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_common_HeartbeatAck_device_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_common_HeartbeatAck_received_timestamp_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u64;
  fn proto2_rust_thunk_nearclip_common_HeartbeatAck_received_timestamp_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u64);

  fn proto2_rust_thunk_nearclip_common_HeartbeatAck_sequence_number_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u32;
  fn proto2_rust_thunk_nearclip_common_HeartbeatAck_sequence_number_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u32);

}

impl<'a> HeartbeatAckMut<'a> {
  pub unsafe fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> HeartbeatAckView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for HeartbeatAck {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<HeartbeatAck>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for HeartbeatAckMut<'a> {
  unsafe fn __unstable_wrap_raw_message_mut(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  unsafe fn __unstable_wrap_raw_message_mut_unchecked_lifetime(
      msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  fn __unstable_as_raw_message_mut(&mut self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageViewInterop<'a> for HeartbeatAckView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

#[allow(non_camel_case_types)]
pub struct ProtocolVersion {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ProtocolVersion>
}

impl ::protobuf::Message for ProtocolVersion {}

impl ::std::default::Default for ProtocolVersion {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for ProtocolVersion {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for ProtocolVersion {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ProtocolVersion {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `ProtocolVersion` is `Sync` because it does not implement interior mutability.
//    Neither does `ProtocolVersionMut`.
unsafe impl Sync for ProtocolVersion {}

// SAFETY:
// - `ProtocolVersion` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ProtocolVersion {}

impl ::protobuf::Proxied for ProtocolVersion {
  type View<'msg> = ProtocolVersionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ProtocolVersion {}

impl ::protobuf::MutProxied for ProtocolVersion {
  type Mut<'msg> = ProtocolVersionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ProtocolVersionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProtocolVersion>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProtocolVersionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ProtocolVersionView<'msg> {
  type Message = ProtocolVersion;
}

impl ::std::fmt::Debug for ProtocolVersionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ProtocolVersionView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    let mut serialized_data = ::protobuf::__internal::runtime::SerializedData::new();
    let success = unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_serialize(self.raw_msg(), &mut serialized_data)
    };
    if success {
      Ok(serialized_data.into_vec())
    } else {
      Err(::protobuf::SerializeError)
    }
  }
}

impl ::std::default::Default for ProtocolVersionView<'_> {
  fn default() -> ProtocolVersionView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_common_ProtocolVersion_default_instance()) };
    ProtocolVersionView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> ProtocolVersionView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProtocolVersion>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> ProtocolVersion {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // major: optional uint32
  pub fn major(self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_major_get(self.raw_msg()) }
  }

  // minor: optional uint32
  pub fn minor(self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_minor_get(self.raw_msg()) }
  }

  // patch: optional uint32
  pub fn patch(self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_patch_get(self.raw_msg()) }
  }

  // build_info: optional string
  pub fn build_info(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_build_info_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ProtocolVersionView` is `Sync` because it does not support mutation.
unsafe impl Sync for ProtocolVersionView<'_> {}

// SAFETY:
// - `ProtocolVersionView` is `Send` because while its alive a `ProtocolVersionMut` cannot.
// - `ProtocolVersionView` does not use thread-local data.
unsafe impl Send for ProtocolVersionView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ProtocolVersionView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for ProtocolVersionView<'msg> {}

impl<'msg> ::protobuf::AsView for ProtocolVersionView<'msg> {
  type Proxied = ProtocolVersion;
  fn as_view(&self) -> ::protobuf::View<'msg, ProtocolVersion> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProtocolVersionView<'msg> {
  fn into_view<'shorter>(self) -> ProtocolVersionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ProtocolVersion> for ProtocolVersionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProtocolVersion {
    let dst = ProtocolVersion::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ProtocolVersion> for ProtocolVersionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProtocolVersion {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for ProtocolVersion {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    // SAFETY:
    // - The thunk returns an unaliased and valid `RepeatedPtrField*`
    unsafe {
      ::protobuf::Repeated::from_inner(::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeated::from_raw(::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_new())
      )
    }
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    // SAFETY
    // - `f.raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_free(f.as_view().as_raw(::protobuf::__internal::Private)) }
  }

  fn repeated_len(f: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_size(f.as_raw(::protobuf::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
    v: impl ::protobuf::IntoProxied<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(
        ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get_mut(f.as_raw(::protobuf::__internal::Private), i),
        v.into_proxied(::protobuf::__internal::Private).raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::protobuf::View<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get(f.as_raw(::protobuf::__internal::Private), i) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(msg) };
    ::protobuf::View::<Self>::new(::protobuf::__internal::Private, inner)
  }

  unsafe fn repeated_get_mut_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::Mut<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get_mut(f.as_raw(::protobuf::__internal::Private), i) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(msg) };
    ::protobuf::Mut::<Self>::new(::protobuf::__internal::Private, inner)
  }

  fn repeated_clear(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_clear(f.as_raw(::protobuf::__internal::Private)) };
  }

  fn repeated_push(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>, v: impl ::protobuf::IntoProxied<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let prototype = <ProtocolVersionView as ::std::default::Default>::default().raw_msg();
      let new_elem = ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_add(f.as_raw(::protobuf::__internal::Private), prototype);
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(new_elem, v.into_proxied(::protobuf::__internal::Private).raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::protobuf::View<::protobuf::Repeated<Self>>,
    mut dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_copy_from(dest.as_raw(::protobuf::__internal::Private), src.as_raw(::protobuf::__internal::Private));
    }
  }

  fn repeated_reserve(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    additional: usize,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_reserve(f.as_raw(::protobuf::__internal::Private), additional) }
  }
}
impl ::protobuf::__internal::runtime::CppMapTypeConversions for ProtocolVersion {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<ProtocolVersionView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> ProtocolVersionView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { ProtocolVersionView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> ProtocolVersionMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        ProtocolVersionMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ProtocolVersionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProtocolVersion>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProtocolVersionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ProtocolVersionMut<'msg> {
  type Message = ProtocolVersion;
}

impl ::std::fmt::Debug for ProtocolVersionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ProtocolVersionMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> ProtocolVersionMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProtocolVersion>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ProtocolVersion> {
    self.inner
  }

  pub fn to_owned(&self) -> ProtocolVersion {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // major: optional uint32
  pub fn major(&self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_major_get(self.raw_msg()) }
  }
  pub fn set_major(&mut self, val: u32) {
    unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_major_set(self.raw_msg(), val) }
  }

  // minor: optional uint32
  pub fn minor(&self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_minor_get(self.raw_msg()) }
  }
  pub fn set_minor(&mut self, val: u32) {
    unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_minor_set(self.raw_msg(), val) }
  }

  // patch: optional uint32
  pub fn patch(&self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_patch_get(self.raw_msg()) }
  }
  pub fn set_patch(&mut self, val: u32) {
    unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_patch_set(self.raw_msg(), val) }
  }

  // build_info: optional string
  pub fn build_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_build_info_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_build_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_common_ProtocolVersion_build_info_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `ProtocolVersionMut` does not perform any shared mutation.
// - `ProtocolVersionMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ProtocolVersionMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ProtocolVersionMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for ProtocolVersionMut<'msg> {}

impl<'msg> ::protobuf::AsView for ProtocolVersionMut<'msg> {
  type Proxied = ProtocolVersion;
  fn as_view(&self) -> ::protobuf::View<'_, ProtocolVersion> {
    ProtocolVersionView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProtocolVersionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ProtocolVersion>
  where
      'msg: 'shorter {
    ProtocolVersionView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for ProtocolVersionMut<'msg> {
  type MutProxied = ProtocolVersion;
  fn as_mut(&mut self) -> ProtocolVersionMut<'msg> {
    ProtocolVersionMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ProtocolVersionMut<'msg> {
  fn into_mut<'shorter>(self) -> ProtocolVersionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ProtocolVersion {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_common_ProtocolVersion_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ProtocolVersion> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }


  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> ProtocolVersionView {
    ProtocolVersionView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> ProtocolVersionMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    ProtocolVersionMut::new(::protobuf::__internal::Private, inner)
  }

  // major: optional uint32
  pub fn major(&self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_major_get(self.raw_msg()) }
  }
  pub fn set_major(&mut self, val: u32) {
    unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_major_set(self.raw_msg(), val) }
  }

  // minor: optional uint32
  pub fn minor(&self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_minor_get(self.raw_msg()) }
  }
  pub fn set_minor(&mut self, val: u32) {
    unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_minor_set(self.raw_msg(), val) }
  }

  // patch: optional uint32
  pub fn patch(&self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_patch_get(self.raw_msg()) }
  }
  pub fn set_patch(&mut self, val: u32) {
    unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_patch_set(self.raw_msg(), val) }
  }

  // build_info: optional string
  pub fn build_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_common_ProtocolVersion_build_info_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_build_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_common_ProtocolVersion_build_info_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl ProtocolVersion

impl ::std::ops::Drop for ProtocolVersion {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for ProtocolVersion {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ProtocolVersion {
  type Proxied = Self;
  fn as_view(&self) -> ProtocolVersionView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ProtocolVersion {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ProtocolVersionMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for ProtocolVersionMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for ProtocolVersionView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_common_ProtocolVersion_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_common_ProtocolVersion_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_common_ProtocolVersion_major_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u32;
  fn proto2_rust_thunk_nearclip_common_ProtocolVersion_major_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u32);

  fn proto2_rust_thunk_nearclip_common_ProtocolVersion_minor_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u32;
  fn proto2_rust_thunk_nearclip_common_ProtocolVersion_minor_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u32);

  fn proto2_rust_thunk_nearclip_common_ProtocolVersion_patch_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u32;
  fn proto2_rust_thunk_nearclip_common_ProtocolVersion_patch_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u32);

  fn proto2_rust_thunk_nearclip_common_ProtocolVersion_build_info_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_common_ProtocolVersion_build_info_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> ProtocolVersionMut<'a> {
  pub unsafe fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ProtocolVersionView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for ProtocolVersion {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<ProtocolVersion>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for ProtocolVersionMut<'a> {
  unsafe fn __unstable_wrap_raw_message_mut(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  unsafe fn __unstable_wrap_raw_message_mut_unchecked_lifetime(
      msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  fn __unstable_as_raw_message_mut(&mut self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageViewInterop<'a> for ProtocolVersionView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

#[allow(non_camel_case_types)]
pub struct CapabilityNegotiation {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CapabilityNegotiation>
}

impl ::protobuf::Message for CapabilityNegotiation {}

impl ::std::default::Default for CapabilityNegotiation {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for CapabilityNegotiation {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for CapabilityNegotiation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for CapabilityNegotiation {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `CapabilityNegotiation` is `Sync` because it does not implement interior mutability.
//    Neither does `CapabilityNegotiationMut`.
unsafe impl Sync for CapabilityNegotiation {}

// SAFETY:
// - `CapabilityNegotiation` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for CapabilityNegotiation {}

impl ::protobuf::Proxied for CapabilityNegotiation {
  type View<'msg> = CapabilityNegotiationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CapabilityNegotiation {}

impl ::protobuf::MutProxied for CapabilityNegotiation {
  type Mut<'msg> = CapabilityNegotiationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CapabilityNegotiationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CapabilityNegotiation>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CapabilityNegotiationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CapabilityNegotiationView<'msg> {
  type Message = CapabilityNegotiation;
}

impl ::std::fmt::Debug for CapabilityNegotiationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for CapabilityNegotiationView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    let mut serialized_data = ::protobuf::__internal::runtime::SerializedData::new();
    let success = unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_serialize(self.raw_msg(), &mut serialized_data)
    };
    if success {
      Ok(serialized_data.into_vec())
    } else {
      Err(::protobuf::SerializeError)
    }
  }
}

impl ::std::default::Default for CapabilityNegotiationView<'_> {
  fn default() -> CapabilityNegotiationView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_common_CapabilityNegotiation_default_instance()) };
    CapabilityNegotiationView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> CapabilityNegotiationView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CapabilityNegotiation>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> CapabilityNegotiation {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // min_version: optional message nearclip.common.ProtocolVersion
  pub fn has_min_version(self) -> bool {
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_has(self.raw_msg())
    }
  }
  pub fn min_version_opt(self) -> ::protobuf::Optional<super::ProtocolVersionView<'msg>> {
        ::protobuf::Optional::new(self.min_version(), self.has_min_version())
  }
  pub fn min_version(self) -> super::ProtocolVersionView<'msg> {
    let submsg = unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_get(self.raw_msg()) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(submsg) };
    super::ProtocolVersionView::new(::protobuf::__internal::Private, inner)
  }

  // max_version: optional message nearclip.common.ProtocolVersion
  pub fn has_max_version(self) -> bool {
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_has(self.raw_msg())
    }
  }
  pub fn max_version_opt(self) -> ::protobuf::Optional<super::ProtocolVersionView<'msg>> {
        ::protobuf::Optional::new(self.max_version(), self.has_max_version())
  }
  pub fn max_version(self) -> super::ProtocolVersionView<'msg> {
    let submsg = unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_get(self.raw_msg()) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(submsg) };
    super::ProtocolVersionView::new(::protobuf::__internal::Private, inner)
  }

  // supported_features: repeated string
  pub fn supported_features(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_common_CapabilityNegotiation_supported_features_get(self.raw_msg()),
      )
    }
  }

  // required_features: repeated string
  pub fn required_features(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_common_CapabilityNegotiation_required_features_get(self.raw_msg()),
      )
    }
  }

}

// SAFETY:
// - `CapabilityNegotiationView` is `Sync` because it does not support mutation.
unsafe impl Sync for CapabilityNegotiationView<'_> {}

// SAFETY:
// - `CapabilityNegotiationView` is `Send` because while its alive a `CapabilityNegotiationMut` cannot.
// - `CapabilityNegotiationView` does not use thread-local data.
unsafe impl Send for CapabilityNegotiationView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for CapabilityNegotiationView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for CapabilityNegotiationView<'msg> {}

impl<'msg> ::protobuf::AsView for CapabilityNegotiationView<'msg> {
  type Proxied = CapabilityNegotiation;
  fn as_view(&self) -> ::protobuf::View<'msg, CapabilityNegotiation> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CapabilityNegotiationView<'msg> {
  fn into_view<'shorter>(self) -> CapabilityNegotiationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CapabilityNegotiation> for CapabilityNegotiationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CapabilityNegotiation {
    let dst = CapabilityNegotiation::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CapabilityNegotiation> for CapabilityNegotiationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CapabilityNegotiation {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for CapabilityNegotiation {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    // SAFETY:
    // - The thunk returns an unaliased and valid `RepeatedPtrField*`
    unsafe {
      ::protobuf::Repeated::from_inner(::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeated::from_raw(::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_new())
      )
    }
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    // SAFETY
    // - `f.raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_free(f.as_view().as_raw(::protobuf::__internal::Private)) }
  }

  fn repeated_len(f: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_size(f.as_raw(::protobuf::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
    v: impl ::protobuf::IntoProxied<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(
        ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get_mut(f.as_raw(::protobuf::__internal::Private), i),
        v.into_proxied(::protobuf::__internal::Private).raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::protobuf::View<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get(f.as_raw(::protobuf::__internal::Private), i) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(msg) };
    ::protobuf::View::<Self>::new(::protobuf::__internal::Private, inner)
  }

  unsafe fn repeated_get_mut_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::Mut<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get_mut(f.as_raw(::protobuf::__internal::Private), i) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(msg) };
    ::protobuf::Mut::<Self>::new(::protobuf::__internal::Private, inner)
  }

  fn repeated_clear(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_clear(f.as_raw(::protobuf::__internal::Private)) };
  }

  fn repeated_push(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>, v: impl ::protobuf::IntoProxied<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let prototype = <CapabilityNegotiationView as ::std::default::Default>::default().raw_msg();
      let new_elem = ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_add(f.as_raw(::protobuf::__internal::Private), prototype);
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(new_elem, v.into_proxied(::protobuf::__internal::Private).raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::protobuf::View<::protobuf::Repeated<Self>>,
    mut dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_copy_from(dest.as_raw(::protobuf::__internal::Private), src.as_raw(::protobuf::__internal::Private));
    }
  }

  fn repeated_reserve(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    additional: usize,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_reserve(f.as_raw(::protobuf::__internal::Private), additional) }
  }
}
impl ::protobuf::__internal::runtime::CppMapTypeConversions for CapabilityNegotiation {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<CapabilityNegotiationView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> CapabilityNegotiationView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { CapabilityNegotiationView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> CapabilityNegotiationMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        CapabilityNegotiationMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CapabilityNegotiationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CapabilityNegotiation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CapabilityNegotiationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CapabilityNegotiationMut<'msg> {
  type Message = CapabilityNegotiation;
}

impl ::std::fmt::Debug for CapabilityNegotiationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for CapabilityNegotiationMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> CapabilityNegotiationMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CapabilityNegotiation>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CapabilityNegotiation> {
    self.inner
  }

  pub fn to_owned(&self) -> CapabilityNegotiation {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // min_version: optional message nearclip.common.ProtocolVersion
  pub fn has_min_version(&self) -> bool {
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_has(self.raw_msg())
    }
  }
  pub fn clear_min_version(&mut self) {
    unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_clear(self.raw_msg()) }
  }
  pub fn min_version_opt(&self) -> ::protobuf::Optional<super::ProtocolVersionView<'_>> {
        ::protobuf::Optional::new(self.min_version(), self.has_min_version())
  }
  pub fn min_version(&self) -> super::ProtocolVersionView<'_> {
    let submsg = unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_get(self.raw_msg()) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(submsg) };
    super::ProtocolVersionView::new(::protobuf::__internal::Private, inner)
  }
  pub fn min_version_mut(&mut self) -> super::ProtocolVersionMut<'_> {
     let raw_msg = unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_get_mut(self.raw_msg()) };
     super::ProtocolVersionMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       raw_msg)
  }
  pub fn set_min_version(&mut self,
    val: impl ::protobuf::IntoProxied<super::ProtocolVersion>) {

    let mut val = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_set(
        self.inner.raw(),
        ::protobuf::__internal::runtime::CppGetRawMessageMut::get_raw_message_mut(&mut val, ::protobuf::__internal::Private));
    }
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let _ = std::mem::ManuallyDrop::new(val);
  }

  // max_version: optional message nearclip.common.ProtocolVersion
  pub fn has_max_version(&self) -> bool {
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_has(self.raw_msg())
    }
  }
  pub fn clear_max_version(&mut self) {
    unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_clear(self.raw_msg()) }
  }
  pub fn max_version_opt(&self) -> ::protobuf::Optional<super::ProtocolVersionView<'_>> {
        ::protobuf::Optional::new(self.max_version(), self.has_max_version())
  }
  pub fn max_version(&self) -> super::ProtocolVersionView<'_> {
    let submsg = unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_get(self.raw_msg()) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(submsg) };
    super::ProtocolVersionView::new(::protobuf::__internal::Private, inner)
  }
  pub fn max_version_mut(&mut self) -> super::ProtocolVersionMut<'_> {
     let raw_msg = unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_get_mut(self.raw_msg()) };
     super::ProtocolVersionMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       raw_msg)
  }
  pub fn set_max_version(&mut self,
    val: impl ::protobuf::IntoProxied<super::ProtocolVersion>) {

    let mut val = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_set(
        self.inner.raw(),
        ::protobuf::__internal::runtime::CppGetRawMessageMut::get_raw_message_mut(&mut val, ::protobuf::__internal::Private));
    }
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let _ = std::mem::ManuallyDrop::new(val);
  }

  // supported_features: repeated string
  pub fn supported_features(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_common_CapabilityNegotiation_supported_features_get(self.raw_msg()),
      )
    }
  }
  pub fn supported_features_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_common_CapabilityNegotiation_supported_features_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_supported_features(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiation_supported_features_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // required_features: repeated string
  pub fn required_features(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_common_CapabilityNegotiation_required_features_get(self.raw_msg()),
      )
    }
  }
  pub fn required_features_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_common_CapabilityNegotiation_required_features_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_required_features(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiation_required_features_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

}

// SAFETY:
// - `CapabilityNegotiationMut` does not perform any shared mutation.
// - `CapabilityNegotiationMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for CapabilityNegotiationMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for CapabilityNegotiationMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for CapabilityNegotiationMut<'msg> {}

impl<'msg> ::protobuf::AsView for CapabilityNegotiationMut<'msg> {
  type Proxied = CapabilityNegotiation;
  fn as_view(&self) -> ::protobuf::View<'_, CapabilityNegotiation> {
    CapabilityNegotiationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CapabilityNegotiationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CapabilityNegotiation>
  where
      'msg: 'shorter {
    CapabilityNegotiationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for CapabilityNegotiationMut<'msg> {
  type MutProxied = CapabilityNegotiation;
  fn as_mut(&mut self) -> CapabilityNegotiationMut<'msg> {
    CapabilityNegotiationMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CapabilityNegotiationMut<'msg> {
  fn into_mut<'shorter>(self) -> CapabilityNegotiationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CapabilityNegotiation {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_common_CapabilityNegotiation_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CapabilityNegotiation> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }


  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> CapabilityNegotiationView {
    CapabilityNegotiationView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> CapabilityNegotiationMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    CapabilityNegotiationMut::new(::protobuf::__internal::Private, inner)
  }

  // min_version: optional message nearclip.common.ProtocolVersion
  pub fn has_min_version(&self) -> bool {
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_has(self.raw_msg())
    }
  }
  pub fn clear_min_version(&mut self) {
    unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_clear(self.raw_msg()) }
  }
  pub fn min_version_opt(&self) -> ::protobuf::Optional<super::ProtocolVersionView<'_>> {
        ::protobuf::Optional::new(self.min_version(), self.has_min_version())
  }
  pub fn min_version(&self) -> super::ProtocolVersionView<'_> {
    let submsg = unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_get(self.raw_msg()) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(submsg) };
    super::ProtocolVersionView::new(::protobuf::__internal::Private, inner)
  }
  pub fn min_version_mut(&mut self) -> super::ProtocolVersionMut<'_> {
     let raw_msg = unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_get_mut(self.raw_msg()) };
     super::ProtocolVersionMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       raw_msg)
  }
  pub fn set_min_version(&mut self,
    val: impl ::protobuf::IntoProxied<super::ProtocolVersion>) {

    let mut val = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_set(
        self.inner.raw(),
        ::protobuf::__internal::runtime::CppGetRawMessageMut::get_raw_message_mut(&mut val, ::protobuf::__internal::Private));
    }
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let _ = std::mem::ManuallyDrop::new(val);
  }

  // max_version: optional message nearclip.common.ProtocolVersion
  pub fn has_max_version(&self) -> bool {
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_has(self.raw_msg())
    }
  }
  pub fn clear_max_version(&mut self) {
    unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_clear(self.raw_msg()) }
  }
  pub fn max_version_opt(&self) -> ::protobuf::Optional<super::ProtocolVersionView<'_>> {
        ::protobuf::Optional::new(self.max_version(), self.has_max_version())
  }
  pub fn max_version(&self) -> super::ProtocolVersionView<'_> {
    let submsg = unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_get(self.raw_msg()) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(submsg) };
    super::ProtocolVersionView::new(::protobuf::__internal::Private, inner)
  }
  pub fn max_version_mut(&mut self) -> super::ProtocolVersionMut<'_> {
     let raw_msg = unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_get_mut(self.raw_msg()) };
     super::ProtocolVersionMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       raw_msg)
  }
  pub fn set_max_version(&mut self,
    val: impl ::protobuf::IntoProxied<super::ProtocolVersion>) {

    let mut val = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_set(
        self.inner.raw(),
        ::protobuf::__internal::runtime::CppGetRawMessageMut::get_raw_message_mut(&mut val, ::protobuf::__internal::Private));
    }
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let _ = std::mem::ManuallyDrop::new(val);
  }

  // supported_features: repeated string
  pub fn supported_features(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_common_CapabilityNegotiation_supported_features_get(self.raw_msg()),
      )
    }
  }
  pub fn supported_features_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_common_CapabilityNegotiation_supported_features_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_supported_features(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiation_supported_features_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // required_features: repeated string
  pub fn required_features(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_common_CapabilityNegotiation_required_features_get(self.raw_msg()),
      )
    }
  }
  pub fn required_features_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_common_CapabilityNegotiation_required_features_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_required_features(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiation_required_features_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

}  // impl CapabilityNegotiation

impl ::std::ops::Drop for CapabilityNegotiation {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for CapabilityNegotiation {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CapabilityNegotiation {
  type Proxied = Self;
  fn as_view(&self) -> CapabilityNegotiationView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CapabilityNegotiation {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CapabilityNegotiationMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for CapabilityNegotiationMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for CapabilityNegotiationView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_common_CapabilityNegotiation_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_common_CapabilityNegotiation_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_has(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> bool;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_clear(raw_msg: ::protobuf::__internal::runtime::RawMessage);
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_get_mut(raw_msg: ::protobuf::__internal::runtime::RawMessage)
     -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiation_min_version_set(raw_msg: ::protobuf::__internal::runtime::RawMessage,
                    field_msg: ::protobuf::__internal::runtime::RawMessage);

  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_has(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> bool;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_clear(raw_msg: ::protobuf::__internal::runtime::RawMessage);
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_get_mut(raw_msg: ::protobuf::__internal::runtime::RawMessage)
     -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiation_max_version_set(raw_msg: ::protobuf::__internal::runtime::RawMessage,
                    field_msg: ::protobuf::__internal::runtime::RawMessage);

  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiation_supported_features_get_mut(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiation_supported_features_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiation_supported_features_move_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, value: ::protobuf::__internal::runtime::RawRepeatedField);

  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiation_required_features_get_mut(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiation_required_features_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiation_required_features_move_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, value: ::protobuf::__internal::runtime::RawRepeatedField);

}

impl<'a> CapabilityNegotiationMut<'a> {
  pub unsafe fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> CapabilityNegotiationView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for CapabilityNegotiation {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<CapabilityNegotiation>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for CapabilityNegotiationMut<'a> {
  unsafe fn __unstable_wrap_raw_message_mut(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  unsafe fn __unstable_wrap_raw_message_mut_unchecked_lifetime(
      msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  fn __unstable_as_raw_message_mut(&mut self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageViewInterop<'a> for CapabilityNegotiationView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

#[allow(non_camel_case_types)]
pub struct CapabilityNegotiationResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CapabilityNegotiationResponse>
}

impl ::protobuf::Message for CapabilityNegotiationResponse {}

impl ::std::default::Default for CapabilityNegotiationResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for CapabilityNegotiationResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for CapabilityNegotiationResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for CapabilityNegotiationResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `CapabilityNegotiationResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `CapabilityNegotiationResponseMut`.
unsafe impl Sync for CapabilityNegotiationResponse {}

// SAFETY:
// - `CapabilityNegotiationResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for CapabilityNegotiationResponse {}

impl ::protobuf::Proxied for CapabilityNegotiationResponse {
  type View<'msg> = CapabilityNegotiationResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CapabilityNegotiationResponse {}

impl ::protobuf::MutProxied for CapabilityNegotiationResponse {
  type Mut<'msg> = CapabilityNegotiationResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CapabilityNegotiationResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CapabilityNegotiationResponse>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CapabilityNegotiationResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CapabilityNegotiationResponseView<'msg> {
  type Message = CapabilityNegotiationResponse;
}

impl ::std::fmt::Debug for CapabilityNegotiationResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for CapabilityNegotiationResponseView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    let mut serialized_data = ::protobuf::__internal::runtime::SerializedData::new();
    let success = unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_serialize(self.raw_msg(), &mut serialized_data)
    };
    if success {
      Ok(serialized_data.into_vec())
    } else {
      Err(::protobuf::SerializeError)
    }
  }
}

impl ::std::default::Default for CapabilityNegotiationResponseView<'_> {
  fn default() -> CapabilityNegotiationResponseView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_common_CapabilityNegotiationResponse_default_instance()) };
    CapabilityNegotiationResponseView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> CapabilityNegotiationResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CapabilityNegotiationResponse>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> CapabilityNegotiationResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // selected_version: optional message nearclip.common.ProtocolVersion
  pub fn has_selected_version(self) -> bool {
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_has(self.raw_msg())
    }
  }
  pub fn selected_version_opt(self) -> ::protobuf::Optional<super::ProtocolVersionView<'msg>> {
        ::protobuf::Optional::new(self.selected_version(), self.has_selected_version())
  }
  pub fn selected_version(self) -> super::ProtocolVersionView<'msg> {
    let submsg = unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_get(self.raw_msg()) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(submsg) };
    super::ProtocolVersionView::new(::protobuf::__internal::Private, inner)
  }

  // supported_features: repeated string
  pub fn supported_features(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_supported_features_get(self.raw_msg()),
      )
    }
  }

  // unsupported_features: repeated string
  pub fn unsupported_features(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_unsupported_features_get(self.raw_msg()),
      )
    }
  }

  // compatibility: optional bool
  pub fn compatibility(self) -> bool {
    unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_compatibility_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `CapabilityNegotiationResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for CapabilityNegotiationResponseView<'_> {}

// SAFETY:
// - `CapabilityNegotiationResponseView` is `Send` because while its alive a `CapabilityNegotiationResponseMut` cannot.
// - `CapabilityNegotiationResponseView` does not use thread-local data.
unsafe impl Send for CapabilityNegotiationResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for CapabilityNegotiationResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for CapabilityNegotiationResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for CapabilityNegotiationResponseView<'msg> {
  type Proxied = CapabilityNegotiationResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, CapabilityNegotiationResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CapabilityNegotiationResponseView<'msg> {
  fn into_view<'shorter>(self) -> CapabilityNegotiationResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CapabilityNegotiationResponse> for CapabilityNegotiationResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CapabilityNegotiationResponse {
    let dst = CapabilityNegotiationResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CapabilityNegotiationResponse> for CapabilityNegotiationResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CapabilityNegotiationResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for CapabilityNegotiationResponse {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    // SAFETY:
    // - The thunk returns an unaliased and valid `RepeatedPtrField*`
    unsafe {
      ::protobuf::Repeated::from_inner(::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeated::from_raw(::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_new())
      )
    }
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    // SAFETY
    // - `f.raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_free(f.as_view().as_raw(::protobuf::__internal::Private)) }
  }

  fn repeated_len(f: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_size(f.as_raw(::protobuf::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
    v: impl ::protobuf::IntoProxied<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(
        ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get_mut(f.as_raw(::protobuf::__internal::Private), i),
        v.into_proxied(::protobuf::__internal::Private).raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::protobuf::View<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get(f.as_raw(::protobuf::__internal::Private), i) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(msg) };
    ::protobuf::View::<Self>::new(::protobuf::__internal::Private, inner)
  }

  unsafe fn repeated_get_mut_unchecked(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    i: usize,
  ) -> ::protobuf::Mut<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_get_mut(f.as_raw(::protobuf::__internal::Private), i) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(msg) };
    ::protobuf::Mut::<Self>::new(::protobuf::__internal::Private, inner)
  }

  fn repeated_clear(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_clear(f.as_raw(::protobuf::__internal::Private)) };
  }

  fn repeated_push(mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>, v: impl ::protobuf::IntoProxied<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let prototype = <CapabilityNegotiationResponseView as ::std::default::Default>::default().raw_msg();
      let new_elem = ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_add(f.as_raw(::protobuf::__internal::Private), prototype);
      ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(new_elem, v.into_proxied(::protobuf::__internal::Private).raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::protobuf::View<::protobuf::Repeated<Self>>,
    mut dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_copy_from(dest.as_raw(::protobuf::__internal::Private), src.as_raw(::protobuf::__internal::Private));
    }
  }

  fn repeated_reserve(
    mut f: ::protobuf::Mut<::protobuf::Repeated<Self>>,
    additional: usize,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { ::protobuf::__internal::runtime::proto2_rust_RepeatedField_Message_reserve(f.as_raw(::protobuf::__internal::Private), additional) }
  }
}
impl ::protobuf::__internal::runtime::CppMapTypeConversions for CapabilityNegotiationResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<CapabilityNegotiationResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> CapabilityNegotiationResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { CapabilityNegotiationResponseView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> CapabilityNegotiationResponseMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        CapabilityNegotiationResponseMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CapabilityNegotiationResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CapabilityNegotiationResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CapabilityNegotiationResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CapabilityNegotiationResponseMut<'msg> {
  type Message = CapabilityNegotiationResponse;
}

impl ::std::fmt::Debug for CapabilityNegotiationResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for CapabilityNegotiationResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> CapabilityNegotiationResponseMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CapabilityNegotiationResponse>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CapabilityNegotiationResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> CapabilityNegotiationResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // selected_version: optional message nearclip.common.ProtocolVersion
  pub fn has_selected_version(&self) -> bool {
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_has(self.raw_msg())
    }
  }
  pub fn clear_selected_version(&mut self) {
    unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_clear(self.raw_msg()) }
  }
  pub fn selected_version_opt(&self) -> ::protobuf::Optional<super::ProtocolVersionView<'_>> {
        ::protobuf::Optional::new(self.selected_version(), self.has_selected_version())
  }
  pub fn selected_version(&self) -> super::ProtocolVersionView<'_> {
    let submsg = unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_get(self.raw_msg()) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(submsg) };
    super::ProtocolVersionView::new(::protobuf::__internal::Private, inner)
  }
  pub fn selected_version_mut(&mut self) -> super::ProtocolVersionMut<'_> {
     let raw_msg = unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_get_mut(self.raw_msg()) };
     super::ProtocolVersionMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       raw_msg)
  }
  pub fn set_selected_version(&mut self,
    val: impl ::protobuf::IntoProxied<super::ProtocolVersion>) {

    let mut val = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_set(
        self.inner.raw(),
        ::protobuf::__internal::runtime::CppGetRawMessageMut::get_raw_message_mut(&mut val, ::protobuf::__internal::Private));
    }
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let _ = std::mem::ManuallyDrop::new(val);
  }

  // supported_features: repeated string
  pub fn supported_features(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_supported_features_get(self.raw_msg()),
      )
    }
  }
  pub fn supported_features_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_supported_features_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_supported_features(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_supported_features_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // unsupported_features: repeated string
  pub fn unsupported_features(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_unsupported_features_get(self.raw_msg()),
      )
    }
  }
  pub fn unsupported_features_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_unsupported_features_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_unsupported_features(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_unsupported_features_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // compatibility: optional bool
  pub fn compatibility(&self) -> bool {
    unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_compatibility_get(self.raw_msg()) }
  }
  pub fn set_compatibility(&mut self, val: bool) {
    unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_compatibility_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `CapabilityNegotiationResponseMut` does not perform any shared mutation.
// - `CapabilityNegotiationResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for CapabilityNegotiationResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for CapabilityNegotiationResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for CapabilityNegotiationResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for CapabilityNegotiationResponseMut<'msg> {
  type Proxied = CapabilityNegotiationResponse;
  fn as_view(&self) -> ::protobuf::View<'_, CapabilityNegotiationResponse> {
    CapabilityNegotiationResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CapabilityNegotiationResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CapabilityNegotiationResponse>
  where
      'msg: 'shorter {
    CapabilityNegotiationResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for CapabilityNegotiationResponseMut<'msg> {
  type MutProxied = CapabilityNegotiationResponse;
  fn as_mut(&mut self) -> CapabilityNegotiationResponseMut<'msg> {
    CapabilityNegotiationResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CapabilityNegotiationResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> CapabilityNegotiationResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CapabilityNegotiationResponse {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_common_CapabilityNegotiationResponse_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CapabilityNegotiationResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }


  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> CapabilityNegotiationResponseView {
    CapabilityNegotiationResponseView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> CapabilityNegotiationResponseMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    CapabilityNegotiationResponseMut::new(::protobuf::__internal::Private, inner)
  }

  // selected_version: optional message nearclip.common.ProtocolVersion
  pub fn has_selected_version(&self) -> bool {
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_has(self.raw_msg())
    }
  }
  pub fn clear_selected_version(&mut self) {
    unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_clear(self.raw_msg()) }
  }
  pub fn selected_version_opt(&self) -> ::protobuf::Optional<super::ProtocolVersionView<'_>> {
        ::protobuf::Optional::new(self.selected_version(), self.has_selected_version())
  }
  pub fn selected_version(&self) -> super::ProtocolVersionView<'_> {
    let submsg = unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_get(self.raw_msg()) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(submsg) };
    super::ProtocolVersionView::new(::protobuf::__internal::Private, inner)
  }
  pub fn selected_version_mut(&mut self) -> super::ProtocolVersionMut<'_> {
     let raw_msg = unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_get_mut(self.raw_msg()) };
     super::ProtocolVersionMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       raw_msg)
  }
  pub fn set_selected_version(&mut self,
    val: impl ::protobuf::IntoProxied<super::ProtocolVersion>) {

    let mut val = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_set(
        self.inner.raw(),
        ::protobuf::__internal::runtime::CppGetRawMessageMut::get_raw_message_mut(&mut val, ::protobuf::__internal::Private));
    }
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let _ = std::mem::ManuallyDrop::new(val);
  }

  // supported_features: repeated string
  pub fn supported_features(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_supported_features_get(self.raw_msg()),
      )
    }
  }
  pub fn supported_features_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_supported_features_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_supported_features(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_supported_features_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // unsupported_features: repeated string
  pub fn unsupported_features(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_unsupported_features_get(self.raw_msg()),
      )
    }
  }
  pub fn unsupported_features_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_unsupported_features_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_unsupported_features(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_unsupported_features_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // compatibility: optional bool
  pub fn compatibility(&self) -> bool {
    unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_compatibility_get(self.raw_msg()) }
  }
  pub fn set_compatibility(&mut self, val: bool) {
    unsafe { proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_compatibility_set(self.raw_msg(), val) }
  }

}  // impl CapabilityNegotiationResponse

impl ::std::ops::Drop for CapabilityNegotiationResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for CapabilityNegotiationResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CapabilityNegotiationResponse {
  type Proxied = Self;
  fn as_view(&self) -> CapabilityNegotiationResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CapabilityNegotiationResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CapabilityNegotiationResponseMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for CapabilityNegotiationResponseMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for CapabilityNegotiationResponseView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_common_CapabilityNegotiationResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_common_CapabilityNegotiationResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_has(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> bool;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_clear(raw_msg: ::protobuf::__internal::runtime::RawMessage);
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_get_mut(raw_msg: ::protobuf::__internal::runtime::RawMessage)
     -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_selected_version_set(raw_msg: ::protobuf::__internal::runtime::RawMessage,
                    field_msg: ::protobuf::__internal::runtime::RawMessage);

  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_supported_features_get_mut(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_supported_features_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_supported_features_move_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, value: ::protobuf::__internal::runtime::RawRepeatedField);

  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_unsupported_features_get_mut(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_unsupported_features_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_unsupported_features_move_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, value: ::protobuf::__internal::runtime::RawRepeatedField);

  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_compatibility_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> bool;
  fn proto2_rust_thunk_nearclip_common_CapabilityNegotiationResponse_compatibility_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: bool);

}

impl<'a> CapabilityNegotiationResponseMut<'a> {
  pub unsafe fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> CapabilityNegotiationResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

impl ::protobuf::OwnedMessageInterop for CapabilityNegotiationResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<CapabilityNegotiationResponse>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for CapabilityNegotiationResponseMut<'a> {
  unsafe fn __unstable_wrap_raw_message_mut(
      msg: &'a mut *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  unsafe fn __unstable_wrap_raw_message_mut_unchecked_lifetime(
      msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(raw) };
    Self { inner }
  }
  fn __unstable_as_raw_message_mut(&mut self) -> *mut ::std::ffi::c_void {
    self.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageViewInterop<'a> for CapabilityNegotiationResponseView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ErrorCode(i32);

#[allow(non_upper_case_globals)]
impl ErrorCode {
  pub const ErrorNone: ErrorCode = ErrorCode(0);
  pub const ErrorInvalidMessage: ErrorCode = ErrorCode(1);
  pub const ErrorInvalidSignature: ErrorCode = ErrorCode(2);
  pub const ErrorExpiredMessage: ErrorCode = ErrorCode(3);
  pub const ErrorUnsupportedVersion: ErrorCode = ErrorCode(4);
  pub const ErrorDeviceNotFound: ErrorCode = ErrorCode(5);
  pub const ErrorPairingFailed: ErrorCode = ErrorCode(6);
  pub const ErrorEncryptionFailed: ErrorCode = ErrorCode(7);
  pub const ErrorNetworkError: ErrorCode = ErrorCode(8);
  pub const ErrorTimeout: ErrorCode = ErrorCode(9);
  pub const ErrorQuotaExceeded: ErrorCode = ErrorCode(10);
  pub const ErrorInternalError: ErrorCode = ErrorCode(11);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "ErrorNone",
      1 => "ErrorInvalidMessage",
      2 => "ErrorInvalidSignature",
      3 => "ErrorExpiredMessage",
      4 => "ErrorUnsupportedVersion",
      5 => "ErrorDeviceNotFound",
      6 => "ErrorPairingFailed",
      7 => "ErrorEncryptionFailed",
      8 => "ErrorNetworkError",
      9 => "ErrorTimeout",
      10 => "ErrorQuotaExceeded",
      11 => "ErrorInternalError",
      _ => return None
    })
  }
}

impl ::std::convert::From<ErrorCode> for i32 {
  fn from(val: ErrorCode) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for ErrorCode {
  fn from(val: i32) -> ErrorCode {
    Self(val)
  }
}

impl ::std::default::Default for ErrorCode {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for ErrorCode {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "ErrorCode::{}", constant_name)
    } else {
      write!(f, "ErrorCode::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for ErrorCode {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for ErrorCode {}

impl ::protobuf::Proxied for ErrorCode {
  type View<'a> = ErrorCode;
}

impl ::protobuf::Proxy<'_> for ErrorCode {}
impl ::protobuf::ViewProxy<'_> for ErrorCode {}

impl ::protobuf::AsView for ErrorCode {
  type Proxied = ErrorCode;

  fn as_view(&self) -> ErrorCode {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ErrorCode {
  fn into_view<'shorter>(self) -> ErrorCode where 'msg: 'shorter {
    self
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for ErrorCode {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    ::protobuf::__internal::runtime::new_enum_repeated()
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    ::protobuf::__internal::runtime::free_enum_repeated(f)
  }

  fn repeated_len(r: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    ::protobuf::__internal::runtime::cast_enum_repeated_view(r).len()
  }

  fn repeated_push(r: ::protobuf::Mut<::protobuf::Repeated<Self>>, val: impl ::protobuf::IntoProxied<ErrorCode>) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(r).push(val.into_proxied(::protobuf::__internal::Private))
  }

  fn repeated_clear(r: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(r).clear()
  }

  unsafe fn repeated_get_unchecked(
      r: ::protobuf::View<::protobuf::Repeated<Self>>,
      index: usize,
  ) -> ::protobuf::View<ErrorCode> {
    // SAFETY: In-bounds as promised by the caller.
    unsafe {
      ::protobuf::__internal::runtime::cast_enum_repeated_view(r)
        .get_unchecked(index)
        .try_into()
        .unwrap_unchecked()
    }
  }

  unsafe fn repeated_set_unchecked(
      r: ::protobuf::Mut<::protobuf::Repeated<Self>>,
      index: usize,
      val: impl ::protobuf::IntoProxied<ErrorCode>,
  ) {
    // SAFETY: In-bounds as promised by the caller.
    unsafe {
      ::protobuf::__internal::runtime::cast_enum_repeated_mut(r)
        .set_unchecked(index, val.into_proxied(::protobuf::__internal::Private))
    }
  }

  fn repeated_copy_from(
      src: ::protobuf::View<::protobuf::Repeated<Self>>,
      dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
  ) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(dest)
      .copy_from(::protobuf::__internal::runtime::cast_enum_repeated_view(src))
  }

  fn repeated_reserve(
      r: ::protobuf::Mut<::protobuf::Repeated<Self>>,
      additional: usize,
  ) {
      // SAFETY:
      // - `f.as_raw()` is valid.
      ::protobuf::__internal::runtime::reserve_enum_repeated_mut(r, additional);
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for ErrorCode {
  const NAME: &'static str = "ErrorCode";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5|6|7|8|9|10|11)
  }
}

impl ::protobuf::__internal::runtime::CppMapTypeConversions for ErrorCode {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        Self::to_map_value(Self::default())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_u32(self.0 as u32)
    }

    unsafe fn from_map_value<'a>(value: ::protobuf::__internal::runtime::MapValue) -> ::protobuf::View<'a, Self> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::U32);
        ErrorCode(unsafe { value.val.u as i32 })
    }
}


