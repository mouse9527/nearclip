const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.32.1-release");
#[allow(non_camel_case_types)]
pub struct ClipboardData {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ClipboardData>
}

impl ::protobuf::Message for ClipboardData {}

impl ::std::default::Default for ClipboardData {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for ClipboardData {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for ClipboardData {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ClipboardData {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `ClipboardData` is `Sync` because it does not implement interior mutability.
//    Neither does `ClipboardDataMut`.
unsafe impl Sync for ClipboardData {}

// SAFETY:
// - `ClipboardData` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ClipboardData {}

impl ::protobuf::Proxied for ClipboardData {
  type View<'msg> = ClipboardDataView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ClipboardData {}

impl ::protobuf::MutProxied for ClipboardData {
  type Mut<'msg> = ClipboardDataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ClipboardDataView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClipboardData>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClipboardDataView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ClipboardDataView<'msg> {
  type Message = ClipboardData;
}

impl ::std::fmt::Debug for ClipboardDataView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ClipboardDataView<'_> {
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

impl ::std::default::Default for ClipboardDataView<'_> {
  fn default() -> ClipboardDataView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_sync_ClipboardData_default_instance()) };
    ClipboardDataView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> ClipboardDataView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClipboardData>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> ClipboardData {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // data_id: optional string
  pub fn data_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_data_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // type: optional enum nearclip.sync.DataType
  pub fn r#type(self) -> super::DataType {
    unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_type_get(self.raw_msg()) }
  }

  // content: optional bytes
  pub fn content(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_content_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }

  // metadata: repeated message nearclip.sync.ClipboardData.MetadataEntry
  pub fn metadata(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::MapView::from_raw(::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_sync_ClipboardData_metadata_get(self.raw_msg()))
    }
  }

  // created_at: optional uint64
  pub fn created_at(self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_created_at_get(self.raw_msg()) }
  }

