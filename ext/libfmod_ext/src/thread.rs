// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::c_void;
use std::mem::MaybeUninit;

use magnus::rb_sys::{AsRawValue, FromRawValue};

/// # Safety
///
/// Must be called on a ruby thread with the gvl acquired.
/// You must not call ruby code from inside the callback, unless the gvl has been reaquired with [`with_gvl`].
pub unsafe fn without_gvl<F: FnOnce() -> R, R>(f: F) -> R {
    unsafe extern "C" fn gvl_fun<F: FnOnce() -> R, R>(function: *mut c_void) -> *mut c_void {
        let (f, r) = std::ptr::read::<(F, *mut R)>(function.cast());

        let result = f();
        std::ptr::write(r, result);

        std::ptr::null_mut()
    }

    // f is initialized before we call rb_thread_call_without_gvl.
    let function = MaybeUninit::new(f);
    let mut result = MaybeUninit::uninit();

    let mut param_tuple = (function, &mut result);

    rb_sys::rb_thread_call_without_gvl(
        Some(gvl_fun::<F, R>),
        std::ptr::from_mut(&mut param_tuple).cast(), // we effectively move f into gvl_fun.
        None,
        std::ptr::null_mut(),
    );
    // f is now uninitialized.

    result.assume_init()
}

/// # Safety
///
/// Must be called on a ruby thread without the gvl acquired.
/// You must not use the [`magnus::Ruby`] parameter outside of the callback.
pub unsafe fn with_gvl<F: FnOnce(&magnus::Ruby) -> R, R>(f: F) -> R {
    unsafe extern "C" fn gvl_fun<F: FnOnce(&magnus::Ruby) -> R, R>(
        function: *mut c_void,
    ) -> *mut c_void {
        let (f, r) = std::ptr::read::<(F, *mut R)>(function.cast());
        let ruby = magnus::Ruby::get_unchecked();

        let result = f(&ruby);
        std::ptr::write(r, result);

        std::ptr::null_mut()
    }

    // f is initialized before we call rb_thread_call_without_gvl.
    let function = MaybeUninit::new(f);
    let mut result = MaybeUninit::uninit();

    let mut param_tuple = (function, &mut result);

    rb_sys::rb_thread_call_with_gvl(
        Some(gvl_fun::<F, R>),
        std::ptr::from_mut(&mut param_tuple).cast(), // we effectively move f into gvl_fun.
    );
    // f is now uninitialized.

    result.assume_init()
}

pub unsafe fn spawn_ruby_thread<F>(f: F) -> magnus::Value
where
    F: FnOnce(&magnus::Ruby) -> magnus::Value + Send + 'static,
{
    unsafe extern "C" fn thread_fn<F>(data: *mut c_void) -> rb_sys::VALUE
    where
        F: FnOnce(&magnus::Ruby) -> magnus::Value + Send + 'static,
    {
        let f = Box::<F>::from_raw(data.cast());
        let ruby = magnus::Ruby::get_unchecked();

        f(&ruby).as_raw()
    }

    let boxed_func = Box::new(f);
    let value = rb_sys::rb_thread_create(Some(thread_fn::<F>), Box::into_raw(boxed_func).cast());
    magnus::Value::from_raw(value)
}
