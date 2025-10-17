const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.32.1-release");
#[allow(non_camel_case_types)]
pub struct PairingRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PairingRequest>
}

impl ::protobuf::Message for PairingRequest {}

impl ::std::default::Default for PairingRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PairingRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PairingRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for PairingRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PairingRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `PairingRequestMut`.
unsafe impl Sync for PairingRequest {}

// SAFETY:
// - `PairingRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PairingRequest {}

impl ::protobuf::Proxied for PairingRequest {
  type View<'msg> = PairingRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PairingRequest {}

impl ::protobuf::MutProxied for PairingRequest {
  type Mut<'msg> = PairingRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PairingRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PairingRequest>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PairingRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PairingRequestView<'msg> {
  type Message = PairingRequest;
}

impl ::std::fmt::Debug for PairingRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for PairingRequestView<'_> {
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

impl ::std::default::Default for PairingRequestView<'_> {
  fn default() -> PairingRequestView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_pairing_PairingRequest_default_instance()) };
    PairingRequestView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PairingRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PairingRequest>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PairingRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // initiator_id: optional string
  pub fn initiator_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_initiator_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // target_id: optional string
  pub fn target_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_target_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // public_key: optional bytes
  pub fn public_key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_public_key_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }

  // device_name: optional string
  pub fn device_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_device_name_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // nonce: optional bytes
  pub fn nonce(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_nonce_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }

  // timestamp: optional uint64
  pub fn timestamp(self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_timestamp_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `PairingRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for PairingRequestView<'_> {}

// SAFETY:
// - `PairingRequestView` is `Send` because while its alive a `PairingRequestMut` cannot.
// - `PairingRequestView` does not use thread-local data.
unsafe impl Send for PairingRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PairingRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PairingRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for PairingRequestView<'msg> {
  type Proxied = PairingRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, PairingRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PairingRequestView<'msg> {
  fn into_view<'shorter>(self) -> PairingRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PairingRequest> for PairingRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PairingRequest {
    let dst = PairingRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PairingRequest> for PairingRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PairingRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for PairingRequest {
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
      let prototype = <PairingRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for PairingRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<PairingRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> PairingRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { PairingRequestView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> PairingRequestMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        PairingRequestMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PairingRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PairingRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PairingRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PairingRequestMut<'msg> {
  type Message = PairingRequest;
}

impl ::std::fmt::Debug for PairingRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for PairingRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PairingRequestMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PairingRequest>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PairingRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> PairingRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // initiator_id: optional string
  pub fn initiator_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_initiator_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_initiator_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingRequest_initiator_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // target_id: optional string
  pub fn target_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_target_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_target_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingRequest_target_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // public_key: optional bytes
  pub fn public_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_public_key_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_public_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingRequest_public_key_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // device_name: optional string
  pub fn device_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_device_name_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingRequest_device_name_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // nonce: optional bytes
  pub fn nonce(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_nonce_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_nonce(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingRequest_nonce_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_timestamp_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `PairingRequestMut` does not perform any shared mutation.
// - `PairingRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PairingRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PairingRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PairingRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for PairingRequestMut<'msg> {
  type Proxied = PairingRequest;
  fn as_view(&self) -> ::protobuf::View<'_, PairingRequest> {
    PairingRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PairingRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PairingRequest>
  where
      'msg: 'shorter {
    PairingRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PairingRequestMut<'msg> {
  type MutProxied = PairingRequest;
  fn as_mut(&mut self) -> PairingRequestMut<'msg> {
    PairingRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PairingRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> PairingRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PairingRequest {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_pairing_PairingRequest_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PairingRequest> {
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

  pub fn as_view(&self) -> PairingRequestView {
    PairingRequestView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PairingRequestMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PairingRequestMut::new(::protobuf::__internal::Private, inner)
  }

  // initiator_id: optional string
  pub fn initiator_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_initiator_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_initiator_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingRequest_initiator_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // target_id: optional string
  pub fn target_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_target_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_target_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingRequest_target_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // public_key: optional bytes
  pub fn public_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_public_key_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_public_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingRequest_public_key_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // device_name: optional string
  pub fn device_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_device_name_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingRequest_device_name_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // nonce: optional bytes
  pub fn nonce(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_nonce_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_nonce(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingRequest_nonce_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingRequest_timestamp_set(self.raw_msg(), val) }
  }

}  // impl PairingRequest

impl ::std::ops::Drop for PairingRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for PairingRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PairingRequest {
  type Proxied = Self;
  fn as_view(&self) -> PairingRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PairingRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PairingRequestMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for PairingRequestMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for PairingRequestView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_pairing_PairingRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_pairing_PairingRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_pairing_PairingRequest_initiator_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_pairing_PairingRequest_initiator_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_pairing_PairingRequest_target_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_pairing_PairingRequest_target_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_pairing_PairingRequest_public_key_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_pairing_PairingRequest_public_key_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_pairing_PairingRequest_device_name_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_pairing_PairingRequest_device_name_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_pairing_PairingRequest_nonce_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_pairing_PairingRequest_nonce_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_pairing_PairingRequest_timestamp_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u64;
  fn proto2_rust_thunk_nearclip_pairing_PairingRequest_timestamp_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u64);

}

impl<'a> PairingRequestMut<'a> {
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

impl<'a> PairingRequestView<'a> {
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

impl ::protobuf::OwnedMessageInterop for PairingRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<PairingRequest>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for PairingRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for PairingRequestView<'a> {
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
pub struct PairingResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PairingResponse>
}

impl ::protobuf::Message for PairingResponse {}

impl ::std::default::Default for PairingResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PairingResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PairingResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for PairingResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PairingResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `PairingResponseMut`.
unsafe impl Sync for PairingResponse {}

// SAFETY:
// - `PairingResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PairingResponse {}

impl ::protobuf::Proxied for PairingResponse {
  type View<'msg> = PairingResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PairingResponse {}

impl ::protobuf::MutProxied for PairingResponse {
  type Mut<'msg> = PairingResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PairingResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PairingResponse>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PairingResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PairingResponseView<'msg> {
  type Message = PairingResponse;
}

impl ::std::fmt::Debug for PairingResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for PairingResponseView<'_> {
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

impl ::std::default::Default for PairingResponseView<'_> {
  fn default() -> PairingResponseView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_pairing_PairingResponse_default_instance()) };
    PairingResponseView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PairingResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PairingResponse>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PairingResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // responder_id: optional string
  pub fn responder_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_responder_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // initiator_id: optional string
  pub fn initiator_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_initiator_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // public_key: optional bytes
  pub fn public_key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_public_key_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }

  // signed_nonce: optional bytes
  pub fn signed_nonce(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_signed_nonce_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }

  // shared_secret: optional bytes
  pub fn shared_secret(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_shared_secret_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }

  // timestamp: optional uint64
  pub fn timestamp(self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_timestamp_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `PairingResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for PairingResponseView<'_> {}

// SAFETY:
// - `PairingResponseView` is `Send` because while its alive a `PairingResponseMut` cannot.
// - `PairingResponseView` does not use thread-local data.
unsafe impl Send for PairingResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PairingResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PairingResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for PairingResponseView<'msg> {
  type Proxied = PairingResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, PairingResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PairingResponseView<'msg> {
  fn into_view<'shorter>(self) -> PairingResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PairingResponse> for PairingResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PairingResponse {
    let dst = PairingResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PairingResponse> for PairingResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PairingResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for PairingResponse {
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
      let prototype = <PairingResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for PairingResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<PairingResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> PairingResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { PairingResponseView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> PairingResponseMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        PairingResponseMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PairingResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PairingResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PairingResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PairingResponseMut<'msg> {
  type Message = PairingResponse;
}

impl ::std::fmt::Debug for PairingResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for PairingResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PairingResponseMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PairingResponse>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PairingResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> PairingResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // responder_id: optional string
  pub fn responder_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_responder_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_responder_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingResponse_responder_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // initiator_id: optional string
  pub fn initiator_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_initiator_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_initiator_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingResponse_initiator_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // public_key: optional bytes
  pub fn public_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_public_key_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_public_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingResponse_public_key_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // signed_nonce: optional bytes
  pub fn signed_nonce(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_signed_nonce_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_signed_nonce(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingResponse_signed_nonce_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // shared_secret: optional bytes
  pub fn shared_secret(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_shared_secret_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_shared_secret(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingResponse_shared_secret_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_timestamp_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `PairingResponseMut` does not perform any shared mutation.
// - `PairingResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PairingResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PairingResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PairingResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for PairingResponseMut<'msg> {
  type Proxied = PairingResponse;
  fn as_view(&self) -> ::protobuf::View<'_, PairingResponse> {
    PairingResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PairingResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PairingResponse>
  where
      'msg: 'shorter {
    PairingResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PairingResponseMut<'msg> {
  type MutProxied = PairingResponse;
  fn as_mut(&mut self) -> PairingResponseMut<'msg> {
    PairingResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PairingResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> PairingResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PairingResponse {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_pairing_PairingResponse_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PairingResponse> {
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

  pub fn as_view(&self) -> PairingResponseView {
    PairingResponseView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PairingResponseMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PairingResponseMut::new(::protobuf::__internal::Private, inner)
  }

  // responder_id: optional string
  pub fn responder_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_responder_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_responder_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingResponse_responder_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // initiator_id: optional string
  pub fn initiator_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_initiator_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_initiator_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingResponse_initiator_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // public_key: optional bytes
  pub fn public_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_public_key_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_public_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingResponse_public_key_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // signed_nonce: optional bytes
  pub fn signed_nonce(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_signed_nonce_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_signed_nonce(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingResponse_signed_nonce_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // shared_secret: optional bytes
  pub fn shared_secret(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_shared_secret_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_shared_secret(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingResponse_shared_secret_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingResponse_timestamp_set(self.raw_msg(), val) }
  }

}  // impl PairingResponse

impl ::std::ops::Drop for PairingResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for PairingResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PairingResponse {
  type Proxied = Self;
  fn as_view(&self) -> PairingResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PairingResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PairingResponseMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for PairingResponseMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for PairingResponseView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_pairing_PairingResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_pairing_PairingResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_pairing_PairingResponse_responder_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_pairing_PairingResponse_responder_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_pairing_PairingResponse_initiator_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_pairing_PairingResponse_initiator_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_pairing_PairingResponse_public_key_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_pairing_PairingResponse_public_key_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_pairing_PairingResponse_signed_nonce_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_pairing_PairingResponse_signed_nonce_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_pairing_PairingResponse_shared_secret_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_pairing_PairingResponse_shared_secret_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_pairing_PairingResponse_timestamp_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u64;
  fn proto2_rust_thunk_nearclip_pairing_PairingResponse_timestamp_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u64);

}

impl<'a> PairingResponseMut<'a> {
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

impl<'a> PairingResponseView<'a> {
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

impl ::protobuf::OwnedMessageInterop for PairingResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<PairingResponse>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for PairingResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for PairingResponseView<'a> {
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
pub struct PairingConfirmation {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PairingConfirmation>
}

impl ::protobuf::Message for PairingConfirmation {}

impl ::std::default::Default for PairingConfirmation {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PairingConfirmation {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PairingConfirmation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for PairingConfirmation {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PairingConfirmation` is `Sync` because it does not implement interior mutability.
//    Neither does `PairingConfirmationMut`.
unsafe impl Sync for PairingConfirmation {}

// SAFETY:
// - `PairingConfirmation` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PairingConfirmation {}

impl ::protobuf::Proxied for PairingConfirmation {
  type View<'msg> = PairingConfirmationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PairingConfirmation {}

impl ::protobuf::MutProxied for PairingConfirmation {
  type Mut<'msg> = PairingConfirmationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PairingConfirmationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PairingConfirmation>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PairingConfirmationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PairingConfirmationView<'msg> {
  type Message = PairingConfirmation;
}

impl ::std::fmt::Debug for PairingConfirmationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for PairingConfirmationView<'_> {
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

impl ::std::default::Default for PairingConfirmationView<'_> {
  fn default() -> PairingConfirmationView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_pairing_PairingConfirmation_default_instance()) };
    PairingConfirmationView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PairingConfirmationView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PairingConfirmation>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PairingConfirmation {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // session_id: optional string
  pub fn session_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingConfirmation_session_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // confirmation_hash: optional bytes
  pub fn confirmation_hash(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingConfirmation_confirmation_hash_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }

  // timestamp: optional uint64
  pub fn timestamp(self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingConfirmation_timestamp_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `PairingConfirmationView` is `Sync` because it does not support mutation.
unsafe impl Sync for PairingConfirmationView<'_> {}

// SAFETY:
// - `PairingConfirmationView` is `Send` because while its alive a `PairingConfirmationMut` cannot.
// - `PairingConfirmationView` does not use thread-local data.
unsafe impl Send for PairingConfirmationView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PairingConfirmationView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PairingConfirmationView<'msg> {}

impl<'msg> ::protobuf::AsView for PairingConfirmationView<'msg> {
  type Proxied = PairingConfirmation;
  fn as_view(&self) -> ::protobuf::View<'msg, PairingConfirmation> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PairingConfirmationView<'msg> {
  fn into_view<'shorter>(self) -> PairingConfirmationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PairingConfirmation> for PairingConfirmationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PairingConfirmation {
    let dst = PairingConfirmation::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PairingConfirmation> for PairingConfirmationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PairingConfirmation {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for PairingConfirmation {
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
      let prototype = <PairingConfirmationView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for PairingConfirmation {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<PairingConfirmationView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> PairingConfirmationView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { PairingConfirmationView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> PairingConfirmationMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        PairingConfirmationMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PairingConfirmationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PairingConfirmation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PairingConfirmationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PairingConfirmationMut<'msg> {
  type Message = PairingConfirmation;
}

impl ::std::fmt::Debug for PairingConfirmationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for PairingConfirmationMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PairingConfirmationMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PairingConfirmation>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PairingConfirmation> {
    self.inner
  }

  pub fn to_owned(&self) -> PairingConfirmation {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // session_id: optional string
  pub fn session_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingConfirmation_session_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_session_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingConfirmation_session_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // confirmation_hash: optional bytes
  pub fn confirmation_hash(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingConfirmation_confirmation_hash_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_confirmation_hash(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingConfirmation_confirmation_hash_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingConfirmation_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingConfirmation_timestamp_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `PairingConfirmationMut` does not perform any shared mutation.
// - `PairingConfirmationMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PairingConfirmationMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PairingConfirmationMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PairingConfirmationMut<'msg> {}

impl<'msg> ::protobuf::AsView for PairingConfirmationMut<'msg> {
  type Proxied = PairingConfirmation;
  fn as_view(&self) -> ::protobuf::View<'_, PairingConfirmation> {
    PairingConfirmationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PairingConfirmationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PairingConfirmation>
  where
      'msg: 'shorter {
    PairingConfirmationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PairingConfirmationMut<'msg> {
  type MutProxied = PairingConfirmation;
  fn as_mut(&mut self) -> PairingConfirmationMut<'msg> {
    PairingConfirmationMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PairingConfirmationMut<'msg> {
  fn into_mut<'shorter>(self) -> PairingConfirmationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PairingConfirmation {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_pairing_PairingConfirmation_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PairingConfirmation> {
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

  pub fn as_view(&self) -> PairingConfirmationView {
    PairingConfirmationView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PairingConfirmationMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PairingConfirmationMut::new(::protobuf::__internal::Private, inner)
  }

  // session_id: optional string
  pub fn session_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingConfirmation_session_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_session_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingConfirmation_session_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // confirmation_hash: optional bytes
  pub fn confirmation_hash(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingConfirmation_confirmation_hash_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_confirmation_hash(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingConfirmation_confirmation_hash_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingConfirmation_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingConfirmation_timestamp_set(self.raw_msg(), val) }
  }

}  // impl PairingConfirmation

impl ::std::ops::Drop for PairingConfirmation {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for PairingConfirmation {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PairingConfirmation {
  type Proxied = Self;
  fn as_view(&self) -> PairingConfirmationView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PairingConfirmation {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PairingConfirmationMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for PairingConfirmationMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for PairingConfirmationView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_pairing_PairingConfirmation_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_pairing_PairingConfirmation_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_pairing_PairingConfirmation_session_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_pairing_PairingConfirmation_session_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_pairing_PairingConfirmation_confirmation_hash_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_pairing_PairingConfirmation_confirmation_hash_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_pairing_PairingConfirmation_timestamp_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u64;
  fn proto2_rust_thunk_nearclip_pairing_PairingConfirmation_timestamp_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u64);

}

impl<'a> PairingConfirmationMut<'a> {
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

impl<'a> PairingConfirmationView<'a> {
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

impl ::protobuf::OwnedMessageInterop for PairingConfirmation {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<PairingConfirmation>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for PairingConfirmationMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for PairingConfirmationView<'a> {
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
pub struct PairingStatusUpdate {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PairingStatusUpdate>
}

impl ::protobuf::Message for PairingStatusUpdate {}

impl ::std::default::Default for PairingStatusUpdate {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PairingStatusUpdate {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PairingStatusUpdate {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for PairingStatusUpdate {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PairingStatusUpdate` is `Sync` because it does not implement interior mutability.
//    Neither does `PairingStatusUpdateMut`.
unsafe impl Sync for PairingStatusUpdate {}

// SAFETY:
// - `PairingStatusUpdate` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PairingStatusUpdate {}

impl ::protobuf::Proxied for PairingStatusUpdate {
  type View<'msg> = PairingStatusUpdateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PairingStatusUpdate {}

impl ::protobuf::MutProxied for PairingStatusUpdate {
  type Mut<'msg> = PairingStatusUpdateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PairingStatusUpdateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PairingStatusUpdate>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PairingStatusUpdateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PairingStatusUpdateView<'msg> {
  type Message = PairingStatusUpdate;
}

impl ::std::fmt::Debug for PairingStatusUpdateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for PairingStatusUpdateView<'_> {
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

impl ::std::default::Default for PairingStatusUpdateView<'_> {
  fn default() -> PairingStatusUpdateView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_pairing_PairingStatusUpdate_default_instance()) };
    PairingStatusUpdateView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PairingStatusUpdateView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PairingStatusUpdate>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PairingStatusUpdate {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // session_id: optional string
  pub fn session_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_session_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // status: optional enum nearclip.pairing.PairingStatus
  pub fn status(self) -> super::PairingStatus {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_status_get(self.raw_msg()) }
  }

  // error_message: optional string
  pub fn error_message(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_error_message_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // timestamp: optional uint64
  pub fn timestamp(self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_timestamp_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `PairingStatusUpdateView` is `Sync` because it does not support mutation.
unsafe impl Sync for PairingStatusUpdateView<'_> {}

// SAFETY:
// - `PairingStatusUpdateView` is `Send` because while its alive a `PairingStatusUpdateMut` cannot.
// - `PairingStatusUpdateView` does not use thread-local data.
unsafe impl Send for PairingStatusUpdateView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PairingStatusUpdateView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PairingStatusUpdateView<'msg> {}

impl<'msg> ::protobuf::AsView for PairingStatusUpdateView<'msg> {
  type Proxied = PairingStatusUpdate;
  fn as_view(&self) -> ::protobuf::View<'msg, PairingStatusUpdate> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PairingStatusUpdateView<'msg> {
  fn into_view<'shorter>(self) -> PairingStatusUpdateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PairingStatusUpdate> for PairingStatusUpdateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PairingStatusUpdate {
    let dst = PairingStatusUpdate::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PairingStatusUpdate> for PairingStatusUpdateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PairingStatusUpdate {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for PairingStatusUpdate {
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
      let prototype = <PairingStatusUpdateView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for PairingStatusUpdate {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<PairingStatusUpdateView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> PairingStatusUpdateView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { PairingStatusUpdateView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> PairingStatusUpdateMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        PairingStatusUpdateMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PairingStatusUpdateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PairingStatusUpdate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PairingStatusUpdateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PairingStatusUpdateMut<'msg> {
  type Message = PairingStatusUpdate;
}

impl ::std::fmt::Debug for PairingStatusUpdateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for PairingStatusUpdateMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PairingStatusUpdateMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PairingStatusUpdate>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PairingStatusUpdate> {
    self.inner
  }

  pub fn to_owned(&self) -> PairingStatusUpdate {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // session_id: optional string
  pub fn session_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_session_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_session_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_session_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // status: optional enum nearclip.pairing.PairingStatus
  pub fn status(&self) -> super::PairingStatus {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_status_get(self.raw_msg()) }
  }
  pub fn set_status(&mut self, val: super::PairingStatus) {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_status_set(self.raw_msg(), val) }
  }

  // error_message: optional string
  pub fn error_message(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_error_message_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_error_message(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_error_message_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_timestamp_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `PairingStatusUpdateMut` does not perform any shared mutation.
// - `PairingStatusUpdateMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PairingStatusUpdateMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PairingStatusUpdateMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PairingStatusUpdateMut<'msg> {}

impl<'msg> ::protobuf::AsView for PairingStatusUpdateMut<'msg> {
  type Proxied = PairingStatusUpdate;
  fn as_view(&self) -> ::protobuf::View<'_, PairingStatusUpdate> {
    PairingStatusUpdateView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PairingStatusUpdateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PairingStatusUpdate>
  where
      'msg: 'shorter {
    PairingStatusUpdateView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PairingStatusUpdateMut<'msg> {
  type MutProxied = PairingStatusUpdate;
  fn as_mut(&mut self) -> PairingStatusUpdateMut<'msg> {
    PairingStatusUpdateMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PairingStatusUpdateMut<'msg> {
  fn into_mut<'shorter>(self) -> PairingStatusUpdateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PairingStatusUpdate {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_pairing_PairingStatusUpdate_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PairingStatusUpdate> {
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

  pub fn as_view(&self) -> PairingStatusUpdateView {
    PairingStatusUpdateView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PairingStatusUpdateMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PairingStatusUpdateMut::new(::protobuf::__internal::Private, inner)
  }

  // session_id: optional string
  pub fn session_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_session_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_session_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_session_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // status: optional enum nearclip.pairing.PairingStatus
  pub fn status(&self) -> super::PairingStatus {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_status_get(self.raw_msg()) }
  }
  pub fn set_status(&mut self, val: super::PairingStatus) {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_status_set(self.raw_msg(), val) }
  }

  // error_message: optional string
  pub fn error_message(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_error_message_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_error_message(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_error_message_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_timestamp_set(self.raw_msg(), val) }
  }

}  // impl PairingStatusUpdate

impl ::std::ops::Drop for PairingStatusUpdate {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for PairingStatusUpdate {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PairingStatusUpdate {
  type Proxied = Self;
  fn as_view(&self) -> PairingStatusUpdateView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PairingStatusUpdate {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PairingStatusUpdateMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for PairingStatusUpdateMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for PairingStatusUpdateView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_pairing_PairingStatusUpdate_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_pairing_PairingStatusUpdate_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_session_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_session_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_status_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> super::PairingStatus;
  fn proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_status_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: super::PairingStatus);

  fn proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_error_message_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_error_message_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_timestamp_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u64;
  fn proto2_rust_thunk_nearclip_pairing_PairingStatusUpdate_timestamp_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u64);

}

impl<'a> PairingStatusUpdateMut<'a> {
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

impl<'a> PairingStatusUpdateView<'a> {
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

impl ::protobuf::OwnedMessageInterop for PairingStatusUpdate {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<PairingStatusUpdate>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for PairingStatusUpdateMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for PairingStatusUpdateView<'a> {
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
pub struct UnpairingRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UnpairingRequest>
}

impl ::protobuf::Message for UnpairingRequest {}

impl ::std::default::Default for UnpairingRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for UnpairingRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for UnpairingRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for UnpairingRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `UnpairingRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `UnpairingRequestMut`.
unsafe impl Sync for UnpairingRequest {}

// SAFETY:
// - `UnpairingRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for UnpairingRequest {}

impl ::protobuf::Proxied for UnpairingRequest {
  type View<'msg> = UnpairingRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UnpairingRequest {}

impl ::protobuf::MutProxied for UnpairingRequest {
  type Mut<'msg> = UnpairingRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UnpairingRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UnpairingRequest>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UnpairingRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UnpairingRequestView<'msg> {
  type Message = UnpairingRequest;
}

impl ::std::fmt::Debug for UnpairingRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for UnpairingRequestView<'_> {
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

impl ::std::default::Default for UnpairingRequestView<'_> {
  fn default() -> UnpairingRequestView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_pairing_UnpairingRequest_default_instance()) };
    UnpairingRequestView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> UnpairingRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UnpairingRequest>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> UnpairingRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // device_id: optional string
  pub fn device_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_UnpairingRequest_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // reason: optional string
  pub fn reason(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_UnpairingRequest_reason_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // signature: optional bytes
  pub fn signature(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_UnpairingRequest_signature_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `UnpairingRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for UnpairingRequestView<'_> {}

// SAFETY:
// - `UnpairingRequestView` is `Send` because while its alive a `UnpairingRequestMut` cannot.
// - `UnpairingRequestView` does not use thread-local data.
unsafe impl Send for UnpairingRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for UnpairingRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for UnpairingRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for UnpairingRequestView<'msg> {
  type Proxied = UnpairingRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, UnpairingRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UnpairingRequestView<'msg> {
  fn into_view<'shorter>(self) -> UnpairingRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UnpairingRequest> for UnpairingRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UnpairingRequest {
    let dst = UnpairingRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UnpairingRequest> for UnpairingRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UnpairingRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for UnpairingRequest {
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
      let prototype = <UnpairingRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for UnpairingRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<UnpairingRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> UnpairingRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { UnpairingRequestView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> UnpairingRequestMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        UnpairingRequestMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UnpairingRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UnpairingRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UnpairingRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UnpairingRequestMut<'msg> {
  type Message = UnpairingRequest;
}

impl ::std::fmt::Debug for UnpairingRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for UnpairingRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> UnpairingRequestMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UnpairingRequest>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UnpairingRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> UnpairingRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_UnpairingRequest_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_UnpairingRequest_device_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // reason: optional string
  pub fn reason(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_UnpairingRequest_reason_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_reason(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_UnpairingRequest_reason_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // signature: optional bytes
  pub fn signature(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_UnpairingRequest_signature_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_signature(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_UnpairingRequest_signature_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `UnpairingRequestMut` does not perform any shared mutation.
// - `UnpairingRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for UnpairingRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for UnpairingRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for UnpairingRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for UnpairingRequestMut<'msg> {
  type Proxied = UnpairingRequest;
  fn as_view(&self) -> ::protobuf::View<'_, UnpairingRequest> {
    UnpairingRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UnpairingRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UnpairingRequest>
  where
      'msg: 'shorter {
    UnpairingRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for UnpairingRequestMut<'msg> {
  type MutProxied = UnpairingRequest;
  fn as_mut(&mut self) -> UnpairingRequestMut<'msg> {
    UnpairingRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UnpairingRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> UnpairingRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UnpairingRequest {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_pairing_UnpairingRequest_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UnpairingRequest> {
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

  pub fn as_view(&self) -> UnpairingRequestView {
    UnpairingRequestView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> UnpairingRequestMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    UnpairingRequestMut::new(::protobuf::__internal::Private, inner)
  }

  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_UnpairingRequest_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_UnpairingRequest_device_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // reason: optional string
  pub fn reason(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_UnpairingRequest_reason_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_reason(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_UnpairingRequest_reason_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // signature: optional bytes
  pub fn signature(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_pairing_UnpairingRequest_signature_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_signature(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_pairing_UnpairingRequest_signature_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl UnpairingRequest

impl ::std::ops::Drop for UnpairingRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for UnpairingRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UnpairingRequest {
  type Proxied = Self;
  fn as_view(&self) -> UnpairingRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UnpairingRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UnpairingRequestMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for UnpairingRequestMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for UnpairingRequestView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_pairing_UnpairingRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_pairing_UnpairingRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_pairing_UnpairingRequest_device_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_pairing_UnpairingRequest_device_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_pairing_UnpairingRequest_reason_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_pairing_UnpairingRequest_reason_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_pairing_UnpairingRequest_signature_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_pairing_UnpairingRequest_signature_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> UnpairingRequestMut<'a> {
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

impl<'a> UnpairingRequestView<'a> {
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

impl ::protobuf::OwnedMessageInterop for UnpairingRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<UnpairingRequest>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for UnpairingRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for UnpairingRequestView<'a> {
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
pub struct PairingStatus(i32);

#[allow(non_upper_case_globals)]
impl PairingStatus {
  pub const PairingUnknown: PairingStatus = PairingStatus(0);
  pub const PairingInitiated: PairingStatus = PairingStatus(1);
  pub const PairingPending: PairingStatus = PairingStatus(2);
  pub const PairingConfirmed: PairingStatus = PairingStatus(3);
  pub const PairingFailed: PairingStatus = PairingStatus(4);
  pub const PairingCompleted: PairingStatus = PairingStatus(5);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "PairingUnknown",
      1 => "PairingInitiated",
      2 => "PairingPending",
      3 => "PairingConfirmed",
      4 => "PairingFailed",
      5 => "PairingCompleted",
      _ => return None
    })
  }
}

impl ::std::convert::From<PairingStatus> for i32 {
  fn from(val: PairingStatus) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for PairingStatus {
  fn from(val: i32) -> PairingStatus {
    Self(val)
  }
}

impl ::std::default::Default for PairingStatus {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for PairingStatus {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "PairingStatus::{}", constant_name)
    } else {
      write!(f, "PairingStatus::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for PairingStatus {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for PairingStatus {}

impl ::protobuf::Proxied for PairingStatus {
  type View<'a> = PairingStatus;
}

impl ::protobuf::Proxy<'_> for PairingStatus {}
impl ::protobuf::ViewProxy<'_> for PairingStatus {}

impl ::protobuf::AsView for PairingStatus {
  type Proxied = PairingStatus;

  fn as_view(&self) -> PairingStatus {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PairingStatus {
  fn into_view<'shorter>(self) -> PairingStatus where 'msg: 'shorter {
    self
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for PairingStatus {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    ::protobuf::__internal::runtime::new_enum_repeated()
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    ::protobuf::__internal::runtime::free_enum_repeated(f)
  }

  fn repeated_len(r: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    ::protobuf::__internal::runtime::cast_enum_repeated_view(r).len()
  }

  fn repeated_push(r: ::protobuf::Mut<::protobuf::Repeated<Self>>, val: impl ::protobuf::IntoProxied<PairingStatus>) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(r).push(val.into_proxied(::protobuf::__internal::Private))
  }

  fn repeated_clear(r: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(r).clear()
  }

  unsafe fn repeated_get_unchecked(
      r: ::protobuf::View<::protobuf::Repeated<Self>>,
      index: usize,
  ) -> ::protobuf::View<PairingStatus> {
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
      val: impl ::protobuf::IntoProxied<PairingStatus>,
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
unsafe impl ::protobuf::__internal::Enum for PairingStatus {
  const NAME: &'static str = "PairingStatus";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5)
  }
}

impl ::protobuf::__internal::runtime::CppMapTypeConversions for PairingStatus {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        Self::to_map_value(Self::default())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_u32(self.0 as u32)
    }

    unsafe fn from_map_value<'a>(value: ::protobuf::__internal::runtime::MapValue) -> ::protobuf::View<'a, Self> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::U32);
        PairingStatus(unsafe { value.val.u as i32 })
    }
}