  // expires_at: optional uint64
  pub fn expires_at(self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_expires_at_get(self.raw_msg()) }
  }

  // source_app: optional string
  pub fn source_app(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_source_app_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ClipboardDataView` is `Sync` because it does not support mutation.
unsafe impl Sync for ClipboardDataView<'_> {}

// SAFETY:
// - `ClipboardDataView` is `Send` because while its alive a `ClipboardDataMut` cannot.
// - `ClipboardDataView` does not use thread-local data.
unsafe impl Send for ClipboardDataView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ClipboardDataView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for ClipboardDataView<'msg> {}

impl<'msg> ::protobuf::AsView for ClipboardDataView<'msg> {
  type Proxied = ClipboardData;
  fn as_view(&self) -> ::protobuf::View<'msg, ClipboardData> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClipboardDataView<'msg> {
  fn into_view<'shorter>(self) -> ClipboardDataView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ClipboardData> for ClipboardDataView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClipboardData {
    let dst = ClipboardData::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ClipboardData> for ClipboardDataMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClipboardData {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for ClipboardData {
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
      let prototype = <ClipboardDataView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for ClipboardData {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<ClipboardDataView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> ClipboardDataView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { ClipboardDataView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> ClipboardDataMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        ClipboardDataMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ClipboardDataMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClipboardData>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClipboardDataMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ClipboardDataMut<'msg> {
  type Message = ClipboardData;
}

impl ::std::fmt::Debug for ClipboardDataMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for ClipboardDataMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> ClipboardDataMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClipboardData>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ClipboardData> {
    self.inner
  }

  pub fn to_owned(&self) -> ClipboardData {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // data_id: optional string
  pub fn data_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_data_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_data_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_ClipboardData_data_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // type: optional enum nearclip.sync.DataType
  pub fn r#type(&self) -> super::DataType {
    unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_type_get(self.raw_msg()) }
  }
  pub fn set_type(&mut self, val: super::DataType) {
    unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_type_set(self.raw_msg(), val) }
  }

  // content: optional bytes
  pub fn content(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_content_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_content(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_ClipboardData_content_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // metadata: repeated message nearclip.sync.ClipboardData.MetadataEntry
  pub fn metadata(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::MapView::from_raw(::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_sync_ClipboardData_metadata_get(self.raw_msg()))
    }
  }
  pub fn metadata_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
      unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_metadata_get_mut(self.raw_msg()) });
    unsafe { ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner) }
  }
  pub fn set_metadata(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    let val = ::std::mem::ManuallyDrop::new(
        src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_sync_ClipboardData_metadata_set(
          self.raw_msg(),
          val.as_raw(::protobuf::__internal::Private));
    }
  }

  // created_at: optional uint64
  pub fn created_at(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_created_at_get(self.raw_msg()) }
  }
  pub fn set_created_at(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_created_at_set(self.raw_msg(), val) }
  }

  // expires_at: optional uint64
  pub fn expires_at(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_expires_at_get(self.raw_msg()) }
  }
  pub fn set_expires_at(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_expires_at_set(self.raw_msg(), val) }
  }

  // source_app: optional string
  pub fn source_app(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_source_app_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_source_app(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_ClipboardData_source_app_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `ClipboardDataMut` does not perform any shared mutation.
// - `ClipboardDataMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ClipboardDataMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ClipboardDataMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for ClipboardDataMut<'msg> {}

impl<'msg> ::protobuf::AsView for ClipboardDataMut<'msg> {
  type Proxied = ClipboardData;
  fn as_view(&self) -> ::protobuf::View<'_, ClipboardData> {
    ClipboardDataView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClipboardDataMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ClipboardData>
  where
      'msg: 'shorter {
    ClipboardDataView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for ClipboardDataMut<'msg> {
  type MutProxied = ClipboardData;
  fn as_mut(&mut self) -> ClipboardDataMut<'msg> {
    ClipboardDataMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ClipboardDataMut<'msg> {
  fn into_mut<'shorter>(self) -> ClipboardDataMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ClipboardData {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_sync_ClipboardData_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ClipboardData> {
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

  pub fn as_view(&self) -> ClipboardDataView {
    ClipboardDataView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> ClipboardDataMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    ClipboardDataMut::new(::protobuf::__internal::Private, inner)
  }

  // data_id: optional string
  pub fn data_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_data_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_data_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_ClipboardData_data_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // type: optional enum nearclip.sync.DataType
  pub fn r#type(&self) -> super::DataType {
    unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_type_get(self.raw_msg()) }
  }
  pub fn set_type(&mut self, val: super::DataType) {
    unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_type_set(self.raw_msg(), val) }
  }

  // content: optional bytes
  pub fn content(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_content_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_content(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_ClipboardData_content_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // metadata: repeated message nearclip.sync.ClipboardData.MetadataEntry
  pub fn metadata(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::MapView::from_raw(::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_sync_ClipboardData_metadata_get(self.raw_msg()))
    }
  }
  pub fn metadata_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
      unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_metadata_get_mut(self.raw_msg()) });
    unsafe { ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner) }
  }
  pub fn set_metadata(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    let val = ::std::mem::ManuallyDrop::new(
        src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_sync_ClipboardData_metadata_set(
          self.raw_msg(),
          val.as_raw(::protobuf::__internal::Private));
    }
  }

  // created_at: optional uint64
  pub fn created_at(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_created_at_get(self.raw_msg()) }
  }
  pub fn set_created_at(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_created_at_set(self.raw_msg(), val) }
  }

  // expires_at: optional uint64
  pub fn expires_at(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_expires_at_get(self.raw_msg()) }
  }
  pub fn set_expires_at(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_expires_at_set(self.raw_msg(), val) }
  }

  // source_app: optional string
  pub fn source_app(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_ClipboardData_source_app_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_source_app(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_ClipboardData_source_app_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl ClipboardData

impl ::std::ops::Drop for ClipboardData {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for ClipboardData {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ClipboardData {
  type Proxied = Self;
  fn as_view(&self) -> ClipboardDataView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ClipboardData {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ClipboardDataMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for ClipboardDataMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for ClipboardDataView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

pub mod clipboard_data {

}  // pub mod clipboard_data
extern "C" {
  fn proto2_rust_thunk_Message_nearclip_sync_ClipboardData_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_sync_ClipboardData_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_sync_ClipboardData_data_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_sync_ClipboardData_data_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_sync_ClipboardData_type_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> super::DataType;
  fn proto2_rust_thunk_nearclip_sync_ClipboardData_type_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: super::DataType);

  fn proto2_rust_thunk_nearclip_sync_ClipboardData_content_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_sync_ClipboardData_content_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_sync_ClipboardData_metadata_get(msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawMap;
  fn proto2_rust_thunk_nearclip_sync_ClipboardData_metadata_get_mut(msg: ::protobuf::__internal::runtime::RawMessage,) -> ::protobuf::__internal::runtime::RawMap;
  fn proto2_rust_thunk_nearclip_sync_ClipboardData_metadata_set(
      raw_msg: ::protobuf::__internal::runtime::RawMessage,
      value: ::protobuf::__internal::runtime::RawMap);

  fn proto2_rust_thunk_nearclip_sync_ClipboardData_created_at_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u64;
  fn proto2_rust_thunk_nearclip_sync_ClipboardData_created_at_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u64);

  fn proto2_rust_thunk_nearclip_sync_ClipboardData_expires_at_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u64;
  fn proto2_rust_thunk_nearclip_sync_ClipboardData_expires_at_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u64);

  fn proto2_rust_thunk_nearclip_sync_ClipboardData_source_app_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_sync_ClipboardData_source_app_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> ClipboardDataMut<'a> {
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

impl<'a> ClipboardDataView<'a> {
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

impl ::protobuf::OwnedMessageInterop for ClipboardData {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<ClipboardData>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for ClipboardDataMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for ClipboardDataView<'a> {
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
pub struct DataChunk {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DataChunk>
}

impl ::protobuf::Message for DataChunk {}

impl ::std::default::Default for DataChunk {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for DataChunk {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for DataChunk {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for DataChunk {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `DataChunk` is `Sync` because it does not implement interior mutability.
//    Neither does `DataChunkMut`.
unsafe impl Sync for DataChunk {}

// SAFETY:
// - `DataChunk` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DataChunk {}

impl ::protobuf::Proxied for DataChunk {
  type View<'msg> = DataChunkView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DataChunk {}

impl ::protobuf::MutProxied for DataChunk {
  type Mut<'msg> = DataChunkMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DataChunkView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DataChunk>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DataChunkView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DataChunkView<'msg> {
  type Message = DataChunk;
}

impl ::std::fmt::Debug for DataChunkView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for DataChunkView<'_> {
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

impl ::std::default::Default for DataChunkView<'_> {
  fn default() -> DataChunkView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_sync_DataChunk_default_instance()) };
    DataChunkView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> DataChunkView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DataChunk>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> DataChunk {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // data_id: optional string
  pub fn data_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_data_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // chunk_index: optional uint32
  pub fn chunk_index(self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_chunk_index_get(self.raw_msg()) }
  }

  // total_chunks: optional uint32
  pub fn total_chunks(self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_total_chunks_get(self.raw_msg()) }
  }

  // chunk_data: optional bytes
  pub fn chunk_data(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_chunk_data_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }

  // checksum: optional bytes
  pub fn checksum(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_checksum_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `DataChunkView` is `Sync` because it does not support mutation.
unsafe impl Sync for DataChunkView<'_> {}

// SAFETY:
// - `DataChunkView` is `Send` because while its alive a `DataChunkMut` cannot.
// - `DataChunkView` does not use thread-local data.
unsafe impl Send for DataChunkView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for DataChunkView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for DataChunkView<'msg> {}

impl<'msg> ::protobuf::AsView for DataChunkView<'msg> {
  type Proxied = DataChunk;
  fn as_view(&self) -> ::protobuf::View<'msg, DataChunk> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DataChunkView<'msg> {
  fn into_view<'shorter>(self) -> DataChunkView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DataChunk> for DataChunkView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DataChunk {
    let dst = DataChunk::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DataChunk> for DataChunkMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DataChunk {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for DataChunk {
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
      let prototype = <DataChunkView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for DataChunk {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<DataChunkView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> DataChunkView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { DataChunkView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> DataChunkMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        DataChunkMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DataChunkMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DataChunk>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DataChunkMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DataChunkMut<'msg> {
  type Message = DataChunk;
}

impl ::std::fmt::Debug for DataChunkMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for DataChunkMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> DataChunkMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DataChunk>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DataChunk> {
    self.inner
  }

  pub fn to_owned(&self) -> DataChunk {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // data_id: optional string
  pub fn data_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_data_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_data_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_DataChunk_data_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // chunk_index: optional uint32
  pub fn chunk_index(&self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_chunk_index_get(self.raw_msg()) }
  }
  pub fn set_chunk_index(&mut self, val: u32) {
    unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_chunk_index_set(self.raw_msg(), val) }
  }

  // total_chunks: optional uint32
  pub fn total_chunks(&self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_total_chunks_get(self.raw_msg()) }
  }
  pub fn set_total_chunks(&mut self, val: u32) {
    unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_total_chunks_set(self.raw_msg(), val) }
  }

  // chunk_data: optional bytes
  pub fn chunk_data(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_chunk_data_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_chunk_data(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_DataChunk_chunk_data_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // checksum: optional bytes
  pub fn checksum(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_checksum_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_checksum(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_DataChunk_checksum_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `DataChunkMut` does not perform any shared mutation.
// - `DataChunkMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for DataChunkMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for DataChunkMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for DataChunkMut<'msg> {}

impl<'msg> ::protobuf::AsView for DataChunkMut<'msg> {
  type Proxied = DataChunk;
  fn as_view(&self) -> ::protobuf::View<'_, DataChunk> {
    DataChunkView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DataChunkMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DataChunk>
  where
      'msg: 'shorter {
    DataChunkView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for DataChunkMut<'msg> {
  type MutProxied = DataChunk;
  fn as_mut(&mut self) -> DataChunkMut<'msg> {
    DataChunkMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DataChunkMut<'msg> {
  fn into_mut<'shorter>(self) -> DataChunkMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DataChunk {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_sync_DataChunk_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DataChunk> {
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

  pub fn as_view(&self) -> DataChunkView {
    DataChunkView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> DataChunkMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    DataChunkMut::new(::protobuf::__internal::Private, inner)
  }

  // data_id: optional string
  pub fn data_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_data_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_data_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_DataChunk_data_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // chunk_index: optional uint32
  pub fn chunk_index(&self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_chunk_index_get(self.raw_msg()) }
  }
  pub fn set_chunk_index(&mut self, val: u32) {
    unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_chunk_index_set(self.raw_msg(), val) }
  }

  // total_chunks: optional uint32
  pub fn total_chunks(&self) -> u32 {
    unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_total_chunks_get(self.raw_msg()) }
  }
  pub fn set_total_chunks(&mut self, val: u32) {
    unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_total_chunks_set(self.raw_msg(), val) }
  }

  // chunk_data: optional bytes
  pub fn chunk_data(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_chunk_data_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_chunk_data(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_DataChunk_chunk_data_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // checksum: optional bytes
  pub fn checksum(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_DataChunk_checksum_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_checksum(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_DataChunk_checksum_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl DataChunk

impl ::std::ops::Drop for DataChunk {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for DataChunk {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DataChunk {
  type Proxied = Self;
  fn as_view(&self) -> DataChunkView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DataChunk {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DataChunkMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for DataChunkMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for DataChunkView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_sync_DataChunk_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_sync_DataChunk_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_sync_DataChunk_data_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_sync_DataChunk_data_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_sync_DataChunk_chunk_index_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u32;
  fn proto2_rust_thunk_nearclip_sync_DataChunk_chunk_index_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u32);

  fn proto2_rust_thunk_nearclip_sync_DataChunk_total_chunks_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u32;
  fn proto2_rust_thunk_nearclip_sync_DataChunk_total_chunks_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u32);

  fn proto2_rust_thunk_nearclip_sync_DataChunk_chunk_data_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_sync_DataChunk_chunk_data_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_sync_DataChunk_checksum_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_sync_DataChunk_checksum_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> DataChunkMut<'a> {
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

impl<'a> DataChunkView<'a> {
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

impl ::protobuf::OwnedMessageInterop for DataChunk {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<DataChunk>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for DataChunkMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for DataChunkView<'a> {
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
pub struct SyncMessage {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SyncMessage>
}

impl ::protobuf::Message for SyncMessage {}

impl ::std::default::Default for SyncMessage {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for SyncMessage {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for SyncMessage {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SyncMessage {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `SyncMessage` is `Sync` because it does not implement interior mutability.
//    Neither does `SyncMessageMut`.
unsafe impl Sync for SyncMessage {}

// SAFETY:
// - `SyncMessage` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SyncMessage {}

impl ::protobuf::Proxied for SyncMessage {
  type View<'msg> = SyncMessageView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SyncMessage {}

impl ::protobuf::MutProxied for SyncMessage {
  type Mut<'msg> = SyncMessageMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SyncMessageView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SyncMessage>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SyncMessageView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SyncMessageView<'msg> {
  type Message = SyncMessage;
}

impl ::std::fmt::Debug for SyncMessageView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SyncMessageView<'_> {
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

impl ::std::default::Default for SyncMessageView<'_> {
  fn default() -> SyncMessageView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_sync_SyncMessage_default_instance()) };
    SyncMessageView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> SyncMessageView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SyncMessage>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> SyncMessage {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // device_id: optional string
  pub fn device_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // operation: optional enum nearclip.sync.SyncOperation
  pub fn operation(self) -> super::SyncOperation {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_operation_get(self.raw_msg()) }
  }

  // data: optional message nearclip.sync.ClipboardData
  pub fn has_data(self) -> bool {
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncMessage_data_has(self.raw_msg())
    }
  }
  pub fn data_opt(self) -> ::protobuf::Optional<super::ClipboardDataView<'msg>> {
        ::protobuf::Optional::new(self.data(), self.has_data())
  }
  pub fn data(self) -> super::ClipboardDataView<'msg> {
    let submsg = unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_data_get(self.raw_msg()) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(submsg) };
    super::ClipboardDataView::new(::protobuf::__internal::Private, inner)
  }

  // chunks: repeated message nearclip.sync.DataChunk
  pub fn chunks(self) -> ::protobuf::RepeatedView<'msg, super::DataChunk> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_sync_SyncMessage_chunks_get(self.raw_msg()),
      )
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_timestamp_get(self.raw_msg()) }
  }

  // signature: optional bytes
  pub fn signature(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_signature_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `SyncMessageView` is `Sync` because it does not support mutation.
unsafe impl Sync for SyncMessageView<'_> {}

// SAFETY:
// - `SyncMessageView` is `Send` because while its alive a `SyncMessageMut` cannot.
// - `SyncMessageView` does not use thread-local data.
unsafe impl Send for SyncMessageView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SyncMessageView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for SyncMessageView<'msg> {}

impl<'msg> ::protobuf::AsView for SyncMessageView<'msg> {
  type Proxied = SyncMessage;
  fn as_view(&self) -> ::protobuf::View<'msg, SyncMessage> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SyncMessageView<'msg> {
  fn into_view<'shorter>(self) -> SyncMessageView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SyncMessage> for SyncMessageView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SyncMessage {
    let dst = SyncMessage::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SyncMessage> for SyncMessageMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SyncMessage {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for SyncMessage {
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
      let prototype = <SyncMessageView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for SyncMessage {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<SyncMessageView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> SyncMessageView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { SyncMessageView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> SyncMessageMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        SyncMessageMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SyncMessageMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SyncMessage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SyncMessageMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SyncMessageMut<'msg> {
  type Message = SyncMessage;
}

impl ::std::fmt::Debug for SyncMessageMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SyncMessageMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> SyncMessageMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SyncMessage>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SyncMessage> {
    self.inner
  }

  pub fn to_owned(&self) -> SyncMessage {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncMessage_device_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // operation: optional enum nearclip.sync.SyncOperation
  pub fn operation(&self) -> super::SyncOperation {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_operation_get(self.raw_msg()) }
  }
  pub fn set_operation(&mut self, val: super::SyncOperation) {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_operation_set(self.raw_msg(), val) }
  }

  // data: optional message nearclip.sync.ClipboardData
  pub fn has_data(&self) -> bool {
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncMessage_data_has(self.raw_msg())
    }
  }
  pub fn clear_data(&mut self) {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_data_clear(self.raw_msg()) }
  }
  pub fn data_opt(&self) -> ::protobuf::Optional<super::ClipboardDataView<'_>> {
        ::protobuf::Optional::new(self.data(), self.has_data())
  }
  pub fn data(&self) -> super::ClipboardDataView<'_> {
    let submsg = unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_data_get(self.raw_msg()) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(submsg) };
    super::ClipboardDataView::new(::protobuf::__internal::Private, inner)
  }
  pub fn data_mut(&mut self) -> super::ClipboardDataMut<'_> {
     let raw_msg = unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_data_get_mut(self.raw_msg()) };
     super::ClipboardDataMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       raw_msg)
  }
  pub fn set_data(&mut self,
    val: impl ::protobuf::IntoProxied<super::ClipboardData>) {

    let mut val = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncMessage_data_set(
        self.inner.raw(),
        ::protobuf::__internal::runtime::CppGetRawMessageMut::get_raw_message_mut(&mut val, ::protobuf::__internal::Private));
    }
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let _ = std::mem::ManuallyDrop::new(val);
  }

  // chunks: repeated message nearclip.sync.DataChunk
  pub fn chunks(&self) -> ::protobuf::RepeatedView<'_, super::DataChunk> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_sync_SyncMessage_chunks_get(self.raw_msg()),
      )
    }
  }
  pub fn chunks_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::DataChunk> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_sync_SyncMessage_chunks_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_chunks(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::DataChunk>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncMessage_chunks_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_timestamp_set(self.raw_msg(), val) }
  }

  // signature: optional bytes
  pub fn signature(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_signature_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_signature(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncMessage_signature_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}

// SAFETY:
// - `SyncMessageMut` does not perform any shared mutation.
// - `SyncMessageMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for SyncMessageMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SyncMessageMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for SyncMessageMut<'msg> {}

impl<'msg> ::protobuf::AsView for SyncMessageMut<'msg> {
  type Proxied = SyncMessage;
  fn as_view(&self) -> ::protobuf::View<'_, SyncMessage> {
    SyncMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SyncMessageMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SyncMessage>
  where
      'msg: 'shorter {
    SyncMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for SyncMessageMut<'msg> {
  type MutProxied = SyncMessage;
  fn as_mut(&mut self) -> SyncMessageMut<'msg> {
    SyncMessageMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SyncMessageMut<'msg> {
  fn into_mut<'shorter>(self) -> SyncMessageMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SyncMessage {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_sync_SyncMessage_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SyncMessage> {
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

  pub fn as_view(&self) -> SyncMessageView {
    SyncMessageView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> SyncMessageMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    SyncMessageMut::new(::protobuf::__internal::Private, inner)
  }

  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncMessage_device_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // operation: optional enum nearclip.sync.SyncOperation
  pub fn operation(&self) -> super::SyncOperation {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_operation_get(self.raw_msg()) }
  }
  pub fn set_operation(&mut self, val: super::SyncOperation) {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_operation_set(self.raw_msg(), val) }
  }

  // data: optional message nearclip.sync.ClipboardData
  pub fn has_data(&self) -> bool {
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncMessage_data_has(self.raw_msg())
    }
  }
  pub fn clear_data(&mut self) {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_data_clear(self.raw_msg()) }
  }
  pub fn data_opt(&self) -> ::protobuf::Optional<super::ClipboardDataView<'_>> {
        ::protobuf::Optional::new(self.data(), self.has_data())
  }
  pub fn data(&self) -> super::ClipboardDataView<'_> {
    let submsg = unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_data_get(self.raw_msg()) };
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(submsg) };
    super::ClipboardDataView::new(::protobuf::__internal::Private, inner)
  }
  pub fn data_mut(&mut self) -> super::ClipboardDataMut<'_> {
     let raw_msg = unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_data_get_mut(self.raw_msg()) };
     super::ClipboardDataMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       raw_msg)
  }
  pub fn set_data(&mut self,
    val: impl ::protobuf::IntoProxied<super::ClipboardData>) {

    let mut val = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncMessage_data_set(
        self.inner.raw(),
        ::protobuf::__internal::runtime::CppGetRawMessageMut::get_raw_message_mut(&mut val, ::protobuf::__internal::Private));
    }
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let _ = std::mem::ManuallyDrop::new(val);
  }

  // chunks: repeated message nearclip.sync.DataChunk
  pub fn chunks(&self) -> ::protobuf::RepeatedView<'_, super::DataChunk> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_sync_SyncMessage_chunks_get(self.raw_msg()),
      )
    }
  }
  pub fn chunks_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::DataChunk> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_sync_SyncMessage_chunks_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_chunks(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::DataChunk>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncMessage_chunks_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_timestamp_set(self.raw_msg(), val) }
  }

  // signature: optional bytes
  pub fn signature(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_SyncMessage_signature_get(self.raw_msg()) };
    unsafe { str_view.as_ref() }
  }
  pub fn set_signature(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncMessage_signature_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

}  // impl SyncMessage

