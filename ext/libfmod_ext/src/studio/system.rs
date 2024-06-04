// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{
    core::structs::{Attributes3D, CPUUsage, Guid, Vector},
    Bindable, FromRuby, IntoRuby, Result,
};
use magnus::prelude::*;

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

use super::{
    bank::RbBank,
    bus::RbBus,
    command_replay::RbCommandReplay,
    flags::{CommandCaptureFlags, CommandReplayFlags, LoadBankFlags, SystemCallbackMask},
    structs::{
        AdvancedSettings, BufferUsage, CPUUsage as StudioCPUUsage, MemoryUsage,
        ParameterDescription, ParameterID, SoundInfo,
    },
    system_callback::SystemCallback,
    vca::RbVCA,
};

extern_struct! {
    struct System: fmod::studio::System => "FMOD::Studio::System"
}

extern_struct_fns! {
  impl System: fmod::studio::System {
    fn load_bank_file(filename: magnus::RString, flags: LoadBankFlags) -> RbBank;
    fn load_bank_memory(buffer: magnus::RString, flags: LoadBankFlags) -> RbBank;
    fn unload_all_banks() -> ();
    fn get_bank(path_or_id: magnus::RString) -> RbBank;
    fn get_bank_by_id(id: Guid) -> RbBank;
    fn bank_count() -> i32;
    fn get_bank_list() -> magnus::r_array::TypedArray<RbBank>;
    fn start_command_capture(filename: magnus::RString, flags: CommandCaptureFlags) -> ();
    fn stop_command_capture() -> ();
    fn load_command_replay(filename: magnus::RString, flags: CommandReplayFlags) -> RbCommandReplay;
    fn get_core_system() -> crate::core::system::RbSystem;
    fn lookup_id(path: magnus::RString) -> Guid;
    fn lookup_path(id: Guid) -> magnus::RString;
    fn is_valid() -> bool;
    fn flush_commands() -> ();
    fn flush_sample_loading() -> ();
    // FIXME optional params
    fn set_listener_attributes(listener: i32, attributes: Attributes3D, attenuation_position: Option<Vector>) -> ();
    fn get_listener_attributes(listener: i32) -> (Attributes3D, Vector); // maybe add array accessors?
    fn set_listener_weight(listener: i32, weight: f32) -> ();
    fn get_listener_weight(listener: i32) -> f32;
    fn set_listener_count(count: i32) -> ();
    fn get_listener_count() -> i32;
    fn get_bus(path_or_id: magnus::RString) -> RbBus;
    fn get_bus_by_id(id: Guid) -> RbBus;
    fn get_vca(path_or_id: magnus::RString) -> RbVCA;
    fn get_vca_by_id(id: Guid) -> RbVCA;
    fn get_advanced_settings() -> AdvancedSettings;
    fn get_parameter_by_id(id: ParameterID) -> (f32, f32);
    fn set_parameter_by_id(id: ParameterID, value: f32, ignore_seek_speed: bool) -> ();
    fn set_parameter_by_id_with_label(id: ParameterID, label: magnus::RString, ignore_seek_speed: bool) -> ();
    fn get_parameter_by_name(name: magnus::RString) -> (f32, f32);
    fn set_parameter_by_name(name: magnus::RString, value: f32, ignore_seek_speed: bool) -> ();
    fn set_parameter_by_name_with_label(name: magnus::RString, label: magnus::RString, ignore_seek_speed: bool) -> ();
    fn get_parameter_description_by_name(name: magnus::RString) -> ParameterDescription;
    fn get_parameter_description_by_id(id: ParameterID) -> ParameterDescription;
    fn parameter_description_count() -> i32;
    fn get_parameter_description_list() -> magnus::r_array::TypedArray<ParameterDescription>;
    fn get_parameter_label_by_name(name: magnus::RString, index: i32) -> magnus::RString;
    fn get_parameter_label_by_id(id: ParameterID, index: i32) -> magnus::RString;
    // TODO plugin
    fn get_buffer_usage() -> BufferUsage;
    fn reset_buffer_usage() -> ();
    fn get_cpu_usage() -> (StudioCPUUsage, CPUUsage);
    fn get_memory_usage() -> MemoryUsage;
  }
}

impl System {
    fn new() -> Result<RbSystem> {
        unsafe { fmod::studio::System::new() }.into_ruby()
    }

