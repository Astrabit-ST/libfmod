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
  struct ChannelGroup: fmod::ChannelGroup => "FMOD::ChannelGroup"
}

extern_struct_fns! {
  impl ChannelGroup {

  }
}

extern_struct_bind! {
  impl Bindable for ChannelGroup: fmod::ChannelGroup {

  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::ChannelGroup::bind(module)?;

    Ok(())
}
