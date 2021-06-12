/// Defines a writer expression from another expression, unsafe function or extern function.
///
/// If unsafe function specified as argument
/// it just creates an `FnMut` wrapper that calls unsafe fn in unsafe block.
/// The function can be specified by identifier or by braced full path.
/// See [Sefety](#sefety) section for important details about safety.
///
/// If extern function specified as argument
/// it creates extern block with this function and
/// creates an `FnMut` wrapper that calls unsafe fn in unsafe block.
/// See [Sefety](#sefety) section for important details about safety.
///
/// If an expression given as argument, the macro just returns it as a result.
///
/// This macro is used by [`define_writer`], [`define_try_writer`] macros.
///
/// # Safety
///
/// Note that writing using writer expression
/// defined with `unsafe fn` or `extern `fn` do not require unsafe,
/// so defining writer expression itself should be treated as an unsafe operation.
#[macro_export]
macro_rules! define_writer_expr {
    ( unsafe fn $func:ident($ty1:ty) $( -> $ret:ty)? ) => {
        |arg1: $ty1| unsafe {
            #[allow(unused_qualifications)]
            $func(arg1)
        }
    };
    ( unsafe fn ($func:path)($ty1:ty) $( -> $ret:ty)? ) => {
        |arg1: $ty1| unsafe {
            #[allow(unused_qualifications)]
            $func(arg1)
        }
    };
    ( unsafe fn $func:ident($ty1:ty, $ty2:ty) $( -> $ret:ty)? ) => {
        |arg1: $ty1, arg2: $ty2| unsafe {
            #[allow(unused_qualifications)]
            $func(arg1, arg2)
        }
    };
    ( unsafe fn ($func:path)($ty1:ty, $ty2:ty) $( -> $ret:ty)? ) => {
        |arg1: $ty1, arg2: $ty2| unsafe {
            #[allow(unused_qualifications)]
            $func(arg1, arg2)
        }
    };
    ( $(#[$extern_meta:meta])* extern $($abi:literal)?
        $(#[$meta:meta])* fn $func:ident($arg1:tt: $ty1:ty) $( -> $ret:ty)?
    ) => {{
        $(#[$extern_meta])* extern $($abi)? {
            $(#[$meta])* fn $func($arg1: $ty1) $( -> $ret)?;
        }
        |arg1: $ty1| unsafe { $func(arg1) }
    }};
    ( $(#[$extern_meta:meta])* extern $($abi:literal)?
        $(#[$meta:meta])* fn $func:ident($arg1:tt: $ty1:ty, $arg2:tt: $ty2:ty) $( -> $ret:ty)?
    ) => {{
        $(#[$extern_meta])* extern $($abi)? {
            $(#[$meta])* fn $func($arg1: $ty1, $arg2: $ty2) $( -> $ret)?;
        }
        |arg1: $ty1, arg2: $ty2| unsafe { $func(arg1, arg2) }
    }};
    ( $expr:expr ) => {
        $expr
    };
}
