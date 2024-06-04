// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]
use magnus::prelude::*;

use crate::{Bindable, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

use super::dsp_connection::RbDSPConnection;
use super::enums::{DspConnectionType, DspType, SpeakerMode};
use super::flags::ChannelMask;
use super::structs::DspMeteringInfo;
use super::system::RbSystem;

extern_struct! {
  struct DSP: fmod::Dsp => "FMOD::DSP"
}

impl DSP {
    fn release(&self) -> Result<()> {
        use crate::{FromRuby, IntoRuby};
        // we dont need to check if the dsp is already removed, because FromRuby will return an error if it is
        let dsp: fmod::Dsp = self.from_ruby()?;
        crate::extern_struct_storage::remove(dsp);
        dsp.release().into_ruby()
    }

    fn get_userdata(rb_self: RbDSP) -> Result<magnus::Value> {
        rb_self.ivar_get("__userdata")
    }

    fn set_userdata(rb_self: RbDSP, data: magnus::Value) -> Result<()> {
        rb_self.ivar_set("__userdata", data)
    }
}

extern_struct_fns! {
  impl DSP: fmod::Dsp {
    fn set_channel_format(mask: ChannelMask, channel_count: i32, speaker_mode: SpeakerMode) -> ();
    fn get_channel_format() -> (ChannelMask, i32, SpeakerMode);
    fn get_output_channel_format(mask: ChannelMask, channel_count: i32, speaker_mode: SpeakerMode) -> (ChannelMask, i32, SpeakerMode);
    fn add_input(input: RbDSP, kind: DspConnectionType) -> RbDSPConnection;
    fn get_input(index: i32) -> (RbDSP, RbDSPConnection);
    fn get_output(index: i32) -> (RbDSP, RbDSPConnection);
    fn get_input_count() -> i32;
    fn get_output_count() -> i32;
    fn disconnect_all(inputs: bool, outputs: bool) -> ();
    fn disconnect_from(target: Option<RbDSP>, connection: Option<RbDSPConnection>) -> ();
    fn reset() -> ();
    fn get_type() -> DspType;
    fn get_cpu_usage() -> (u32, u32);
    fn get_system() -> RbSystem;
    fn get_metering_info() -> (DspMeteringInfo, DspMeteringInfo);
    fn set_metering_enabled(input_enabled: bool, output_enabled: bool) -> ();
    fn get_metering_enabled() -> (bool, bool);
    // TODO data parameter
    fn get_parameter_count() -> i32;
    // FIXME add type-agnostic version
    fn set_parameter_bool(index: i32, value: bool) -> ();
    fn get_parameter_bool(index: i32) -> bool;
    // TODO parameter data
    fn set_parameter_float(index: i32, value: f32) -> ();
    fn get_parameter_float(index: i32) -> f32;
    fn set_parameter_int(index: i32, value: i32) -> ();
    fn get_parameter_int(index: i32) -> i32;
    // TODO parameter info
    fn set_active(active: bool) -> ();
    fn get_active() -> bool;
    fn set_bypass(bypass: bool) -> ();
    fn get_bypass() -> bool;
    fn set_wet_dry_mix(pre: f32, post: f32, dry: f32) -> ();
    fn get_wet_dry_mix() -> (f32, f32, f32);
    fn get_idle() -> bool;
  }
}

extern_struct_bind! {
  impl Bindable for DSP: fmod::Dsp {
    fn set_channel_format -> 3;
    fn get_channel_format -> 0;
    fn get_output_channel_format -> 3;
    fn add_input -> 2;
    fn get_input -> 1;
    fn get_output -> 1;
    fn get_input_count -> 0;
    fn get_output_count -> 0;
    fn disconnect_all -> 2;
    fn disconnect_from -> 2;
    fn reset -> 0;
    fn release -> 0;
    fn get_type -> 0;
    fn get_cpu_usage -> 0;
    fn get_userdata -> 0;
    fn set_userdata -> 1;
    fn get_system -> 0;
    fn get_metering_info -> 0;
    fn set_metering_enabled -> 2;
    fn get_metering_enabled -> 0;
    fn get_parameter_count -> 0;
    fn set_parameter_bool -> 2;
    fn get_parameter_bool -> 1;
    fn set_parameter_float -> 2;
    fn get_parameter_float -> 1;
    fn set_parameter_int -> 2;
    fn get_parameter_int -> 1;
    fn set_active -> 1;
    fn get_active -> 0;
    fn set_bypass -> 1;
    fn get_bypass -> 0;
    fn set_wet_dry_mix -> 3;
    fn get_wet_dry_mix -> 0;
    fn get_idle -> 0;
    ruby_compat_methods: true
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::Dsp::bind(module)?;

    Ok(())
}