impl ::std::ops::Drop for SyncMessage {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for SyncMessage {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SyncMessage {
  type Proxied = Self;
  fn as_view(&self) -> SyncMessageView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SyncMessage {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SyncMessageMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for SyncMessageMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for SyncMessageView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_sync_SyncMessage_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_sync_SyncMessage_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_sync_SyncMessage_device_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_sync_SyncMessage_device_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_sync_SyncMessage_operation_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> super::SyncOperation;
  fn proto2_rust_thunk_nearclip_sync_SyncMessage_operation_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: super::SyncOperation);

  fn proto2_rust_thunk_nearclip_sync_SyncMessage_data_has(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> bool;
  fn proto2_rust_thunk_nearclip_sync_SyncMessage_data_clear(raw_msg: ::protobuf::__internal::runtime::RawMessage);
  fn proto2_rust_thunk_nearclip_sync_SyncMessage_data_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_sync_SyncMessage_data_get_mut(raw_msg: ::protobuf::__internal::runtime::RawMessage)
     -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_sync_SyncMessage_data_set(raw_msg: ::protobuf::__internal::runtime::RawMessage,
                    field_msg: ::protobuf::__internal::runtime::RawMessage);

  fn proto2_rust_thunk_nearclip_sync_SyncMessage_chunks_get_mut(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_sync_SyncMessage_chunks_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_sync_SyncMessage_chunks_move_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, value: ::protobuf::__internal::runtime::RawRepeatedField);

  fn proto2_rust_thunk_nearclip_sync_SyncMessage_timestamp_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u64;
  fn proto2_rust_thunk_nearclip_sync_SyncMessage_timestamp_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u64);

