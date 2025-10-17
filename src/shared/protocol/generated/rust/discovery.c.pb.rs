const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.32.1-release");
#[allow(non_camel_case_types)]
pub struct DeviceBroadcast {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DeviceBroadcast>
}

impl ::protobuf::Message for DeviceBroadcast {}

impl ::std::default::Default for DeviceBroadcast {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for DeviceBroadcast {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for DeviceBroadcast {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for DeviceBroadcast {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `DeviceBroadcast` is `Sync` because it does not implement interior mutability.
//    Neither does `DeviceBroadcastMut`.
unsafe impl Sync for DeviceBroadcast {}

// SAFETY:
// - `DeviceBroadcast` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DeviceBroadcast {}

impl ::protobuf::Proxied for DeviceBroadcast {
  type View<'msg> = DeviceBroadcastView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DeviceBroadcast {}

impl ::protobuf::MutProxied for DeviceBroadcast {
  type Mut<'msg> = DeviceBroadcastMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DeviceBroadcastView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeviceBroadcast>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeviceBroadcastView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DeviceBroadcastView<'msg> {
  type Message = DeviceBroadcast;
}

impl ::std::fmt::Debug for DeviceBroadcastView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for DeviceBroadcastView<'_> {
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

impl ::std::default::Default for DeviceBroadcastView<'_> {
  fn default() -> DeviceBroadcastView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_discovery_DeviceBroadcast_default_instance()) };
    DeviceBroadcastView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> DeviceBroadcastView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeviceBroadcast>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> DeviceBroadcast {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // device_id: optional string
  pub fn device_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // device_name: optional string
  pub fn device_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_name_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // device_type: optional enum nearclip.discovery.DeviceType
  pub fn device_type(self) -> super::DeviceType {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_type_get(self.raw_msg()) }
  }

  // capabilities: repeated enum nearclip.discovery.DeviceCapability
  pub fn capabilities(self) -> ::protobuf::RepeatedView<'msg, super::DeviceCapability> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_capabilities_get(self.raw_msg()),
      )
    }
  }

  // version: optional string
  pub fn version(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_version_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // timestamp: optional uint64
  pub fn timestamp(self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_timestamp_get(self.raw_msg()) }
  }

  // public_key: optional bytes
  pub fn public_key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_public_key_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }

  // metadata: repeated message nearclip.discovery.DeviceBroadcast.MetadataEntry
  pub fn metadata(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::MapView::from_raw(::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_metadata_get(self.raw_msg()))
    }
  }

}

// SAFETY:
// - `DeviceBroadcastView` is `Sync` because it does not support mutation.
unsafe impl Sync for DeviceBroadcastView<'_> {}

// SAFETY:
// - `DeviceBroadcastView` is `Send` because while its alive a `DeviceBroadcastMut` cannot.
// - `DeviceBroadcastView` does not use thread-local data.
unsafe impl Send for DeviceBroadcastView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for DeviceBroadcastView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for DeviceBroadcastView<'msg> {}

impl<'msg> ::protobuf::AsView for DeviceBroadcastView<'msg> {
  type Proxied = DeviceBroadcast;
  fn as_view(&self) -> ::protobuf::View<'msg, DeviceBroadcast> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeviceBroadcastView<'msg> {
  fn into_view<'shorter>(self) -> DeviceBroadcastView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DeviceBroadcast> for DeviceBroadcastView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeviceBroadcast {
    let dst = DeviceBroadcast::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DeviceBroadcast> for DeviceBroadcastMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeviceBroadcast {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for DeviceBroadcast {
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
      let prototype = <DeviceBroadcastView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for DeviceBroadcast {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<DeviceBroadcastView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> DeviceBroadcastView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { DeviceBroadcastView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> DeviceBroadcastMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        DeviceBroadcastMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DeviceBroadcastMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeviceBroadcast>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeviceBroadcastMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DeviceBroadcastMut<'msg> {
  type Message = DeviceBroadcast;
}

impl ::std::fmt::Debug for DeviceBroadcastMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for DeviceBroadcastMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> DeviceBroadcastMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeviceBroadcast>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DeviceBroadcast> {
    self.inner
  }

  pub fn to_owned(&self) -> DeviceBroadcast {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // device_name: optional string
  pub fn device_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_name_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_name_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // device_type: optional enum nearclip.discovery.DeviceType
  pub fn device_type(&self) -> super::DeviceType {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_type_get(self.raw_msg()) }
  }
  pub fn set_device_type(&mut self, val: super::DeviceType) {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_type_set(self.raw_msg(), val) }
  }

  // capabilities: repeated enum nearclip.discovery.DeviceCapability
  pub fn capabilities(&self) -> ::protobuf::RepeatedView<'_, super::DeviceCapability> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_capabilities_get(self.raw_msg()),
      )
    }
  }
  pub fn capabilities_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::DeviceCapability> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_capabilities_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_capabilities(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::DeviceCapability>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_capabilities_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // version: optional string
  pub fn version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_version_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_version_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_timestamp_set(self.raw_msg(), val) }
  }

  // public_key: optional bytes
  pub fn public_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_public_key_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_public_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_public_key_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // metadata: repeated message nearclip.discovery.DeviceBroadcast.MetadataEntry
  pub fn metadata(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::MapView::from_raw(::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_metadata_get(self.raw_msg()))
    }
  }
  pub fn metadata_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
      unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_metadata_get_mut(self.raw_msg()) });
    unsafe { ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner) }
  }
  pub fn set_metadata(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    let val = ::std::mem::ManuallyDrop::new(
        src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_metadata_set(
          self.raw_msg(),
          val.as_raw(::protobuf::__internal::Private));
    }
  }

}

