// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::core::dsp::RbDSP;
use crate::core::sound_builder::SoundBuilder;
use crate::{Bindable, Result};

use super::enums::{InstanceType, ParameterKind};
use super::flags::ParameterFlags;
use crate::core::structs::Guid;

use crate::ruby_struct;

ruby_struct! {
  struct AdvancedSettings: fmod::studio::AdvancedSettings {
    command_queue_size: u32,
    handle_initial_size: u32,
    studioupdateperiod: i32,
    idle_sample_data_pool_size: i32,
    streaming_schedule_delay: u32,
    encryption_key: Option<magnus::RString>,
  }
}

ruby_struct! {
  struct MemoryUsage: fmod::studio::MemoryUsage {
    exclusive: i32,
    inclusive: i32,
    sample_data: i32,
  }
}

ruby_struct! {
  struct ParameterID: fmod::studio::ParameterID {
    data_1: u32,
    data_2: u32,
  }
}

ruby_struct! {
  struct ParameterDescription: fmod::studio::ParameterDescription {
    name: magnus::RString,
    id: ParameterID,
    minimum: f32,
    maximum: f32,
    default_value: f32,
    kind: ParameterKind,
    flags: ParameterFlags,
    guid: Guid,
  }
}

ruby_struct! {
  struct BufferInfo: fmod::studio::BufferInfo {
    current_usage: i32,
    peak_usage: i32,
    capacity: i32,
    stall_count: i32,
    stall_time: f32,
  }
}

ruby_struct! {
  struct BufferUsage: fmod::studio::BufferUsage {
    studio_command_queue: BufferInfo,
    studio_handle: BufferInfo,
  }
}

ruby_struct! {
  struct CPUUsage: fmod::studio::CpuUsage => "StudioCPUUsage" {
    update: f32
  }
}

ruby_struct! {
  struct CommandInfo: fmod::studio::CommandInfo {
    command_name: magnus::RString,
    parent_command_index: i32,
    frame_number: i32,
    frame_time: f32,
    instance_type: InstanceType,
    output_type: InstanceType,
    instance_handle: u32,
    output_handle: u32,
  }
}

ruby_struct! {
  struct SoundInfo: fmod::studio::SoundInfo<'static> {
    builder: &SoundBuilder,
    subsound_index: i32,
  }
}

ruby_struct! {
  struct PluginInstanceProperties: fmod::studio::PluginInstanceProperties {
    name: magnus::RString,
    dsp: RbDSP,
  }
}

ruby_struct! {
  struct TimelineMarkerProperties: fmod::studio::TimelineMarkerProperties {
    name: magnus::RString,
    position: i32,
  }
}

ruby_struct! {
  struct TimelineBeatProperties: fmod::studio::TimelineBeatProperties {
    bar: i32,
    beat: i32,
    position: i32,
    tempo: f32,
    time_signature_upper: i32,
    time_signature_lower: i32,
  }
}

ruby_struct! {
  struct TimelineNestedBeatProperties: fmod::studio::TimelineNestedBeatProperties {
    event_guid: Guid,
    properties: TimelineBeatProperties,
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::studio::AdvancedSettings::bind(module)?;
    fmod::studio::MemoryUsage::bind(module)?;
    fmod::studio::ParameterID::bind(module)?;
    fmod::studio::ParameterDescription::bind(module)?;
    fmod::studio::BufferInfo::bind(module)?;
    fmod::studio::BufferUsage::bind(module)?;
    fmod::studio::CpuUsage::bind(module)?;
    fmod::studio::CommandInfo::bind(module)?;
    fmod::studio::SoundInfo::bind(module)?;
    fmod::studio::PluginInstanceProperties::bind(module)?;
    fmod::studio::TimelineMarkerProperties::bind(module)?;
    fmod::studio::TimelineBeatProperties::bind(module)?;
    fmod::studio::TimelineNestedBeatProperties::bind(module)?;

    Ok(())
}