  fn proto2_rust_thunk_nearclip_sync_SyncMessage_signature_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_sync_SyncMessage_signature_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

}

impl<'a> SyncMessageMut<'a> {
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

impl<'a> SyncMessageView<'a> {
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

impl ::protobuf::OwnedMessageInterop for SyncMessage {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<SyncMessage>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for SyncMessageMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for SyncMessageView<'a> {
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
pub struct SyncAck {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SyncAck>
}

impl ::protobuf::Message for SyncAck {}

impl ::std::default::Default for SyncAck {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for SyncAck {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for SyncAck {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SyncAck {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `SyncAck` is `Sync` because it does not implement interior mutability.
//    Neither does `SyncAckMut`.
unsafe impl Sync for SyncAck {}

// SAFETY:
// - `SyncAck` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SyncAck {}

impl ::protobuf::Proxied for SyncAck {
  type View<'msg> = SyncAckView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SyncAck {}

impl ::protobuf::MutProxied for SyncAck {
  type Mut<'msg> = SyncAckMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SyncAckView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SyncAck>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SyncAckView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SyncAckView<'msg> {
  type Message = SyncAck;
}

impl ::std::fmt::Debug for SyncAckView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SyncAckView<'_> {
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

impl ::std::default::Default for SyncAckView<'_> {
  fn default() -> SyncAckView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_sync_SyncAck_default_instance()) };
    SyncAckView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> SyncAckView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SyncAck>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> SyncAck {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // data_id: optional string
  pub fn data_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_SyncAck_data_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // success: optional bool
  pub fn success(self) -> bool {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncAck_success_get(self.raw_msg()) }
  }

