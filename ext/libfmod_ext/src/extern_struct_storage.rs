// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod::{
    studio::{
        Bank, Bus, CommandReplay, EventDescription, EventInstance, System as StudioSystem, Vca,
    },
    ChannelControl, Dsp, DspConnection, Geometry, Reverb3D, Sound, SoundGroup, SyncPoint, System,
};
use once_cell::sync::Lazy;
use std::{collections::HashMap, os::raw::c_void, sync::Mutex};

use magnus::prelude::*;
use magnus::{error::Result, typed_data::Obj};

#[derive(Debug, Default)]
struct ExternStructStorage {
    map: Mutex<HashMap<ExternStruct, magnus::RTypedData>>,
}

#[derive(PartialEq, Hash, Eq, Debug, Clone, Copy)]
pub(crate) enum ExternStruct {
    StudioSystem(StudioSystem),
    Bank(Bank),
    EventDescription(EventDescription),
    EventInstance(EventInstance),
    CommandReplay(CommandReplay),
    Bus(Bus),
    Vca(Vca),
    //
    Reverb3D(Reverb3D),
    SoundGroup(SoundGroup),
    ChannelControl(ChannelControl),
    Dsp(Dsp),
    Sound(Sound),
    Geometry(Geometry),
    DspConnection(DspConnection),
    SyncPoint(SyncPoint),
    System(System),
}

// everything is fiiiiiiiiine :3
unsafe impl Send for ExternStructStorage {}
unsafe impl Sync for ExternStructStorage {}

const DEFAULT_USERDATA_PTR: *mut c_void = 0xDEAD_CAFE as *mut c_void;
static STORAGE: Lazy<ExternStructStorage> = Lazy::new(Default::default);

#[derive(magnus::TypedData)]
#[magnus(class = "FMOD::__UserdataWrapper")]
struct __UserdataWrapper;

impl magnus::DataTypeFunctions for __UserdataWrapper {
    fn mark(&self, marker: &magnus::gc::Marker) {
        let mut storage = STORAGE.map.lock().unwrap();
        storage.retain(|key, &mut value| {
            key.is_valid()
                .then(|| {
                    marker.mark(value);
                })
                .is_some()
        });
    }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    let class = module.define_class("__UserdataWrapper", magnus::class::basic_object())?;
    class.ivar_set("__inst", __UserdataWrapper)?;

    Ok(())
}

pub(crate) fn get_or_insert<T, R>(value: T, ruby_val: R) -> Result<Obj<R>>
where
    T: Into<ExternStruct>,
    R: magnus::TypedData,
{
    let mut storage = STORAGE.map.lock().unwrap();
    let key = value.into();
    let &mut value = storage
        .entry(key)
        .or_insert_with(|| Obj::wrap(ruby_val).into());
    Obj::try_convert(value.as_value())
}

pub fn remove_value(value: impl Into<ExternStruct>) {
    let mut storage = STORAGE.map.lock().unwrap();
    let key = value.into();
    storage.remove(&key);
}

pub fn set_userdata(value: impl Into<ExternStruct>, userdata: magnus::Value) -> Result<()> {
    let mut storage = STORAGE.map.lock().unwrap();
    let key = value.into();
    let rb_val = storage.get_mut(&key).unwrap(); // we should always have a value here!
    rb_val.ivar_set("__userdata", userdata)
}

pub fn get_userdata(value: impl Into<ExternStruct>) -> Result<magnus::Value> {
    let storage = STORAGE.map.lock().unwrap();
    let key = value.into();
    let rb_val = storage.get(&key).unwrap(); // we should always have a value here!
    rb_val.ivar_get("__userdata")
}

pub fn set_callback(value: impl Into<ExternStruct>, rb_val: magnus::Value) -> Result<()> {
    let mut storage = STORAGE.map.lock().unwrap();
    let key = value.into();
    let userdata = storage.get_mut(&key).unwrap(); // we should always have a value here!
    userdata.ivar_set("__callback", rb_val)
}

pub(crate) fn clear_storage() {
    let mut storage = STORAGE.map.lock().unwrap();
    storage.clear();
}

