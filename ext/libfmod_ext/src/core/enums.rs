// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::{Bindable, Result};

use crate::num_enum;

num_enum! {
    #[repr(u32)]
    enum SpeakerMode: fmod::SpeakerMode {
        Default,
        Raw,
        Mono,
        Stereo,
        Quad,
        Surround,
        FivePointOne,
        SevenPointOne,
        SevenPointOneFour,
    }
}

num_enum! {
    #[repr(u32)]
    enum OutputType: fmod::OutputType {
        AutoDetect,
        Unknown,
        NoSound,
        WavWriter,
        NoSoundNRT,
        WavWriterNRT,
        WASAPI,
        ASIO,
        PulseAudio,
        Alsa,
        CoreAudio,
        AudioTrack,
        OpenSL,
        AudioOut,
        WebAudio,
        NNAudio,
        WinSonic,
        AAudio,
        AudioWorklet,
        Phase,
        OHAudio,
    }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::SpeakerMode::bind(module)?;
    fmod::OutputType::bind(module)?;

    Ok(())
}