  // error_message: optional string
  pub fn error_message(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_SyncAck_error_message_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // timestamp: optional uint64
  pub fn timestamp(self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncAck_timestamp_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `SyncAckView` is `Sync` because it does not support mutation.
unsafe impl Sync for SyncAckView<'_> {}

// SAFETY:
// - `SyncAckView` is `Send` because while its alive a `SyncAckMut` cannot.
// - `SyncAckView` does not use thread-local data.
unsafe impl Send for SyncAckView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SyncAckView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for SyncAckView<'msg> {}

impl<'msg> ::protobuf::AsView for SyncAckView<'msg> {
  type Proxied = SyncAck;
  fn as_view(&self) -> ::protobuf::View<'msg, SyncAck> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SyncAckView<'msg> {
  fn into_view<'shorter>(self) -> SyncAckView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SyncAck> for SyncAckView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SyncAck {
    let dst = SyncAck::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SyncAck> for SyncAckMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SyncAck {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for SyncAck {
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
      let prototype = <SyncAckView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for SyncAck {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<SyncAckView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> SyncAckView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { SyncAckView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> SyncAckMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        SyncAckMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SyncAckMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SyncAck>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SyncAckMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SyncAckMut<'msg> {
  type Message = SyncAck;
}

impl ::std::fmt::Debug for SyncAckMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SyncAckMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> SyncAckMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SyncAck>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SyncAck> {
    self.inner
  }

  pub fn to_owned(&self) -> SyncAck {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // data_id: optional string
  pub fn data_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_SyncAck_data_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_data_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncAck_data_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // success: optional bool
  pub fn success(&self) -> bool {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncAck_success_get(self.raw_msg()) }
  }
  pub fn set_success(&mut self, val: bool) {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncAck_success_set(self.raw_msg(), val) }
  }

  // error_message: optional string
  pub fn error_message(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_SyncAck_error_message_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_error_message(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncAck_error_message_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncAck_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncAck_timestamp_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `SyncAckMut` does not perform any shared mutation.
// - `SyncAckMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for SyncAckMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SyncAckMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for SyncAckMut<'msg> {}

impl<'msg> ::protobuf::AsView for SyncAckMut<'msg> {
  type Proxied = SyncAck;
  fn as_view(&self) -> ::protobuf::View<'_, SyncAck> {
    SyncAckView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SyncAckMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SyncAck>
  where
      'msg: 'shorter {
    SyncAckView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for SyncAckMut<'msg> {
  type MutProxied = SyncAck;
  fn as_mut(&mut self) -> SyncAckMut<'msg> {
    SyncAckMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SyncAckMut<'msg> {
  fn into_mut<'shorter>(self) -> SyncAckMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SyncAck {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_sync_SyncAck_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SyncAck> {
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

  pub fn as_view(&self) -> SyncAckView {
    SyncAckView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> SyncAckMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    SyncAckMut::new(::protobuf::__internal::Private, inner)
  }

  // data_id: optional string
  pub fn data_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_SyncAck_data_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_data_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncAck_data_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // success: optional bool
  pub fn success(&self) -> bool {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncAck_success_get(self.raw_msg()) }
  }
  pub fn set_success(&mut self, val: bool) {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncAck_success_set(self.raw_msg(), val) }
  }

  // error_message: optional string
  pub fn error_message(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_SyncAck_error_message_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_error_message(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncAck_error_message_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // timestamp: optional uint64
  pub fn timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncAck_timestamp_get(self.raw_msg()) }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncAck_timestamp_set(self.raw_msg(), val) }
  }

}  // impl SyncAck

impl ::std::ops::Drop for SyncAck {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for SyncAck {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SyncAck {
  type Proxied = Self;
  fn as_view(&self) -> SyncAckView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SyncAck {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SyncAckMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for SyncAckMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for SyncAckView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_sync_SyncAck_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_sync_SyncAck_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_sync_SyncAck_data_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_sync_SyncAck_data_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_sync_SyncAck_success_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> bool;
  fn proto2_rust_thunk_nearclip_sync_SyncAck_success_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: bool);

  fn proto2_rust_thunk_nearclip_sync_SyncAck_error_message_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_sync_SyncAck_error_message_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_sync_SyncAck_timestamp_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u64;
  fn proto2_rust_thunk_nearclip_sync_SyncAck_timestamp_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u64);

}

impl<'a> SyncAckMut<'a> {
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

impl<'a> SyncAckView<'a> {
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

impl ::protobuf::OwnedMessageInterop for SyncAck {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<SyncAck>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for SyncAckMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for SyncAckView<'a> {
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
pub struct SyncStatusQuery {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SyncStatusQuery>
}

impl ::protobuf::Message for SyncStatusQuery {}

impl ::std::default::Default for SyncStatusQuery {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for SyncStatusQuery {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for SyncStatusQuery {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SyncStatusQuery {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `SyncStatusQuery` is `Sync` because it does not implement interior mutability.
//    Neither does `SyncStatusQueryMut`.
unsafe impl Sync for SyncStatusQuery {}

// SAFETY:
// - `SyncStatusQuery` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SyncStatusQuery {}

impl ::protobuf::Proxied for SyncStatusQuery {
  type View<'msg> = SyncStatusQueryView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SyncStatusQuery {}

impl ::protobuf::MutProxied for SyncStatusQuery {
  type Mut<'msg> = SyncStatusQueryMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SyncStatusQueryView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SyncStatusQuery>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SyncStatusQueryView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SyncStatusQueryView<'msg> {
  type Message = SyncStatusQuery;
}

impl ::std::fmt::Debug for SyncStatusQueryView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SyncStatusQueryView<'_> {
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

impl ::std::default::Default for SyncStatusQueryView<'_> {
  fn default() -> SyncStatusQueryView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_sync_SyncStatusQuery_default_instance()) };
    SyncStatusQueryView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> SyncStatusQueryView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SyncStatusQuery>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> SyncStatusQuery {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // device_id: optional string
  pub fn device_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_SyncStatusQuery_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // since_timestamp: optional uint64
  pub fn since_timestamp(self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncStatusQuery_since_timestamp_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `SyncStatusQueryView` is `Sync` because it does not support mutation.
unsafe impl Sync for SyncStatusQueryView<'_> {}

// SAFETY:
// - `SyncStatusQueryView` is `Send` because while its alive a `SyncStatusQueryMut` cannot.
// - `SyncStatusQueryView` does not use thread-local data.
unsafe impl Send for SyncStatusQueryView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SyncStatusQueryView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for SyncStatusQueryView<'msg> {}

impl<'msg> ::protobuf::AsView for SyncStatusQueryView<'msg> {
  type Proxied = SyncStatusQuery;
  fn as_view(&self) -> ::protobuf::View<'msg, SyncStatusQuery> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SyncStatusQueryView<'msg> {
  fn into_view<'shorter>(self) -> SyncStatusQueryView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SyncStatusQuery> for SyncStatusQueryView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SyncStatusQuery {
    let dst = SyncStatusQuery::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SyncStatusQuery> for SyncStatusQueryMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SyncStatusQuery {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for SyncStatusQuery {
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
      let prototype = <SyncStatusQueryView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for SyncStatusQuery {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<SyncStatusQueryView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> SyncStatusQueryView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { SyncStatusQueryView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> SyncStatusQueryMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        SyncStatusQueryMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SyncStatusQueryMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SyncStatusQuery>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SyncStatusQueryMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SyncStatusQueryMut<'msg> {
  type Message = SyncStatusQuery;
}

impl ::std::fmt::Debug for SyncStatusQueryMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SyncStatusQueryMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> SyncStatusQueryMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SyncStatusQuery>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SyncStatusQuery> {
    self.inner
  }

  pub fn to_owned(&self) -> SyncStatusQuery {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_SyncStatusQuery_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncStatusQuery_device_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // since_timestamp: optional uint64
  pub fn since_timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncStatusQuery_since_timestamp_get(self.raw_msg()) }
  }
  pub fn set_since_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncStatusQuery_since_timestamp_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `SyncStatusQueryMut` does not perform any shared mutation.
// - `SyncStatusQueryMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for SyncStatusQueryMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SyncStatusQueryMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for SyncStatusQueryMut<'msg> {}

impl<'msg> ::protobuf::AsView for SyncStatusQueryMut<'msg> {
  type Proxied = SyncStatusQuery;
  fn as_view(&self) -> ::protobuf::View<'_, SyncStatusQuery> {
    SyncStatusQueryView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SyncStatusQueryMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SyncStatusQuery>
  where
      'msg: 'shorter {
    SyncStatusQueryView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for SyncStatusQueryMut<'msg> {
  type MutProxied = SyncStatusQuery;
  fn as_mut(&mut self) -> SyncStatusQueryMut<'msg> {
    SyncStatusQueryMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SyncStatusQueryMut<'msg> {
  fn into_mut<'shorter>(self) -> SyncStatusQueryMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SyncStatusQuery {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_sync_SyncStatusQuery_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SyncStatusQuery> {
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

  pub fn as_view(&self) -> SyncStatusQueryView {
    SyncStatusQueryView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> SyncStatusQueryMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    SyncStatusQueryMut::new(::protobuf::__internal::Private, inner)
  }

  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe { proto2_rust_thunk_nearclip_sync_SyncStatusQuery_device_id_get(self.raw_msg()) };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncStatusQuery_device_id_set(
        self.inner.raw(),
        s.into_inner(::protobuf::__internal::Private).into_raw()
      );
    }
  }

  // since_timestamp: optional uint64
  pub fn since_timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncStatusQuery_since_timestamp_get(self.raw_msg()) }
  }
  pub fn set_since_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncStatusQuery_since_timestamp_set(self.raw_msg(), val) }
  }

}  // impl SyncStatusQuery

impl ::std::ops::Drop for SyncStatusQuery {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for SyncStatusQuery {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SyncStatusQuery {
  type Proxied = Self;
  fn as_view(&self) -> SyncStatusQueryView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SyncStatusQuery {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SyncStatusQueryMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for SyncStatusQueryMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for SyncStatusQueryView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_sync_SyncStatusQuery_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_sync_SyncStatusQuery_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_sync_SyncStatusQuery_device_id_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::PtrAndLen;
  fn proto2_rust_thunk_nearclip_sync_SyncStatusQuery_device_id_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: ::protobuf::__internal::runtime::CppStdString);

  fn proto2_rust_thunk_nearclip_sync_SyncStatusQuery_since_timestamp_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u64;
  fn proto2_rust_thunk_nearclip_sync_SyncStatusQuery_since_timestamp_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u64);

}

impl<'a> SyncStatusQueryMut<'a> {
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

impl<'a> SyncStatusQueryView<'a> {
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

impl ::protobuf::OwnedMessageInterop for SyncStatusQuery {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<SyncStatusQuery>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for SyncStatusQueryMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for SyncStatusQueryView<'a> {
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
pub struct SyncStatusResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SyncStatusResponse>
}

impl ::protobuf::Message for SyncStatusResponse {}

impl ::std::default::Default for SyncStatusResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for SyncStatusResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for SyncStatusResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SyncStatusResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `SyncStatusResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `SyncStatusResponseMut`.
unsafe impl Sync for SyncStatusResponse {}

// SAFETY:
// - `SyncStatusResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SyncStatusResponse {}

impl ::protobuf::Proxied for SyncStatusResponse {
  type View<'msg> = SyncStatusResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SyncStatusResponse {}

impl ::protobuf::MutProxied for SyncStatusResponse {
  type Mut<'msg> = SyncStatusResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SyncStatusResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SyncStatusResponse>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SyncStatusResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SyncStatusResponseView<'msg> {
  type Message = SyncStatusResponse;
}

impl ::std::fmt::Debug for SyncStatusResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SyncStatusResponseView<'_> {
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

impl ::std::default::Default for SyncStatusResponseView<'_> {
  fn default() -> SyncStatusResponseView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(proto2_rust_thunk_Message_nearclip_sync_SyncStatusResponse_default_instance()) };
    SyncStatusResponseView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> SyncStatusResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SyncStatusResponse>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> SyncStatusResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // pending_data_ids: repeated string
  pub fn pending_data_ids(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_sync_SyncStatusResponse_pending_data_ids_get(self.raw_msg()),
      )
    }
  }

  // completed_data_ids: repeated string
  pub fn completed_data_ids(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_sync_SyncStatusResponse_completed_data_ids_get(self.raw_msg()),
      )
    }
  }

  // last_sync_timestamp: optional uint64
  pub fn last_sync_timestamp(self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncStatusResponse_last_sync_timestamp_get(self.raw_msg()) }
  }

}

// SAFETY:
// - `SyncStatusResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for SyncStatusResponseView<'_> {}

// SAFETY:
// - `SyncStatusResponseView` is `Send` because while its alive a `SyncStatusResponseMut` cannot.
// - `SyncStatusResponseView` does not use thread-local data.
unsafe impl Send for SyncStatusResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SyncStatusResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for SyncStatusResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for SyncStatusResponseView<'msg> {
  type Proxied = SyncStatusResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, SyncStatusResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SyncStatusResponseView<'msg> {
  fn into_view<'shorter>(self) -> SyncStatusResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SyncStatusResponse> for SyncStatusResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SyncStatusResponse {
    let dst = SyncStatusResponse::new();
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_copy_from(dst.inner.raw(), self.inner.raw()) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SyncStatusResponse> for SyncStatusResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SyncStatusResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for SyncStatusResponse {
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
      let prototype = <SyncStatusResponseView as ::std::default::Default>::default().raw_msg();
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
impl ::protobuf::__internal::runtime::CppMapTypeConversions for SyncStatusResponse {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(<SyncStatusResponseView as ::std::default::Default>::default().raw_msg())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_message(std::mem::ManuallyDrop::new(self).raw_msg())
    }

    unsafe fn from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> SyncStatusResponseView<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        unsafe { SyncStatusResponseView::new(::protobuf::__internal::Private, ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(value.val.m)) }
    }

    unsafe fn mut_from_map_value<'b>(value: ::protobuf::__internal::runtime::MapValue) -> SyncStatusResponseMut<'b> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::Message);
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::wrap_raw(value.val.m) };
        SyncStatusResponseMut { inner }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SyncStatusResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SyncStatusResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SyncStatusResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SyncStatusResponseMut<'msg> {
  type Message = SyncStatusResponse;
}

impl ::std::fmt::Debug for SyncStatusResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    ::protobuf::__internal::runtime::debug_string(self.raw_msg(), f)
  }
}

impl ::protobuf::Serialize for SyncStatusResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> SyncStatusResponseMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SyncStatusResponse>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SyncStatusResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> SyncStatusResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }


  // pending_data_ids: repeated string
  pub fn pending_data_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_sync_SyncStatusResponse_pending_data_ids_get(self.raw_msg()),
      )
    }
  }
  pub fn pending_data_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_sync_SyncStatusResponse_pending_data_ids_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_pending_data_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncStatusResponse_pending_data_ids_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // completed_data_ids: repeated string
  pub fn completed_data_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_sync_SyncStatusResponse_completed_data_ids_get(self.raw_msg()),
      )
    }
  }
  pub fn completed_data_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_sync_SyncStatusResponse_completed_data_ids_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_completed_data_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncStatusResponse_completed_data_ids_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // last_sync_timestamp: optional uint64
  pub fn last_sync_timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncStatusResponse_last_sync_timestamp_get(self.raw_msg()) }
  }
  pub fn set_last_sync_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncStatusResponse_last_sync_timestamp_set(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `SyncStatusResponseMut` does not perform any shared mutation.
// - `SyncStatusResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for SyncStatusResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for SyncStatusResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for SyncStatusResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for SyncStatusResponseMut<'msg> {
  type Proxied = SyncStatusResponse;
  fn as_view(&self) -> ::protobuf::View<'_, SyncStatusResponse> {
    SyncStatusResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SyncStatusResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SyncStatusResponse>
  where
      'msg: 'shorter {
    SyncStatusResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for SyncStatusResponseMut<'msg> {
  type MutProxied = SyncStatusResponse;
  fn as_mut(&mut self) -> SyncStatusResponseMut<'msg> {
    SyncStatusResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SyncStatusResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> SyncStatusResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SyncStatusResponse {
  pub fn new() -> Self {
    let raw = unsafe { proto2_rust_thunk_Message_nearclip_sync_SyncStatusResponse_new() };
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::wrap_raw(raw) };
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SyncStatusResponse> {
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

  pub fn as_view(&self) -> SyncStatusResponseView {
    SyncStatusResponseView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> SyncStatusResponseMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    SyncStatusResponseMut::new(::protobuf::__internal::Private, inner)
  }

  // pending_data_ids: repeated string
  pub fn pending_data_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_sync_SyncStatusResponse_pending_data_ids_get(self.raw_msg()),
      )
    }
  }
  pub fn pending_data_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_sync_SyncStatusResponse_pending_data_ids_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_pending_data_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncStatusResponse_pending_data_ids_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // completed_data_ids: repeated string
  pub fn completed_data_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedView::from_raw(
        ::protobuf::__internal::Private,
        proto2_rust_thunk_nearclip_sync_SyncStatusResponse_completed_data_ids_get(self.raw_msg()),
      )
    }
  }
  pub fn completed_data_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          proto2_rust_thunk_nearclip_sync_SyncStatusResponse_completed_data_ids_get_mut(self.raw_msg()),
        ),
      )
    }
  }
  pub fn set_completed_data_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    // Prevent the memory from being deallocated. The setter
    // transfers ownership of the memory to the parent message.
    let val = std::mem::ManuallyDrop::new(src.into_proxied(::protobuf::__internal::Private));
    unsafe {
      proto2_rust_thunk_nearclip_sync_SyncStatusResponse_completed_data_ids_move_set(self.raw_msg(),
        val.inner(::protobuf::__internal::Private).raw());
    }
  }

