use crate::{Result, WrapFMOD};

use crate::{
    extern_struct, extern_struct_bind, extern_struct_fns, num_enum, ruby_bitflags, ruby_struct,
    Bindable,
};

ruby_struct! {
  struct Vector: fmod::Vector {
    x: f32,
    y: f32,
    z: f32,
  }
}

extern_struct!(struct System: fmod::System => "FMOD::System");

extern_struct_fns! {
    impl System {
        fn lock_dsp() -> ();
    }
}

impl System {
    fn new() -> Result<Self> {
        let system = unsafe { fmod::System::new() }.wrap_fmod()?;
        Ok(Self(system))
    }
}

extern_struct_bind! {
    impl Bindable for System: fmod::System {
        fn lock_dsp -> 0;
        |class| {
            use magnus::Object;
            class.define_singleton_method("new", magnus::function!(System::new, 0))?;
        }
    }
}

ruby_bitflags! {
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

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::Vector::bind(module)?;
    fmod::System::bind(module)?;
    fmod::InitFlags::bind(module)?;
    fmod::TimeUnit::bind(module)?;

    Ok(())
}
