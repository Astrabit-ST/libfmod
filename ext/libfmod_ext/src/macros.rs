// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod::{Utf8CStr, Utf8CString};
use magnus::Class;

use crate::Result;

#[macro_export]
macro_rules! extern_struct {
    (struct $name:ident: $fmod_ty:path => $ruby_path:literal) => {
        #[magnus::wrap(class = $ruby_path, free_immediately, size)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct $name(pub $fmod_ty);

        impl $crate::FromRuby<$fmod_ty> for $name {
            fn from_ruby(self) -> $crate::Result<$fmod_ty> {
                Ok(self.0)
            }
        }

        impl $crate::IntoRuby<$name> for $fmod_ty {
            fn into_ruby(self) -> $crate::Result<$name> {
                Ok($name(self))
            }
        }
    };
}

#[macro_export]
macro_rules! extern_struct_fns {
    (impl $name:ident: $fmod_ty:path {
      $( fn $fn_name:ident($($arg_name:ident: $arg_type:ty),* $(,)?) -> $fn_return:ty );* $(;)?
    }) => {
      impl $name {
        $(
          pub(crate) fn $fn_name(&self, $($arg_name: $arg_type),*) -> $crate::Result<$fn_return> {
            #[allow(unused_imports)]
            use $crate::{FromRuby, IntoRuby};
            let this: $fmod_ty = self.from_ruby()?;
            this.$fn_name($($arg_name.from_ruby()?),*).into_ruby()
          }
        )*
      }
    };
}

#[macro_export]
macro_rules! extern_struct_bind {
    (impl Bindable for $name:ident: $fmod_ty:path $( , super = $superclass:path )? $( , class_name = $class_name:literal )? {
      $( fn $fn_name:ident -> $arity:literal );* $(;)?
      $( |$class_ident:ident| $block:block)?
    }) => {
      const _: () ={
        static CLASS: once_cell::sync::OnceCell<magnus::value::Opaque<magnus::RClass>> = once_cell::sync::OnceCell::new();
        impl $crate::Bindable for $fmod_ty {
          #[allow(unused_imports, unused_variables)]
          fn bind(module: impl magnus::Module) -> $crate::Result<()> {
            use magnus::Module;
            let _superclass = magnus::class::object();
            $( let _superclass = $superclass(); )?
            let class_name = stringify!($name);
            $( let class_name = $class_name; )?
            let class = module.define_class(class_name, _superclass)?;
            $(
              paste::paste! {
                class.define_method(stringify!($fn_name), magnus::method!($name::$fn_name, $arity))?;
              }
            )*
            let _ = CLASS.set(class.into());
            $( let $class_ident = class; $block )?
            Ok(())
          }

          #[allow(refining_impl_trait)]
          fn class() -> magnus::RClass {
            use magnus::value::InnerValue;
            let ruby = magnus::Ruby::get().unwrap();
            CLASS.get().unwrap().get_inner_with(&ruby)
          }
        }
      };
    };
}

#[macro_export]
macro_rules! num_enum {
    (
      #[repr($repr:ty)]
      enum $name:ident: $fmod_ty:path {
        $( $variant:ident ),* $(,)?
      }
    ) => {
      pub type $name = $repr;
      const _: () = {
        type _Wrapped = $fmod_ty;
        static MODULE: once_cell::sync::OnceCell<magnus::value::Opaque<magnus::RModule>> = once_cell::sync::OnceCell::new();
        impl $crate::FromRuby<_Wrapped> for $name {
          fn from_ruby(self) -> $crate::Result<_Wrapped> {
            let _wrapped: _Wrapped = _Wrapped::try_from(self)
              .map_err(|e|
                magnus::Error::new($crate::error::class(), e.to_string())
              )?;
            Ok(_wrapped)
          }
        }

        impl $crate::IntoRuby<$name> for _Wrapped {
          fn into_ruby(self) -> $crate::Result<$name> {
            Ok(self.into())
          }
        }

        #[allow(unused_imports, unused_variables)]
        impl $crate::Bindable for $fmod_ty {
          fn bind(module: impl magnus::Module) -> $crate::Result<()> {
            use magnus::Module;
            use $crate::{IntoRuby};
            let module = module.define_module(stringify!($name))?;
            $(
              module.const_set(stringify!($variant), _Wrapped::$variant.into_ruby()?)?;
            )*
            let _ = MODULE.set(module.into());
            Ok(())
          }

          #[allow(refining_impl_trait)]
          fn class() -> magnus::RModule {
            use magnus::value::InnerValue;
            let ruby = magnus::Ruby::get().unwrap();
            MODULE.get().unwrap().get_inner_with(&ruby)
          }
        }
      };
    };
}