  // last_sync_timestamp: optional uint64
  pub fn last_sync_timestamp(&self) -> u64 {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncStatusResponse_last_sync_timestamp_get(self.raw_msg()) }
  }
  pub fn set_last_sync_timestamp(&mut self, val: u64) {
    unsafe { proto2_rust_thunk_nearclip_sync_SyncStatusResponse_last_sync_timestamp_set(self.raw_msg(), val) }
  }

}  // impl SyncStatusResponse

impl ::std::ops::Drop for SyncStatusResponse {
  fn drop(&mut self) {
    unsafe { ::protobuf::__internal::runtime::proto2_rust_Message_delete(self.raw_msg()); }
  }
}

impl ::std::clone::Clone for SyncStatusResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SyncStatusResponse {
  type Proxied = Self;
  fn as_view(&self) -> SyncStatusResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SyncStatusResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SyncStatusResponseMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessageMut for SyncStatusResponseMut<'_> {
  fn get_raw_message_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

unsafe impl ::protobuf::__internal::runtime::CppGetRawMessage for SyncStatusResponseView<'_> {
  fn get_raw_message(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }
}

extern "C" {
  fn proto2_rust_thunk_Message_nearclip_sync_SyncStatusResponse_new() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_Message_nearclip_sync_SyncStatusResponse_default_instance() -> ::protobuf::__internal::runtime::RawMessage;
  fn proto2_rust_thunk_nearclip_sync_SyncStatusResponse_pending_data_ids_get_mut(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_sync_SyncStatusResponse_pending_data_ids_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_sync_SyncStatusResponse_pending_data_ids_move_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, value: ::protobuf::__internal::runtime::RawRepeatedField);

  fn proto2_rust_thunk_nearclip_sync_SyncStatusResponse_completed_data_ids_get_mut(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_sync_SyncStatusResponse_completed_data_ids_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> ::protobuf::__internal::runtime::RawRepeatedField;
  fn proto2_rust_thunk_nearclip_sync_SyncStatusResponse_completed_data_ids_move_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, value: ::protobuf::__internal::runtime::RawRepeatedField);

  fn proto2_rust_thunk_nearclip_sync_SyncStatusResponse_last_sync_timestamp_get(raw_msg: ::protobuf::__internal::runtime::RawMessage) -> u64;
  fn proto2_rust_thunk_nearclip_sync_SyncStatusResponse_last_sync_timestamp_set(raw_msg: ::protobuf::__internal::runtime::RawMessage, val: u64);

}