// SAFETY:
// - `DeviceBroadcastMut` does not perform any shared mutation.
// - `DeviceBroadcastMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for DeviceBroadcastMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for DeviceBroadcastMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for DeviceBroadcastMut<'msg> {}

impl<'msg> ::protobuf::AsView for DeviceBroadcastMut<'msg> {
  type Proxied = DeviceBroadcast;
  fn as_view(&self) -> ::protobuf::View<'_, DeviceBroadcast> {
    DeviceBroadcastView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeviceBroadcastMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DeviceBroadcast>
  where
      'msg: 'shorter {
    DeviceBroadcastView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for DeviceBroadcastMut<'msg> {
  type MutProxied = DeviceBroadcast;
  fn as_mut(&mut self) -> DeviceBroadcastMut<'msg> {
    DeviceBroadcastMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DeviceBroadcastMut<'msg> {
  fn into_mut<'shorter>(self) -> DeviceBroadcastMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DeviceBroadcast {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_discovery_DeviceBroadcast_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DeviceBroadcast> {
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

  pub fn as_view(&self) -> DeviceBroadcastView {
    DeviceBroadcastView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> DeviceBroadcastMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    DeviceBroadcastMut::new(::protobuf::__internal::Private, inner)
  }

  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // device_name: optional string
  pub fn device_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_name_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_name_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // device_type: optional enum nearclip.discovery.DeviceType
  pub fn device_type(&self) -> super::DeviceType {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_type_get(self.raw_msg()) }
  }
  pub fn set_device_type(&mut self, val: super::DeviceType) {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_type_set(self.raw_msg(), val) }
  }

  // capabilities: repeated enum nearclip.discovery.DeviceCapability
  pub fn capabilities(&self) -> ::protobuf::RepeatedView<'_, super::DeviceCapability> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_capabilities_get(self.raw_msg()),
      )
    }
  }
  pub fn capabilities_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::DeviceCapability> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_capabilities_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_capabilities(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::DeviceCapability>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_capabilities_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // version: optional string
  pub fn version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_version_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_version_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_timestamp_set(self.raw_msg(), val) }
  }

  // public_key: optional bytes
  pub fn public_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_public_key_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_public_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_public_key_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // metadata: repeated message nearclip.discovery.DeviceBroadcast.MetadataEntry
  pub fn metadata(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::MapView::from_raw(::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_metadata_get(self.raw_msg()))
    }
  }
  pub fn metadata_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
      unsafe { proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_metadata_get_mut(self.raw_msg()) });
    unsafe { ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner) }
  }
  pub fn set_metadata(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    let val = ::std::mem::ManuallyDrop::new(
        src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_metadata_set(
          self.raw_msg(),
          val.as_raw(::protobuf::__internal::Private));
    }
  }

}  // impl DeviceBroadcast

impl ::std::ops::Drop for DeviceBroadcast {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for DeviceBroadcast {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DeviceBroadcast {
  type Proxied = Self;
  fn as_view(&self) -> DeviceBroadcastView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DeviceBroadcast {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DeviceBroadcastMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for DeviceBroadcastMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for DeviceBroadcastView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

pub mod device_broadcast {

}  // pub mod device_broadcast
extern "C" {
  fn proto2_rust_thunk_Message_nearclip_discovery_DeviceBroadcast_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_discovery_DeviceBroadcast_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_name_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_name_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_type_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> super::DeviceType;
  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_device_type_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: super::DeviceType);

  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_capabilities_get_mut(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_capabilities_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_capabilities_move_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, value: ::protobuf::__internal::runtime::RawRepeatedField);

  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_version_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_version_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_timestamp_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u64;
  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_timestamp_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u64);

  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_public_key_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_public_key_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_metadata_get(msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawMap;
  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_metadata_get_mut(msg: ::protobuf::__internal::runtime::RawMessage,) -> ::protobuf::__internal::runtime::RawMap;
  fn proto2_rust_thunk_nearclip_discovery_DeviceBroadcast_metadata_set(
      raw_msg: ::protobuf::__internal::runtime::RawMessage,
      value: ::protobuf::__internal::runtime::RawMap);

}

impl<'a> DeviceBroadcastMut<'a> {
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

impl<'a> DeviceBroadcastView<'a> {
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

impl ::protobuf::OwnedMessageInterop for DeviceBroadcast {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<DeviceBroadcast>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for DeviceBroadcastMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for DeviceBroadcastView<'a> {
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
pub struct ScanRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ScanRequest>
}

impl ::protobuf::Message for ScanRequest {}

impl ::std::default::Default for ScanRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for ScanRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for ScanRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ScanRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `ScanRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ScanRequestMut`.
unsafe impl Sync for ScanRequest {}

// SAFETY:
// - `ScanRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ScanRequest {}

impl ::protobuf::Proxied for ScanRequest {
  type View<'msg> = ScanRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ScanRequest {}

impl ::protobuf::MutProxied for ScanRequest {
  type Mut<'msg> = ScanRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ScanRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScanRequest>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScanRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ScanRequestView<'msg> {
  type Message = ScanRequest;
}

impl ::std::fmt::Debug for ScanRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ScanRequestView<'_> {
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

impl ::std::default::Default for ScanRequestView<'_> {
  fn default() -> ScanRequestView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_discovery_ScanRequest_default_instance()) };
    ScanRequestView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> ScanRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScanRequest>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> ScanRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // timeout_seconds: optional uint32
  pub fn timeout_seconds(self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_discovery_ScanRequest_timeout_seconds_get(self.raw_msg()) }
  }

