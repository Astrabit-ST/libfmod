// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]
use crate::{Bindable, FromRuby, Result};
use magnus::prelude::*;

use crate::{extern_struct_bind, extern_struct_fns};

use super::{dsp::RbDSP, structs::Vector, system::RbSystem};

#[derive(Clone, Copy)]
#[magnus::wrap(class = "FMOD::ChannelControl", size, free_immediately)]
pub struct ChannelControl(
    pub(super) fmod::ChannelControl,
    pub(super) ChannelControlType,
);

#[derive(Clone, Copy)]
pub enum ChannelControlType {
    Channel,
    ChannelGroup,
}

impl FromRuby<fmod::ChannelControl> for ChannelControl {
    fn from_ruby(self) -> Result<fmod::ChannelControl> {
        Ok(self.0)
    }
}

impl ChannelControl {
    pub fn into_channel(self) -> Result<fmod::Channel> {
        if !matches!(self.1, ChannelControlType::Channel) {
            return Err(magnus::Error::new(
                magnus::exception::runtime_error(),
                "ChannelControl is not a Channel",
            ));
        }

        let channel: *mut fmod::ffi::FMOD_CHANNELCONTROL = self.0.into();
        let channel: *mut fmod::ffi::FMOD_CHANNEL = channel.cast();
        Ok(fmod::Channel::from(channel))
    }

    pub fn into_channel_group(self) -> Result<fmod::ChannelGroup> {
        if !matches!(self.1, ChannelControlType::ChannelGroup) {
            return Err(magnus::Error::new(
                magnus::exception::runtime_error(),
                "ChannelControl is not a ChannelGroup",
            ));
        }

        let channel_group: *mut fmod::ffi::FMOD_CHANNELCONTROL = self.0.into();
        let channel_group: *mut fmod::ffi::FMOD_CHANNELGROUP = channel_group.cast();
        Ok(fmod::ChannelGroup::from(channel_group))
    }
}

extern_struct_fns! {
  impl ChannelControl: fmod::ChannelControl {
    fn add_dsp(index: i32, dsp: RbDSP) -> ();
    fn remove_dsp(dsp: RbDSP) -> ();
    fn get_dsp_count() -> i32;
    fn get_dsp(index: i32) -> RbDSP;
    fn set_dsp_index(dsp: RbDSP, index: i32) -> ();
    fn get_dsp_index(dsp: RbDSP) -> i32;
    fn set_reverb_properties(instance: i32, wet: f32) -> ();
    fn get_reverb_properties(instance: i32) -> f32;
    fn set_low_pass_gain(gain: f32) -> ();
    fn get_low_pass_gain() -> f32;
    // TODO userdata
    fn get_system() -> RbSystem;
    fn set_pan(pan: f32) -> ();
    // TODO mix matrix
    fn is_playing() -> bool;
    fn stop() -> ();
    fn set_paused(pause: bool) -> ();
    fn get_paused() -> bool;
    // TODO mode
    fn set_pitch(pitch: f32) -> ();
    fn get_pitch() -> f32;
    fn get_dsp_clock() -> (u64, u64);
    fn set_delay(start: u64, end: u64, stopchannels: bool) -> ();
    fn get_delay() -> (u64, u64, bool);
    fn add_fade_point(dsp_clock: u64, volume: f32) -> ();
    fn set_fade_point_ramp(dsp_clock: u64, volume: f32) -> ();
    fn remove_fade_points(dsp_clock_start: u64, dsp_clock_end: u64) -> ();
    fn get_fade_points() -> (Vec<u64>, Vec<f32>);
    fn set_3d_attributes(pos: Option<Vector>, vel: Option<Vector>) -> ();
    fn get_3d_attributes() -> (Vector, Vector);
    fn set_3d_cone_orientation(orientation: Vector) -> ();
    fn get_3d_cone_orientation() -> Vector;
    fn set_3d_cone_settings(inside_angle: f32, outside_angle: f32, outside_volume: f32) -> ();
    fn get_3d_cone_settings() -> (f32, f32, f32);
    // TODO custom rolloff
    fn set_3d_distance_filter(custom: bool, custom_level: f32, center_freq: f32) -> ();
    fn get_3d_distance_filter() -> (bool, f32, f32);
    fn set_3d_doppler_level(level: f32) -> ();
    fn get_3d_doppler_level() -> f32;
    fn set_3d_level(level: f32) -> ();
    fn get_3d_level() -> f32;
    fn set_3d_min_max_distance(min: f32, max: f32) -> ();
    fn get_3d_min_max_distance() -> (f32, f32);
    fn set_3d_occlusion(direct: f32, reverb: f32) -> ();
    fn get_3d_occlusion() -> (f32, f32);
    fn set_3d_spread(angle: f32) -> ();
    fn get_3d_spread() -> f32;
    fn get_audibility() -> f32;
    fn set_volume(volume: f32) -> ();
    fn get_volume() -> f32;
    fn set_volume_ramp(ramp: bool) -> ();
    fn get_volume_ramp() -> bool;
    fn set_mute(mute: bool) -> ();
    fn get_mute() -> bool;
  }
}

extern_struct_bind! {
  impl Bindable for ChannelControl: fmod::ChannelControl {
    fn add_dsp -> 2;
    fn remove_dsp -> 1;
    fn get_dsp_count -> 0;
    fn get_dsp -> 1;
    fn set_dsp_index -> 2;
    fn get_dsp_index -> 1;
    fn set_reverb_properties -> 2;
    fn get_reverb_properties -> 1;
    fn set_low_pass_gain -> 1;
    fn get_low_pass_gain -> 0;
    fn get_system -> 0;
    fn set_pan -> 1;
    fn is_playing -> 0;
    fn stop -> 0;
    fn set_paused -> 1;
    fn get_paused -> 0;
    fn set_pitch -> 1;
    fn get_pitch -> 0;
    fn get_dsp_clock -> 0;
    fn set_delay -> 3;
    fn get_delay -> 0;
    fn add_fade_point -> 2;
    fn set_fade_point_ramp -> 2;
    fn remove_fade_points -> 2;
    fn get_fade_points -> 0;
    fn set_3d_attributes -> 2;
    fn get_3d_attributes -> 0;
    fn set_3d_cone_orientation -> 1;
    fn get_3d_cone_orientation -> 0;
    fn set_3d_cone_settings -> 3;
    fn get_3d_cone_settings -> 0;
    fn set_3d_distance_filter -> 3;
    fn get_3d_distance_filter -> 0;
    fn set_3d_doppler_level -> 1;
    fn get_3d_doppler_level -> 0;
    fn set_3d_level -> 1;
    fn get_3d_level -> 0;
    fn set_3d_min_max_distance -> 2;
    fn get_3d_min_max_distance -> 0;
    fn set_3d_occlusion -> 2;
    fn get_3d_occlusion -> 0;
    fn set_3d_spread -> 1;
    fn get_3d_spread -> 0;
    fn get_audibility -> 0;
    fn set_volume -> 1;
    fn get_volume -> 0;
    fn set_volume_ramp -> 1;
    fn get_volume_ramp -> 0;
    fn set_mute -> 1;
    fn get_mute -> 0;
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::ChannelControl::bind(module)?;

    let class = fmod::ChannelControl::class();
    class.const_set("DSP_HEAD", fmod::ChannelControl::DSP_HEAD)?;
    class.const_set("DSP_TAIL", fmod::ChannelControl::DSP_TAIL)?;
    class.const_set("DSP_FADER", fmod::ChannelControl::DSP_FADER)?;

    Ok(())
}
