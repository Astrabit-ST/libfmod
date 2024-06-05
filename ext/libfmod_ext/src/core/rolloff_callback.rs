// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::{callback, IntoRuby, Result};
use magnus::{
    prelude::*,
    value::{InnerValue, Opaque},
};
use once_cell::sync::OnceCell;

use super::{channel_control::RbChannelControl, system::RbSystem};

static CLASS: OnceCell<Opaque<magnus::RClass>> = OnceCell::new();

pub struct RolloffCallback;

impl fmod::RolloffCallback for RolloffCallback {
    fn rolloff(
        channel_control: fmod::ChannelControl,
        distance: std::ffi::c_float,
    ) -> std::ffi::c_float {
        let (sender, reciever) = oneshot::channel::<std::ffi::c_float>();
        callback::process(move |_| {
            let channel_control = channel_control.into_ruby().unwrap();
            let system: RbSystem = channel_control.funcall("get_system", ()).unwrap();
            let callback: magnus::RObject = system.ivar_get("__rolloff_callback").unwrap();
            let result: f32 = callback
                .funcall("rolloff", (channel_control, distance))
                .unwrap();
            sender.send(result).unwrap();
        });
        reciever.recv().unwrap()
    }
}

pub fn class() -> magnus::RClass {
    let ruby = magnus::Ruby::get().unwrap();
    CLASS.get().unwrap().get_inner_with(&ruby)
}

fn dummy_rolloff(_: magnus::Value, _: RbChannelControl, _: f32) -> f32 {
    0.0
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    let class = module.define_class("RolloffCallback", magnus::class::object())?;
    class.define_method("rolloff", magnus::method!(dummy_rolloff, 2))?;

    let _ = CLASS.set(class.into());

    Ok(())
}
