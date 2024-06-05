// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]
use magnus::prelude::*;

use crate::{Bindable, FromRuby, IntoRuby, Result};

use crate::{extern_struct_bind, extern_struct_fns};

use super::channel_control::{ChannelControl, ChannelControlType};
use super::channel_group::RbChannelGroup;
use super::enums::TimeUnit;
use super::sound::RbSound;

// public api
pub type RbChannel = magnus::typed_data::Obj<ChannelControl>;
// implementation details
type Channel = ChannelControl;

impl IntoRuby<RbChannel> for fmod::Channel {
    fn into_ruby(self) -> Result<RbChannel> {
        let channel_control = ChannelControl(*self, ChannelControlType::Channel);
        crate::extern_struct_storage::get_or_insert_with(*self, || {
            magnus::typed_data::Obj::wrap_as(channel_control, fmod::Channel::class())
        })
    }
}

impl FromRuby<fmod::Channel> for RbChannel {
    fn from_ruby(self) -> Result<fmod::Channel> {
        self.into_channel()
    }
}

impl FromRuby<fmod::Channel> for Channel {
    fn from_ruby(self) -> Result<fmod::Channel> {
        self.into_channel()
    }
}

extern_struct_fns! {
  impl Channel: fmod::Channel {
    fn is_virtual() -> bool;
    fn get_current_sound() -> Option<RbSound>;
    fn get_index() -> i32;
    fn set_frequency(frequency: f32) -> ();
    fn get_frequency() -> f32;
    fn set_priority(priority: i32) -> ();
    fn get_priority() -> i32;
    fn set_position(position: u32, unit: TimeUnit) -> ();
    fn get_position(unit: TimeUnit) -> u32;
    fn set_channel_group(group: RbChannelGroup) -> ();
    fn get_channel_group() -> RbChannelGroup;
    fn set_loop_count(loop_count: i32) -> ();
    fn get_loop_count() -> i32;
    fn set_loop_points(start: u32, start_unit: TimeUnit, end: u32, end_unit: TimeUnit) -> ();
    fn get_loop_points(start_unit: TimeUnit, end_unit: TimeUnit) -> (u32, u32);
  }
}

extern_struct_bind! {
  impl Bindable for Channel: fmod::Channel, super = fmod::ChannelControl::class, class_name = "Channel" {
    fn is_virtual -> 0;
    fn get_current_sound -> 0;
    fn get_index -> 0;
    fn set_frequency -> 1;
    fn get_frequency -> 0;
    fn set_priority -> 1;
    fn get_priority -> 0;
    fn set_position -> 2;
    fn get_position -> 1;
    fn set_channel_group -> 1;
    fn get_channel_group -> 0;
    fn set_loop_count -> 1;
    fn get_loop_count -> 0;
    fn set_loop_points -> 4;
    fn get_loop_points -> 2;
    |class| {
      class.undef_default_alloc_func();
    }
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::Channel::bind(module)?;

    Ok(())
}
