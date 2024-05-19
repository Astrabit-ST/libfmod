// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]

use std::cell::RefCell;

use magnus::value::ReprValue;
use magnus::Object;

use crate::{Bindable, FromRuby, IntoRuby, Result};

use crate::extern_struct_bind;

use super::enums::{ChannelOrder, SoundFormat, TimeUnit};
use super::flags::Mode;
use super::sound_group::SoundGroup;

#[magnus::wrap(class = "FMOD::SoundBuilder", free_immediately, size)]
pub struct SoundBuilder(pub(super) RefCell<Option<fmod::SoundBuilder<'static>>>);
type _SoundBuilder = magnus::typed_data::Obj<SoundBuilder>;

unsafe impl Send for SoundBuilder {}
unsafe impl Sync for SoundBuilder {}

impl IntoRuby<SoundBuilder> for fmod::SoundBuilder<'static> {
    fn into_ruby(self) -> Result<SoundBuilder> {
        Ok(SoundBuilder(RefCell::new(Some(self))))
    }
}

// should never be called
impl FromRuby<fmod::SoundBuilder<'static>> for &SoundBuilder {
    fn from_ruby(self) -> Result<fmod::SoundBuilder<'static>> {
        unimplemented!()
    }
}

impl SoundBuilder {
    pub fn open(rb_name: magnus::RString) -> Result<_SoundBuilder> {
        // This is technically REALLY dangerous! We're creating a SoundBuilder with an arbitrary lifetime AND storing a pointer to the inside of the Ruby string.
        // We can make this better somewhat be freezing the string and storing it but this is still nasty.
        let name = rb_name.from_ruby()?;
        let builder = fmod::SoundBuilder::open(name);
        let rb_self = _SoundBuilder::wrap(builder.into_ruby()?);
        rb_self.ivar_set("name", rb_name)?;
        rb_name.freeze();
        Ok(rb_self)
    }

    pub fn open_memory(rb_data: magnus::RString) -> Result<_SoundBuilder> {
        // This is technically REALLY dangerous! We're creating a SoundBuilder with an arbitrary lifetime AND storing a pointer to the inside of the Ruby string.
        // We can make this better somewhat be freezing the string and storing it but this is still nasty.
        // What's even worse is that we can't even free the string until the sound is loaded...
        let data = rb_data.from_ruby()?;
        let builder = unsafe { fmod::SoundBuilder::open_memory(data) };
        let rb_self = _SoundBuilder::wrap(builder.into_ruby()?);
        rb_self.ivar_set("data", rb_data)?;
        rb_data.freeze();
        Ok(rb_self)
    }

    // no open memory point because that is WAY too dangerous
}

impl SoundBuilder {
    fn with_file_offset(this: _SoundBuilder, offset: u32) -> Result<_SoundBuilder> {
        let mut borrow = this.0.borrow_mut();
        let builder = borrow.take().ok_or_else(Self::invalid_state_error)?;
        let builder = builder.with_file_offset(offset);
        *borrow = Some(builder);
        Ok(this)
    }

    fn with_open_raw(
        this: _SoundBuilder,
        channel_count: i32,
        default_frequency: i32,
        format: SoundFormat,
    ) -> Result<_SoundBuilder> {
        let format = format.from_ruby()?; // do this before so on error the builder is still valid

        let mut borrow = this.0.borrow_mut();
        let builder = borrow.take().ok_or_else(Self::invalid_state_error)?;
        let builder = builder.with_open_raw(channel_count, default_frequency, format);
        *borrow = Some(builder);
        Ok(this)
    }

    fn with_mode(this: _SoundBuilder, mode: Mode) -> Result<_SoundBuilder> {
        let mode = mode.from_ruby()?; // do this before so on error the builder is still valid

        let mut borrow = this.0.borrow_mut();
        let builder = borrow.take().ok_or_else(Self::invalid_state_error)?;
        let builder = builder.with_mode(mode);
        *borrow = Some(builder);
        Ok(this)
    }

    fn with_decode_buffer_size(this: _SoundBuilder, size: u32) -> Result<_SoundBuilder> {
        let mut borrow = this.0.borrow_mut();
        let builder = borrow.take().ok_or_else(Self::invalid_state_error)?;
        let builder = builder.with_decode_buffer_size(size);
        *borrow = Some(builder);
        Ok(this)
    }