  // filter_types: repeated enum nearclip.discovery.DeviceType
  pub fn filter_types(self) -> ::protobuf::RepeatedView<'msg, super::DeviceType> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_ScanRequest_filter_types_get(self.raw_msg()),
      )
    }
  }

  // required_capabilities: repeated enum nearclip.discovery.DeviceCapability
  pub fn required_capabilities(self) -> ::protobuf::RepeatedView<'msg, super::DeviceCapability> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_ScanRequest_required_capabilities_get(self.raw_msg()),
      )
    }
  }

}

// SAFETY:
// - `ScanRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ScanRequestView<'_> {}

// SAFETY:
// - `ScanRequestView` is `Send` because while its alive a `ScanRequestMut` cannot.
// - `ScanRequestView` does not use thread-local data.
unsafe impl Send for ScanRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ScanRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for ScanRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for ScanRequestView<'msg> {
  type Proxied = ScanRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ScanRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScanRequestView<'msg> {
  fn into_view<'shorter>(self) -> ScanRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ScanRequest> for ScanRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScanRequest {
    let dst = ScanRequest::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ScanRequest> for ScanRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScanRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for ScanRequest {
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
      let prototype = <ScanRequestView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for ScanRequest {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<ScanRequestView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> ScanRequestView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { ScanRequestView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> ScanRequestMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        ScanRequestMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ScanRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScanRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScanRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ScanRequestMut<'msg> {
  type Message = ScanRequest;
}

impl ::std::fmt::Debug for ScanRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ScanRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> ScanRequestMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScanRequest>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ScanRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> ScanRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // timeout_seconds: optional uint32
  pub fn timeout_seconds(&self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_discovery_ScanRequest_timeout_seconds_get(self.raw_msg()) }
  }
  pub fn set_timeout_seconds(&mut self, val: u32) {
    unsafe { proto2_rust_thunk_nearclip_discovery_ScanRequest_timeout_seconds_set(self.raw_msg(), val) }
  }

  // filter_types: repeated enum nearclip.discovery.DeviceType
  pub fn filter_types(&self) -> ::protobuf::RepeatedView<'_, super::DeviceType> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_ScanRequest_filter_types_get(self.raw_msg()),
      )
    }
  }
  pub fn filter_types_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::DeviceType> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_discovery_ScanRequest_filter_types_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_filter_types(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::DeviceType>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_discovery_ScanRequest_filter_types_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // required_capabilities: repeated enum nearclip.discovery.DeviceCapability
  pub fn required_capabilities(&self) -> ::protobuf::RepeatedView<'_, super::DeviceCapability> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_ScanRequest_required_capabilities_get(self.raw_msg()),
      )
    }
  }
  pub fn required_capabilities_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::DeviceCapability> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_discovery_ScanRequest_required_capabilities_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_required_capabilities(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::DeviceCapability>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_discovery_ScanRequest_required_capabilities_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

}

// SAFETY:
// - `ScanRequestMut` does not perform any shared mutation.
// - `ScanRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ScanRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ScanRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for ScanRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for ScanRequestMut<'msg> {
  type Proxied = ScanRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ScanRequest> {
    ScanRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScanRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ScanRequest>
  where
      'msg: 'shorter {
    ScanRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for ScanRequestMut<'msg> {
  type MutProxied = ScanRequest;
  fn as_mut(&mut self) -> ScanRequestMut<'msg> {
    ScanRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ScanRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ScanRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ScanRequest {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_discovery_ScanRequest_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ScanRequest> {
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

  pub fn as_view(&self) -> ScanRequestView {
    ScanRequestView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> ScanRequestMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    ScanRequestMut::new(::protobuf::__internal::Private, inner)
  }

  // timeout_seconds: optional uint32
  pub fn timeout_seconds(&self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_discovery_ScanRequest_timeout_seconds_get(self.raw_msg()) }
  }
  pub fn set_timeout_seconds(&mut self, val: u32) {
    unsafe { proto2_rust_thunk_nearclip_discovery_ScanRequest_timeout_seconds_set(self.raw_msg(), val) }
  }

  // filter_types: repeated enum nearclip.discovery.DeviceType
  pub fn filter_types(&self) -> ::protobuf::RepeatedView<'_, super::DeviceType> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_ScanRequest_filter_types_get(self.raw_msg()),
      )
    }
  }
  pub fn filter_types_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::DeviceType> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_discovery_ScanRequest_filter_types_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_filter_types(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::DeviceType>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_discovery_ScanRequest_filter_types_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // required_capabilities: repeated enum nearclip.discovery.DeviceCapability
  pub fn required_capabilities(&self) -> ::protobuf::RepeatedView<'_, super::DeviceCapability> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_ScanRequest_required_capabilities_get(self.raw_msg()),
      )
    }
  }
  pub fn required_capabilities_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::DeviceCapability> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_discovery_ScanRequest_required_capabilities_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_required_capabilities(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::DeviceCapability>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_discovery_ScanRequest_required_capabilities_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

}  // impl ScanRequest

impl ::std::ops::Drop for ScanRequest {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for ScanRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ScanRequest {
  type Proxied = Self;
  fn as_view(&self) -> ScanRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ScanRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ScanRequestMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for ScanRequestMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for ScanRequestView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_discovery_ScanRequest_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_discovery_ScanRequest_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_discovery_ScanRequest_timeout_seconds_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u32;
  fn proto2_rust_thunk_nearclip_discovery_ScanRequest_timeout_seconds_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u32);

