// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]
use crate::{Bindable, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

// TODO subclass channelcontrol
extern_struct! {
  struct Sound: fmod::Sound => "FMOD::Sound"
}

extern_struct_fns! {
  impl Sound {

  }
}

extern_struct_bind! {
  impl Bindable for Sound: fmod::Sound {

  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::Sound::bind(module)?;

    Ok(())
}
