use crate::{Result, WrapFMOD};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns, ruby_struct, Bindable};

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

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::Vector::bind(module)?;
    fmod::System::bind(module)?;

    Ok(())
}