  fn proto2_rust_thunk_nearclip_discovery_ScanRequest_filter_types_get_mut(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_discovery_ScanRequest_filter_types_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_discovery_ScanRequest_filter_types_move_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, value: ::protobuf::__internal::runtime::RawRepeatedField);

  fn proto2_rust_thunk_nearclip_discovery_ScanRequest_required_capabilities_get_mut(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_discovery_ScanRequest_required_capabilities_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_discovery_ScanRequest_required_capabilities_move_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, value: ::protobuf::__internal::runtime::RawRepeatedField);

}

impl<'a> ScanRequestMut<'a> {
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

impl<'a> ScanRequestView<'a> {
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

impl ::protobuf::OwnedMessageInterop for ScanRequest {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<ScanRequest>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for ScanRequestMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for ScanRequestView<'a> {
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
pub struct ScanResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ScanResponse>
}

impl ::protobuf::Message for ScanResponse {}

impl ::std::default::Default for ScanResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for ScanResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for ScanResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ScanResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `ScanResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ScanResponseMut`.
unsafe impl Sync for ScanResponse {}

// SAFETY:
// - `ScanResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ScanResponse {}

impl ::protobuf::Proxied for ScanResponse {
  type View<'msg> = ScanResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ScanResponse {}

impl ::protobuf::MutProxied for ScanResponse {
  type Mut<'msg> = ScanResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ScanResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScanResponse>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScanResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ScanResponseView<'msg> {
  type Message = ScanResponse;
}

impl ::std::fmt::Debug for ScanResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ScanResponseView<'_> {
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

impl ::std::default::Default for ScanResponseView<'_> {
  fn default() -> ScanResponseView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_discovery_ScanResponse_default_instance()) };
    ScanResponseView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> ScanResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScanResponse>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> ScanResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // devices: repeated message nearclip.discovery.DeviceBroadcast
  pub fn devices(self) -> ::protobuf::RepeatedView<'msg, super::DeviceBroadcast> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_ScanResponse_devices_get(self.raw_msg()),
      )
    }
  }

  // scan_duration_ms: optional uint64
  pub fn scan_duration_ms(self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_discovery_ScanResponse_scan_duration_ms_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `ScanResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for ScanResponseView<'_> {}

// SAFETY:
// - `ScanResponseView` is `Send` because while its alive a `ScanResponseMut` cannot.
// - `ScanResponseView` does not use thread-local data.
unsafe impl Send for ScanResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ScanResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for ScanResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for ScanResponseView<'msg> {
  type Proxied = ScanResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ScanResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScanResponseView<'msg> {
  fn into_view<'shorter>(self) -> ScanResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ScanResponse> for ScanResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScanResponse {
    let dst = ScanResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ScanResponse> for ScanResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScanResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for ScanResponse {
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
      let prototype = <ScanResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for ScanResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<ScanResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> ScanResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { ScanResponseView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> ScanResponseMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        ScanResponseMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ScanResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScanResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScanResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ScanResponseMut<'msg> {
  type Message = ScanResponse;
}

impl ::std::fmt::Debug for ScanResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ScanResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> ScanResponseMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScanResponse>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ScanResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> ScanResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // devices: repeated message nearclip.discovery.DeviceBroadcast
  pub fn devices(&self) -> ::protobuf::RepeatedView<'_, super::DeviceBroadcast> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_ScanResponse_devices_get(self.raw_msg()),
      )
    }
  }
  pub fn devices_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::DeviceBroadcast> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_discovery_ScanResponse_devices_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_devices(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::DeviceBroadcast>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_discovery_ScanResponse_devices_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // scan_duration_ms: optional uint64
  pub fn scan_duration_ms(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_discovery_ScanResponse_scan_duration_ms_get(self.raw_msg()) }
  }
  pub fn set_scan_duration_ms(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_discovery_ScanResponse_scan_duration_ms_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `ScanResponseMut` does not perform any shared mutation.
// - `ScanResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ScanResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ScanResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for ScanResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for ScanResponseMut<'msg> {
  type Proxied = ScanResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ScanResponse> {
    ScanResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScanResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ScanResponse>
  where
      'msg: 'shorter {
    ScanResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for ScanResponseMut<'msg> {
  type MutProxied = ScanResponse;
  fn as_mut(&mut self) -> ScanResponseMut<'msg> {
    ScanResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ScanResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ScanResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ScanResponse {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_discovery_ScanResponse_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ScanResponse> {
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

  pub fn as_view(&self) -> ScanResponseView {
    ScanResponseView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> ScanResponseMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    ScanResponseMut::new(::protobuf::__internal::Private, inner)
  }

  // devices: repeated message nearclip.discovery.DeviceBroadcast
  pub fn devices(&self) -> ::protobuf::RepeatedView<'_, super::DeviceBroadcast> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_ScanResponse_devices_get(self.raw_msg()),
      )
    }
  }
  pub fn devices_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::DeviceBroadcast> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_discovery_ScanResponse_devices_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_devices(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::DeviceBroadcast>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_discovery_ScanResponse_devices_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // scan_duration_ms: optional uint64
  pub fn scan_duration_ms(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_discovery_ScanResponse_scan_duration_ms_get(self.raw_msg()) }
  }
  pub fn set_scan_duration_ms(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_discovery_ScanResponse_scan_duration_ms_set(self.raw_msg(), val) }
  }

}  // impl ScanResponse

