// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::{callback, core::structs::Guid, FromRuby, IntoRuby, Result};
use magnus::{
    prelude::*,
    value::{InnerValue, Opaque},
};
use once_cell::sync::OnceCell;

use super::{
    bank::RbBank, command_replay::RbCommandReplay, event_description::RbEventDescription,
    event_instance::RbEventInstance, flags::LoadBankFlags,
};

static CREATE_INST_CLASS: OnceCell<Opaque<magnus::RClass>> = OnceCell::new();
static FRAME_CLASS: OnceCell<Opaque<magnus::RClass>> = OnceCell::new();
static LOAD_BANK_CLASS: OnceCell<Opaque<magnus::RClass>> = OnceCell::new();

pub struct CreateInstanceCallback;

impl fmod::studio::CreateInstanceCallback for CreateInstanceCallback {
    fn create_instance_callback(
        replay: fmod::studio::CommandReplay,
        command_index: std::ffi::c_int,
        description: fmod::studio::EventDescription,
        _: *mut std::ffi::c_void,
    ) -> fmod::Result<Option<fmod::studio::EventInstance>> {
        let (sender, reciever) = oneshot::channel();
        callback::process(move |_| {
            let replay: RbCommandReplay = replay.into_ruby().unwrap();
            let description: RbEventDescription = description.into_ruby().unwrap();
            let userdata: magnus::Value = replay.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = replay.ivar_get("__create_instance_callback").unwrap();

            let result: Option<RbEventInstance> = callback
                .funcall(
                    "create_programmer_sound",
                    (replay, command_index, description, userdata),
                )
                .unwrap();
            let result = result.map(|event_instance| event_instance.from_ruby().unwrap());

            sender.send(result).unwrap();
        });
        let result = reciever.recv().unwrap();
        Ok(result)
    }
}

pub fn create_instance_class() -> magnus::RClass {
    let ruby = magnus::Ruby::get().unwrap();
    CREATE_INST_CLASS.get().unwrap().get_inner_with(&ruby)
}

pub struct FrameCallback;

impl fmod::studio::FrameCallback for FrameCallback {
    fn frame_callback(
        replay: fmod::studio::CommandReplay,
        command_index: std::ffi::c_int,
        current_time: std::ffi::c_float,
        _: *mut std::ffi::c_void,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel();
        callback::process(move |_| {
            let replay: RbCommandReplay = replay.into_ruby().unwrap();
            let userdata: magnus::Value = replay.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = replay.ivar_get("__frame_callback").unwrap();

            let _: magnus::Value = callback
                .funcall(
                    "frame_callback",
                    (replay, command_index, current_time, userdata),
                )
                .unwrap();

            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }
}

pub fn frame_class() -> magnus::RClass {
    let ruby = magnus::Ruby::get().unwrap();
    FRAME_CLASS.get().unwrap().get_inner_with(&ruby)
}

pub struct LoadBankCallback;

impl fmod::studio::LoadBankCallback for LoadBankCallback {
    fn load_bank_callback(
        replay: fmod::studio::CommandReplay,
        command_index: std::ffi::c_int,
        guid: Option<fmod::Guid>,
        filename: Option<&fmod::Utf8CStr>,
        flags: fmod::studio::LoadBankFlags,
        _: *mut std::ffi::c_void,
    ) -> fmod::Result<Option<fmod::studio::Bank>> {
        let (sender, reciever) = oneshot::channel();
        let filename = filename.map(fmod::Utf8CStr::to_cstring);
        callback::process(move |_| {
            let replay: RbCommandReplay = replay.into_ruby().unwrap();
            let guid = guid.map(|guid| guid.into_ruby().unwrap());
            let filename = filename.map(|filename| filename.into_ruby().unwrap());
            let flags = flags.into_ruby().unwrap();
            let userdata: magnus::Value = replay.ivar_get("__userdata").unwrap();
            let callback: magnus::RObject = replay.ivar_get("__load_bank_callback").unwrap();

            let result: Option<RbBank> = callback
                .funcall(
                    "load_bank_callback",
                    (replay, command_index, guid, filename, flags, userdata),
                )
                .unwrap();
            let result = result.map(|bank| bank.from_ruby().unwrap());

            sender.send(result).unwrap();
        });
        let result = reciever.recv().unwrap();
        Ok(result)
    }
}

pub fn load_bank_class() -> magnus::RClass {
    let ruby = magnus::Ruby::get().unwrap();
    LOAD_BANK_CLASS.get().unwrap().get_inner_with(&ruby)
}

fn dummy_create_inst(
    _: magnus::Value,
    _: RbCommandReplay,
    _: i32,
    _: RbEventDescription,
    _: magnus::Value,
) -> Option<RbEventInstance> {
    None
}

fn dummy_frame(_: magnus::Value, _: RbCommandReplay, _: i32, _: f32, _: magnus::Value) {}

fn dummy_load_bank(
    _: magnus::Value,
    _: RbCommandReplay,
    _: i32,
    _: Option<Guid>,
    _: Option<magnus::RString>,
    _: LoadBankFlags,
    _: magnus::Value,
) -> Option<RbBank> {
    None
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    let class = module.define_class("CreateInstanceCallback", magnus::class::object())?;
    class.define_method(
        "create_instance_callback",
        magnus::method!(dummy_create_inst, 4),
    )?;
    let _ = CREATE_INST_CLASS.set(class.into());

    let class = module.define_class("FrameCallback", magnus::class::object())?;
    class.define_method("frame_callback", magnus::method!(dummy_frame, 4))?;
    let _ = FRAME_CLASS.set(class.into());

    let class = module.define_class("LoadBankCallback", magnus::class::object())?;
    class.define_method("load_bank_callback", magnus::method!(dummy_load_bank, 6))?;
    let _ = LOAD_BANK_CLASS.set(class.into());

    Ok(())
}