    fn with_initial_subsound(this: _SoundBuilder, subsound: i32) -> Result<_SoundBuilder> {
        let mut borrow = this.0.borrow_mut();
        let builder = borrow.take().ok_or_else(Self::invalid_state_error)?;
        let builder = builder.with_initial_subsound(subsound);
        *borrow = Some(builder);
        Ok(this)
    }

    // no idea how to do inclusion list (possibly use a rstring) but that would be dangerous and result in possibly misaligned pointers

    fn with_dls_name(this: _SoundBuilder, rb_name: magnus::RString) -> Result<_SoundBuilder> {
        let name = rb_name.from_ruby()?; // do this before so on error the builder is still valid

        let mut borrow = this.0.borrow_mut();
        let builder = borrow.take().ok_or_else(Self::invalid_state_error)?;
        let builder = builder.with_dls_name(name);
        *borrow = Some(builder);

        // this is pretty dangerous but its the best we can do
        this.ivar_set("dls_name", rb_name)?; // store the name so it doesn't get freed
        rb_name.freeze(); // also freeze it so it doesn't get modified

        Ok(this)
    }

    fn with_encryption_key(this: _SoundBuilder, rb_key: magnus::RString) -> Result<_SoundBuilder> {
        let key = rb_key.from_ruby()?; // do this before so on error the builder is still valid

        let mut borrow = this.0.borrow_mut();
        let builder = borrow.take().ok_or_else(Self::invalid_state_error)?;
        let builder = builder.with_encryption_key(key);
        *borrow = Some(builder);

        // this is pretty dangerous but its the best we can do
        this.ivar_set("encryption_key", rb_key)?; // store the key so it doesn't get freed
        rb_key.freeze(); // also freeze it so it doesn't get modified

        Ok(this)
    }

    fn with_max_polyphony(this: _SoundBuilder, max: i32) -> Result<_SoundBuilder> {
        let mut borrow = this.0.borrow_mut();
        let builder = borrow.take().ok_or_else(Self::invalid_state_error)?;
        let builder = builder.with_max_polyphony(max);
        *borrow = Some(builder);
        Ok(this)
    }

    fn with_suggested_sound_type(
        this: _SoundBuilder,
        sound_type: SoundFormat,
    ) -> Result<_SoundBuilder> {
        let sound_type = sound_type.from_ruby()?; // do this before so on error the builder is still valid

        let mut borrow = this.0.borrow_mut();
        let builder = borrow.take().ok_or_else(Self::invalid_state_error)?;
        let builder = builder.with_suggested_sound_type(sound_type);
        *borrow = Some(builder);
        Ok(this)
    }

    fn with_file_buffer_size(this: _SoundBuilder, size: i32) -> Result<_SoundBuilder> {
        let mut borrow = this.0.borrow_mut();
        let builder = borrow.take().ok_or_else(Self::invalid_state_error)?;
        let builder = builder.with_file_buffer_size(size);
        *borrow = Some(builder);
        Ok(this)
    }

    fn with_channel_order(this: _SoundBuilder, order: ChannelOrder) -> Result<_SoundBuilder> {
        let order = order.from_ruby()?; // do this before so on error the builder is still valid

        let mut borrow = this.0.borrow_mut();
        let builder = borrow.take().ok_or_else(Self::invalid_state_error)?;
        let builder = builder.with_channel_order(order);
        *borrow = Some(builder);
        Ok(this)
    }

    fn with_initial_sound_group(this: _SoundBuilder, group: &SoundGroup) -> Result<_SoundBuilder> {
        let group = group.from_ruby()?; // do this before so on error the builder is still valid

        let mut borrow = this.0.borrow_mut();
        let builder = borrow.take().ok_or_else(Self::invalid_state_error)?;
        let builder = builder.with_initial_sound_group(group);
        *borrow = Some(builder);

        Ok(this)
    }