impl ::std::ops::Drop for ScanResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for ScanResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ScanResponse {
  type Proxied = Self;
  fn as_view(&self) -> ScanResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ScanResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ScanResponseMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for ScanResponseMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for ScanResponseView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_discovery_ScanResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_discovery_ScanResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_discovery_ScanResponse_devices_get_mut(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_discovery_ScanResponse_devices_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_discovery_ScanResponse_devices_move_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, value: ::protobuf::__internal::runtime::RawRepeatedField);

  fn proto2_rust_thunk_nearclip_discovery_ScanResponse_scan_duration_ms_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u64;
  fn proto2_rust_thunk_nearclip_discovery_ScanResponse_scan_duration_ms_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u64);

}

impl<'a> ScanResponseMut<'a> {
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

impl<'a> ScanResponseView<'a> {
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

impl ::protobuf::OwnedMessageInterop for ScanResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<ScanResponse>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for ScanResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for ScanResponseView<'a> {
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
pub struct DeviceQuery {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DeviceQuery>
}

impl ::protobuf::Message for DeviceQuery {}

impl ::std::default::Default for DeviceQuery {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for DeviceQuery {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for DeviceQuery {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for DeviceQuery {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `DeviceQuery` is `Sync` because it does not implement interior mutability.
//    Neither does `DeviceQueryMut`.
unsafe impl Sync for DeviceQuery {}

// SAFETY:
// - `DeviceQuery` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DeviceQuery {}

impl ::protobuf::Proxied for DeviceQuery {
  type View<'msg> = DeviceQueryView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DeviceQuery {}

impl ::protobuf::MutProxied for DeviceQuery {
  type Mut<'msg> = DeviceQueryMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DeviceQueryView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeviceQuery>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeviceQueryView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DeviceQueryView<'msg> {
  type Message = DeviceQuery;
}

impl ::std::fmt::Debug for DeviceQueryView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for DeviceQueryView<'_> {
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

impl ::std::default::Default for DeviceQueryView<'_> {
  fn default() -> DeviceQueryView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_discovery_DeviceQuery_default_instance()) };
    DeviceQueryView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> DeviceQueryView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeviceQuery>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> DeviceQuery {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // device_id: optional string
  pub fn device_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQuery_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // capabilities: repeated enum nearclip.discovery.DeviceCapability
  pub fn capabilities(self) -> ::protobuf::RepeatedView<'msg, super::DeviceCapability> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_DeviceQuery_capabilities_get(self.raw_msg()),
      )
    }
  }

}

// SAFETY:
// - `DeviceQueryView` is `Sync` because it does not support mutation.
unsafe impl Sync for DeviceQueryView<'_> {}

// SAFETY:
// - `DeviceQueryView` is `Send` because while its alive a `DeviceQueryMut` cannot.
// - `DeviceQueryView` does not use thread-local data.
unsafe impl Send for DeviceQueryView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for DeviceQueryView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for DeviceQueryView<'msg> {}

impl<'msg> ::protobuf::AsView for DeviceQueryView<'msg> {
  type Proxied = DeviceQuery;
  fn as_view(&self) -> ::protobuf::View<'msg, DeviceQuery> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeviceQueryView<'msg> {
  fn into_view<'shorter>(self) -> DeviceQueryView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DeviceQuery> for DeviceQueryView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeviceQuery {
    let dst = DeviceQuery::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DeviceQuery> for DeviceQueryMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeviceQuery {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for DeviceQuery {
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
      let prototype = <DeviceQueryView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for DeviceQuery {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<DeviceQueryView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> DeviceQueryView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { DeviceQueryView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> DeviceQueryMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        DeviceQueryMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DeviceQueryMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeviceQuery>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeviceQueryMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DeviceQueryMut<'msg> {
  type Message = DeviceQuery;
}

impl ::std::fmt::Debug for DeviceQueryMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for DeviceQueryMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> DeviceQueryMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeviceQuery>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DeviceQuery> {
    self.inner
  }

  pub fn to_owned(&self) -> DeviceQuery {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQuery_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceQuery_device_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // capabilities: repeated enum nearclip.discovery.DeviceCapability
  pub fn capabilities(&self) -> ::protobuf::RepeatedView<'_, super::DeviceCapability> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_DeviceQuery_capabilities_get(self.raw_msg()),
      )
    }
  }
  pub fn capabilities_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::DeviceCapability> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_discovery_DeviceQuery_capabilities_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_capabilities(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::DeviceCapability>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceQuery_capabilities_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

}

// SAFETY:
// - `DeviceQueryMut` does not perform any shared mutation.
// - `DeviceQueryMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for DeviceQueryMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for DeviceQueryMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for DeviceQueryMut<'msg> {}

impl<'msg> ::protobuf::AsView for DeviceQueryMut<'msg> {
  type Proxied = DeviceQuery;
  fn as_view(&self) -> ::protobuf::View<'_, DeviceQuery> {
    DeviceQueryView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeviceQueryMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DeviceQuery>
  where
      'msg: 'shorter {
    DeviceQueryView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for DeviceQueryMut<'msg> {
  type MutProxied = DeviceQuery;
  fn as_mut(&mut self) -> DeviceQueryMut<'msg> {
    DeviceQueryMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DeviceQueryMut<'msg> {
  fn into_mut<'shorter>(self) -> DeviceQueryMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DeviceQuery {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_discovery_DeviceQuery_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DeviceQuery> {
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

  pub fn as_view(&self) -> DeviceQueryView {
    DeviceQueryView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> DeviceQueryMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    DeviceQueryMut::new(::protobuf::__internal::Private, inner)
  }

  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQuery_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceQuery_device_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // capabilities: repeated enum nearclip.discovery.DeviceCapability
  pub fn capabilities(&self) -> ::protobuf::RepeatedView<'_, super::DeviceCapability> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_discovery_DeviceQuery_capabilities_get(self.raw_msg()),
      )
    }
  }
  pub fn capabilities_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::DeviceCapability> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_discovery_DeviceQuery_capabilities_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_capabilities(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::DeviceCapability>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceQuery_capabilities_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

}  // impl DeviceQuery

impl ::std::ops::Drop for DeviceQuery {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for DeviceQuery {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DeviceQuery {
  type Proxied = Self;
  fn as_view(&self) -> DeviceQueryView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DeviceQuery {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DeviceQueryMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for DeviceQueryMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for DeviceQueryView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_discovery_DeviceQuery_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_discovery_DeviceQuery_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_discovery_DeviceQuery_device_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_discovery_DeviceQuery_device_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_discovery_DeviceQuery_capabilities_get_mut(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_discovery_DeviceQuery_capabilities_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_discovery_DeviceQuery_capabilities_move_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, value: ::protobuf::__internal::runtime::RawRepeatedField);

}

impl<'a> DeviceQueryMut<'a> {
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

impl<'a> DeviceQueryView<'a> {
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

impl ::protobuf::OwnedMessageInterop for DeviceQuery {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<DeviceQuery>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for DeviceQueryMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for DeviceQueryView<'a> {
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
pub struct DeviceQueryResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DeviceQueryResponse>
}

impl ::protobuf::Message for DeviceQueryResponse {}

impl ::std::default::Default for DeviceQueryResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for DeviceQueryResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for DeviceQueryResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for DeviceQueryResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `DeviceQueryResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `DeviceQueryResponseMut`.
unsafe impl Sync for DeviceQueryResponse {}

// SAFETY:
// - `DeviceQueryResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DeviceQueryResponse {}

impl ::protobuf::Proxied for DeviceQueryResponse {
  type View<'msg> = DeviceQueryResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DeviceQueryResponse {}

impl ::protobuf::MutProxied for DeviceQueryResponse {
  type Mut<'msg> = DeviceQueryResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DeviceQueryResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeviceQueryResponse>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeviceQueryResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DeviceQueryResponseView<'msg> {
  type Message = DeviceQueryResponse;
}

impl ::std::fmt::Debug for DeviceQueryResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for DeviceQueryResponseView<'_> {
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

impl ::std::default::Default for DeviceQueryResponseView<'_> {
  fn default() -> DeviceQueryResponseView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_discovery_DeviceQueryResponse_default_instance()) };
    DeviceQueryResponseView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> DeviceQueryResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeviceQueryResponse>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> DeviceQueryResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // device: optional message nearclip.discovery.DeviceBroadcast
  pub fn has_device(self) -> bool {
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_has(self.raw_msg())
    }
  }
  pub fn device_opt(self) -> ::protobuf::Optional<super::DeviceBroadcastView<'msg>> {
        ::protobuf::Optional::new(self.device(), self.has_device())
  }
  pub fn device(self) -> super::DeviceBroadcastView<'msg> {
    let submsg = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_get(self.raw_msg()) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(submsg) };
    super::DeviceBroadcastView::new(::protobuf::__internal::Private, inner)
  }

  // is_online: optional bool
  pub fn is_online(self) -> bool {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_is_online_get(self.raw_msg()) }
  }

  // last_seen: optional uint64
  pub fn last_seen(self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_last_seen_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `DeviceQueryResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for DeviceQueryResponseView<'_> {}

// SAFETY:
// - `DeviceQueryResponseView` is `Send` because while its alive a `DeviceQueryResponseMut` cannot.
// - `DeviceQueryResponseView` does not use thread-local data.
unsafe impl Send for DeviceQueryResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for DeviceQueryResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for DeviceQueryResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for DeviceQueryResponseView<'msg> {
  type Proxied = DeviceQueryResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, DeviceQueryResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeviceQueryResponseView<'msg> {
  fn into_view<'shorter>(self) -> DeviceQueryResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DeviceQueryResponse> for DeviceQueryResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeviceQueryResponse {
    let dst = DeviceQueryResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DeviceQueryResponse> for DeviceQueryResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeviceQueryResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for DeviceQueryResponse {
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
      let prototype = <DeviceQueryResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for DeviceQueryResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<DeviceQueryResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> DeviceQueryResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { DeviceQueryResponseView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> DeviceQueryResponseMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        DeviceQueryResponseMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DeviceQueryResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeviceQueryResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeviceQueryResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DeviceQueryResponseMut<'msg> {
  type Message = DeviceQueryResponse;
}

impl ::std::fmt::Debug for DeviceQueryResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for DeviceQueryResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> DeviceQueryResponseMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeviceQueryResponse>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DeviceQueryResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> DeviceQueryResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // device: optional message nearclip.discovery.DeviceBroadcast
  pub fn has_device(&self) -> bool {
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_has(self.raw_msg())
    }
  }
  pub fn clear_device(&mut self) {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_clear(self.raw_msg()) }
  }
  pub fn device_opt(&self) -> ::protobuf::Optional<super::DeviceBroadcastView<'_>> {
        ::protobuf::Optional::new(self.device(), self.has_device())
  }
  pub fn device(&self) -> super::DeviceBroadcastView<'_> {
    let submsg = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_get(self.raw_msg()) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(submsg) };
    super::DeviceBroadcastView::new(::protobuf::__internal::Private, inner)
  }
  pub fn device_mut(&mut self) -> super::DeviceBroadcastMut<'_> {
     let raw_msg = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_get_mut(self.raw_msg()) };
     super::DeviceBroadcastMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       raw_msg)
  }
  pub fn set_device(&mut self,
    val: impl ::protobuf::IntoProxied<super::DeviceBroadcast>) {

    let mut val = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_set(
        self.inner.raw(),
        ::protobuf::__internal::runtime::CppGetRawMessageMut::get_raw_message_mut(&mut val, ::protobuf::__internal::Private));
    }
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let _ = std::mem::ManuallyDrop::new(val);
  }

  // is_online: optional bool
  pub fn is_online(&self) -> bool {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_is_online_get(self.raw_msg()) }
  }
  pub fn set_is_online(&mut self, val: bool) {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_is_online_set(self.raw_msg(), val) }
  }

  // last_seen: optional uint64
  pub fn last_seen(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_last_seen_get(self.raw_msg()) }
  }
  pub fn set_last_seen(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_last_seen_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `DeviceQueryResponseMut` does not perform any shared mutation.
// - `DeviceQueryResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for DeviceQueryResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for DeviceQueryResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for DeviceQueryResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for DeviceQueryResponseMut<'msg> {
  type Proxied = DeviceQueryResponse;
  fn as_view(&self) -> ::protobuf::View<'_, DeviceQueryResponse> {
    DeviceQueryResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeviceQueryResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DeviceQueryResponse>
  where
      'msg: 'shorter {
    DeviceQueryResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for DeviceQueryResponseMut<'msg> {
  type MutProxied = DeviceQueryResponse;
  fn as_mut(&mut self) -> DeviceQueryResponseMut<'msg> {
    DeviceQueryResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DeviceQueryResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> DeviceQueryResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DeviceQueryResponse {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_discovery_DeviceQueryResponse_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DeviceQueryResponse> {
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

  pub fn as_view(&self) -> DeviceQueryResponseView {
    DeviceQueryResponseView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> DeviceQueryResponseMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    DeviceQueryResponseMut::new(::protobuf::__internal::Private, inner)
  }

  // device: optional message nearclip.discovery.DeviceBroadcast
  pub fn has_device(&self) -> bool {
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_has(self.raw_msg())
    }
  }
  pub fn clear_device(&mut self) {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_clear(self.raw_msg()) }
  }
  pub fn device_opt(&self) -> ::protobuf::Optional<super::DeviceBroadcastView<'_>> {
        ::protobuf::Optional::new(self.device(), self.has_device())
  }
  pub fn device(&self) -> super::DeviceBroadcastView<'_> {
    let submsg = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_get(self.raw_msg()) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(submsg) };
    super::DeviceBroadcastView::new(::protobuf::__internal::Private, inner)
  }
  pub fn device_mut(&mut self) -> super::DeviceBroadcastMut<'_> {
     let raw_msg = unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_get_mut(self.raw_msg()) };
     super::DeviceBroadcastMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       raw_msg)
  }
  pub fn set_device(&mut self,
    val: impl ::protobuf::IntoProxied<super::DeviceBroadcast>) {

    let mut val = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_set(
        self.inner.raw(),
        ::protobuf::__internal::runtime::CppGetRawMessageMut::get_raw_message_mut(&mut val, ::protobuf::__internal::Private));
    }
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let _ = std::mem::ManuallyDrop::new(val);
  }

  // is_online: optional bool
  pub fn is_online(&self) -> bool {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_is_online_get(self.raw_msg()) }
  }
  pub fn set_is_online(&mut self, val: bool) {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_is_online_set(self.raw_msg(), val) }
  }

  // last_seen: optional uint64
  pub fn last_seen(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_last_seen_get(self.raw_msg()) }
  }
  pub fn set_last_seen(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_last_seen_set(self.raw_msg(), val) }
  }

}  // impl DeviceQueryResponse

