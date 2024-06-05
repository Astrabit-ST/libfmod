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

use super::channel_control::RbChannelControl;

static CLASS: OnceCell<Opaque<magnus::RClass>> = OnceCell::new();

pub struct ChannelControlCallback;

impl IntoRuby<RbChannelControl> for fmod::ChannelControlType {
    fn into_ruby(self) -> Result<RbChannelControl> {
        // kinda abuses the fact that channel/channelgroup both impl IntoRuby<RbChannelControl>
        match self {
            Self::Channel(c) => c.into_ruby(),
            Self::ChannelGroup(c) => c.into_ruby(),
        }
    }
}

impl fmod::ChannelControlCallback for ChannelControlCallback {
    fn end(channel_control: fmod::ChannelControlType) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let channel_control = channel_control.into_ruby().unwrap();
            let callback: magnus::RObject = channel_control.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback.funcall("end", (channel_control,)).unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn virtual_voice(
        channel_control: fmod::ChannelControlType,
        is_virtual: bool,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let channel_control = channel_control.into_ruby().unwrap();
            let callback: magnus::RObject = channel_control.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback
                .funcall("virtual_voice", (channel_control, is_virtual))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn sync_point(
        channel_control: fmod::ChannelControlType,
        sync_point: std::ffi::c_int,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let channel_control = channel_control.into_ruby().unwrap();
            let callback: magnus::RObject = channel_control.ivar_get("__callback").unwrap();
            let _: magnus::Value = callback
                .funcall("sync_point", (channel_control, sync_point))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn occlusion(
        channel_control: fmod::ChannelControlType,
        direct: &mut std::ffi::c_float,
        reverb: &mut std::ffi::c_float,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel();
        callback::process(move |_| {
            let channel_control = channel_control.into_ruby().unwrap();
            let callback: magnus::RObject = channel_control.ivar_get("__callback").unwrap();
            let result: Option<(f32, f32)> =
                callback.funcall("occlusion", (channel_control,)).unwrap();
            sender.send(result).unwrap();
        });
        let result = reciever.recv().unwrap();
        if let Some((d, r)) = result {
            *direct = d;
            *reverb = r;
        }
        Ok(())
    }
}

pub fn class() -> magnus::RClass {
    let ruby = magnus::Ruby::get().unwrap();
    CLASS.get().unwrap().get_inner_with(&ruby)
}

fn dummy_end(_: magnus::Value, _: RbChannelControl) {}
fn dummy_virtual_voice(_: magnus::Value, _: RbChannelControl, _: bool) {}
fn dummy_sync_point(_: magnus::Value, _: RbChannelControl, _: i32) {}
fn dummy_occlusion(_: magnus::Value, _: RbChannelControl) -> Option<(f32, f32)> {
    None
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    let class = module.define_class("ChannelControlCallback", magnus::class::object())?;
    class.define_method("end", magnus::method!(dummy_end, 1))?;
    class.define_method("virtual_voice", magnus::method!(dummy_virtual_voice, 2))?;
    class.define_method("sync_point", magnus::method!(dummy_sync_point, 2))?;
    class.define_method("occlusion", magnus::method!(dummy_occlusion, 1))?;

    let _ = CLASS.set(class.into());

    Ok(())
}
