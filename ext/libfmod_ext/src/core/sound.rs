// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]
use magnus::prelude::*;

use crate::{thread, Bindable, FromRuby, IntoRuby, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

use super::enums::{OpenState, TimeUnit};
use super::flags::Mode;
use super::sound_group::RbSoundGroup;
use super::structs::{Tag, Vector};
use super::sync_point::RbSyncPoint;
use super::system::RbSystem;

extern_struct! {
  struct Sound: fmod::Sound => "FMOD::Sound"
}

impl Sound {
    fn release(rb_self: RbSound) -> Result<()> {
        // we dont need to check if the sound is already removed, because FromRuby will return an error if it is
        let sound: fmod::Sound = rb_self.from_ruby()?;

        let mut index = 0;
        while let Ok(point) = sound.get_sync_point(index) {
            crate::extern_struct_storage::remove(point);
            index += 1;
        }

        crate::extern_struct_storage::remove(sound);
        unsafe { thread::without_gvl_no_ubf(|| sound.release()) }.into_ruby()
    }

    fn get_sub_sound(rb_self: RbSound, index: i32) -> Result<RbSound> {
        let sound = rb_self.from_ruby()?;
        unsafe { thread::without_gvl_no_ubf(|| sound.get_sub_sound(index)) }.into_ruby()
    }

    fn delete_sync_point(rb_self: RbSound, point: RbSyncPoint) -> Result<()> {
        let sound: fmod::Sound = rb_self.from_ruby()?;
        let point = point.from_ruby()?;

        crate::extern_struct_storage::remove(point);
        sound.delete_sync_point(point).into_ruby()
    }

    fn get_userdata(rb_self: RbSound) -> Result<magnus::Value> {
        rb_self.ivar_get("__userdata")
    }

    fn set_userdata(rb_self: RbSound, data: magnus::Value) -> Result<()> {
        rb_self.ivar_set("__userdata", data)
    }
}

extern_struct_fns! {
  impl Sound: fmod::Sound {
    fn get_open_state() -> (OpenState, u32, bool, bool);
    fn set_3d_cone_settings(inside: f32, outside: f32, volume: f32) -> ();
    fn get_3d_cone_settings() -> (f32, f32, f32);
    // TODO 3d custom rolloff
    fn get_3d_custom_rolloff() -> magnus::r_array::TypedArray<Vector>;
    fn set_3d_min_max_distance(min: f32, max: f32) -> ();
    fn get_3d_min_max_distance() -> (f32, f32);
    fn set_defaults(frequency: f32, priority: i32) -> ();
    fn get_defaults() -> (f32, i32);
    fn get_mode() -> Mode;
    fn set_loop_count(count: i32) -> ();
    fn get_loop_count() -> i32;
    fn set_loop_points(start: u32, start_type: TimeUnit, end: u32, end_type: TimeUnit) -> ();
    fn get_loop_points(start: TimeUnit, end: TimeUnit) -> (u32, u32);
    fn get_system() -> RbSystem;
    fn get_name() -> magnus::RString;
    fn get_length(time_unit: TimeUnit) -> u32;
    fn get_tag_count() -> (i32, i32);
    fn get_tag(name: Option<magnus::RString>, index: i32) -> Tag;
    fn get_music_channel_count() -> i32;
    fn set_music_channel_volume(channel: i32, volume: f32) -> ();
    fn get_music_channel_volume(channel: i32) -> f32;
    fn set_music_speed(speed: f32) -> ();
    fn get_music_speed() -> f32;
    fn set_sound_group(group: RbSoundGroup) -> ();
    fn sound_group() -> RbSoundGroup;
    fn get_sub_sound_count() -> i32;
    fn get_sub_sound_parent() -> Option<RbSound>;
    fn get_sync_point(index: i32) -> RbSyncPoint;
    fn get_sync_point_info(point: RbSyncPoint, unit: TimeUnit) -> (magnus::RString, u32);
    fn add_sync_point(offset: u32, unit: TimeUnit, name: magnus::RString) -> RbSyncPoint;
  }
}

extern_struct_bind! {
  impl Bindable for Sound: fmod::Sound {
    fn get_open_state -> 0;
    fn set_3d_cone_settings -> 3;
    fn get_3d_cone_settings -> 0;
    fn get_3d_custom_rolloff -> 0;
    fn set_3d_min_max_distance -> 2;
    fn get_3d_min_max_distance -> 0;
    fn set_defaults -> 2;
    fn get_defaults -> 0;
    fn get_mode -> 0;
    fn set_loop_count -> 1;
    fn get_loop_count -> 0;
    fn set_loop_points -> 4;
    fn get_loop_points -> 2;
    fn get_userdata -> 0;
    fn set_userdata -> 1;
    fn release -> 0;
    fn get_system -> 0;
    fn get_name -> 0;
    fn get_length -> 1;
    fn get_tag_count -> 0;
    fn get_tag -> 2;
    fn get_music_channel_count -> 0;
    fn set_music_channel_volume -> 2;
    fn get_music_channel_volume -> 1;
    fn set_music_speed -> 1;
    fn get_music_speed -> 0;
    fn set_sound_group -> 1;
    fn sound_group -> 0;
    fn get_sub_sound_count -> 0;
    fn get_sub_sound -> 1;
    fn get_sub_sound_parent -> 0;
    fn get_sync_point -> 1;
    fn get_sync_point_info -> 2;
    fn add_sync_point -> 3;
    fn delete_sync_point -> 1;
    ruby_compat_methods: true
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::Sound::bind(module)?;

    Ok(())
}
