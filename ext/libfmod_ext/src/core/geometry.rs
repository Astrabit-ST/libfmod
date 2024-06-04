// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(clippy::upper_case_acronyms)]
use magnus::prelude::*;

use crate::{Bindable, FromRuby, IntoRuby, Result};

use crate::{extern_struct, extern_struct_bind, extern_struct_fns};

use super::structs::Vector;

extern_struct! {
  struct Geometry: fmod::Geometry => "FMOD::Geometry"
}

impl Geometry {
    fn release(&self) -> Result<()> {
        use crate::{FromRuby, IntoRuby};
        // we dont need to check if the geometry is already removed, because FromRuby will return an error if it is
        let geometry: fmod::Geometry = self.from_ruby()?;
        crate::extern_struct_storage::remove(geometry);
        geometry.release().into_ruby()
    }

    fn get_userdata(rb_self: RbGeometry) -> Result<magnus::Value> {
        rb_self.ivar_get("__userdata")
    }

    fn set_userdata(rb_self: RbGeometry, data: magnus::Value) -> Result<()> {
        rb_self.ivar_set("__userdata", data)
    }
}

extern_struct_fns! {
  impl Geometry: fmod::Geometry {
    fn set_active(active: bool) -> ();
    fn get_active() -> bool;
    fn get_max_polygons() -> (i32, i32);
    fn get_polygon_count() -> i32;
    fn set_polygon_attributes(index: i32, direct_occlusion: f32, reverb_occlusion: f32, double_sided: bool) -> ();
    fn get_polygon_attributes(index: i32) -> (f32, f32, bool);
    fn get_polygon_vertex_count(index: i32) -> i32;
    fn set_polygon_vertex(index: i32, vertex_index: i32, vertex: Vector) -> ();
    fn get_polygon_vertex(index: i32, vertex_index: i32) -> Vector;
    fn set_position(position: Vector) -> ();
    fn get_position() -> Vector;
    fn set_rotation(forward: Vector, up: Vector) -> ();
    fn get_rotation() -> (Vector, Vector);
    fn set_scale(scale: Vector) -> ();
    fn get_scale() -> Vector;
  }
}

impl Geometry {
    fn add_polygon(
        &self,
        direct_occlusion: f32,
        reverb_occlusion: f32,
        double_sided: bool,
        vertices: magnus::RArray,
    ) -> Result<i32> {
        let vertices = vertices.typecheck::<Vector>()?;
        let vertices: Vec<fmod::Vector> = vertices
            .into_iter()
            .map(FromRuby::from_ruby)
            .collect::<Result<_>>()?;

        self.0
            .add_polygon(direct_occlusion, reverb_occlusion, double_sided, &vertices)
            .into_ruby()
    }

    fn save(&self) -> Result<magnus::RString> {
        let bytes: Vec<u8> = self.0.save().into_ruby()?;
        Ok(magnus::RString::from_slice(&bytes))
    }
}

extern_struct_bind! {
  impl Bindable for Geometry: fmod::Geometry {
    fn add_polygon -> 4;
    fn set_active -> 1;
    fn get_active -> 0;
    fn get_max_polygons -> 0;
    fn get_polygon_count -> 0;
    fn get_userdata -> 0;
    fn set_userdata -> 1;
    fn release -> 0;
    fn save -> 0;
    fn set_polygon_attributes -> 4;
    fn get_polygon_attributes -> 1;
    fn get_polygon_vertex_count -> 1;
    fn set_polygon_vertex -> 3;
    fn get_polygon_vertex -> 2;
    fn set_position -> 1;
    fn get_position -> 0;
    fn set_rotation -> 2;
    fn get_rotation -> 0;
    fn set_scale -> 1;
    fn get_scale -> 0;
    ruby_compat_methods: true
  }
}

pub fn bind(module: magnus::RModule) -> Result<()> {
    fmod::Geometry::bind(module)?;

    Ok(())
}
