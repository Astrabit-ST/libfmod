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

use super::{bank::RbBank, system::RbSystem};

static CLASS: OnceCell<Opaque<magnus::RClass>> = OnceCell::new();

pub struct SystemCallback;

impl fmod::studio::SystemCallback for SystemCallback {
    fn preupdate(system: fmod::studio::System, _: *mut std::ffi::c_void) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback.funcall("preupdate", (system, userdata)).unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn postupdate(system: fmod::studio::System, _: *mut std::ffi::c_void) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback.funcall("postupdate", (system, userdata)).unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn bank_unload(
        system: fmod::studio::System,
        bank: fmod::studio::Bank,
        _: *mut std::ffi::c_void,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let bank = bank.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback
                .funcall("bank_unload", (system, bank, userdata))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn liveupdate_connected(
        system: fmod::studio::System,
        _: *mut std::ffi::c_void,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback
                .funcall("liveupdate_connected", (system, userdata))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn liveupdate_disconnected(
        system: fmod::studio::System,
        _: *mut std::ffi::c_void,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback
                .funcall("liveupdate_disconnected", (system, userdata))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }
}

fn dummy_preupdate(_: magnus::Value, _: RbSystem, _: magnus::Value) {}
fn dummy_postupdate(_: magnus::Value, _: RbSystem, _: magnus::Value) {}
fn dummy_bank_unload(_: magnus::Value, _: RbSystem, _: RbBank, _: magnus::Value) {}
fn liveupdate_connected(_: magnus::Value, _: RbSystem, _: magnus::Value) {}
fn liveupdate_disconnected(_: magnus::Value, _: RbSystem, _: magnus::Value) {}

pub fn class() -> magnus::RClass {
    let ruby = magnus::Ruby::get().unwrap();
    CLASS.get().unwrap().get_inner_with(&ruby)
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    let class = module.define_class("SystemCallback", magnus::class::object())?;

    let _ = CLASS.set(class.into());

    class.define_method("preupdate", magnus::method!(dummy_preupdate, 2))?;
    class.define_method("postupdate", magnus::method!(dummy_postupdate, 2))?;
    class.define_method("bank_unload", magnus::method!(dummy_bank_unload, 3))?;
    class.define_method(
        "liveupdate_connected",
        magnus::method!(liveupdate_connected, 2),
    )?;
    class.define_method(
        "liveupdate_disconnected",
        magnus::method!(liveupdate_disconnected, 2),
    )?;

    Ok(())
}
