// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod::{Utf8CStr, Utf8CString};

use crate::Result;

#[macro_export]
macro_rules! extern_struct {
    (struct $name:ident: $fmod_ty:path => $ruby_path:literal) => {
        #[magnus::wrap(class = $ruby_path, free_immediately, size)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct $name(pub $fmod_ty);

        impl $crate::UnwrapFMOD<$fmod_ty> for $name {
            fn unwrap_fmod(self) -> $crate::Result<$fmod_ty> {
                Ok(self.0)
            }
        }

        impl $crate::WrapFMOD<$name> for $fmod_ty {
            fn wrap_fmod(self) -> $crate::Result<$name> {
                Ok($name(self))
            }
        }
    };
}

#[macro_export]
macro_rules! extern_struct_fns {
    (impl $name:ident {
      $( fn $fn_name:ident($($arg_name:ident: $arg_type:ty),* $(,)?) -> $fn_return:ty );* $(;)?
    }) => {
      impl $name {
        $(
          pub fn $fn_name(&self, $($arg_name: $arg_type),*) -> $crate::Result<$fn_return> {
            #[allow(unused_imports)]
            use $crate::{UnwrapFMOD, WrapFMOD};
            self.0.$fn_name($($arg_name.unwrap_fmod()?),*).wrap_fmod()?.wrap_fmod()
          }
        )*
      }
    };
}

#[macro_export]
macro_rules! extern_struct_bind {
    (impl Bindable for $name:ident: $fmod_ty:path {
      $( fn $fn_name:ident -> $arity:literal );* $(;)?
      $( |$class_ident:ident| $block:block)?
    }) => {
      impl $crate::Bindable for $fmod_ty {
        fn bind(module: impl magnus::Module) -> $crate::Result<()> {
          use magnus::Module;
          let class = module.define_class(stringify!($name), magnus::class::object())?;
          $(
            paste::paste! {
              class.define_method(stringify!($fn_name), magnus::method!($name::$fn_name, $arity))?;
            }
          )*
          $( let $class_ident = class; $block )?
          Ok(())
        }
      }
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
        impl $crate::UnwrapFMOD<_Wrapped> for $name {
          fn unwrap_fmod(self) -> $crate::Result<_Wrapped> {
            let _wrapped = self.try_into().map_err(Into::into).wrap_fmod()?;
            Ok(_wrapped)
          }
        }

        impl $crate::WrapFMOD<$name> for _Wrapped {
          fn wrap_fmod(self) -> $crate::Result<$name> {
            Ok(self.into())
          }
        }

        #[allow(unused_imports, unused_variables)]
        impl $crate::Bindable for $fmod_ty {
          fn bind(module: impl magnus::Module) -> $crate::Result<()> {
            use magnus::Module;
            let module = module.define_module(stringify!($name))?;
            $(
              module.const_set(stringify!($variant), _Wrapped::$variant.wrap_fmod()?)?;
            )*
            Ok(())
          }
        }
      };
    };
}

