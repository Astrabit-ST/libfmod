// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]
use crate::{Bindable, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

extern_struct! {
  struct DspConnection: fmod::DspConnection => "FMOD::DspConnection"
}

extern_struct_fns! {
  impl DspConnection {

  }
}

extern_struct_bind! {
  impl Bindable for DspConnection: fmod::DspConnection {

  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::DspConnection::bind(module)?;

    Ok(())
}
