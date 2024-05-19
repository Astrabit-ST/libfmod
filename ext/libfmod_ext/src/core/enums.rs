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

pub type OpenState = (u32, Option<magnus::Exception>);

const _: () = {
    static MODULE: once_cell::sync::OnceCell<magnus::value::Opaque<magnus::RModule>> =
        once_cell::sync::OnceCell::new();

    const READY: u32 = fmod::ffi::FMOD_OPENSTATE_READY;
    const LOADING: u32 = fmod::ffi::FMOD_OPENSTATE_LOADING;
    const ERROR: u32 = fmod::ffi::FMOD_OPENSTATE_ERROR;
    const CONNECTING: u32 = fmod::ffi::FMOD_OPENSTATE_CONNECTING;
    const BUFFERING: u32 = fmod::ffi::FMOD_OPENSTATE_BUFFERING;
    const SEEKING: u32 = fmod::ffi::FMOD_OPENSTATE_SEEKING;
    const PLAYING: u32 = fmod::ffi::FMOD_OPENSTATE_PLAYING;
    const SETPOSITION: u32 = fmod::ffi::FMOD_OPENSTATE_SETPOSITION;

    impl IntoRuby<OpenState> for fmod::OpenState {
        fn into_ruby(self) -> Result<OpenState> {
            let tuple: (u32, Option<fmod::Error>) = match self {
                fmod::OpenState::Ready => (READY, None),
                fmod::OpenState::Loading => (LOADING, None),
                fmod::OpenState::Error(error) => (ERROR, Some(error)),
                fmod::OpenState::Connecting => (CONNECTING, None),
                fmod::OpenState::Buffering => (BUFFERING, None),
                fmod::OpenState::Seeking => (SEEKING, None),
                fmod::OpenState::Playing => (PLAYING, None),
                fmod::OpenState::SetPosition => (SETPOSITION, None),
            };
            tuple.into_ruby()
        }
    }

    impl Bindable for fmod::OpenState {
        fn bind(module: impl magnus::Module) -> Result<()> {
            let module = module.define_module("OpenState")?;
            module.const_set("Ready", READY)?;
            module.const_set("Loading", LOADING)?;
            module.const_set("Error", ERROR)?;
            module.const_set("Connecting", CONNECTING)?;
            module.const_set("Buffering", BUFFERING)?;
            module.const_set("Seeking", SEEKING)?;
            module.const_set("Playing", PLAYING)?;
            module.const_set("SetPosition", SETPOSITION)?;

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

num_enum! {
    #[repr(u32)]
    enum TagType: fmod::TagType {
        Unknown,
        ID3V1,
        ID3V2,
        VorbisComment,
        ShoutCast,
        IceCast,
        ASF,
        MIDI,
        Playlist,
        Fmod,
        User,
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
    fmod::OpenState::bind(module)?;
    fmod::TagType::bind(module)?;

    Ok(())
}
