// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::{callback, core::sound::RbSound, FromRuby, IntoRuby, Result};
use magnus::{
    prelude::*,
    value::{InnerValue, Opaque},
};
use once_cell::sync::OnceCell;

use super::{event_description::RbEventDescription, event_instance::RbEventInstance};

static CLASS: OnceCell<Opaque<magnus::RClass>> = OnceCell::new();

pub struct EventInstanceCallback;

impl fmod::studio::EventInstanceCallback for EventInstanceCallback {
    fn created(event_instance: fmod::studio::EventInstance) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let event_instance = event_instance.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let _: magnus::Value = callback.funcall("created", (event_instance,)).unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn destroyed(event: fmod::studio::EventInstance) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let _: magnus::Value = callback.funcall("destroyed", (event_instance,)).unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn starting(event: fmod::studio::EventInstance) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let _: magnus::Value = callback.funcall("starting", (event_instance,)).unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn started(event: fmod::studio::EventInstance) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let _: magnus::Value = callback.funcall("started", (event_instance,)).unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn restarted(event: fmod::studio::EventInstance) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let _: magnus::Value = callback.funcall("restarted", (event_instance,)).unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn stopped(event: fmod::studio::EventInstance) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let _: magnus::Value = callback.funcall("stopped", (event_instance,)).unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn start_failed(event: fmod::studio::EventInstance) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let _: magnus::Value = callback.funcall("start_failed", (event_instance,)).unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn create_programmer_sound(
        event: fmod::studio::EventInstance,
        sound_props: fmod::studio::ProgrammerSoundProperties<'_>,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<Option<(fmod::Sound, i32)>>();
        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let name = sound_props.name.into_ruby().unwrap();

            let result: Option<(RbSound, i32)> = callback
                .funcall("create_programmer_sound", (event_instance, name))
                .unwrap();
            let result =
                result.map(|(sound, subsound_index)| (sound.from_ruby().unwrap(), subsound_index));

            sender.send(result).unwrap();
        });
        let result = reciever.recv().unwrap();
        if let Some((sound, subsound_index)) = result {
            *sound_props.sound = sound;
            *sound_props.subsound_index = subsound_index;
        }
        Ok(())
    }

    fn destroy_programmer_sound(
        event: fmod::studio::EventInstance,
        sound_props: fmod::studio::ProgrammerSoundProperties<'_>,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        let fmod::studio::ProgrammerSoundProperties {
            sound: &mut sound,
            name,
            subsound_index: &mut subsound_index,
        } = sound_props;

        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let name = name.into_ruby().unwrap();
            let sound = sound.into_ruby().unwrap();
            let _: magnus::Value = callback
                .funcall(
                    "destroy_programmer_sound",
                    (event_instance, name, sound, subsound_index),
                )
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();

        Ok(())
    }

    fn plugin_created(
        event: fmod::studio::EventInstance,
        plugin_props: fmod::studio::PluginInstanceProperties,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let plugin_props = plugin_props.into_ruby().unwrap();
            let _: magnus::Value = callback
                .funcall("plugin_created", (event_instance, plugin_props))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn plugin_destroyed(
        event: fmod::studio::EventInstance,
        plugin_props: fmod::studio::PluginInstanceProperties,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let plugin_props = plugin_props.into_ruby().unwrap();
            let _: magnus::Value = callback
                .funcall("plugin_destroyed", (event_instance, plugin_props))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn timeline_marker(
        event: fmod::studio::EventInstance,
        timeline_props: fmod::studio::TimelineMarkerProperties,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let timeline_props = timeline_props.into_ruby().unwrap();
            let _: magnus::Value = callback
                .funcall("timeline_marker", (event_instance, timeline_props))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn timeline_beat(
        event: fmod::studio::EventInstance,
        timeline_beat: fmod::studio::TimelineBeatProperties,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let timeline_beat = timeline_beat.into_ruby().unwrap();
            let _: magnus::Value = callback
                .funcall("timeline_beat", (event_instance, timeline_beat))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn sound_played(event: fmod::studio::EventInstance, sound: fmod::Sound) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let sound = sound.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let _: magnus::Value = callback
                .funcall("sound_played", (event_instance, sound))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn sound_stopped(event: fmod::studio::EventInstance, sound: fmod::Sound) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let sound = sound.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let _: magnus::Value = callback
                .funcall("sound_stopped", (event_instance, sound))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn real_to_virtual(event: fmod::studio::EventInstance) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let _: magnus::Value = callback
                .funcall("real_to_virtual", (event_instance,))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn virtual_to_real(event: fmod::studio::EventInstance) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let _: magnus::Value = callback
                .funcall("virtual_to_real", (event_instance,))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn start_event_command(
        event: fmod::studio::EventInstance,
        new_event: fmod::studio::EventInstance,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let new_event_instance = new_event.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let _: magnus::Value = callback
                .funcall("start_event_command", (event_instance, new_event_instance))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }

    fn nested_timeline_beat(
        event: fmod::studio::EventInstance,
        timeline_props: fmod::studio::TimelineNestedBeatProperties,
    ) -> fmod::Result<()> {
        let (sender, reciever) = oneshot::channel::<()>();
        callback::process(move |_| {
            let event_instance = event.into_ruby().unwrap();
            let callback = EventInstanceCallback::get_callback(event_instance);
            let timeline_props = timeline_props.into_ruby().unwrap();
            let _: magnus::Value = callback
                .funcall("nested_timeline_beat", (event_instance, timeline_props))
                .unwrap();
            sender.send(()).unwrap();
        });
        let _ = reciever.recv();
        Ok(())
    }
}

impl EventInstanceCallback {
    fn get_callback(inst: RbEventInstance) -> magnus::Value {
        let callback: magnus::Value = inst.ivar_get("__callback").unwrap();
        if callback.is_nil() {
            let desc: RbEventDescription = inst.funcall("get_description", ()).unwrap();
            desc.ivar_get("__callback").unwrap()
        } else {
            callback
        }
    }
}

fn dummy_created(_: magnus::Value, _: RbEventInstance) {}
fn dummy_destroyed(_: magnus::Value, _: RbEventInstance) {}
fn dummy_starting(_: magnus::Value, _: RbEventInstance) {}
fn dummy_started(_: magnus::Value, _: RbEventInstance) {}
fn dummy_restarted(_: magnus::Value, _: RbEventInstance) {}
fn dummy_stopped(_: magnus::Value, _: RbEventInstance) {}
fn dummy_start_failed(_: magnus::Value, _: RbEventInstance) {}
fn dummy_create_programmer_sound(
    _: magnus::Value,
    _: RbEventInstance,
    _: magnus::RString,
) -> Option<(RbSound, i32)> {
    None
}
fn dummy_destroy_programmer_sound(
    _: magnus::Value,
    _: RbEventInstance,
    _: magnus::RString,
    _: RbSound,
    _: i32,
) {
}
fn dummy_plugin_created(_: magnus::Value, _: RbEventInstance, _: magnus::RStruct) {}
fn dummy_plugin_destroyed(_: magnus::Value, _: RbEventInstance, _: magnus::RStruct) {}
fn dummy_timeline_marker(_: magnus::Value, _: RbEventInstance, _: magnus::RStruct) {}
fn dummy_timeline_beat(_: magnus::Value, _: RbEventInstance, _: magnus::RStruct) {}
fn dummy_sound_played(_: magnus::Value, _: RbEventInstance, _: RbSound) {}
fn dummy_sound_stopped(_: magnus::Value, _: RbEventInstance, _: RbSound) {}
fn dummy_real_to_virtual(_: magnus::Value, _: RbEventInstance) {}
fn dummy_virtual_to_real(_: magnus::Value, _: RbEventInstance) {}
fn dummy_start_event_command(_: magnus::Value, _: RbEventInstance, _: RbEventInstance) {}
fn dummy_nested_timeline_beat(_: magnus::Value, _: RbEventInstance, _: magnus::RStruct) {}

pub fn class() -> magnus::RClass {
    let ruby = magnus::Ruby::get().unwrap();
    CLASS.get().unwrap().get_inner_with(&ruby)
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    let class = module.define_class("EventInstanceCallback", magnus::class::object())?;

    class.define_method("created", magnus::method!(dummy_created, 1))?;
    class.define_method("destroyed", magnus::method!(dummy_destroyed, 1))?;
    class.define_method("starting", magnus::method!(dummy_starting, 1))?;
    class.define_method("started", magnus::method!(dummy_started, 1))?;
    class.define_method("restarted", magnus::method!(dummy_restarted, 1))?;
    class.define_method("stopped", magnus::method!(dummy_stopped, 1))?;
    class.define_method("start_failed", magnus::method!(dummy_start_failed, 1))?;
    class.define_method(
        "create_programmer_sound",
        magnus::method!(dummy_create_programmer_sound, 2),
    )?;
    class.define_method(
        "destroy_programmer_sound",
        magnus::method!(dummy_destroy_programmer_sound, 4),
    )?;
    class.define_method("plugin_created", magnus::method!(dummy_plugin_created, 2))?;
    class.define_method(
        "plugin_destroyed",
        magnus::method!(dummy_plugin_destroyed, 2),
    )?;
    class.define_method("timeline_marker", magnus::method!(dummy_timeline_marker, 2))?;
    class.define_method("timeline_beat", magnus::method!(dummy_timeline_beat, 2))?;
    class.define_method("sound_played", magnus::method!(dummy_sound_played, 2))?;
    class.define_method("sound_stopped", magnus::method!(dummy_sound_stopped, 2))?;
    class.define_method("real_to_virtual", magnus::method!(dummy_real_to_virtual, 1))?;
    class.define_method("virtual_to_real", magnus::method!(dummy_virtual_to_real, 1))?;
    class.define_method(
        "start_event_command",
        magnus::method!(dummy_start_event_command, 2),
    )?;
    class.define_method(
        "nested_timeline_beat",
        magnus::method!(dummy_nested_timeline_beat, 2),
    )?;

    let _ = CLASS.set(class.into());

    Ok(())
}
