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
pub type Channel = magnus::typed_data::Obj<ChannelControl>;
// implementation details
type ChannelImpl = ChannelControl;

impl IntoRuby<Channel> for fmod::Channel {
    fn into_ruby(self) -> Result<Channel> {
        let channel_control = ChannelControl(*self);
        let obj = magnus::typed_data::Obj::wrap_as(channel_control, fmod::Channel::class());
        Ok(obj)
    }
}

impl FromRuby<fmod::Channel> for Channel {
    fn from_ruby(self) -> Result<fmod::Channel> {
        Ok(self.into_channel())
    }
}

impl FromRuby<fmod::Channel> for ChannelImpl {
    fn from_ruby(self) -> Result<fmod::Channel> {
        Ok(self.into_channel())
    }
}

extern_struct_fns! {
  impl ChannelImpl: fmod::Channel {
    fn is_virtual() -> bool;
  }
}

extern_struct_bind! {
  impl Bindable for ChannelImpl: fmod::Channel, super = fmod::ChannelControl::class {
    fn is_virtual -> 0;
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::Channel::bind(module)?;

    Ok(())
}
