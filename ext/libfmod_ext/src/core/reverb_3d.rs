// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]
use crate::{Bindable, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

use super::structs::{ReverbProperties, Vector};

extern_struct! {
  struct Reverb3D: fmod::Reverb3D => "FMOD::Reverb3D"
}

extern_struct_fns! {
  impl Reverb3D {
    fn set_3d_attributes(position: Option<Vector>, min_distance: f32, max_distance: f32) -> ();
    fn get_3d_attributes() -> (Vector, f32, f32);
    fn set_properties(properties: ReverbProperties) -> ();
    fn get_properties() -> ReverbProperties;
    fn set_active(active: bool) -> ();
    fn get_active() -> bool;
    // TODO userdata
    fn release() -> ();
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
    fn release -> 0;
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::Reverb3D::bind(module)?;

    Ok(())
}