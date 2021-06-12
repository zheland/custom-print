#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "std")]
use std::ffi::{CStr, CString};
#[cfg(feature = "std")]
use std::os::raw::c_char;

#[cfg(feature = "alloc")]
use crate::WriteStringFn;
use crate::{WriteBytesFn, WriteLenPtrFn, WritePtrLenFn, WriteStrFn};
#[cfg(feature = "std")]
use crate::{WriteCCharPtrFn, WriteCStrFn, WriteCStringFn};

/// A trait used to inference type of write closure wrapper.
///
/// This trait used by [`FmtWriter`], [`ConcatWriter`] and [`IoWriter`].
///
/// Both `IntoWriteFn` and [`IntoTryWriteFn`] traits provides the same wrappers for
/// closures with `*const u8`, `usize`, `&[u8]`, [`&str`] and [`String`] arguments.
/// This variant uses panicking versions for
/// closures with [`&CStr`], [`CString`], and [`*const c_char`] arguments,
/// for a "fail fast" approach.
///
/// [`&str`]: https://doc.rust-lang.org/std/str/index.html
/// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
/// [`&CStr`]: https://doc.rust-lang.org/std/ffi/struct.CStr.html
/// [`CString`]: https://doc.rust-lang.org/std/ffi/struct.CString.html
/// [`*const c_char`]: https://doc.rust-lang.org/std/os/raw/type.c_char.html
/// [`IntoTryWriteFn`]: trait.IntoTryWriteFn.html
/// [`FmtWriter`]: struct.FmtWriter.html
/// [`ConcatWriter`]: struct.ConcatWriter.html
/// [`IoWriter`]: struct.IoWriter.html
pub trait IntoWriteFn<Ts> {
    /// The corresponding write function wrapper.
    type WriteFn;

    /// Returns the wrapped function.
    fn into_write_fn(self) -> Self::WriteFn;
}

macro_rules! def {
    ( ($F:tt, $R:tt), $func:ty, ($($ty:ty),*) ) => {
        impl<$F, $R> IntoWriteFn<($($ty,)*)> for $F
        where
            $F: FnMut($($ty),*) -> $R,
        {
            type WriteFn = $func;
            fn into_write_fn(self) -> Self::WriteFn {
                Self::WriteFn::new(self)
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
def!((F, R), WriteCStrFn<F, R>, (&CStr));
#[cfg(feature = "std")]
def!((F, R), WriteCStringFn<F, R>, (CString));
#[cfg(feature = "std")]
def!((F, R), WriteCCharPtrFn<F, R>, (*const c_char));