    fn with_initial_seek_position(
        this: _SoundBuilder,
        position: u32,
        unit: TimeUnit,
    ) -> Result<_SoundBuilder> {
        let unit = unit.from_ruby()?; // do this before so on error the builder is still valid

        let mut borrow = this.0.borrow_mut();
        let builder = borrow.take().ok_or_else(Self::invalid_state_error)?;
        let builder = builder.with_initial_seek_position(position, unit);
        *borrow = Some(builder);
        Ok(this)
    }

    fn with_ignore_set_filesystem(this: _SoundBuilder, ignore: bool) -> Result<_SoundBuilder> {
        let mut borrow = this.0.borrow_mut();
        let builder = borrow.take().ok_or_else(Self::invalid_state_error)?;
        let builder = builder.with_ignore_set_filesystem(ignore);
        *borrow = Some(builder);
        Ok(this)
    }

    fn with_min_midi_granularity(this: _SoundBuilder, granularity: u32) -> Result<_SoundBuilder> {
        let mut borrow = this.0.borrow_mut();
        let builder = borrow.take().ok_or_else(Self::invalid_state_error)?;
        let builder = builder.with_min_midi_granularity(granularity);
        *borrow = Some(builder);
        Ok(this)
    }

    fn with_non_block_thread_id(this: _SoundBuilder, thread_id: i32) -> Result<_SoundBuilder> {
        let mut borrow = this.0.borrow_mut();
        let builder = borrow.take().ok_or_else(Self::invalid_state_error)?;
        let builder = builder.with_non_block_thread_id(thread_id);
        *borrow = Some(builder);
        Ok(this)
    }

    // no idea how to do fsb guid

    pub(super) fn invalid_state_error() -> magnus::Error {
        magnus::Error::new(
            crate::error::class(),
            "SoundBuilder is in an invalid state! PLEASE REPORT THIS.",
        )
    }
}

impl SoundBuilder {
    pub fn mode(&self) -> Result<Mode> {
        let borrow = self.0.borrow();
        let builder = borrow.as_ref().ok_or_else(Self::invalid_state_error)?;
        builder.mode().into_ruby()
    }

    pub fn name_or_url(this: _SoundBuilder) -> Result<Option<magnus::RString>> {
        this.ivar_get("name")
    }

    pub fn data(this: _SoundBuilder) -> Result<Option<magnus::RString>> {
        this.ivar_get("data")
    }

    pub fn length(&self) -> Result<u32> {
        let borrow = self.0.borrow();
        let builder = borrow.as_ref().ok_or_else(Self::invalid_state_error)?;
        Ok(builder.length())
    }

    pub fn file_offset(&self) -> Result<u32> {
        let borrow = self.0.borrow();
        let builder = borrow.as_ref().ok_or_else(Self::invalid_state_error)?;
        Ok(builder.file_offset())
    }

    pub fn default_frequency(&self) -> Result<i32> {
        let borrow = self.0.borrow();
        let builder = borrow.as_ref().ok_or_else(Self::invalid_state_error)?;
        Ok(builder.default_frequency())
    }

    pub fn format(&self) -> Result<SoundFormat> {
        let borrow = self.0.borrow();
        let builder = borrow.as_ref().ok_or_else(Self::invalid_state_error)?;
        builder.format().into_ruby()
    }

    pub fn decode_buffer_size(&self) -> Result<u32> {
        let borrow = self.0.borrow();
        let builder = borrow.as_ref().ok_or_else(Self::invalid_state_error)?;
        Ok(builder.decode_buffer_size())
    }

    pub fn initial_subsound(&self) -> Result<i32> {
        let borrow = self.0.borrow();
        let builder = borrow.as_ref().ok_or_else(Self::invalid_state_error)?;
        Ok(builder.initial_subsound())
    }

    pub fn subsound_count(&self) -> Result<i32> {
        let borrow = self.0.borrow();
        let builder = borrow.as_ref().ok_or_else(Self::invalid_state_error)?;
        Ok(builder.subsound_count())
    }

    pub fn dls_name(this: _SoundBuilder) -> Result<Option<magnus::RString>> {
        this.ivar_get("dls_name")
    }

    pub fn encryption_key(this: _SoundBuilder) -> Result<Option<magnus::RString>> {
        this.ivar_get("encryption_key")
    }

