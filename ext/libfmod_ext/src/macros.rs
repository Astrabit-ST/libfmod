// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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
            self.0.$fn_name($($arg_name.unwrap_fmod()),*).wrap_fmod()
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

    ) => {};
}

#[macro_export]
macro_rules! ruby_struct {
    (struct $name:ident: $fmod_ty:path {
      $( $member:ident: $member_ty:ty ),* $(,)?
    }) => {
      const _: () = {
        static CLASS: once_cell::sync::OnceCell<magnus::value::Opaque<magnus::RClass>> = once_cell::sync::OnceCell::new();
        type _Wrapped = $fmod_ty;
        impl $crate::UnwrapFMOD<_Wrapped> for magnus::RStruct {
          fn unwrap_fmod(self) -> $crate::Result<_Wrapped> {
            Ok(_Wrapped {
              $(
                  $member: self.aref::<_, $member_ty>(stringify!($member))?.unwrap_fmod()?,
              )*
            })
          }
        }

        impl $crate::WrapFMOD<magnus::Value> for _Wrapped {
          fn wrap_fmod(self) -> $crate::Result<magnus::Value> {
            use magnus::{Class, value::InnerValue};
            // FIXME put this in a Lazy somehow
            let ruby = magnus::Ruby::get().unwrap();
            let rstruct = CLASS.get().unwrap().get_inner_with(&ruby);
            rstruct.new_instance((
              $(
                self.$member.wrap_fmod()?,
              )*
            ))
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
      const _: () = {
        use bitflags::Flags;
        use magnus::Module;
        type _Wrapped = $fmod_ty;
        type _Bits = <$fmod_ty as Flags>::Bits;

        impl $crate::UnwrapFMOD<_Wrapped> for _Bits {
          fn unwrap_fmod(self) -> $crate::Result<_Wrapped> {
              Ok(self.into())
          }
        }

        impl $crate::WrapFMOD<_Bits> for _Wrapped {
          fn wrap_fmod(self) -> $crate::Result<_Bits> {
              Ok(self.into())
          }
        }

        impl $crate::Bindable for _Wrapped {
          fn bind(module: impl Module) -> $crate::Result<()> {
            let class = module.define_module(stringify!($flag_name))?;
            $(
              class.const_set::<_, _Bits>(stringify!($flag), _Wrapped::$flag.wrap_fmod()?)?;
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