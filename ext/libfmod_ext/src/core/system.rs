// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::{thread, Bindable, FromRuby, IntoRuby, Result};
use magnus::prelude::*;

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

use super::{
    channel::RbChannel,
    channel_group::RbChannelGroup,
    dsp::RbDSP,
    enums::{DspType, OutputType, PluginType, PortType, Speaker, SpeakerMode, TimeUnit},
    flags::{DriverState, SystemCallbackMask},
    geometry::RbGeometry,
    reverb_3d::RbReverb3D,
    rolloff_callback::RolloffCallback,
    sound::RbSound,
    sound_builder::SoundBuilder,
    sound_group::RbSoundGroup,
    structs::{CPUUsage, Guid, ReverbProperties, Vector},
    system_builder::SystemBuilder,
    system_callback::SystemCallback,
};

extern_struct! {
  struct System: fmod::System => "FMOD::System"
}

impl System {
    fn new() -> Result<RbSystem> {
        unsafe { fmod::System::new() }.into_ruby()
    }

    fn update(rb_self: RbSystem) -> Result<()> {
        let system: fmod::System = rb_self.from_ruby()?;
        unsafe { thread::without_gvl_no_ubf(|| system.update()) }.into_ruby()?;
        crate::extern_struct_storage::cleanup();
        Ok(())
    }

    fn release(rb_self: RbSystem) -> Result<()> {
        let system: fmod::System = rb_self.from_ruby()?;
        unsafe { system.release() }.into_ruby()?;
        crate::extern_struct_storage::remove(system);
        crate::extern_struct_storage::cleanup();
        Ok(())
    }

    fn lock_dsp(rb_self: RbSystem) -> Result<()> {
        let system: fmod::System = rb_self.from_ruby()?;
        unsafe { thread::without_gvl_no_ubf(|| system.lock_dsp()) }.into_ruby()
    }

    fn create_sound(rb_self: RbSystem, builder: &SoundBuilder) -> Result<RbSound> {
        let system: fmod::System = rb_self.from_ruby()?;
        let borrow = builder.0.borrow();
        let builder = borrow
            .as_ref()
            .ok_or_else(SoundBuilder::invalid_state_error)?;
        unsafe { thread::without_gvl_no_ubf(|| system.create_sound(builder)) }.into_ruby()
    }

    fn create_stream(rb_self: RbSystem, builder: &SoundBuilder) -> Result<RbSound> {
        let system: fmod::System = rb_self.from_ruby()?;
        let borrow = builder.0.borrow();
        let builder = borrow
            .as_ref()
            .ok_or_else(SoundBuilder::invalid_state_error)?;
        unsafe { thread::without_gvl_no_ubf(|| system.create_stream(builder)) }.into_ruby()
    }

    fn get_userdata(rb_self: RbSystem) -> Result<magnus::Value> {
        rb_self.ivar_get("__userdata")
    }

    fn set_userdata(rb_self: RbSystem, data: magnus::Value) -> Result<()> {
        rb_self.ivar_set("__userdata", data)
    }

    fn set_callback(
        rb_self: RbSystem,
        mask: SystemCallbackMask,
        callback: magnus::Value,
    ) -> Result<()> {
        let system: fmod::System = rb_self.from_ruby()?;
        let mask = mask.from_ruby()?;

        if !callback
            .class()
            .is_inherited(super::system_callback::class())
        {
            return Err(magnus::Error::new(
                magnus::exception::runtime_error(),
                "callback must be a SystemCallback",
            ));
        }

        rb_self.ivar_set("__callback", callback)?;
        system.set_callback::<SystemCallback>(mask).into_ruby()
    }

    fn set_3d_rolloff_callback(rb_self: RbSystem, callback: magnus::Value) -> Result<()> {
        let system: fmod::System = rb_self.from_ruby()?;

        if !callback
            .class()
            .is_inherited(super::rolloff_callback::class())
        {
            return Err(magnus::Error::new(
                magnus::exception::runtime_error(),
                "callback must be a SystemCallback",
            ));
        }

        rb_self.ivar_set("__rolloff_callback", callback)?;
        system
            .set_3d_rolloff_callback::<RolloffCallback>()
            .into_ruby()
    }
}

