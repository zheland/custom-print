#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "std")]
use std::ffi::{CStr, CString};
#[cfg(feature = "std")]
use std::os::raw::c_char;

#[cfg(feature = "alloc")]
use crate::WriteStringFn;
#[cfg(feature = "std")]
use crate::{TryWriteCCharPtrFn, TryWriteCStrFn, TryWriteCStringFn};
use crate::{WriteBytesFn, WriteLenPtrFn, WritePtrLenFn, WriteStrFn};

/// A trait used to inference type of fallible write closure wrapper.
///
/// This trait used by [`FmtTryWriter`], [`ConcatTryWriter`] and [`IoTryWriter`].
///
/// Both [`IntoWriteFn`] and `IntoTryWriteFn` traits provides the same wrappers for
/// closures with `*const u8`, `usize`, `&[u8]`, [`&str`] and [`String`] arguments.
/// This variant uses non-panicking versions for
/// closures with [`&CStr`], [`CString`], and [`*const c_char`] arguments.
///
/// [`&str`]: https://doc.rust-lang.org/std/str/index.html
/// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
/// [`&CStr`]: https://doc.rust-lang.org/std/ffi/struct.CStr.html
/// [`CString`]: https://doc.rust-lang.org/std/ffi/struct.CString.html
/// [`*const c_char`]: https://doc.rust-lang.org/std/os/raw/type.c_char.html
/// [`IntoWriteFn`]: trait.IntoWriteFn.html
/// [`FmtTryWriter`]: struct.FmtTryWriter.html
/// [`ConcatTryWriter`]: struct.ConcatTryWriter.html
/// [`IoTryWriter`]: struct.IoTryWriter.html
pub trait IntoTryWriteFn<Ts> {
    /// The corresponding fallible write function wrapper.
    type TryWriteFn;

    /// Returns the wrapped function.
    fn into_try_write_fn(self) -> Self::TryWriteFn;
}

macro_rules! def {
    ( ($F:tt, $R:tt), $func:ty, ($($ty:ty),*) ) => {
        impl<$F, $R> IntoTryWriteFn<($($ty,)*)> for $F
        where
            $F: FnMut($($ty),*) -> $R,
        {
            type TryWriteFn = $func;
            fn into_try_write_fn(self) -> Self::TryWriteFn {
                Self::TryWriteFn::new(self)
            }
        }

    }
}

def!((F, R), WritePtrLenFn<F, R>, (*const u8, usize));
def!((F, R), WriteLenPtrFn<F, R>, (usize, *const u8));
def!((F, R), WriteBytesFn<F, R>, (&[u8]));

def!((F, R), WriteStrFn<F, R>, (&str));
#[cfg(feature = "alloc")]
def!((F, R), WriteStringFn<F, R>, (String));

#[cfg(feature = "std")]
def!((F, R), TryWriteCStrFn<F, R>, (&CStr));
#[cfg(feature = "std")]
def!((F, R), TryWriteCStringFn<F, R>, (CString));
#[cfg(feature = "std")]
def!((F, R), TryWriteCCharPtrFn<F, R>, (*const c_char));
