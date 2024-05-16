// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::{Bindable, Result};

use crate::ruby_struct;

ruby_struct! {
  struct Guid: fmod::Guid {
    data_1: u32,
    data_2: u16,
    data_3: u16,
    data_4: [u8; 8],
  }
}

ruby_struct! {
  struct Vector: fmod::Vector {
    x: f32,
    y: f32,
    z: f32,
  }
}

ruby_struct! {
  struct Attributes3D: fmod::Attributes3D {
    position: Vector,
    velocity: Vector,
    forward: Vector,
    up: Vector,
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::Guid::bind(module)?;
    fmod::Vector::bind(module)?;
    fmod::Attributes3D::bind(module)?;

    Ok(())
}
