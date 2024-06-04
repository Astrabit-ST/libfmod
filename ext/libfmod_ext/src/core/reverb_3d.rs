// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]
use magnus::prelude::*;

use crate::{Bindable, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

use super::structs::{ReverbProperties, Vector};

extern_struct! {
  struct Reverb3D: fmod::Reverb3D => "FMOD::Reverb3D"
}

impl Reverb3D {
    fn release(&self) -> Result<()> {
        use crate::{FromRuby, IntoRuby};
        // we dont need to check if the reverb is already removed, because FromRuby will return an error if it is
        let reverb: fmod::Reverb3D = self.from_ruby()?;
        crate::extern_struct_storage::remove(reverb);
        reverb.release().into_ruby()
    }

    fn get_userdata(rb_self: RbReverb3D) -> Result<magnus::Value> {
        rb_self.ivar_get("__userdata")
    }

    fn set_userdata(rb_self: RbReverb3D, data: magnus::Value) -> Result<()> {
        rb_self.ivar_set("__userdata", data)
    }
}

extern_struct_fns! {
  impl Reverb3D: fmod::Reverb3D {
    fn set_3d_attributes(position: Option<Vector>, min_distance: f32, max_distance: f32) -> ();
    fn get_3d_attributes() -> (Vector, f32, f32);
    fn set_properties(properties: ReverbProperties) -> ();
    fn get_properties() -> ReverbProperties;
    fn set_active(active: bool) -> ();
    fn get_active() -> bool;
  }
}

extern_struct_bind! {
  impl Bindable for Reverb3D: fmod::Reverb3D {
    fn set_3d_attributes -> 3;
    fn get_3d_attributes -> 0;
    fn set_properties -> 1;
    fn get_properties -> 0;
    fn set_active -> 1;
    fn get_active -> 0;
    fn get_userdata -> 0;
    fn set_userdata -> 1;
    fn release -> 0;
    ruby_compat_methods: true
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::Reverb3D::bind(module)?;

    Ok(())
}
