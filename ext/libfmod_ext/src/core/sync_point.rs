// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]
use crate::{Bindable, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

extern_struct! {
  struct SyncPoint: fmod::SyncPoint => "FMOD::SyncPoint"
}

extern_struct_fns! {
  impl SyncPoint: fmod::SyncPoint {

  }
}

extern_struct_bind! {
  impl Bindable for SyncPoint: fmod::SyncPoint {
    ruby_compat_methods: true
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::SyncPoint::bind(module)?;

    Ok(())
}
