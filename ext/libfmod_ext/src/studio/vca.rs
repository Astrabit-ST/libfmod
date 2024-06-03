// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]

use crate::core::structs::Guid;
use crate::{Bindable, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

extern_struct! {
    struct VCA: fmod::studio::Vca => "FMOD::Studio::VCA"
}

extern_struct_fns! {
    impl VCA: fmod::studio::Vca {
      fn set_volume(volume: f32) -> ();
      fn get_volume() -> (f32, f32);
      fn get_id() -> Guid;
      fn get_path() -> magnus::RString;
      fn is_valid() -> bool;
    }
}

extern_struct_bind! {
    impl Bindable for VCA: fmod::studio::Vca {
        fn set_volume -> 1;
        fn get_volume -> 0;
        fn get_id -> 0;
        fn get_path -> 0;
        fn is_valid -> 0;
        ruby_compat_methods: true
    }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::studio::Vca::bind(module)?;

    Ok(())
}