extern_struct_fns! {
  impl System: fmod::System {
    fn create_dsp_by_type(dsp_type: DspType) -> RbDSP;
    fn create_channel_group(name: magnus::RString) -> RbChannelGroup;
    fn create_sound_group(name: magnus::RString) -> RbSoundGroup;
    fn create_reverb_3d() -> RbReverb3D;
    fn play_sound(sound: RbSound, channel_group: Option<RbChannelGroup>, paused: bool) -> RbChannel;
    fn play_dsp(dsp: RbDSP, channel_group: Option<RbChannelGroup>, paused: bool) -> RbChannel;
    fn get_channel(channel_id: i32) -> RbChannel;
    // TODO get dsp info
    fn get_master_channel_group() -> RbChannelGroup;
    fn get_master_sound_group() -> RbSoundGroup;
    fn set_output(output_type: OutputType) -> ();
    fn get_output_type() -> OutputType;
    fn get_driver_count() -> i32;
    fn get_driver_info(driver_id: i32) -> (magnus::RString, Guid, i32, SpeakerMode, i32);
    fn set_driver(driver_id: i32) -> ();
    fn get_driver() -> i32;
    fn unlock_dsp() -> ();
    fn create_geometry(max_polygons: i32, max_vertices: i32) -> RbGeometry;
    fn set_geometry_settings(max_world_size: f32) -> ();
    fn get_geometry_settings() -> f32;
    fn load_geometry(data: magnus::RString) -> RbGeometry;
    fn get_version() -> u32;
    // TODO get ouput handle
    fn get_playing_channels() -> (i32, i32);
    fn get_cpu_usage() -> CPUUsage;
    fn get_file_usage() -> (i64, i64, i64);
    // TODO mix matrix
    fn get_speaker_mode_channels(speaker_mode: SpeakerMode) -> i32;
    fn close() -> SystemBuilder;
    fn suspend_mixer() -> ();
    fn resume_mixer() -> ();
    fn set_network_proxy(proxy: magnus::RString) -> ();
    fn get_network_proxy() -> magnus::RString;
    fn set_network_timeout(timeout: i32) -> ();
    fn get_network_timeout() -> i32;
    fn set_plugin_path(path: magnus::RString) -> ();
    // TODO load plugin
    fn unload_plugin(handle: u32) -> ();
    fn get_nested_plugin_count(handle: u32) -> i32;
    fn get_nested_plugin(handle: u32, index: i32) -> u32;
    fn get_plugin_info(handle: u32) -> (PluginType, magnus::RString, u32);
    fn set_output_by_plugin(handle: u32) -> ();
    fn get_output_by_plugin() -> u32;
    fn create_dsp_by_plugin(handle: u32) -> RbDSP;
    // TODO dsp info by plugin
    fn get_recording_driver_count() -> (i32, i32);
    fn get_record_driver_info(driver_id: i32) -> (magnus::RString, Guid, i32, SpeakerMode, i32, DriverState);
    fn get_record_position(driver_id: i32) -> u32;
    fn record_start(driver_id: i32, sound: RbSound, do_loop: bool) -> ();
    fn record_stop(driver_id: i32) -> ();
    fn is_recording(driver_id: i32) -> bool;
    fn set_3d_listener_attributes(listener: i32, pos: Option<Vector>, velocity: Option<Vector>, forward: Option<Vector>, up: Option<Vector>) -> ();
    fn get_3d_listener_attributes(listener: i32) -> (Vector, Vector, Vector, Vector);
    fn set_reverb_properties(instance: i32, properties: Option<ReverbProperties>) -> ();
    fn get_reverb_properties(instance: i32) -> ReverbProperties;
    fn attach_channel_group_to_port(kind: PortType, index: Option<u64>, channel_group: RbChannelGroup, pass_through: bool) -> ();
    fn detach_channel_group_from_port(channel_group: RbChannelGroup) -> ();
    fn get_software_channels() -> i32;
    fn get_software_format() -> (i32, SpeakerMode, i32);
    fn get_dsp_buffer_size() -> (u32, i32);
    fn set_stream_buffer_size(size: u32, unit: TimeUnit) -> ();
    fn get_stream_buffer_size() -> (u32, TimeUnit);
    fn set_speaker_position(speaker: Speaker, x: f32, y: f32, active: bool) -> ();
    fn get_speaker_position(speaker: Speaker) -> (f32, f32, bool);
    fn set_3d_settings(doppler_scale: f32, distance_factor: f32, rolloff_scale: f32) -> ();
    fn get_3d_settings() -> (f32, f32, f32);
    fn set_3d_listener_count(count: i32) -> ();
    fn get_3d_listener_count() -> i32;
    fn unset_3d_rolloff_callback() -> ();
  }
}

