// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::{Bindable, IntoRuby, Result};
use magnus::{prelude::*, value::InnerValue};

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

num_enum! {
    #[repr(u32)]
    enum InstanceType: fmod::studio::InstanceType {
        None,
        System,
        EventDescription,
        EventInstance,
        ParameterInstance,
        Bus,
        Vca,
        Bank,
        CommandReplay,
    }
}

pub type LoadingState = (u32, Option<magnus::Exception>);

const _: () = {
    static MODULE: once_cell::sync::OnceCell<magnus::value::Opaque<magnus::RModule>> =
        once_cell::sync::OnceCell::new();

    const LOADING: u32 = fmod::ffi::FMOD_STUDIO_LOADING_STATE_LOADING;
    const LOADED: u32 = fmod::ffi::FMOD_STUDIO_LOADING_STATE_LOADED;
    const UNLOADED: u32 = fmod::ffi::FMOD_STUDIO_LOADING_STATE_UNLOADED;
    const UNLOADING: u32 = fmod::ffi::FMOD_STUDIO_LOADING_STATE_UNLOADING;
    const ERROR: u32 = fmod::ffi::FMOD_STUDIO_LOADING_STATE_ERROR;

    impl IntoRuby<LoadingState> for fmod::studio::LoadingState {
        fn into_ruby(self) -> Result<LoadingState> {
            let tuple: (u32, Option<fmod::Error>) = match self {
                fmod::studio::LoadingState::Unloading => (UNLOADING, None),
                fmod::studio::LoadingState::Unloaded => (UNLOADED, None),
                fmod::studio::LoadingState::Loading => (LOADING, None),
                fmod::studio::LoadingState::Loaded => (LOADED, None),
                fmod::studio::LoadingState::Error(error) => (ERROR, Some(error)),
            };
            tuple.into_ruby()
        }
    }

    impl Bindable for fmod::studio::LoadingState {
        fn bind(module: impl magnus::Module) -> Result<()> {
            let module = module.define_module("LoadingState")?;
            module.const_set("Unloading", UNLOADING)?;
            module.const_set("Unloaded", UNLOADED)?;
            module.const_set("Loading", LOADING)?;
            module.const_set("Loaded", LOADED)?;
            module.const_set("Error", ERROR)?;

            let _ = MODULE.set(module.into());

            Ok(())
        }

        #[allow(refining_impl_trait)]
        fn class() -> magnus::RModule {
            let ruby = magnus::Ruby::get().unwrap();
            MODULE.get().unwrap().get_inner_with(&ruby)
        }
    }
};

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::studio::StopMode::bind(module)?;
    fmod::studio::ParameterKind::bind(module)?;
    fmod::studio::PlaybackState::bind(module)?;
    fmod::studio::EventProperty::bind(module)?;
    fmod::studio::InstanceType::bind(module)?;
    fmod::studio::LoadingState::bind(module)?;

    Ok(())
}
