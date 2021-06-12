/// Prints and returns the value of a given expression for quick and dirty debugging
/// with the specified write macro, writer, format and error-handling policy.
///
/// The implementation of the `dbgwrite` macro is based on [`std::dbg`] macro implementation,
/// but the exact output printed by [`std::dbg`]
/// should not be relied upon and is subject to future changes.
///
/// If the `try` policy is used, it propagates write error and
/// returns values wrapper into `Result`.
///
/// # Panics
///
/// The macro panics if writing fails and the `expect` policy is used.
///
/// [`std::dbg`]: https://doc.rust-lang.org/std/macro.dbg.html
///
/// # Examples
///
/// ```rust
/// use core::fmt::Write;
/// let mut string = String::new();
///
/// assert_eq!(custom_print::dbgwrite!(writeln, &mut string, expect, ":?", "first"), ("first"));
/// assert!(string.contains("\"first\""));
/// assert_eq!(custom_print::dbgwrite!(writeln, &mut string, try, ":?", "second"), Ok(("second")));
/// assert!(string.contains("\"second\""));
/// ```
#[macro_export]
macro_rules! dbgwrite {
    ( $macro:path, $writer:expr, expect, $format:literal $(, $($args:tt)+)? ) => {
        $crate::_dbgwrite_impl!( $macro, $writer, $format $(, $($args)+)? )
    };
    ( $macro:path, $writer:expr, try, $format:literal $(, $($args:tt)+)? ) => {
        $crate::_try_dbgwrite_impl!( $macro, $writer, $format $(, $($args)+)? )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _dbgwrite_impl {
    ( $macro:path, $writer:expr, $format:literal ) => {
        $crate::write!(
            $macro, $writer, expect, "[{}:{}]", ::core::file!(), ::core::line!()
        );
    };
    ( $macro:path, $writer:expr, $format:literal, $val:expr $(,)? ) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::write!(
                    $macro, $writer, expect,
                    ::core::concat!("[{}:{}] {} = {", $format, "}"),
                    ::core::file!(), ::core::line!(), ::core::stringify!($val), &tmp
                );
                (tmp)
            }
        }
    };
    ( $macro:path, $writer:expr, $format:literal, $($val:expr),+ $(,)? ) => {
        ($(
            match $val {
                tmp => {
                    $crate::write!(
                        $macro, $writer, expect,
                        ::core::concat!("[{}:{}] {} = {", $format, "}"),
                        ::core::file!(), ::core::line!(), ::core::stringify!($val), &tmp
                    );
                    tmp
                }
            }
        ),+,)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _try_dbgwrite_impl {
    ( $macro:path, $writer:expr, $format:literal ) => {
        $crate::write!(
            $macro, $writer, try, "[{}:{}]", ::core::file!(), ::core::line!()
        ).map(|_| ());
    };
    ( $macro:path, $writer:expr, $format:literal, $val:expr $(,)? ) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::write!(
                    $macro, $writer, try,
                    ::core::concat!("[{}:{}] {} = {", $format, "}"),
                    ::core::file!(), ::core::line!(), ::core::stringify!($val), &tmp
                ).map(|_| tmp)
            }
        }
    };
    ( $macro:path, $writer:expr, $format:literal, $($val:expr),+ $(,)? ) => {
        (|| {
            Ok(($(
                match $val {
                    tmp => {
                        match $crate::write!(
                            $macro, $writer, try,
                            ::core::concat!("[{}:{}] {} = {", $format, "}"),
                            ::core::file!(), ::core::line!(), ::core::stringify!($val), &tmp
                        ) {
                            Ok(_) => tmp,
                            Err(err) => return Err(err),
                        }
                    }
                }
            ),+,))
        })()
    };
}