impl<'a> SyncStatusResponseMut<'a> {
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

impl<'a> SyncStatusResponseView<'a> {
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

impl ::protobuf::OwnedMessageInterop for SyncStatusResponse {
  unsafe fn __unstable_take_ownership_of_raw_message(msg: *mut ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::OwnedMessageInner::<SyncStatusResponse>::wrap_raw(raw) };
    Self { inner }
  }

  fn __unstable_leak_raw_message(self) -> *mut ::std::ffi::c_void {
    let s = ::std::mem::ManuallyDrop::new(self);
    s.raw_msg().as_ptr() as *mut _
  }
}

impl<'a> ::protobuf::MessageMutInterop<'a> for SyncStatusResponseMut<'a> {
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

impl<'a> ::protobuf::MessageViewInterop<'a> for SyncStatusResponseView<'a> {
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
pub struct DataType(i32);

#[allow(non_upper_case_globals)]
impl DataType {
  pub const Unknown: DataType = DataType(0);
  pub const Text: DataType = DataType(1);
  pub const Image: DataType = DataType(2);
  pub const File: DataType = DataType(3);
  pub const Url: DataType = DataType(4);
  pub const RichText: DataType = DataType(5);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unknown",
      1 => "Text",
      2 => "Image",
      3 => "File",
      4 => "Url",
      5 => "RichText",
      _ => return None
    })
  }
}