#[macro_export]
macro_rules! ruby_struct {
    (struct $name:ident: $fmod_ty:path {
      $( $member:ident: $member_ty:ty),* $(,)?
    }) => {
      pub type $name = magnus::RStruct;
      const _: () = {
        static CLASS: once_cell::sync::OnceCell<magnus::value::Opaque<magnus::RClass>> = once_cell::sync::OnceCell::new();
        type _Wrapped = $fmod_ty;
        impl $crate::FromRuby<_Wrapped> for $name {
          fn from_ruby(self) -> $crate::Result<_Wrapped> {
            $(
              let $member: $member_ty = self.aref(stringify!($member))?;
              let $member = $member.from_ruby()?;
            )*
            Ok(_Wrapped {
              $(
                  $member,
              )*
            })
          }
        }

        impl $crate::IntoRuby<$name> for _Wrapped {
          fn into_ruby(self) -> $crate::Result<$name> {
            use magnus::{Class, value::InnerValue, TryConvert};
            let ruby = magnus::Ruby::get().unwrap();
            let rstruct = CLASS.get().unwrap().get_inner_with(&ruby);
            let rstruct = rstruct.new_instance((
              $(
                self.$member.into_ruby()?,
              )*
            ))?;
            $name::try_convert(rstruct)
          }
        }

        impl $crate::Bindable for _Wrapped {
          fn bind(module: impl magnus::Module) -> $crate::Result<()> {
            let class: magnus::RClass = magnus::r_struct::define_struct(
              Some(stringify!($name)),
              (
                $( stringify!($member), )*
              )
            )?;
            let _ = CLASS.set(class.into());
            module.const_set(stringify!($name), class)
          }

          #[allow(refining_impl_trait)]
          fn class() -> magnus::RClass {
            use magnus::value::InnerValue;
            let ruby = magnus::Ruby::get().unwrap();
            CLASS.get().unwrap().get_inner_with(&ruby)
          }
        }
      };
    };
}

#[macro_export]
macro_rules! ruby_bitflags {
    (#[repr($repr_ty:ty)] mod $flag_name:ident: $fmod_ty:path {
      $( const $flag:ident; )* $(;)?
    }) => {
      pub type $flag_name = $repr_ty; // ideally we would use bitflags::Flags::Bits here but it gives weird conflicts with the FromRuby<Result> impl
      const _: () = {
        use magnus::Module;
        type _Wrapped = $fmod_ty;
        static MODULE: once_cell::sync::OnceCell<magnus::value::Opaque<magnus::RModule>> = once_cell::sync::OnceCell::new();

        impl $crate::FromRuby<_Wrapped> for $flag_name {
          fn from_ruby(self) -> $crate::Result<_Wrapped> {
              Ok(self.into())
          }
        }

        impl $crate::IntoRuby<$flag_name> for _Wrapped {
          fn into_ruby(self) -> $crate::Result<$flag_name> {
              Ok(self.into())
          }
        }

        impl $crate::Bindable for _Wrapped {
          fn bind(module: impl Module) -> $crate::Result<()> {
            use $crate::IntoRuby;
            let class = module.define_module(stringify!($flag_name))?;
            $(
              class.const_set::<_, $flag_name>(stringify!($flag), _Wrapped::$flag.into_ruby()?)?;
            )*
            let _ = MODULE.set(class.into());
            Ok(())
          }

          #[allow(refining_impl_trait)]
          fn class() -> magnus::RModule {
            use magnus::value::InnerValue;
            let ruby = magnus::Ruby::get().unwrap();
            MODULE.get().unwrap().get_inner_with(&ruby)
          }
        }
      };
    };
}

pub trait IntoRuby<T> {
    fn into_ruby(self) -> Result<T>;
}

// FIXME change the trait definition to be like From this has confusing semantics
pub trait FromRuby<T>: Sized {
    #[allow(clippy::wrong_self_convention)]
    fn from_ruby(self) -> Result<T>;
}

