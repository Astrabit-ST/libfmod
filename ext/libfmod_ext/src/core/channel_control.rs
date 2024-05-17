// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]
use crate::{Bindable, FromRuby, IntoRuby, Result};

use crate::{extern_struct_bind, extern_struct_fns};

#[derive(Clone, Copy)]
#[magnus::wrap(class = "FMOD::ChannelControl", size, free_immediately)]
// FIXME add type validation
pub struct ChannelControl(pub(super) fmod::ChannelControl);

impl IntoRuby<ChannelControl> for fmod::ChannelControl {
    fn into_ruby(self) -> Result<ChannelControl> {
        Ok(ChannelControl(self))
    }
}

impl FromRuby<fmod::ChannelControl> for ChannelControl {
    fn from_ruby(self) -> Result<fmod::ChannelControl> {
        Ok(self.0)
    }
}

impl ChannelControl {
    pub fn into_channel(self) -> fmod::Channel {
        let channel: *mut fmod::ffi::FMOD_CHANNELCONTROL = self.0.into();
        let channel: *mut fmod::ffi::FMOD_CHANNEL = channel.cast();
        fmod::Channel::from(channel)
    }

    pub fn into_channel_group(self) -> fmod::ChannelGroup {
        let channel_group: *mut fmod::ffi::FMOD_CHANNELCONTROL = self.0.into();
        let channel_group: *mut fmod::ffi::FMOD_CHANNELGROUP = channel_group.cast();
        fmod::ChannelGroup::from(channel_group)
    }
}

extern_struct_fns! {
  impl ChannelControl: fmod::ChannelControl {

  }
}

extern_struct_bind! {
  impl Bindable for ChannelControl: fmod::ChannelControl {

  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::ChannelControl::bind(module)?;

    Ok(())
}
