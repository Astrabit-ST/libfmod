// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::{Bindable, Result};

use crate::ruby_bitflags;

ruby_bitflags! {
    #[repr(u32)]
    mod InitFlags: fmod::InitFlags {
        const NORMAL;
        const STREAM_FROM_UPDATE;
        const MIX_FROM_UPDATE;
        const RIGHTHANDED_3D;
        const CLIP_OUTPUT;
        const CHANNEL_LOWPASS;
        const CHANNEL_DISTANCE_FILTER;
        const PROFILE_ENABLE;
        const VOL_0_BECOMES_VIRTUAL;
        const GEOMETRY_USE_CLOSEST;
        const PREFER_DOLBY_DOWNMIX;
        const THREAD_UNSAFE;
        const PROFILE_METER_ALL;
        const MEMORY_TRACKING;
    }
}

ruby_bitflags! {
    #[repr(u32)]
    mod DriverState: fmod::DriverState {
        const DEFAULT;
        const CONNECTED;
    }
}

ruby_bitflags! {
    #[repr(u32)]
    mod ChannelMask: fmod::ChannelMask {
        const FRONT_LEFT;
        const FRONT_RIGHT;
        const FRONT_CENTER;
        const LOW_FREQUENCY;
        const SURROUND_LEFT;
        const SURROUND_RIGHT;
        const BACK_LEFT;
        const BACK_RIGHT;
        const BACK_CENTER;
        const MONO;
        const STEREO;
        const LRC;
        const QUAD;
        const SURROUND;
        const _5POINT1;
        const _5POINT1_REARS;
        const _7POINT0;
        const _7POINT1;
    }
}

ruby_bitflags! {
  #[repr(u32)]
  mod Mode: fmod::Mode {
    const DEFAULT;
    const LOOP_OFF;
    const LOOP_NORMAL;
    const LOOP_BIDI;
    const D2;
    const D3;
    const CREATE_STREAM;
    const CREATE_SAMPLE;
    const CREATE_COMPRESSED_SAMPLE;
    const OPEN_USER;
    const OPEN_MEMORY;
    const OPEN_MEMORY_POINT;
    const OPEN_RAW;
    const OPEN_ONLY;
    const ACCURATE_TIME;
    const MPEG_SEARCH;
    const NONBLOCKING;
    const UNIQUE;
    const HEADRELATIVE_3D;
    const WORLDRELATIVE_3D;
    const INVERSE_ROLLOFF_3D;
    const LINEAR_ROLLOFF_3D;
    const LINEAR_SQUARE_ROLLOFF_3D;
    const INVERSE_TAPERED_ROLLOFF_3D;
    const CUSTOM_ROLLOFF_3D;
    const IGNORE_GEOMETRY_3D;
    const IGNORE_TAGS;
    const LOWMEM;
    const VIRTUAL_PLAYFROM_START;
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::InitFlags::bind(module)?;
    fmod::DriverState::bind(module)?;
    fmod::ChannelMask::bind(module)?;
    fmod::Mode::bind(module)?;

    Ok(())
}
