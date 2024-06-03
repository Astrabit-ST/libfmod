// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::core::structs::Guid;
use crate::{Bindable, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

use super::enums::LoadingState;
use super::event_instance::RbEventInstance;
use super::structs::{ParameterDescription, ParameterID};

extern_struct! {
    struct EventDescription: fmod::studio::EventDescription => "FMOD::Studio::EventDescription"
}

extern_struct_fns! {
    impl EventDescription: fmod::studio::EventDescription {
        fn is_3d() -> bool;
        fn is_doppler_enabled() -> bool;
        fn is_oneshot() -> bool;
        fn is_snapshot() -> bool;
        fn is_stream() -> bool;
        fn has_sustain_point() -> bool;
        fn get_min_max_distance() -> (f32, f32);
        fn get_sound_size() -> f32;
        // TODO userdata & callbacks
        fn get_id() -> Guid;
        fn get_length() -> i32;
        fn get_path() -> magnus::RString;
        fn is_valid() -> bool;
        fn create_instance() -> RbEventInstance;
        fn instance_count() -> i32;
        fn get_instance_list() -> magnus::r_array::TypedArray<RbEventInstance>;
        fn release_all_instances() -> ();
        fn get_parameter_description_by_name(name: magnus::RString) -> ParameterDescription;
        fn get_parameter_description_by_id(id: ParameterID) -> ParameterDescription;
        fn get_parameter_description_by_index(index: i32) -> ParameterDescription;
        fn parameter_description_count() -> i32;
        fn get_parameter_label_by_name(name: magnus::RString, label_index: i32) -> magnus::RString;
        fn get_parameter_label_by_id(id: ParameterID, label_index: i32) -> magnus::RString;
        fn get_parameter_label_by_index(index: i32, label_index: i32) -> magnus::RString;
        fn load_sample_data() -> ();
        fn unload_sample_data() -> ();
        fn get_sample_loading_state() -> LoadingState;
        // TODO user properties
    }
}

extern_struct_bind! {
    impl Bindable for EventDescription: fmod::studio::EventDescription {
        fn is_3d -> 0;
        fn is_doppler_enabled -> 0;
        fn is_oneshot -> 0;
        fn is_snapshot -> 0;
        fn is_stream -> 0;
        fn has_sustain_point -> 0;
        fn get_min_max_distance -> 0;
        fn get_sound_size -> 0;
        fn get_id -> 0;
        fn get_length -> 0;
        fn get_path -> 0;
        fn is_valid -> 0;
        fn create_instance -> 0;
        fn instance_count -> 0;
        fn get_instance_list -> 0;
        fn release_all_instances -> 0;
        fn get_parameter_description_by_name -> 1;
        fn get_parameter_description_by_id -> 1;
        fn get_parameter_description_by_index -> 1;
        fn parameter_description_count -> 0;
        fn get_parameter_label_by_name -> 2;
        fn get_parameter_label_by_id -> 2;
        fn get_parameter_label_by_index -> 2;
        fn load_sample_data -> 0;
        fn unload_sample_data -> 0;
        fn get_sample_loading_state -> 0;
        ruby_compat_methods: true
    }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::studio::EventDescription::bind(module)?;

    Ok(())
}
