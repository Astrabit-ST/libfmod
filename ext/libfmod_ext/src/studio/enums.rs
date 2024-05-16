// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::{Bindable, Result};

use crate::num_enum;

num_enum! {
    #[repr(u32)]
    enum StopMode: fmod::studio::StopMode {
        AllowFadeout,
        Immediate,
    }
}

num_enum! {
    #[repr(u32)]
    enum ParameterKind: fmod::studio::ParameterKind {
        GameControlled,
        AutomaticDistance,
        AutomaticEventConeAngle,
        AutomaticEventOrientation,
        AutomaticDirection,
        AutomaticElevation,
        AutomaticListenerOrientation,
        AutomaticSpeed,
        AutomaticSpeedAbsolute ,
        AutomaticDistanceNormalized,
    }
}

num_enum! {
    #[repr(u32)]
    enum PlaybackState: fmod::studio::PlaybackState {
        Playing,
        Sustaining,
        Stopped,
        Starting,
        Stopping,
    }
}

num_enum! {
    #[repr(u32)]
    enum EventProperty: fmod::studio::EventProperty {
        ChannelPriority,
        ScheduleDelay,
        ScheduleLookahead,
        MinimumDistance,
        MaximumDistance,
        Cooldown,
    }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::studio::StopMode::bind(module)?;
    fmod::studio::ParameterKind::bind(module)?;
    fmod::studio::PlaybackState::bind(module)?;
    fmod::studio::EventProperty::bind(module)?;

    Ok(())
}
