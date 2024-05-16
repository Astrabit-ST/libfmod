// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::{Bindable, IntoRuby, Result};
use magnus::prelude::*;

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

extern_struct! {
  struct System: fmod::System => "FMOD::System"
}

impl System {
    fn new() -> Result<Self> {
        unsafe { fmod::System::new() }.into_ruby()
    }
}

extern_struct_bind! {
  impl Bindable for System: fmod::System {
    |class| {
      class.define_singleton_method("new", magnus::function!(System::new, 0))?;
    }
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::System::bind(module)?;

    Ok(())
}
