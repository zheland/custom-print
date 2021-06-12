#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};
#[cfg(feature = "std")]
use std::ffi::{CStr, CString, NulError};
#[cfg(feature = "std")]
use std::os::raw::c_char;

use crate::{WriteBytes, WriteStr, WriteStrAsBytes};

macro_rules! with_docs {
    ( docs: { $( $doc:expr ),* $(,)? }, item: { $item:item } ) => {
        $( #[doc = $doc] )*
        $item
    };
}

macro_rules! define_write_fn {
    (
        $name:ident,
        ($($params:ty),*),
        $writes:literal,
        $into_write_fns:literal
    ) => {
        with_docs!(
            docs: {
                ::core::concat!(
                    "A wrapper for write functions `for<R> FnMut(",
                    ::core::stringify!($($params),*),
                    ") -> R`."
                ),
                "",
                ::core::concat!(
                    "It implements ",
                    $writes,
                    " and can be used in conjunction with ",
                    $into_write_fns,
                    " to simplify type inference."
                ),
                "",
                "[`IntoWriteFn`]: trait.IntoWriteFn.html",
                "[`IntoTryWriteFn`]: trait.IntoTryWriteFn.html"
            },
            item: {
                #[derive(Clone, Copy, Debug, Eq, PartialEq)]
                pub struct $name<F, R>(F)
                where
                    F: FnMut($($params),*) -> R;
            }
        );

        impl<F, R> $name<F, R>
        where
            F: FnMut($($params),*) -> R,
        {
            with_docs!(
                docs: {
                    ::core::concat!(
                        "Creates a new `",
                        ::core::stringify!($name),
                        "` containing the given closure or function."
                    )
                },
                item: {
                    pub fn new(closure: F) -> Self {
                        Self(closure)
                    }
                }
            );
        }
    };
}

macro_rules! define_write_str_fn {
    ($name:ident, ($($params:ty),*), $buf:ident => ($($args:tt)*)) => {
        define_write_fn!(
            $name,
            ($($params),*),
            "[`WriteStr`] trait",
            "[`IntoWriteFn`] and [`IntoTryWriteFn`] traits"
        );

        impl<F, R> WriteStr for $name<F, R>
        where
            F: FnMut($($params),*) -> R,
        {
            type Output = R;

            fn write_str(&mut self, $buf: &str) -> Self::Output {
                self.0($($args)*)
            }
        }
    };
}

macro_rules! define_write_bytes_fn {
    ($name:ident, ($($params:ty),*), $buf:ident => ($($args:tt)*)) => {
        define_write_fn!(
            $name,
            ($($params),*),
            "[`WriteBytes`] and [`WriteStr`] traits",
            "[`IntoWriteFn`] and [`IntoTryWriteFn`] traits"
        );

        impl<F, R> WriteBytes for $name<F, R>
        where
            F: FnMut($($params),*) -> R,
        {
            type Output = R;

            fn write_bytes(&mut self, $buf: &[u8]) -> Self::Output {
                self.0($($args)*)
            }
        }

        impl<F, R> WriteStrAsBytes for $name<F, R> where F: FnMut($($params),*) -> R {}
    };
}

#[cfg(feature = "std")]
macro_rules! define_write_cstr_fn {
    ($name:ident, ($($params:ty),*), $cstr:ident => ($($args:tt)*)) => {
        define_write_fn!(
            $name,
            ($($params),*),
            "[`WriteBytes`] and [`WriteStr`] traits",
            "[`IntoWriteFn`] trait"
        );

        impl<F, R> WriteBytes for $name<F, R>
        where
            F: FnMut($($params),*) -> R,
        {
            type Output = R;

            fn write_bytes(&mut self, buf: &[u8]) -> Self::Output {
                let $cstr = match CString::new(buf.as_ref()) {
                    Ok(ok) => ok,
                    Err(err) => panic!(
                        "nul byte found in provided data at position: {}, buffer: {:?}",
                        err.nul_position(),
                        buf
                    ),
                };
                self.0($($args)*)
            }
        }

        impl<F, R> WriteStrAsBytes for $name<F, R> where F: FnMut($($params),*) -> R {}
    };
}

#[cfg(feature = "std")]
macro_rules! define_try_write_cstr_fn {
    ($name:ident, ($($params:ty),*), $cstr:ident => ($($args:tt)*)) => {
        define_write_fn!(
            $name,
            ($($params),*),
            "[`WriteBytes`] and [`WriteStr`] traits",
            "[`IntoTryWriteFn`] trait"
        );

        impl<F, R> WriteBytes for $name<F, R>
        where
            F: FnMut($($params),*) -> R,
        {
            type Output = Result<R, NulError>;

            fn write_bytes(&mut self, buf: &[u8]) -> Self::Output {
                CString::new(buf.as_ref()).map(|$cstr| self.0($($args)*))
            }
        }

        impl<F, R> WriteStrAsBytes for $name<F, R>
        where
            F: FnMut($($params),*) -> R {}
    };
}

define_write_bytes_fn!(WritePtrLenFn, (*const u8, usize), buf => (buf.as_ptr(), buf.len()));
define_write_bytes_fn!(WriteLenPtrFn, (usize, *const u8), buf => (buf.len(), buf.as_ptr()));
define_write_bytes_fn!(WriteBytesFn, (&[u8]), buf => (buf));

define_write_str_fn!(WriteStrFn, (&str), buf => (buf));
#[cfg(feature = "alloc")]
define_write_str_fn!(WriteStringFn, (String), buf => (buf.to_string()));

#[cfg(feature = "std")]
define_write_cstr_fn!(WriteCStrFn, (&CStr), cstr => (cstr.as_c_str()));
#[cfg(feature = "std")]
define_write_cstr_fn!(WriteCStringFn, (CString), cstr => (cstr));
#[cfg(feature = "std")]
define_write_cstr_fn!(WriteCCharPtrFn, (*const c_char), cstr => (cstr.as_ptr()));

#[cfg(feature = "std")]
define_try_write_cstr_fn!(TryWriteCStrFn, (&CStr), cstr => (cstr.as_c_str()));
#[cfg(feature = "std")]
define_try_write_cstr_fn!(TryWriteCStringFn, (CString), cstr => (cstr));
#[cfg(feature = "std")]
define_try_write_cstr_fn!(TryWriteCCharPtrFn, (*const c_char), cstr => (cstr.as_ptr()));
