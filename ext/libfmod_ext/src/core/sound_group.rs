// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]
use crate::{Bindable, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

use super::enums::SoundGroupBehavior;
use super::sound::Sound;
use super::system::System;

extern_struct! {
  struct SoundGroup: fmod::SoundGroup => "FMOD::SoundGroup"
}

extern_struct_fns! {
  impl SoundGroup: fmod::SoundGroup {
    fn get_name() -> magnus::RString;
    fn release() -> ();
    // TODO userdata
    fn get_system() -> System;
    fn set_max_audible(max_audible: i32) -> ();
    fn get_max_audible() -> i32;
    fn set_max_audible_behavior(behavior: SoundGroupBehavior) -> ();
    fn get_max_audible_behavior() -> SoundGroupBehavior;
    fn set_mute_fade_speed(speed: f32) -> ();
    fn get_mute_fade_speed() -> f32;
    fn set_volume(volume: f32) -> ();
    fn get_volume() -> f32;
    fn get_sound_count() -> i32;
    fn get_sound(index: i32) -> Sound;
    fn get_playing_count() -> i32;
    fn stop() -> ();
  }
}

extern_struct_bind! {
  impl Bindable for SoundGroup: fmod::SoundGroup {
    fn get_name -> 0;
    fn release -> 0;
    fn get_system -> 0;
    fn set_max_audible -> 1;
    fn get_max_audible -> 0;
    fn set_max_audible_behavior -> 1;
    fn get_max_audible_behavior -> 0;
    fn set_mute_fade_speed -> 1;
    fn get_mute_fade_speed -> 0;
    fn set_volume -> 1;
    fn get_volume -> 0;
    fn get_sound_count -> 0;
    fn get_sound -> 1;
    fn get_playing_count -> 0;
    fn stop -> 0;
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::SoundGroup::bind(module)?;

    Ok(())
}
