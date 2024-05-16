// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::{Bindable, Result};

use crate::ruby_bitflags;

ruby_bitflags! {
  #[repr(u32)]
  mod InitFlags: fmod::studio::InitFlags {
    const NORMAL;
    const LIVEUPDATE;
    const ALLOW_MISSING_PLUGINS;
    const SYNCHRONOUS_UPDATE;
    const DEFERRED_CALLBACKS;
    const LOAD_FROM_UPDATE;
    const MEMORY_TRACKING;
  }
}

ruby_bitflags! {
  #[repr(u32)]
  mod LoadBankFlags: fmod::studio::LoadBankFlags {
    const NORMAL;
    const NONBLOCKING;
    const DECOMPRESS_SAMPLES;
    const UNENCRYPTED;
  }
}

ruby_bitflags! {
  #[repr(u32)]
  mod ParameterFlags: fmod::studio::ParameterFlags {
    const READONLY;
    const AUTOMATIC;
    const GLOBAL;
    const DISCRETE;
    const LABELED;
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::studio::InitFlags::bind(module)?;
    fmod::studio::LoadBankFlags::bind(module)?;
    fmod::studio::ParameterFlags::bind(module)?;

    Ok(())
}
