// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]
use crate::{Bindable, FromRuby, IntoRuby, Result};

use crate::{extern_struct_bind, extern_struct_fns};

use super::channel_control::ChannelControl;

// public api
pub type ChannelGroup = magnus::typed_data::Obj<ChannelControl>;
// implementation details
type ChannelGroupImpl = ChannelControl;

impl IntoRuby<ChannelGroup> for fmod::ChannelGroup {
    fn into_ruby(self) -> Result<ChannelGroup> {
        let channel_control = ChannelControl(*self);
        let obj = magnus::typed_data::Obj::wrap_as(channel_control, fmod::ChannelGroup::class());
        Ok(obj)
    }
}

impl FromRuby<fmod::ChannelGroup> for ChannelGroupImpl {
    fn from_ruby(self) -> Result<fmod::ChannelGroup> {
        Ok(self.into_channel_group())
    }
}

impl FromRuby<fmod::ChannelGroup> for ChannelGroup {
    fn from_ruby(self) -> Result<fmod::ChannelGroup> {
        Ok(self.into_channel_group())
    }
}

extern_struct_fns! {
  impl ChannelGroupImpl: fmod::ChannelGroup {
    fn get_channel_count() -> i32;
  }
}

extern_struct_bind! {
  impl Bindable for ChannelGroupImpl: fmod::ChannelGroup, super = fmod::ChannelControl::class {
    fn get_channel_count -> 0;
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::ChannelGroup::bind(module)?;

    Ok(())
}
