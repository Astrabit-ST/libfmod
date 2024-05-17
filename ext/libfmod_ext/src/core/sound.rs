// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]
use crate::{Bindable, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

use super::enums::TimeUnit;
use super::sound_group::SoundGroup;
use super::structs::Vector;
use super::system::System;

extern_struct! {
  struct Sound: fmod::Sound => "FMOD::Sound"
}

extern_struct_fns! {
  impl Sound {
    // TODO openstate
    fn set_3d_cone_settings(inside: f32, outside: f32, volume: f32) -> ();
    fn get_3d_cone_settings() -> (f32, f32, f32);
    // TODO 3d custom rolloff
    fn get_3d_custom_rolloff() -> magnus::r_array::TypedArray<Vector>;
    fn set_3d_min_max_distance(min: f32, max: f32) -> ();
    fn get_3d_min_max_distance() -> (f32, f32);
    fn set_defaults(frequency: f32, priority: i32) -> ();
    fn get_defaults() -> (f32, i32);
    // TODO mode
    fn set_loop_count(count: i32) -> ();
    fn get_loop_count() -> i32;
    fn set_loop_points(start: u32, start_type: TimeUnit, end: u32, end_type: TimeUnit) -> ();
    fn get_loop_points(start: TimeUnit, end: TimeUnit) -> (u32, u32);
    fn release() -> ();
    // TODO userdata
    fn get_system() -> System;
    fn get_name() -> magnus::RString;
    fn get_length(time_unit: TimeUnit) -> u32;
    fn get_tag_count() -> (i32, i32);
    // TODO tag
    fn get_music_channel_count() -> i32;
    fn set_music_channel_volume(channel: i32, volume: f32) -> ();
    fn get_music_channel_volume(channel: i32) -> f32;
    fn set_music_speed(speed: f32) -> ();
    fn get_music_speed() -> f32;
    fn set_sound_group(group: &SoundGroup) -> ();
    fn sound_group() -> SoundGroup;
    fn get_sub_sound_count() -> i32;
    fn get_sub_sound(index: i32) -> Sound;
    fn get_sub_sound_parent() -> Option<Sound>;
    // TODO sync point
  }
}

extern_struct_bind! {
  impl Bindable for Sound: fmod::Sound {
    fn set_3d_cone_settings -> 3;
    fn get_3d_cone_settings -> 0;
    fn get_3d_custom_rolloff -> 0;
    fn set_3d_min_max_distance -> 2;
    fn get_3d_min_max_distance -> 0;
    fn set_defaults -> 2;
    fn get_defaults -> 0;
    fn set_loop_count -> 1;
    fn get_loop_count -> 0;
    fn set_loop_points -> 4;
    fn get_loop_points -> 2;
    fn release -> 0;
    fn get_system -> 0;
    fn get_name -> 0;
    fn get_length -> 1;
    fn get_tag_count -> 0;
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
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::Sound::bind(module)?;

    Ok(())
}