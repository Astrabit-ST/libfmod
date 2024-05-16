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

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::InitFlags::bind(module)?;

    Ok(())
}
