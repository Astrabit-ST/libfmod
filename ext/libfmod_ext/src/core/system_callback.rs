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

use super::{enums::OutputType, sound::RbSound, structs::ErrorCallbackInfo, system::RbSystem};

static CLASS: OnceCell<Opaque<magnus::RClass>> = OnceCell::new();

pub struct SystemCallback;

impl fmod::SystemCallback for SystemCallback {
    fn device_list_changed(system: fmod::System, _: *mut std::ffi::c_void) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback
                .funcall("device_list_changed", (system, userdata))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn device_lost(system: fmod::System, _: *mut std::ffi::c_void) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback.funcall("device_lost", (system, userdata)).unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn memory_allocation_failed(
        system: fmod::System,
        file: &fmod::Utf8CStr,
        size: std::ffi::c_int,
        _: *mut std::ffi::c_void,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        let file = file.to_cstring();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let file = file.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback
                .funcall("memory_allocation_failed", (system, file, size, userdata))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn thread_created(
        system: fmod::System,
        _: *mut std::ffi::c_void,
        thread_name: &fmod::Utf8CStr,
        _: *mut std::ffi::c_void,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        let thread_name = thread_name.to_cstring();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let thread_name = thread_name.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback
                .funcall("thread_created", (system, thread_name, userdata))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn bad_dsp_connection(system: fmod::System, _: *mut std::ffi::c_void) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback
                .funcall("bad_dsp_connection", (system, userdata))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn premix(system: fmod::System, _: *mut std::ffi::c_void) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback.funcall("premix", (system, userdata)).unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn postmix(system: fmod::System, _: *mut std::ffi::c_void) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback.funcall("postmix", (system, userdata)).unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn error(
        system: fmod::System,
        error_info: fmod::ErrorCallbackInfo<'_>,
        _: *mut std::ffi::c_void,
    ) -> fmod::Result<()> {
        // Safety: we block until the callback is done executing.
        // Therefore, the callback can't use error_info for longer than its lifetime
        let error_info: fmod::ErrorCallbackInfo<'static> =
            unsafe { std::mem::transmute(error_info) };
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let error_info = error_info.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback
                .funcall("error", (system, error_info, userdata))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn mid_mix(system: fmod::System, _: *mut std::ffi::c_void) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback.funcall("mid_mix", (system, userdata)).unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn thread_destroyed(
        system: fmod::System,
        _: *mut std::ffi::c_void,
        thread_name: &fmod::Utf8CStr,
        _: *mut std::ffi::c_void,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        let thread_name = thread_name.to_cstring();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let thread_name = thread_name.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback
                .funcall("thread_destroyed", (system, thread_name, userdata))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn pre_update(system: fmod::System, _: *mut std::ffi::c_void) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback.funcall("pre_update", (system, userdata)).unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn post_update(system: fmod::System, _: *mut std::ffi::c_void) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback.funcall("post_update", (system, userdata)).unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn record_list_changed(system: fmod::System, _: *mut std::ffi::c_void) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback
                .funcall("record_list_changed", (system, userdata))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn buffered_no_mix(system: fmod::System, _: *mut std::ffi::c_void) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback
                .funcall("buffered_no_mix", (system, userdata))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn device_reinitialize(
        system: fmod::System,
        output_type: fmod::OutputType,
        driver_index: std::ffi::c_int,
        _: *mut std::ffi::c_void,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let output_type = output_type.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback
                .funcall(
                    "device_reinitialize",
                    (system, output_type, driver_index, userdata),
                )
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn output_underrun(system: fmod::System, _: *mut std::ffi::c_void) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback
                .funcall("output_underrun", (system, userdata))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn record_position_changed(
        system: fmod::System,
        sound: fmod::Sound,
        record_position: std::ffi::c_int,
        _: *mut std::ffi::c_void,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let system = system.into_ruby().unwrap();
            let sound = sound.into_ruby().unwrap();
            let userdata: magnus::Value = system.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = system.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback
                .funcall(
                    "record_position_changed",
                    (system, sound, record_position, userdata),
                )
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }
}

pub fn class() -> magnus::RClass {
    let ruby = magnus::Ruby::get().unwrap();
    CLASS.get().unwrap().get_inner_with(&ruby)
}

fn dummy_device_list_changed(_: magnus::Value, _: RbSystem, _: magnus::Value) {}
fn dummy_device_lost(_: magnus::Value, _: RbSystem, _: magnus::Value) {}
fn dummy_memory_allocation_failed(
    _: magnus::Value,
    _: RbSystem,
    _: magnus::RString,
    _: i32,
    _: magnus::Value,
) {
}
fn dummy_thread_created(_: magnus::Value, _: RbSystem, _: magnus::RString, _: magnus::Value) {}
fn dummy_bad_dsp_connection(_: magnus::Value, _: RbSystem, _: magnus::Value) {}
fn dummy_premix(_: magnus::Value, _: RbSystem, _: magnus::Value) {}
fn dummy_postmix(_: magnus::Value, _: RbSystem, _: magnus::Value) {}
fn dummy_error(_: magnus::Value, _: RbSystem, _: ErrorCallbackInfo, _: magnus::Value) {}
fn dummy_mid_mix(_: magnus::Value, _: RbSystem, _: magnus::Value) {}
fn dummy_thread_destroyed(_: magnus::Value, _: RbSystem, _: magnus::RString, _: magnus::Value) {}
fn dummy_pre_update(_: magnus::Value, _: RbSystem, _: magnus::Value) {}
fn dummy_post_update(_: magnus::Value, _: RbSystem, _: magnus::Value) {}
fn dummy_record_list_changed(_: magnus::Value, _: RbSystem, _: magnus::Value) {}
fn dummy_buffered_no_mix(_: magnus::Value, _: RbSystem, _: magnus::Value) {}
fn dummy_device_reinitialize(
    _: magnus::Value,
    _: RbSystem,
    _: OutputType,
    _: i32,
    _: magnus::Value,
) {
}
fn dummy_output_underrun(_: magnus::Value, _: RbSystem, _: magnus::Value) {}
fn dummy_record_position_changed(
    _: magnus::Value,
    _: RbSystem,
    _: RbSound,
    _: i32,
    _: magnus::Value,
) {
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    let class = module.define_class("SystemCallback", magnus::class::object())?;
    class.define_method(
        "device_list_changed",
        magnus::method!(dummy_device_list_changed, 2),
    )?;
    class.define_method("device_lost", magnus::method!(dummy_device_lost, 2))?;
    class.define_method(
        "memory_allocation_failed",
        magnus::method!(dummy_memory_allocation_failed, 4),
    )?;
    class.define_method("thread_created", magnus::method!(dummy_thread_created, 3))?;
    class.define_method(
        "bad_dsp_connection",
        magnus::method!(dummy_bad_dsp_connection, 2),
    )?;
    class.define_method("premix", magnus::method!(dummy_premix, 2))?;
    class.define_method("postmix", magnus::method!(dummy_postmix, 2))?;
    class.define_method("error", magnus::method!(dummy_error, 3))?;
    class.define_method("mid_mix", magnus::method!(dummy_mid_mix, 2))?;
    class.define_method(
        "thread_destroyed",
        magnus::method!(dummy_thread_destroyed, 3),
    )?;
    class.define_method("pre_update", magnus::method!(dummy_pre_update, 2))?;
    class.define_method("post_update", magnus::method!(dummy_post_update, 2))?;
    class.define_method(
        "record_list_changed",
        magnus::method!(dummy_record_list_changed, 2),
    )?;
    class.define_method("buffered_no_mix", magnus::method!(dummy_buffered_no_mix, 2))?;
    class.define_method(
        "device_reinitialize",
        magnus::method!(dummy_device_reinitialize, 4),
    )?;
    class.define_method("output_underrun", magnus::method!(dummy_output_underrun, 2))?;
    class.define_method(
        "record_position_changed",
        magnus::method!(dummy_record_position_changed, 4),
    )?;

    let _ = CLASS.set(class.into());

    Ok(())
}
