use crate::{IntoRuby, Result};

use crate::{
    extern_struct, extern_struct_bind, extern_struct_fns, num_enum, ruby_bitflags, ruby_struct,
    Bindable,
};

ruby_struct! {
  struct Guid: fmod::Guid {
    data_1: u32,
    data_2: u16,
    data_3: u16,
    data_4: [u8; 8],
  }
}

num_enum! {
    #[repr(u32)]
    enum SpeakerMode: fmod::SpeakerMode {

    }
}

extern_struct!(struct System: fmod::System => "FMOD::System");

extern_struct_fns! {
    impl System {
        fn get_driver_info(id: i32) -> (magnus::RString, Guid, i32, SpeakerMode, i32);
    }
}

impl System {
    fn new() -> Result<Self> {
        let system = unsafe { fmod::System::new() }.into_ruby()?;
        Ok(Self(system))
    }
}

extern_struct_bind! {
    impl Bindable for System: fmod::System {
        fn get_driver_info -> 1;
        |class| {
            use magnus::Object;
            class.define_singleton_method("new", magnus::function!(System::new, 0))?;
        }
    }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::System::bind(module)?;
    fmod::Guid::bind(module)?;

    Ok(())
}
