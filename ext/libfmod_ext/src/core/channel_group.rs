// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]
use crate::{Bindable, FromRuby, IntoRuby, Result};
use magnus::prelude::*;

use crate::{extern_struct_bind, extern_struct_fns};

use super::channel::Channel;
use super::channel_control::{ChannelControl, ChannelControlType};
use super::dsp_connection::RbDSPConnection;

// public api
pub type ChannelGroup = magnus::typed_data::Obj<ChannelControl>;
// implementation details
type ChannelGroupImpl = ChannelControl;

impl IntoRuby<ChannelGroup> for fmod::ChannelGroup {
    fn into_ruby(self) -> Result<ChannelGroup> {
        let channel_control = ChannelControl(*self, ChannelControlType::ChannelGroup);
        crate::extern_struct_storage::get_or_insert_with(*self, || {
            magnus::typed_data::Obj::wrap_as(channel_control, fmod::ChannelGroup::class())
        })
    }
}

impl FromRuby<fmod::ChannelGroup> for ChannelGroupImpl {
    fn from_ruby(self) -> Result<fmod::ChannelGroup> {
        self.into_channel_group()
    }
}

impl FromRuby<fmod::ChannelGroup> for ChannelGroup {
    fn from_ruby(self) -> Result<fmod::ChannelGroup> {
        self.into_channel_group()
    }
}

extern_struct_fns! {
  impl ChannelGroupImpl: fmod::ChannelGroup {
    fn get_channel_count() -> i32;
    fn get_channel(index: i32) -> Channel;
    fn get_name() -> magnus::RString;
    fn release() -> ();
    fn add_group(group: ChannelGroup, propgate_dsp_clock: bool) -> RbDSPConnection;
    fn get_group_count() -> i32;
    fn get_group(index: i32) -> ChannelGroup;
    fn get_parent_group() -> ChannelGroup;
  }
}

extern_struct_bind! {
  impl Bindable for ChannelGroupImpl: fmod::ChannelGroup, super = fmod::ChannelControl::class, class_name = "ChannelGroup" {
    fn get_channel_count -> 0;
    fn get_channel -> 1;
    fn get_name -> 0;
    fn release -> 0;
    fn add_group -> 2;
    fn get_group_count -> 0;
    fn get_group -> 1;
    fn get_parent_group -> 0;
    |class| {
      class.undef_default_alloc_func();
    }
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::ChannelGroup::bind(module)?;

    Ok(())
}