    pub fn max_polyphony(&self) -> Result<i32> {
        let borrow = self.0.borrow();
        let builder = borrow.as_ref().ok_or_else(Self::invalid_state_error)?;
        Ok(builder.max_polyphony())
    }

    pub fn suggested_sound_type(&self) -> Result<SoundFormat> {
        let borrow = self.0.borrow();
        let builder = borrow.as_ref().ok_or_else(Self::invalid_state_error)?;
        builder.suggested_sound_type().into_ruby()
    }

    pub fn file_buffer_size(&self) -> Result<i32> {
        let borrow = self.0.borrow();
        let builder = borrow.as_ref().ok_or_else(Self::invalid_state_error)?;
        Ok(builder.file_buffer_size())
    }

    pub fn channel_order(&self) -> Result<ChannelOrder> {
        let borrow = self.0.borrow();
        let builder = borrow.as_ref().ok_or_else(Self::invalid_state_error)?;
        builder.channel_order().into_ruby()
    }

    pub fn initial_sound_group(&self) -> Result<SoundGroup> {
        let borrow = self.0.borrow();
        let builder = borrow.as_ref().ok_or_else(Self::invalid_state_error)?;
        builder.initial_sound_group().into_ruby()
    }

    pub fn initial_seek_position(&self) -> Result<(u32, TimeUnit)> {
        let borrow = self.0.borrow();
        let builder = borrow.as_ref().ok_or_else(Self::invalid_state_error)?;
        let (position, unit) = builder.initial_seek_position();
        Ok((position, unit.into_ruby()?))
    }

    pub fn ignore_set_filesystem(&self) -> Result<bool> {
        let borrow = self.0.borrow();
        let builder = borrow.as_ref().ok_or_else(Self::invalid_state_error)?;
        Ok(builder.ignore_set_filesystem())
    }

    pub fn min_midi_granularity(&self) -> Result<u32> {
        let borrow = self.0.borrow();
        let builder = borrow.as_ref().ok_or_else(Self::invalid_state_error)?;
        Ok(builder.min_midi_granularity())
    }

    pub fn non_block_thread_id(&self) -> Result<i32> {
        let borrow = self.0.borrow();
        let builder = borrow.as_ref().ok_or_else(Self::invalid_state_error)?;
        Ok(builder.non_block_thread_id())
    }
}

extern_struct_bind! {
  impl Bindable for SoundBuilder: fmod::SoundBuilder<'static> {
    fn with_file_offset -> 1;
    fn with_open_raw -> 3;
    fn with_mode -> 1;
    fn with_decode_buffer_size -> 1;
    fn with_initial_subsound -> 1;
    fn with_dls_name -> 1;
    fn with_encryption_key -> 1;
    fn with_max_polyphony -> 1;
    fn with_suggested_sound_type -> 1;
    fn with_file_buffer_size -> 1;
    fn with_channel_order -> 1;
    fn with_initial_sound_group -> 1;
    fn with_initial_seek_position -> 2;
    fn with_ignore_set_filesystem -> 1;
    fn with_min_midi_granularity -> 1;
    fn with_non_block_thread_id -> 1;

    fn mode -> 0;
    fn name_or_url -> 0;
    fn data -> 0;
    fn length -> 0;
    fn file_offset -> 0;
    fn default_frequency -> 0;
    fn format -> 0;
    fn decode_buffer_size -> 0;
    fn initial_subsound -> 0;
    fn subsound_count -> 0;
    fn dls_name -> 0;
    fn encryption_key -> 0;
    fn max_polyphony -> 0;
    fn suggested_sound_type -> 0;
    fn file_buffer_size -> 0;
    fn channel_order -> 0;
    fn initial_sound_group -> 0;
    fn initial_seek_position -> 0;
    fn ignore_set_filesystem -> 0;
    fn min_midi_granularity -> 0;
    fn non_block_thread_id -> 0;

    |class| {
      class.define_singleton_method("open", magnus::function!(SoundBuilder::open, 1))?;
      class.define_singleton_method("open_memory", magnus::function!(SoundBuilder::open_memory, 1))?;
    }
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::SoundBuilder::bind(module)?;

    Ok(())
}