impl ::std::ops::Drop for DeviceQueryResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for DeviceQueryResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DeviceQueryResponse {
  type Proxied = Self;
  fn as_view(&self) -> DeviceQueryResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DeviceQueryResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DeviceQueryResponseMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for DeviceQueryResponseMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for DeviceQueryResponseView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_discovery_DeviceQueryResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_discovery_DeviceQueryResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_has(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> bool;
  fn proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_clear(raw_msg: ::protobuf::__internal::runtime::RawMessage);
  fn proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_get_mut(raw_msg: ::protobuf::__internal::runtime::RawMessage)
     -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_device_set(raw_msg: ::protobuf::__internal::runtime::RawMessage,
                    field_msg: ::protobuf::__internal::runtime::RawMessage);

  fn proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_is_online_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> bool;
  fn proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_is_online_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: bool);

  fn proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_last_seen_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u64;
  fn proto2_rust_thunk_nearclip_discovery_DeviceQueryResponse_last_seen_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u64);

}

impl<'a> DeviceQueryResponseMut<'a> {
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

impl<'a> DeviceQueryResponseView<'a> {
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

impl ::protobuf::OwnedMessageInterop for DeviceQueryResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<DeviceQueryResponse>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for DeviceQueryResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for DeviceQueryResponseView<'a> {
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
pub struct DeviceCapability(i32);

#[allow(non_upper_case_globals)]
impl DeviceCapability {
  pub const CapabilityUnknown: DeviceCapability = DeviceCapability(0);
  pub const CapabilityClipboardRead: DeviceCapability = DeviceCapability(1);
  pub const CapabilityClipboardWrite: DeviceCapability = DeviceCapability(2);
  pub const CapabilityFileTransfer: DeviceCapability = DeviceCapability(3);
  pub const CapabilityEncryption: DeviceCapability = DeviceCapability(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "CapabilityUnknown",
      1 => "CapabilityClipboardRead",
      2 => "CapabilityClipboardWrite",
      3 => "CapabilityFileTransfer",
      4 => "CapabilityEncryption",
      _ => return None
    })
  }
}

