// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]
use magnus::prelude::*;

use crate::{Bindable, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

use super::dsp::RbDSP;
use super::enums::DspConnectionType;

extern_struct! {
  struct DSPConnection: fmod::DspConnection => "FMOD::DSPConnection"
}

extern_struct_fns! {
  impl DSPConnection: fmod::DspConnection {
    fn get_input() -> RbDSP;
    fn get_output() -> RbDSP;
    fn get_type() -> DspConnectionType;
    fn set_mix(mix: f32) -> ();
    fn get_mix() -> f32;
  }
}

impl DSPConnection {
    fn get_userdata(rb_self: RbDSPConnection) -> Result<magnus::Value> {
        rb_self.ivar_get("__userdata")
    }

    fn set_userdata(rb_self: RbDSPConnection, data: magnus::Value) -> Result<()> {
        rb_self.ivar_set("__userdata", data)
    }
}

extern_struct_bind! {
  impl Bindable for DSPConnection: fmod::DspConnection {
    fn get_input -> 0;
    fn get_output -> 0;
    fn get_type -> 0;
    fn get_userdata -> 0;
    fn set_userdata -> 1;
    fn set_mix -> 1;
    fn get_mix -> 0;
    ruby_compat_methods: true
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::DspConnection::bind(module)?;

    Ok(())
}
