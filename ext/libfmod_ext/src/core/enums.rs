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

num_enum! {
    #[repr(u32)]
    enum DspType: fmod::DspType {
        Unknown,
        Mixer,
        Oscillator,
        Lowpass,
        ItLowpass,
        Highpass,
        Echo,
        Fader,
        Flange,
        Distortion,
        Normalize,
        Limiter,
        ParamEq,
        PitchShift,
        Chorus,
        VstPlugin,
        WinampPlugin,
        ItEcho,
        Compressor,
        SfxReverb,
        LowpassSimple,
        Delay,
        Tremolo,
        LadspaPlugin,
        Send,
        Return,
        HighpassSimple,
        Pan,
        ThreeEq,
        Fft,
        LoudnessMeter,
        EnvelopeFollower,
        ConvolutionReverb,
        ChannelMix,
        ObjectPan,
        MultibandEq,
    }
}

num_enum! {
    #[repr(u32)]
    enum PluginType: fmod::PluginType {
        Output,
        Codec,
        DSP
    }
}

num_enum! {
    #[repr(u32)]
    enum PortType: fmod::PortType {
        Music,
        CopyrightMusic,
        Voice,
        Controller,
        Personal,
        Vibration,
        AUX,
    }
}

num_enum! {
    #[repr(u32)]
    enum TimeUnit: fmod::TimeUnit {
        MS,
        PCM,
        PCMBytes,
        RawBytes,
        PCMFraction,
        ModOrder,
        ModRow,
        ModPattern,
    }
}

num_enum! {
    #[repr(i32)]
    enum Speaker: fmod::Speaker {
        None,
        FrontLeft,
        FrontRight,
        FrontCenter,
        LowFrequency,
        SurroundLeft,
        SurroundRight,
        BackLeft,
        BackRight,
        TopFrontLeft,
        TopFrontRight,
        TopBackLeft,
        TopBackRight,
    }
}

num_enum! {
    #[repr(u32)]
    enum SoundGroupBehavior: fmod::SoundGroupBehavior {
        Fail,
        Mute,
        StealLowest
    }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::SpeakerMode::bind(module)?;
    fmod::OutputType::bind(module)?;
    fmod::DspType::bind(module)?;
    fmod::PluginType::bind(module)?;
    fmod::PortType::bind(module)?;
    fmod::TimeUnit::bind(module)?;
    fmod::Speaker::bind(module)?;
    fmod::SoundGroupBehavior::bind(module)?;

    Ok(())
}
