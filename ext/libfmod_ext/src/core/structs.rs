// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::{Bindable, IntoRuby, Result};
use magnus::prelude::*;

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

ruby_struct! {
  struct CPUUsage: fmod::CpuUsage {
    dsp: f32,
    stream: f32,
    geometry: f32,
    update: f32,
    convolution_1: f32,
    convolution_2: f32,
  }
}

ruby_struct! {
  struct ReverbProperties: fmod::ReverbProperties {
    decay_time: f32,
    early_delay: f32,
    late_delay: f32,
    hf_reference: f32,
    hf_decay_ratio: f32,
    diffusion: f32,
    density: f32,
    low_shelf_frequency: f32,
    low_shelf_gain: f32,
    high_cut: f32,
    early_late_mix: f32,
    wet_level: f32,
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::Guid::bind(module)?;
    fmod::Vector::bind(module)?;
    fmod::Attributes3D::bind(module)?;
    fmod::CpuUsage::bind(module)?;
    fmod::ReverbProperties::bind(module)?;

    let class = fmod::ReverbProperties::class();
    let module: magnus::RModule = class.define_module("Presets")?;
    module.const_set("OFF", fmod::ReverbProperties::OFF.into_ruby()?)?;
    module.const_set("GENERIC", fmod::ReverbProperties::GENERIC.into_ruby()?)?;
    module.const_set(
        "PADDEDCELL",
        fmod::ReverbProperties::PADDEDCELL.into_ruby()?,
    )?;
    module.const_set("ROOM", fmod::ReverbProperties::ROOM.into_ruby()?)?;
    module.const_set("BATHROOM", fmod::ReverbProperties::BATHROOM.into_ruby()?)?;
    module.const_set(
        "LIVINGROOM",
        fmod::ReverbProperties::LIVINGROOM.into_ruby()?,
    )?;
    module.const_set("STONEROOM", fmod::ReverbProperties::STONEROOM.into_ruby()?)?;
    module.const_set(
        "AUDITORIUM",
        fmod::ReverbProperties::AUDITORIUM.into_ruby()?,
    )?;
    module.const_set(
        "CONCERT_HALL",
        fmod::ReverbProperties::CONCERTHALL.into_ruby()?,
    )?;
    module.const_set("CAVE", fmod::ReverbProperties::CAVE.into_ruby()?)?;
    module.const_set("ARENA", fmod::ReverbProperties::ARENA.into_ruby()?)?;
    module.const_set("HANGAR", fmod::ReverbProperties::HANGAR.into_ruby()?)?;
    module.const_set(
        "CARPETTED_HALLWAY",
        fmod::ReverbProperties::CARPETTEDHALLWAY.into_ruby()?,
    )?;
    module.const_set("HALLWAY", fmod::ReverbProperties::HALLWAY.into_ruby()?)?;
    module.const_set(
        "STONE_CORRIDOR",
        fmod::ReverbProperties::STONECORRIDOR.into_ruby()?,
    )?;
    module.const_set("ALLEY", fmod::ReverbProperties::ALLEY.into_ruby()?)?;
    module.const_set("FOREST", fmod::ReverbProperties::FOREST.into_ruby()?)?;
    module.const_set("CITY", fmod::ReverbProperties::CITY.into_ruby()?)?;
    module.const_set("MOUNTAINS", fmod::ReverbProperties::MOUNTAINS.into_ruby()?)?;
    module.const_set("QUARRY", fmod::ReverbProperties::QUARRY.into_ruby()?)?;
    module.const_set("PLAIN", fmod::ReverbProperties::PLAIN.into_ruby()?)?;
    module.const_set(
        "PARKING_LOT",
        fmod::ReverbProperties::PARKINGLOT.into_ruby()?,
    )?;
    module.const_set("SEWER_PIPE", fmod::ReverbProperties::SEWERPIPE.into_ruby()?)?;
    module.const_set(
        "UNDERWATER",
        fmod::ReverbProperties::UNDERWATER.into_ruby()?,
    )?;

    Ok(())
}
