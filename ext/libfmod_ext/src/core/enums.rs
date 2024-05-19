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

num_enum! {
    #[repr(u32)]
    enum DspConnectionType: fmod::DspConnectionType {
        Standard,
        Sidechain,
        Send,
        SendSidechain,
    }
}

num_enum! {
    #[repr(u32)]
    enum SoundFormat: fmod::SoundFormat {
        None,
        PCM8,
        PCM16,
        PCM24,
        PCM32,
        PCMFloat,
        BitStream
    }
}

num_enum! {
    #[repr(u32)]
    enum ChannelOrder: fmod::ChannelOrder {
        Default,
        WaveFormat,
        ProTools,
        AllMono,
        AllStereo,
        Alsa
    }
}

num_enum! {
    #[repr(u32)]
    enum SoundType: fmod::SoundType {
        Unknown,
        AIFF,
        ASF,
        DLS,
        FLAC,
        FSB,
        IT,
        MIDI,
        MOD,
        MPEG,
        OGGVORBIS,
        Playlist,
        RAW,
        S3M,
        User,
        WAV,
        XM,
        XMA,
        AudioQueue,
        AT9,
        Vorbis,
        MediaFoundation,
        MediaCodec,
        FADPCM,
        OPUS,
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
    fmod::DspConnectionType::bind(module)?;
    fmod::SoundFormat::bind(module)?;
    fmod::ChannelOrder::bind(module)?;
    fmod::SoundType::bind(module)?;

    Ok(())
}
