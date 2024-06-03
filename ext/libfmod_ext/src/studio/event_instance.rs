// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::core::structs::Attributes3D;
use crate::{Bindable, FromRuby, IntoRuby, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

use super::enums::{EventProperty, PlaybackState, StopMode};
use super::event_description::RbEventDescription;
use super::structs::ParameterID;

extern_struct! {
    struct EventInstance: fmod::studio::EventInstance => "FMOD::Studio::EventInstance"
}

extern_struct_fns! {
    impl EventInstance: fmod::studio::EventInstance {
      fn set_3d_attributes(attributes: Attributes3D) -> ();
      fn get_3d_attributes() -> Attributes3D;
      fn set_listener_mask(mask: u32) -> ();
      fn get_listener_mask() -> u32;
      fn get_min_max_distance() -> (f32, f32);
      // TODO userdata & callbacks
      fn get_description() -> RbEventDescription;
      fn release() -> ();
      fn is_valid() -> bool;
      // FIXME: default parameters
      fn set_parameter_by_name(name: magnus::RString, value: f32, ignore_seek_speed: bool) -> ();
      fn set_parameter_by_name_with_label(name: magnus::RString, label: magnus::RString, ignore_seek_speed: bool) -> ();
      fn get_parameter_by_name(name: magnus::RString) -> (f32, f32);
      fn set_parameter_by_id(id: ParameterID, valye: f32, ignore_seek_speed: bool) -> ();
      fn set_parameter_by_id_with_label(id: ParameterID, label: magnus::RString, ignore_seek_speed: bool) -> ();
      fn get_parameter_by_id(id: ParameterID) -> (f32, f32);
      fn start() -> ();
      fn stop(stop_mode: StopMode) -> ();
      fn get_playback_state() -> PlaybackState;
      fn set_paused(paused: bool) -> ();
      fn get_paused() -> bool;
      fn key_off() -> ();
      fn set_pitch(pitch: f32) -> ();
      fn get_pitch() -> (f32, f32);
      fn set_property(property: EventProperty, value: f32) -> ();
      fn get_property(property: EventProperty) -> f32;
      fn set_timeline_position(position: i32) -> ();
      fn get_timeline_position() -> i32;
      fn set_volume(volume: f32) -> ();
      fn get_volume() -> (f32, f32);
      fn is_virtual() -> bool;
    }
}

// FIXME turn some get/set methods into properties

impl EventInstance {
    // have to handwrite this one unfortunately, slice conversion is a bit tricky
    // if set_parameters_by_ids took an AsRef<T> though...
    // FIXME do the above
    fn set_parameter_by_ids(
        &self,
        ids: magnus::RArray,
        values: magnus::RArray,
        ignore_seek_speed: bool,
    ) -> Result<()> {
        let ids = ids.typecheck::<ParameterID>()?;
        let ids: Vec<fmod::studio::ParameterID> = ids
            .into_iter()
            .map(FromRuby::from_ruby)
            .collect::<Result<_>>()?;
        let mut values = values.to_vec()?;

        self.0
            .set_parameters_by_ids(&ids, &mut values, ignore_seek_speed)
            .into_ruby()?;

        Ok(())
    }
}

extern_struct_bind! {
    impl Bindable for EventInstance: fmod::studio::EventInstance {
      fn set_3d_attributes -> 1;
      fn get_3d_attributes -> 0;
      fn set_listener_mask -> 1;
      fn get_listener_mask -> 0;
      fn get_min_max_distance -> 0;
      fn get_description -> 0;
      fn release -> 0;
      fn is_valid -> 0;
      fn set_parameter_by_name -> 3;
      fn set_parameter_by_name_with_label -> 3;
      fn get_parameter_by_name -> 1;
      fn set_parameter_by_id -> 3;
      fn set_parameter_by_id_with_label -> 3;
      fn get_parameter_by_id -> 1;
      fn set_parameter_by_ids -> 3;
      fn start -> 0;
      fn stop -> 1;
      fn get_playback_state -> 0;
      fn set_paused -> 1;
      fn get_paused -> 0;
      fn key_off -> 0;
      fn set_pitch -> 1;
      fn get_pitch -> 0;
      fn set_property -> 2;
      fn get_property -> 1;
      fn set_timeline_position -> 1;
      fn get_timeline_position -> 0;
      fn set_volume -> 1;
      fn get_volume -> 0;
      fn is_virtual -> 0;
    }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::studio::EventInstance::bind(module)?;

    Ok(())
}
