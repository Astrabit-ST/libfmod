// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{Bindable, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

extern_struct! {
    struct Bus: fmod::studio::Bus => "FMOD::Studio::Bus"
}

extern_struct_fns! {
    impl Bus {

    }
}

extern_struct_bind! {
    impl Bindable for Bus: fmod::studio::Bus {

    }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::studio::Bus::bind(module)?;

    Ok(())
}
