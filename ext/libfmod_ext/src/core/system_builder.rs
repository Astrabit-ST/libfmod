// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use crate::{Bindable, FromRuby, IntoRuby, Result};
use magnus::prelude::*;
use std::cell::RefCell;

use crate::extern_struct_bind;

use super::{
    enums::{OutputType, SpeakerMode},
    flags::InitFlags,
    system::System,
};

#[magnus::wrap(class = "FMOD::Studio::SystemBuilder", free_immediately, size)]
pub struct SystemBuilder(RefCell<Option<fmod::SystemBuilder>>);

unsafe impl Send for SystemBuilder {}
unsafe impl Sync for SystemBuilder {}

impl SystemBuilder {
    fn new() -> Result<Self> {
        unsafe { fmod::SystemBuilder::new() }.into_ruby()
    }

    fn software_format(
        &self,
        sample_rate: i32,
        speaker_mode: SpeakerMode,
        raw_speakers: i32,
    ) -> Result<()> {
        let mut builder = self.0.borrow_mut();

        let Some(builder) = &mut *builder else {
            let error = magnus::Error::new(
                magnus::exception::runtime_error(),
                "SystemBuilder is already built",
            );
            return Err(error);
        };

        let speaker_mode = speaker_mode.from_ruby()?;
        builder
            .software_format(sample_rate, speaker_mode, raw_speakers)
            .map(|_| ())
            .into_ruby()
    }

    fn software_channels(&self, software_channels: i32) -> Result<()> {
        let mut builder = self.0.borrow_mut();

        let Some(builder) = &mut *builder else {
            let error = magnus::Error::new(
                magnus::exception::runtime_error(),
                "SystemBuilder is already built",
            );
            return Err(error);
        };

        builder
            .software_channels(software_channels)
            .map(|_| ())
            .into_ruby()
    }

    fn dsp_buffer_size(&self, buffer_size: u32, buffer_count: i32) -> Result<()> {
        let mut builder = self.0.borrow_mut();

        let Some(builder) = &mut *builder else {
            let error = magnus::Error::new(
                magnus::exception::runtime_error(),
                "SystemBuilder is already built",
            );
            return Err(error);
        };

        builder
            .dsp_buffer_size(buffer_size, buffer_count)
            .map(|_| ())
            .into_ruby()
    }

    fn output(&self, kind: OutputType) -> Result<()> {
        let mut builder = self.0.borrow_mut();

        let Some(builder) = &mut *builder else {
            let error = magnus::Error::new(
                magnus::exception::runtime_error(),
                "SystemBuilder is already built",
            );
            return Err(error);
        };

        let kind = kind.from_ruby()?;
        builder.output(kind).map(|_| ()).into_ruby()
    }

    fn output_by_plugin(&self, handle: u32) -> Result<()> {
        let mut builder = self.0.borrow_mut();

        let Some(builder) = &mut *builder else {
            let error = magnus::Error::new(
                magnus::exception::runtime_error(),
                "SystemBuilder is already built",
            );
            return Err(error);
        };

        builder.output_by_plugin(handle).map(|_| ()).into_ruby()
    }

    fn build(&self, max_channels: i32, flags: InitFlags) -> Result<System> {
        let mut builder = self.0.borrow_mut();

        let Some(builder) = builder.take() else {
            let error = magnus::Error::new(
                magnus::exception::runtime_error(),
                "SystemBuilder is already built",
            );
            return Err(error);
        };

        builder.build(max_channels, flags.from_ruby()?).into_ruby()
    }
}

impl IntoRuby<SystemBuilder> for fmod::SystemBuilder {
    fn into_ruby(self) -> Result<SystemBuilder> {
        Ok(SystemBuilder(RefCell::new(Some(self))))
    }
}

extern_struct_bind! {
  impl Bindable for SystemBuilder: fmod::SystemBuilder {
    fn software_format -> 3;
    fn software_channels -> 1;
    fn dsp_buffer_size -> 2;
    fn output -> 1;
    fn output_by_plugin -> 1;
    fn build -> 2;

    |class| {
      class.define_singleton_method("new", magnus::function!(SystemBuilder::new, 0))?;
    }
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::SystemBuilder::bind(module)?;

    Ok(())
}
