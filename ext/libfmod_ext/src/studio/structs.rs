// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::{Bindable, Result};

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

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::studio::AdvancedSettings::bind(module)?;
    fmod::studio::MemoryUsage::bind(module)?;

    Ok(())
}