impl ExternStruct {
    fn get_raw_userdata(&self) -> fmod::Result<*mut std::ffi::c_void> {
        match self {
            ExternStruct::StudioSystem(s) => s.get_raw_userdata(),
            ExternStruct::Bank(b) => b.get_raw_userdata(),
            ExternStruct::EventDescription(e) => e.get_raw_userdata(),
            ExternStruct::EventInstance(e) => e.get_raw_userdata(),
            ExternStruct::CommandReplay(c) => c.get_raw_userdata(),
            // core
            ExternStruct::Reverb3D(r) => r.get_raw_userdata(),
            ExternStruct::SoundGroup(s) => s.get_raw_userdata(),
            // may occasionally be called when the channel is lost, but this should return Err() in that case
            ExternStruct::ChannelControl(c) => c.get_raw_userdata(),
            ExternStruct::Dsp(d) => d.get_raw_userdata(),
            ExternStruct::Sound(s) => s.get_raw_userdata(),
            ExternStruct::Geometry(g) => g.get_raw_userdata(),
            ExternStruct::DspConnection(c) => c.get_raw_userdata(),
            ExternStruct::System(s) => s.get_raw_userdata(),
            _ => Err(fmod::Error::Fmod(
                fmod::ffi::FMOD_RESULT::FMOD_ERR_INVALID_PARAM,
            )),
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            ExternStruct::StudioSystem(s) => s.is_valid(),
            ExternStruct::Bank(b) => b.is_valid(),
            ExternStruct::EventDescription(e) => e.is_valid(),
            ExternStruct::EventInstance(e) => e.is_valid(),
            ExternStruct::CommandReplay(c) => c.is_valid(),
            ExternStruct::Bus(b) => b.is_valid(),
            ExternStruct::Vca(v) => v.is_valid(),
            // We have no way of checking if these are valid, so we just assume they are
            ExternStruct::SyncPoint(_) | ExternStruct::System(_) => true,
            // when released, get_raw_userdata will return null
            // this is a bit of a hack though (and not very safe)
            // this should almost never be called when released though, so it should be fine...?
            _ => self
                .get_raw_userdata()
                .is_ok_and(|ptr| ptr == DEFAULT_USERDATA_PTR),
        }
    }
}

impl From<StudioSystem> for ExternStruct {
    fn from(s: StudioSystem) -> Self {
        ExternStruct::StudioSystem(s)
    }
}

impl From<Bank> for ExternStruct {
    fn from(b: Bank) -> Self {
        ExternStruct::Bank(b)
    }
}

impl From<EventDescription> for ExternStruct {
    fn from(e: EventDescription) -> Self {
        ExternStruct::EventDescription(e)
    }
}

impl From<EventInstance> for ExternStruct {
    fn from(e: EventInstance) -> Self {
        ExternStruct::EventInstance(e)
    }
}

impl From<CommandReplay> for ExternStruct {
    fn from(c: CommandReplay) -> Self {
        ExternStruct::CommandReplay(c)
    }
}

impl From<Bus> for ExternStruct {
    fn from(b: Bus) -> Self {
        ExternStruct::Bus(b)
    }
}

impl From<Vca> for ExternStruct {
    fn from(v: Vca) -> Self {
        ExternStruct::Vca(v)
    }
}

impl From<Reverb3D> for ExternStruct {
    fn from(r: Reverb3D) -> Self {
        ExternStruct::Reverb3D(r)
    }
}

impl From<SoundGroup> for ExternStruct {
    fn from(s: SoundGroup) -> Self {
        ExternStruct::SoundGroup(s)
    }
}

impl From<ChannelControl> for ExternStruct {
    fn from(c: ChannelControl) -> Self {
        ExternStruct::ChannelControl(c)
    }
}

impl From<Dsp> for ExternStruct {
    fn from(d: Dsp) -> Self {
        ExternStruct::Dsp(d)
    }
}

impl From<Sound> for ExternStruct {
    fn from(s: Sound) -> Self {
        ExternStruct::Sound(s)
    }
}

impl From<Geometry> for ExternStruct {
    fn from(g: Geometry) -> Self {
        ExternStruct::Geometry(g)
    }
}

impl From<DspConnection> for ExternStruct {
    fn from(c: DspConnection) -> Self {
        ExternStruct::DspConnection(c)
    }
}

impl From<SyncPoint> for ExternStruct {
    fn from(s: SyncPoint) -> Self {
        ExternStruct::SyncPoint(s)
    }
}

impl From<System> for ExternStruct {
    fn from(s: System) -> Self {
        ExternStruct::System(s)
    }
}