impl ::std::convert::From<DataType> for i32 {
  fn from(val: DataType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for DataType {
  fn from(val: i32) -> DataType {
    Self(val)
  }
}

impl ::std::default::Default for DataType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for DataType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "DataType::{}", constant_name)
    } else {
      write!(f, "DataType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for DataType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for DataType {}

impl ::protobuf::Proxied for DataType {
  type View<'a> = DataType;
}

impl ::protobuf::Proxy<'_> for DataType {}
impl ::protobuf::ViewProxy<'_> for DataType {}

impl ::protobuf::AsView for DataType {
  type Proxied = DataType;

  fn as_view(&self) -> DataType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DataType {
  fn into_view<'shorter>(self) -> DataType where 'msg: 'shorter {
    self
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for DataType {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    ::protobuf::__internal::runtime::new_enum_repeated()
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    ::protobuf::__internal::runtime::free_enum_repeated(f)
  }

  fn repeated_len(r: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    ::protobuf::__internal::runtime::cast_enum_repeated_view(r).len()
  }

  fn repeated_push(r: ::protobuf::Mut<::protobuf::Repeated<Self>>, val: impl ::protobuf::IntoProxied<DataType>) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(r).push(val.into_proxied(::protobuf::__internal::Private))
  }

  fn repeated_clear(r: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(r).clear()
  }

  unsafe fn repeated_get_unchecked(
      r: ::protobuf::View<::protobuf::Repeated<Self>>,
      index: usize,
  ) -> ::protobuf::View<DataType> {
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
      val: impl ::protobuf::IntoProxied<DataType>,
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
unsafe impl ::protobuf::__internal::Enum for DataType {
  const NAME: &'static str = "DataType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5)
  }
}

impl ::protobuf::__internal::runtime::CppMapTypeConversions for DataType {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        Self::to_map_value(Self::default())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_u32(self.0 as u32)
    }

    unsafe fn from_map_value<'a>(value: ::protobuf::__internal::runtime::MapValue) -> ::protobuf::View<'a, Self> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::U32);
        DataType(unsafe { value.val.u as i32 })
    }
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SyncOperation(i32);

#[allow(non_upper_case_globals)]
impl SyncOperation {
  pub const SyncUnknown: SyncOperation = SyncOperation(0);
  pub const SyncCreate: SyncOperation = SyncOperation(1);
  pub const SyncUpdate: SyncOperation = SyncOperation(2);
  pub const SyncDelete: SyncOperation = SyncOperation(3);
  pub const SyncReplace: SyncOperation = SyncOperation(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "SyncUnknown",
      1 => "SyncCreate",
      2 => "SyncUpdate",
      3 => "SyncDelete",
      4 => "SyncReplace",
      _ => return None
    })
  }
}

impl ::std::convert::From<SyncOperation> for i32 {
  fn from(val: SyncOperation) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for SyncOperation {
  fn from(val: i32) -> SyncOperation {
    Self(val)
  }
}

impl ::std::default::Default for SyncOperation {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for SyncOperation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "SyncOperation::{}", constant_name)
    } else {
      write!(f, "SyncOperation::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for SyncOperation {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for SyncOperation {}

impl ::protobuf::Proxied for SyncOperation {
  type View<'a> = SyncOperation;
}

impl ::protobuf::Proxy<'_> for SyncOperation {}
impl ::protobuf::ViewProxy<'_> for SyncOperation {}

impl ::protobuf::AsView for SyncOperation {
  type Proxied = SyncOperation;

  fn as_view(&self) -> SyncOperation {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SyncOperation {
  fn into_view<'shorter>(self) -> SyncOperation where 'msg: 'shorter {
    self
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for SyncOperation {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    ::protobuf::__internal::runtime::new_enum_repeated()
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    ::protobuf::__internal::runtime::free_enum_repeated(f)
  }

  fn repeated_len(r: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    ::protobuf::__internal::runtime::cast_enum_repeated_view(r).len()
  }

  fn repeated_push(r: ::protobuf::Mut<::protobuf::Repeated<Self>>, val: impl ::protobuf::IntoProxied<SyncOperation>) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(r).push(val.into_proxied(::protobuf::__internal::Private))
  }

  fn repeated_clear(r: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(r).clear()
  }

  unsafe fn repeated_get_unchecked(
      r: ::protobuf::View<::protobuf::Repeated<Self>>,
      index: usize,
  ) -> ::protobuf::View<SyncOperation> {
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
      val: impl ::protobuf::IntoProxied<SyncOperation>,
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
unsafe impl ::protobuf::__internal::Enum for SyncOperation {
  const NAME: &'static str = "SyncOperation";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::runtime::CppMapTypeConversions for SyncOperation {
    fn get_prototype() -> ::protobuf::__internal::runtime::MapValue {
        Self::to_map_value(Self::default())
    }

    fn to_map_value(self) -> ::protobuf::__internal::runtime::MapValue {
        ::protobuf::__internal::runtime::MapValue::make_u32(self.0 as u32)
    }

    unsafe fn from_map_value<'a>(value: ::protobuf::__internal::runtime::MapValue) -> ::protobuf::View<'a, Self> {
        debug_assert_eq!(value.tag, ::protobuf::__internal::runtime::MapValueTag::U32);
        SyncOperation(unsafe { value.val.u as i32 })
    }
}