extern_struct_bind! {
  impl Bindable for System: fmod::System {
    fn set_callback -> 2;
    fn create_sound -> 1;
    fn create_stream -> 1;
    fn create_dsp_by_type -> 1;
    fn create_channel_group -> 1;
    fn create_sound_group -> 1;
    fn create_reverb_3d -> 0;
    fn play_sound -> 3;
    fn play_dsp -> 3;
    fn get_channel -> 1;
    fn get_master_channel_group -> 0;
    fn get_master_sound_group -> 0;
    fn set_output -> 1;
    fn get_output_type -> 0;
    fn get_driver_count -> 0;
    fn get_driver_info -> 1;
    fn set_driver -> 1;
    fn get_driver -> 0;
    fn lock_dsp -> 0;
    fn unlock_dsp -> 0;
    fn get_userdata -> 0;
    fn set_userdata -> 1;
    fn create_geometry -> 2;
    fn set_geometry_settings -> 1;
    fn get_geometry_settings -> 0;
    fn load_geometry -> 1;
    fn get_version -> 0;
    fn get_playing_channels -> 0;
    fn get_cpu_usage -> 0;
    fn get_file_usage -> 0;
    fn get_speaker_mode_channels -> 1;
    fn close -> 0;
    fn release -> 0;
    fn update -> 0;
    fn suspend_mixer -> 0;
    fn resume_mixer -> 0;
    fn set_network_proxy -> 1;
    fn get_network_proxy -> 0;
    fn set_network_timeout -> 1;
    fn get_network_timeout -> 0;
    fn set_plugin_path -> 1;
    fn unload_plugin -> 1;
    fn get_nested_plugin_count -> 1;
    fn get_nested_plugin -> 2;
    fn get_plugin_info -> 1;
    fn set_output_by_plugin -> 1;
    fn get_output_by_plugin -> 0;
    fn create_dsp_by_plugin -> 1;
    fn get_recording_driver_count -> 0;
    fn get_record_driver_info -> 1;
    fn get_record_position -> 1;
    fn record_start -> 3;
    fn record_stop -> 1;
    fn is_recording -> 1;
    fn set_3d_listener_attributes -> 5;
    fn get_3d_listener_attributes -> 1;
    fn set_reverb_properties -> 2;
    fn get_reverb_properties -> 1;
    fn attach_channel_group_to_port -> 4;
    fn detach_channel_group_from_port -> 1;
    fn get_software_channels -> 0;
    fn get_software_format -> 0;
    fn get_dsp_buffer_size -> 0;
    fn set_stream_buffer_size -> 2;
    fn get_stream_buffer_size -> 0;
    fn set_speaker_position -> 4;
    fn get_speaker_position -> 1;
    fn set_3d_settings -> 3;
    fn get_3d_settings -> 0;
    fn set_3d_listener_count -> 1;
    fn get_3d_listener_count -> 0;
    fn set_3d_rolloff_callback -> 1;
    fn unset_3d_rolloff_callback -> 0;
    ruby_compat_methods: true

    |class| {
      class.define_singleton_method("new", magnus::function!(System::new, 0))?;
    }
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::System::bind(module)?;

    Ok(())
}