    fn release(rb_self: RbSystem) -> Result<()> {
        let system: fmod::studio::System = rb_self.from_ruby()?;
        unsafe { system.release() }.into_ruby()?;
        crate::extern_struct_storage::remove(system);
        crate::extern_struct_storage::cleanup();
        Ok(())
    }

    fn update(rb_self: RbSystem) -> Result<()> {
        let system: fmod::studio::System = rb_self.from_ruby()?;
        system.update().into_ruby()?;
        crate::extern_struct_storage::cleanup();
        Ok(())
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
        let system: fmod::studio::System = rb_self.from_ruby()?;
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

    // have to handwrite this one unfortunately, slice conversion is a bit tricky
    // if set_parameters_by_ids took an AsRef<T> though...
    // FIXME do the above
    fn set_parameter_by_ids(
        rb_self: RbSystem,
        ids: magnus::RArray,
        values: magnus::RArray,
        ignore_seek_speed: bool,
    ) -> Result<()> {
        let system: fmod::studio::System = rb_self.from_ruby()?;
        let ids = ids.typecheck::<ParameterID>()?;
        let ids: Vec<fmod::studio::ParameterID> = ids
            .into_iter()
            .map(FromRuby::from_ruby)
            .collect::<Result<_>>()?;
        let mut values = values.to_vec()?;

        system
            .set_parameters_by_ids(&ids, &mut values, ignore_seek_speed)
            .into_ruby()?;

        Ok(())
    }

    fn get_sound_info(rb_self: RbSystem, key: magnus::RString) -> Result<SoundInfo> {
        let system: fmod::studio::System = rb_self.from_ruby()?;
        let key = key.from_ruby()?;
        // FIXME BAD BAD DANGER
        let info = unsafe { system.get_sound_info(key) }
            .map_err(|e| magnus::Error::new(crate::error::class(), e.to_string()))?;

        info.into_ruby()
    }
}

extern_struct_bind! {
  impl Bindable for System: fmod::studio::System {
    fn load_bank_file -> 2;
    fn load_bank_memory -> 2;
    fn unload_all_banks -> 0;
    fn get_bank -> 1;
    fn get_bank_by_id -> 1;
    fn bank_count -> 0;
    fn get_bank_list -> 0;
    fn get_userdata -> 0;
    fn set_userdata -> 1;
    fn set_callback -> 2;
    fn start_command_capture -> 2;
    fn stop_command_capture -> 0;
    fn load_command_replay -> 2;
    fn get_core_system -> 0;
    fn lookup_id -> 1;
    fn lookup_path -> 1;
    fn is_valid -> 0;
    fn release -> 0;
    fn update -> 0;
    fn flush_commands -> 0;
    fn flush_sample_loading -> 0;
    fn set_listener_attributes -> 3;
    fn get_listener_attributes -> 1;
    fn set_listener_weight -> 2;
    fn get_listener_weight -> 1;
    fn set_listener_count -> 1;
    fn get_listener_count -> 0;
    fn get_bus -> 1;
    fn get_bus_by_id -> 1;
    fn get_vca -> 1;
    fn get_vca_by_id -> 1;
    fn get_advanced_settings -> 0;
    fn get_sound_info -> 1;
    fn get_parameter_by_id -> 1;
    fn set_parameter_by_id -> 3;
    fn set_parameter_by_id_with_label -> 3;
    fn set_parameter_by_ids -> 3;
    fn get_parameter_by_name -> 1;
    fn set_parameter_by_name -> 3;
    fn set_parameter_by_name_with_label -> 3;
    fn get_parameter_description_by_name -> 1;
    fn get_parameter_description_by_id -> 1;
    fn parameter_description_count -> 0;
    fn get_parameter_description_list -> 0;
    fn get_parameter_label_by_name -> 2;
    fn get_parameter_label_by_id -> 2;
    fn get_buffer_usage -> 0;
    fn reset_buffer_usage -> 0;
    fn get_cpu_usage -> 0;
    fn get_memory_usage -> 0;
    ruby_compat_methods: true

    |class| {
      class.define_singleton_method("new", magnus::function!(System::new, 0))?;
    }
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::studio::SystemBuilder::bind(module)?;
    fmod::studio::System::bind(module)?;

    Ok(())
}
