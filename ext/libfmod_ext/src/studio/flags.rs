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

ruby_bitflags! {
  #[repr(u32)]
  mod CommandCaptureFlags: fmod::studio::CommandCaptureFlags {
    const NORMAL;
    const FILE_FLUSH;
    const SKIP_INITIAL_STATE;
  }
}

ruby_bitflags! {
  #[repr(u32)]
  mod CommandReplayFlags: fmod::studio::CommandReplayFlags {
    const NORMAL;
    const SKIP_CLEANUP;
    const FAST_FORWARD;
    const SKIP_BANK_LOAD;
  }
}

ruby_bitflags! {
  #[repr(u32)]
  mod SystemCallbackMask: fmod::studio::SystemCallbackMask {
    const PREUPDATE;
    const POSTUPDATE;
    const BANK_UNLOAD;
    const LIVEUPDATE_CONNECTED;
    const LIVEUPDATE_DISCONNECTED;
  }
}

ruby_bitflags! {
  #[repr(u32)]
  mod EventCallbackMask: fmod::studio::EventCallbackMask {
    const CREATED;
    const DESTROYED;
    const STARTING;
    const STARTED;
    const STOPPED;
    const START_FAILED;
    const CREATE_PROGRAMMER_SOUND;
    const DESTROY_PROGRAMMER_SOUND;
    const PLUGIN_CREATED;
    const PLUGIN_DESTROYED;
    const TIMELINE_MARKER;
    const TIMELINE_BEAT;
    const SOUND_PLAYED;
    const SOUND_STOPPED;
    const REAL_TO_VIRTUAL;
    const VIRTUAL_TO_REAL;
    const START_EVENT_COMMAND;
    const NESTED_TIMELINE_BEAT;
    const ALL;
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::studio::InitFlags::bind(module)?;
    fmod::studio::LoadBankFlags::bind(module)?;
    fmod::studio::ParameterFlags::bind(module)?;
    fmod::studio::CommandCaptureFlags::bind(module)?;
    fmod::studio::CommandReplayFlags::bind(module)?;
    fmod::studio::SystemCallbackMask::bind(module)?;
    fmod::studio::EventCallbackMask::bind(module)?;

    Ok(())
}
