// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]
use crate::{Bindable, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

use super::dsp::DSP;
use super::enums::DspConnectionType;

extern_struct! {
  struct DSPConnection: fmod::DspConnection => "FMOD::DSPConnection"
}

extern_struct_fns! {
  impl DSPConnection {
    fn get_input() -> DSP;
    fn get_output() -> DSP;
    fn get_type() -> DspConnectionType;
    // TODO userdata
    fn set_mix(mix: f32) -> ();
    fn get_mix() -> f32;
  }
}

extern_struct_bind! {
  impl Bindable for DSPConnection: fmod::DspConnection {
    fn get_input -> 0;
    fn get_output -> 0;
    fn get_type -> 0;
    fn set_mix -> 1;
    fn get_mix -> 0;
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::DspConnection::bind(module)?;

    Ok(())
}