impl ::std::convert::From<DeviceCapability> for i32 {
  fn from(val: DeviceCapability) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for DeviceCapability {
  fn from(val: i32) -> DeviceCapability {
    Self(val)
  }
}

impl ::std::default::Default for DeviceCapability {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for DeviceCapability {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "DeviceCapability::{}", constant_name)
    } else {
      write!(f, "DeviceCapability::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for DeviceCapability {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for DeviceCapability {}

impl ::protobuf::Proxied for DeviceCapability {
  type View<'a> = DeviceCapability;
}

impl ::protobuf::Proxy<'_> for DeviceCapability {}
impl ::protobuf::ViewProxy<'_> for DeviceCapability {}

impl ::protobuf::AsView for DeviceCapability {
  type Proxied = DeviceCapability;

  fn as_view(&self) -> DeviceCapability {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeviceCapability {
  fn into_view<'shorter>(self) -> DeviceCapability where 'msg: 'shorter {
    self
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for DeviceCapability {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    ::protobuf::__internal::runtime::new_enum_repeated()
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    ::protobuf::__internal::runtime::free_enum_repeated(f)
  }

  fn repeated_len(r: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    ::protobuf::__internal::runtime::cast_enum_repeated_view(r).len()
  }

  fn repeated_push(r: ::protobuf::Mut<::protobuf::Repeated<Self>>, val: impl ::protobuf::IntoProxied<DeviceCapability>) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(r).push(val.into_proxied(::protobuf::__internal::Private))
  }

  fn repeated_clear(r: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(r).clear()
  }

  unsafe fn repeated_get_unchecked(
      r: ::protobuf::View<::protobuf::Repeated<Self>>,
      index: usize,
  ) -> ::protobuf::View<DeviceCapability> {
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
      val: impl ::protobuf::IntoProxied<DeviceCapability>,
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
unsafe impl ::protobuf::__internal::Enum for DeviceCapability {
  const NAME: &'static str = "DeviceCapability";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::runtime::CppMapTypeConversions for DeviceCapability {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        Self::to_map_value(Self::default())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_u32(self.0 as u32)
    }

    unsafe fn from_map_value<'a>(value: ::protobuf::__internal::runtime::MapValue) -> ::protobuf::View<'a, Self> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::U32);
        DeviceCapability(unsafe { value.val.u as i32 })
    }
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DeviceType(i32);

#[allow(non_upper_case_globals)]
impl DeviceType {
  pub const Unknown: DeviceType = DeviceType(0);
  pub const Android: DeviceType = DeviceType(1);
  pub const Mac: DeviceType = DeviceType(2);
  pub const Windows: DeviceType = DeviceType(3);
  pub const Ios: DeviceType = DeviceType(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unknown",
      1 => "Android",
      2 => "Mac",
      3 => "Windows",
      4 => "Ios",
      _ => return None
    })
  }
}

impl ::std::convert::From<DeviceType> for i32 {
  fn from(val: DeviceType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for DeviceType {
  fn from(val: i32) -> DeviceType {
    Self(val)
  }
}

impl ::std::default::Default for DeviceType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for DeviceType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "DeviceType::{}", constant_name)
    } else {
      write!(f, "DeviceType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for DeviceType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for DeviceType {}

impl ::protobuf::Proxied for DeviceType {
  type View<'a> = DeviceType;
}

impl ::protobuf::Proxy<'_> for DeviceType {}
impl ::protobuf::ViewProxy<'_> for DeviceType {}

impl ::protobuf::AsView for DeviceType {
  type Proxied = DeviceType;

  fn as_view(&self) -> DeviceType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeviceType {
  fn into_view<'shorter>(self) -> DeviceType where 'msg: 'shorter {
    self
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for DeviceType {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    ::protobuf::__internal::runtime::new_enum_repeated()
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    ::protobuf::__internal::runtime::free_enum_repeated(f)
  }

  fn repeated_len(r: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    ::protobuf::__internal::runtime::cast_enum_repeated_view(r).len()
  }

  fn repeated_push(r: ::protobuf::Mut<::protobuf::Repeated<Self>>, val: impl ::protobuf::IntoProxied<DeviceType>) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(r).push(val.into_proxied(::protobuf::__internal::Private))
  }

  fn repeated_clear(r: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(r).clear()
  }

  unsafe fn repeated_get_unchecked(
      r: ::protobuf::View<::protobuf::Repeated<Self>>,
      index: usize,
  ) -> ::protobuf::View<DeviceType> {
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
      val: impl ::protobuf::IntoProxied<DeviceType>,
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
unsafe impl ::protobuf::__internal::Enum for DeviceType {
  const NAME: &'static str = "DeviceType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::runtime::CppMapTypeConversions for DeviceType {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        Self::to_map_value(Self::default())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_u32(self.0 as u32)
    }

    unsafe fn from_map_value<'a>(value: ::protobuf::__internal::runtime::MapValue) -> ::protobuf::View<'a, Self> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::U32);
        DeviceType(unsafe { value.val.u as i32 })
    }
}


