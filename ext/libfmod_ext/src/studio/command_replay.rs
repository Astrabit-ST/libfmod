// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use magnus::prelude::*;

use crate::{Bindable, FromRuby, IntoRuby, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

use super::command_replay_callbacks::{CreateInstanceCallback, FrameCallback, LoadBankCallback};
use super::enums::PlaybackState;
use super::structs::CommandInfo;
use super::system::RbSystem;

extern_struct! {
    struct CommandReplay: fmod::studio::CommandReplay => "FMOD::Studio::CommandReplay"
}

extern_struct_fns! {
    impl CommandReplay: fmod::studio::CommandReplay {
        fn release() -> ();
        fn start() -> ();
        fn stop() -> ();
        fn get_current_command() -> (i32, f32);
        fn get_playback_state() -> PlaybackState;
        fn set_paused(paused: bool) -> ();
        fn get_paused() -> bool;
        fn seek_to_command(index: i32) -> ();
        fn seek_to_time(time: f32) -> ();
        fn set_bank_path(path: magnus::RString) -> ();
        fn command_at_time(time: f32) -> i32;
        fn get_command_count() -> i32;
        fn get_command_info(index: i32) -> CommandInfo;
        fn get_command_string(index: i32) -> magnus::RString;
        fn get_length() -> f32;
        fn get_system() -> RbSystem;
        fn is_valid() -> bool;
    }
}

impl CommandReplay {
    fn get_userdata(rb_self: RbCommandReplay) -> Result<magnus::Value> {
        rb_self.ivar_get("__userdata")
    }

    fn set_userdata(rb_self: RbCommandReplay, data: magnus::Value) -> Result<()> {
        rb_self.ivar_set("__userdata", data)
    }

    fn set_create_instance_callback(
        rb_self: RbCommandReplay,
        callback: magnus::RObject,
    ) -> Result<()> {
        let instance: fmod::studio::CommandReplay = rb_self.from_ruby()?;

        if !callback
            .class()
            .is_inherited(super::command_replay_callbacks::create_instance_class())
        {
            return Err(magnus::Error::new(
                magnus::exception::runtime_error(),
                "callback must be a CreateInstanceCallback",
            ));
        }

        rb_self.ivar_set("__create_instance_callback", callback)?;
        instance
            .set_create_instance_callback::<CreateInstanceCallback>()
            .into_ruby()
    }

    fn set_frame_callback(rb_self: RbCommandReplay, callback: magnus::RObject) -> Result<()> {
        let instance: fmod::studio::CommandReplay = rb_self.from_ruby()?;

        if !callback
            .class()
            .is_inherited(super::command_replay_callbacks::frame_class())
        {
            return Err(magnus::Error::new(
                magnus::exception::runtime_error(),
                "callback must be a FrameCallback",
            ));
        }

        rb_self.ivar_set("__frame_callback", callback)?;
        instance.set_frame_callback::<FrameCallback>().into_ruby()
    }

    fn set_load_bank_callback(rb_self: RbCommandReplay, callback: magnus::RObject) -> Result<()> {
        let instance: fmod::studio::CommandReplay = rb_self.from_ruby()?;

        if !callback
            .class()
            .is_inherited(super::command_replay_callbacks::load_bank_class())
        {
            return Err(magnus::Error::new(
                magnus::exception::runtime_error(),
                "callback must be a LoadBankCallback",
            ));
        }

        rb_self.ivar_set("__load_bank_callback", callback)?;
        instance
            .set_load_bank_callback::<LoadBankCallback>()
            .into_ruby()
    }
}

extern_struct_bind! {
    impl Bindable for CommandReplay: fmod::studio::CommandReplay {
        fn set_create_instance_callback -> 1;
        fn set_frame_callback -> 1;
        fn set_load_bank_callback -> 1;
        fn get_userdata -> 0;
        fn set_userdata -> 1;
        fn release -> 0;
        fn start -> 0;
        fn stop -> 0;
        fn get_current_command -> 0;
        fn get_playback_state -> 0;
        fn set_paused -> 1;
        fn get_paused -> 0;
        fn seek_to_command -> 1;
        fn seek_to_time -> 1;
        fn set_bank_path -> 1;
        fn command_at_time -> 1;
        fn get_command_count -> 0;
        fn get_command_info -> 1;
        fn get_command_string -> 1;
        fn get_length -> 0;
        fn get_system -> 0;
        fn is_valid -> 0;
        ruby_compat_methods: true
    }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::studio::CommandReplay::bind(module)?;

    Ok(())
}
