// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::core::structs::Guid;
use crate::{Bindable, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

use super::enums::StopMode;
use super::structs::MemoryUsage;

extern_struct! {
    struct Bus: fmod::studio::Bus => "FMOD::Studio::Bus"
}

extern_struct_fns! {
    impl Bus {
      fn set_paused(paused: bool) -> ();
      fn get_paused() -> bool;
      fn stop_all_events(stop_mode: StopMode) -> ();
      fn set_volume(volume: f32) -> ();
      fn get_volume() -> (f32, f32);
      fn set_mute(mute: bool) -> ();
      fn get_mute() -> bool;
      fn set_port_index(port_index: u64) -> ();
      fn get_port_index() -> u64;
      // TODO channel group
      fn get_cpu_usage() -> (u32, u32);
      fn get_memory_usage() -> MemoryUsage;
      fn get_id() -> Guid;
      fn get_path() -> String;
      fn is_valid() -> bool;
    }
}

extern_struct_bind! {
    impl Bindable for Bus: fmod::studio::Bus {
        fn set_paused -> 1;
        fn get_paused -> 0;
        fn stop_all_events -> 1;
        fn set_volume -> 1;
        fn get_volume -> 0;
        fn set_mute -> 1;
        fn get_mute -> 0;
        fn set_port_index -> 1;
        fn get_port_index -> 0;
        fn get_cpu_usage -> 0;
        fn get_memory_usage -> 0;
        fn get_id -> 0;
        fn get_path -> 0;
        fn is_valid -> 0;
    }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::studio::Bus::bind(module)?;

    Ok(())
}