pub trait Bindable {
    fn bind(module: impl magnus::Module) -> Result<()>;

    fn class() -> impl magnus::Module;
}

macro_rules! identity_impl {
    ($($identity:ty),*) => {
      $(
        impl $crate::IntoRuby<$identity> for $identity {
            fn into_ruby(self) -> Result<$identity> {
                Ok(self)
            }
        }

        impl $crate::FromRuby<$identity> for $identity {
            fn from_ruby(self) -> Result<$identity> {
                Ok(self)
            }
        }
      )*
    };
}

identity_impl!(
    u8,
    u16,
    u32,
    u64,
    usize,
    i8,
    i16,
    i32,
    i64,
    isize,
    f32,
    f64,
    bool,
    String,
    char,
    ()
);

// ruby strings are usually null-terminated, so we can freely convert them to Utf8 C strings
unsafe fn ruby_string_as_cstr(string: magnus::RString) -> Result<*mut i8> {
    use magnus::rb_sys::AsRawValue;
    use magnus::value::ReprValue;

    let mut value = string.as_value().as_raw();
    let mut ptr = std::ptr::null_mut();
    magnus::rb_sys::protect(|| {
        ptr = rb_sys::rb_string_value_cstr(&mut value);

        rb_sys::Qnil.into()
    })?;
    Ok(ptr)
}

impl<'a> IntoRuby<magnus::RString> for &'a Utf8CStr {
    fn into_ruby(self) -> Result<magnus::RString> {
        Ok(magnus::RString::new(self))
    }
}

impl<'a> FromRuby<&'a Utf8CStr> for magnus::RString {
    fn from_ruby(self) -> Result<&'a Utf8CStr> {
        // FIXME assert UTF-8
        unsafe {
            // this is pretty dangerous because the lifetime of the string is not tied to the lifetime of the Ruby object
            // its... fine for now
            let ptr = ruby_string_as_cstr(self)?;
            let cstr = Utf8CStr::from_ptr_unchecked(ptr);
            Ok(cstr)
        }
    }
}

impl<'a> FromRuby<&'a [u8]> for magnus::RString {
    fn from_ruby(self) -> Result<&'a [u8]> {
        use magnus::{rb_sys::AsRawValue, value::ReprValue};
        // this is really dangerous but all uses of this wont be holding onto the slice long
        Ok(unsafe {
            let value = self.as_value().as_raw();
            let ptr = rb_sys::RSTRING_PTR(value);
            let len = rb_sys::RSTRING_LEN(value);
            std::slice::from_raw_parts(ptr.cast(), len as usize)
        })
    }
}

impl IntoRuby<magnus::RString> for Utf8CString {
    fn into_ruby(self) -> Result<magnus::RString> {
        Ok(magnus::RString::new(&self))
    }
}

impl FromRuby<Utf8CString> for magnus::RString {
    fn from_ruby(self) -> Result<Utf8CString> {
        // Quite a lot safer than the above impl
        // Still pretty awful though!
        unsafe {
            let ptr = ruby_string_as_cstr(self)?;
            let cstr = Utf8CStr::from_ptr_unchecked(ptr).to_cstring();
            Ok(cstr)
        }
    }
}

macro_rules! tuple_wrap {
    ($(
      ($($n:tt $name:ident)+)
    )+) => {
        $(
          paste::paste! {
            impl<$( $name, [<$name Wrap>],)*> $crate::IntoRuby<( $( [<$name Wrap>], )* )> for ( $( $name, )* )
            where $( $name: $crate::IntoRuby<[<$name Wrap>]>, )*
            {
                fn into_ruby(self) ->
                    $crate::Result<(
                        $(
                            [<$name Wrap>],
                        )*
                    )>
                {
                    Ok((
                        $(
                            self.$n.into_ruby()?,
                        )*
                    ))
                }
            }

            impl<$( $name, [<$name Unwrap>],)*> $crate::FromRuby<( $( [<$name Unwrap>], )* )> for ( $( $name, )* )
            where $( $name: $crate::FromRuby<[<$name Unwrap>]>, )*
            {
                fn from_ruby(self) ->
                    $crate::Result<(
                        $(
                            [<$name Unwrap>],
                        )*
                    )>
                {
                    Ok((
                        $(
                            self.$n.from_ruby()?,
                        )*
                    ))
                }
            }
          }
        )+
    };
}

