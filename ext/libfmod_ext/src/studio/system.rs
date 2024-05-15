// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{core::structs::Guid, Bindable, FromRuby, IntoRuby, Result};
use magnus::prelude::*;
use std::cell::RefCell;

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

use super::{
    bank::Bank,
    flags::{InitFlags as StudioInitFlags, LoadBankFlags},
};

extern_struct! {
    struct System: fmod::studio::System => "FMOD::Studio::System"
}

#[magnus::wrap(class = "FMOD::Studio::SystemBuilder", free_immediately, size)]
pub struct SystemBuilder(RefCell<Option<fmod::studio::SystemBuilder>>);

unsafe impl Send for SystemBuilder {}
unsafe impl Sync for SystemBuilder {}

impl SystemBuilder {
    pub fn new() -> Result<Self> {
        unsafe { fmod::studio::SystemBuilder::new() }.into_ruby()
    }

    pub fn settings(&self, settings: magnus::RStruct) -> Result<()> {
        let advanced_settings = settings.from_ruby()?;
        let mut builder = self.0.borrow_mut();

        let Some(builder) = &mut *builder else {
            let error = magnus::Error::new(
                magnus::exception::runtime_error(),
                "SystemBuilder is already built",
            );
            return Err(error);
        };

        builder.settings(&advanced_settings).map(|_| ()).into_ruby()
    }

    pub fn build(
        &self,
        max_channels: i32,
        studio_flags: StudioInitFlags,
        flags: u32,
    ) -> Result<System> {
        let mut builder = self.0.borrow_mut();

        let Some(builder) = builder.take() else {
            let error = magnus::Error::new(
                magnus::exception::runtime_error(),
                "SystemBuilder is already built",
            );
            return Err(error);
        };

        builder
            .build(max_channels, studio_flags.from_ruby()?, flags.into())
            .into_ruby()
    }
}

impl IntoRuby<SystemBuilder> for fmod::studio::SystemBuilder {
    fn into_ruby(self) -> Result<SystemBuilder> {
        Ok(SystemBuilder(RefCell::new(Some(self))))
    }
}

extern_struct_bind! {
  impl Bindable for SystemBuilder: fmod::studio::SystemBuilder {
    fn settings -> 1;
    fn build -> 3;
    |class| {
      class.define_singleton_method("new", magnus::function!(SystemBuilder::new, 0))?;
    }
  }
}

extern_struct_fns! {
  impl System {
    fn load_bank_file(filename: magnus::RString, flags: LoadBankFlags) -> Bank;
    fn load_bank_memory(buffer: magnus::RString, flags: LoadBankFlags) -> Bank;
    fn unload_all_banks() -> ();
    fn get_bank(path_or_id: magnus::RString) -> Bank;
    fn get_bank_by_id(id: Guid) -> Bank;
    fn bank_count() -> i32;
    fn get_bank_list() -> magnus::r_array::TypedArray<Bank>;
    // TODO userdata & callback
  }
}

impl System {
    fn new() -> Result<Self> {
        unsafe { fmod::studio::System::new() }.into_ruby()
    }
}

extern_struct_bind! {
  impl Bindable for System: fmod::studio::System {
    fn load_bank_file -> 2;
    fn load_bank_memory -> 2;
    fn unload_all_banks -> 0;
    fn get_bank -> 1;
    fn get_bank_by_id -> 1;
    fn bank_count -> 0;
    fn get_bank_list -> 0;
    |class| {
      class.define_singleton_method("new", magnus::function!(System::new, 0))?;
    }
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::studio::SystemBuilder::bind(module)?;
    fmod::studio::System::bind(module)?;

    Ok(())
}
