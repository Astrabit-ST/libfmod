// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use magnus::r_array::TypedArray;

use crate::{core::structs::Guid, Bindable, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

use super::bus::Bus;
use super::event_description::EventDescription;
use super::vca::VCA;

extern_struct! {
    struct Bank: fmod::studio::Bank => "FMOD::Studio::Bank"
}

extern_struct_fns! {
    impl Bank: fmod::studio::Bank {
        fn get_id() -> Guid;
        fn get_path() -> magnus::RString;
        fn is_valid() -> bool;
        // TODO userdata
        // TODO loading state
        fn load_sample_data() -> ();
        fn unload_sample_data() -> ();
        // sample loading state
        fn unload() -> ();
        fn bus_count() -> i32;
        fn get_bus_list() -> TypedArray<Bus>;
        fn event_count() -> i32;
        fn get_event_list() -> TypedArray<EventDescription>;
        fn string_count() -> i32;
        fn get_string_info(index: i32) -> (Guid, magnus::RString);
        fn vca_count() -> i32;
        fn get_vca_list() -> TypedArray<VCA>;
    }
}

extern_struct_bind! {
    impl Bindable for Bank: fmod::studio::Bank {
        fn get_id -> 0;
        fn get_path -> 0;
        fn is_valid -> 0;
        fn load_sample_data -> 0;
        fn unload_sample_data -> 0;
        fn unload -> 0;
        fn bus_count -> 0;
        fn get_bus_list -> 0;
        fn event_count -> 0;
        fn get_event_list -> 0;
        fn string_count -> 0;
        fn get_string_info -> 1;
        fn vca_count -> 0;
        fn get_vca_list -> 0;
    }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::studio::Bank::bind(module)?;

    Ok(())
}
