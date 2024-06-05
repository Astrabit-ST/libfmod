// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::Result;
use magnus::{
    prelude::*,
    value::{InnerValue, Opaque},
};
use once_cell::sync::OnceCell;

static CLASS: OnceCell<Opaque<magnus::ExceptionClass>> = OnceCell::new();

pub fn class() -> magnus::ExceptionClass {
    let ruby = magnus::Ruby::get().unwrap();
    CLASS.get().expect("class not set").get_inner_with(&ruby)
}

pub fn use_after_free(v: impl std::fmt::Debug) -> magnus::Error {
    magnus::Error::new(
        class(),
        format!("Use after free: {v:?} has already been released!"),
    )
}

pub fn from_fmod(error: fmod::Error) -> magnus::Error {
    magnus::Error::new(class(), error.to_string())
}

pub fn bind(module: impl magnus::Module) -> Result<()> {
    let class = module.define_class("Error", magnus::exception::runtime_error().as_r_class())?;
    let exception_class = magnus::ExceptionClass::from_value(class.as_value()).unwrap();
    let _ = CLASS.set(Opaque::from(exception_class));

    Ok(())
}