tuple_wrap! {
  (0 T0)
  (0 T0 1 T1)
  (0 T0 1 T1 2 T2)
  (0 T0 1 T1 2 T2 3 T3)
  (0 T0 1 T1 2 T2 3 T3 4 T4)
  (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
  (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
  (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
  (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
  (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
  (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
  (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
  (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
  (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
  (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
  (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

// magnus does not provide an impl of IntoValue for arrays so we have to use magnus::RArray
impl<const N: usize, T> IntoRuby<magnus::RArray> for [T; N]
where
    T: magnus::IntoValue,
{
    fn into_ruby(self) -> Result<magnus::RArray> {
        let arr = magnus::RArray::with_capacity(N);
        for item in self.into_iter() {
            arr.push(item.into_value())?;
        }
        Ok(arr)
    }
}

impl<const N: usize, T, TUnwrap> FromRuby<[TUnwrap; N]> for [T; N]
where
    T: FromRuby<TUnwrap>,
{
    fn from_ruby(self) -> Result<[TUnwrap; N]> {
        use std::mem::MaybeUninit;

        let mut result: [MaybeUninit<TUnwrap>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for (i, item) in self.into_iter().enumerate().skip(1) {
            let item = item.from_ruby()?;
            result[i].write(item);
        }
        // we initialize every element in the array, so this is safe
        let result = result.map(|item| unsafe { item.assume_init() });
        Ok(result)
    }
}

// currently unreleased feature. we need to use it to constraint TWrap, otherwise rust complains
impl<T, TWrap> IntoRuby<magnus::r_array::TypedArray<TWrap>> for Vec<T>
where
    T: IntoRuby<TWrap>,
    TWrap: magnus::IntoValue,
{
    fn into_ruby(self) -> Result<magnus::r_array::TypedArray<TWrap>> {
        let ruby = unsafe { magnus::Ruby::get_unchecked() };
        let arr = ruby.typed_ary_new();
        for item in self.into_iter() {
            arr.push(item.into_ruby()?)?;
        }
        Ok(arr)
    }
}

impl<T, TWrap> IntoRuby<Vec<TWrap>> for Vec<T>
where
    T: IntoRuby<TWrap>,
{
    fn into_ruby(self) -> Result<Vec<TWrap>> {
        self.into_iter().map(|item| item.into_ruby()).collect()
    }
}

impl<T, TWrap> FromRuby<Vec<TWrap>> for Vec<T>
where
    T: FromRuby<TWrap>,
{
    fn from_ruby(self) -> Result<Vec<TWrap>> {
        self.into_iter().map(|item| item.from_ruby()).collect()
    }
}

impl<T, TWrap> IntoRuby<TWrap> for fmod::Result<T>
where
    T: IntoRuby<TWrap>,
{
    fn into_ruby(self) -> Result<TWrap> {
        self.map_err(|e| magnus::Error::new(crate::error::class(), e.to_string()))?
            .into_ruby()
    }
}

impl<T, TWrap> FromRuby<TWrap> for fmod::Result<T>
where
    T: FromRuby<TWrap>,
{
    fn from_ruby(self) -> Result<TWrap> {
        self.map_err(|e| magnus::Error::new(crate::error::class(), e.to_string()))?
            .from_ruby()
    }
}

impl IntoRuby<magnus::Exception> for fmod::Error {
    fn into_ruby(self) -> Result<magnus::Exception> {
        crate::error::class().new_instance((self.to_string(),))
    }
}

impl<T, TWrap> IntoRuby<Option<TWrap>> for Option<T>
where
    T: IntoRuby<TWrap>,
{
    fn into_ruby(self) -> Result<Option<TWrap>> {
        self.map(|item| item.into_ruby()).transpose()
    }
}

impl<T, TWrap> FromRuby<Option<TWrap>> for Option<T>
where
    T: FromRuby<TWrap>,
{
    fn from_ruby(self) -> Result<Option<TWrap>> {
        self.map(|item| item.from_ruby()).transpose()
    }
}

impl<T, TWrap> FromRuby<TWrap> for &T
where
    T: Clone,
    T: FromRuby<TWrap>,
{
    fn from_ruby(self) -> Result<TWrap> {
        T::from_ruby(self.clone())
    }
}