#[macro_export]
macro_rules! ruby_struct {
    (struct $name:ident: $fmod_ty:path {
      $( $member:ident: $member_ty:ty ),* $(,)?
    }) => {
      pub type $name = magnus::RStruct;
      const _: () = {
        static CLASS: once_cell::sync::OnceCell<magnus::value::Opaque<magnus::RClass>> = once_cell::sync::OnceCell::new();
        type _Wrapped = $fmod_ty;
        impl $crate::UnwrapFMOD<_Wrapped> for $name {
          fn unwrap_fmod(self) -> $crate::Result<_Wrapped> {
            Ok(_Wrapped {
              $(
                  $member: self.aref::<_, $member_ty>(stringify!($member))?.unwrap_fmod()?,
              )*
            })
          }
        }

        impl $crate::WrapFMOD<$name> for _Wrapped {
          fn wrap_fmod(self) -> $crate::Result<$name> {
            use magnus::{Class, value::InnerValue, TryConvert};
            // FIXME put this in a Lazy somehow
            let ruby = magnus::Ruby::get().unwrap();
            let rstruct = CLASS.get().unwrap().get_inner_with(&ruby);
            let rstruct = rstruct.new_instance((
              $(
                self.$member.wrap_fmod()?,
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
        }
      };
    };
}

#[macro_export]
macro_rules! ruby_bitflags {
    (mod $flag_name:ident: $fmod_ty:path {
      $( const $flag:ident; )*
    }) => {
      pub type $flag_name = <$fmod_ty as bitflags::Flags>::Bits;
      const _: () = {
        use bitflags::Flags;
        use magnus::Module;
        type _Wrapped = $fmod_ty;

        impl $crate::UnwrapFMOD<_Wrapped> for $flag_name {
          fn unwrap_fmod(self) -> $crate::Result<_Wrapped> {
              Ok(self.into())
          }
        }

        impl $crate::WrapFMOD<$flag_name> for _Wrapped {
          fn wrap_fmod(self) -> $crate::Result<$flag_name> {
              Ok(self.into())
          }
        }

        impl $crate::Bindable for _Wrapped {
          fn bind(module: impl Module) -> $crate::Result<()> {
            let class = module.define_module(stringify!($flag_name))?;
            $(
              class.const_set::<_, $flag_name>(stringify!($flag), _Wrapped::$flag.wrap_fmod()?)?;
            )*
            Ok(())
          }
        }
      };
    };
}

pub trait WrapFMOD<T> {
    fn wrap_fmod(self) -> Result<T>;
}

impl<T> WrapFMOD<T> for fmod::Result<T> {
    fn wrap_fmod(self) -> Result<T> {
        self.map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), e.to_string()))
    }
}

pub trait UnwrapFMOD<T>: Sized {
    fn unwrap_fmod(self) -> Result<T>;
}

pub trait Bindable {
    fn bind(module: impl magnus::Module) -> Result<()>;
}

macro_rules! identity_impl {
    ($($identity:ty),*) => {
      $(
        impl $crate::WrapFMOD<$identity> for $identity {
            fn wrap_fmod(self) -> Result<$identity> {
                Ok(self)
            }
        }

        impl $crate::UnwrapFMOD<$identity> for $identity {
            fn unwrap_fmod(self) -> Result<$identity> {
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

impl<'a> WrapFMOD<magnus::RString> for &'a Utf8CStr {
    fn wrap_fmod(self) -> Result<magnus::RString> {
        Ok(magnus::RString::new(self))
    }
}

impl<'a> UnwrapFMOD<&'a Utf8CStr> for magnus::RString {
    fn unwrap_fmod(self) -> Result<&'a Utf8CStr> {
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

impl WrapFMOD<magnus::RString> for Utf8CString {
    fn wrap_fmod(self) -> Result<magnus::RString> {
        Ok(magnus::RString::new(&self))
    }
}

impl UnwrapFMOD<Utf8CString> for magnus::RString {
    fn unwrap_fmod(self) -> Result<Utf8CString> {
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
            impl<$( $name, [<$name Wrap>],)*> $crate::WrapFMOD<( $( [<$name Wrap>], )* )> for ( $( $name, )* )
            where $( $name: $crate::WrapFMOD<[<$name Wrap>]>, )*
            {
                fn wrap_fmod(self) ->
                    $crate::Result<(
                        $(
                            [<$name Wrap>],
                        )*
                    )>
                {
                    Ok((
                        $(
                            self.$n.wrap_fmod()?,
                        )*
                    ))
                }
            }

            impl<$( $name, [<$name Unwrap>],)*> $crate::UnwrapFMOD<( $( [<$name Unwrap>], )* )> for ( $( $name, )* )
            where $( $name: $crate::UnwrapFMOD<[<$name Unwrap>]>, )*
            {
                fn unwrap_fmod(self) ->
                    $crate::Result<(
                        $(
                            [<$name Unwrap>],
                        )*
                    )>
                {
                    Ok((
                        $(
                            self.$n.unwrap_fmod()?,
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
impl<const N: usize, T> WrapFMOD<magnus::RArray> for [T; N]
where
    T: magnus::IntoValue,
{
    fn wrap_fmod(self) -> Result<magnus::RArray> {
        let arr = magnus::RArray::with_capacity(N);
        for item in self.into_iter() {
            arr.push(item.into_value())?;
        }
        Ok(arr)
    }
}

impl<const N: usize, T, TUnwrap> UnwrapFMOD<[TUnwrap; N]> for [T; N]
where
    T: UnwrapFMOD<TUnwrap>,
{
    fn unwrap_fmod(self) -> Result<[TUnwrap; N]> {
        use std::mem::MaybeUninit;

        let mut result: [MaybeUninit<TUnwrap>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for (i, item) in self.into_iter().enumerate().skip(1) {
            let item = item.unwrap_fmod()?;
            result[i].write(item);
        }
        // we initialize every element in the array, so this is safe
        let result = result.map(|item| unsafe { item.assume_init() });
        Ok(result)
    }
}
