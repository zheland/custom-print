/// Defines a writer from writer expression arguments or from the provided one.
///
/// If more than one argument is used, the first argument specifies the writer type,
/// and the others are used to define the expression:
/// - `concat, args...`: [`ConcatWriter`]`::from_closure(`[`define_writer_expr`]`!(args...))`
/// - `fmt, args...`: [`FmtWriter`]`::from_closure(`[`define_writer_expr`]`!(args...))`
/// - `io, args...`: [`IoWriter`]`::from_closure(`[`define_writer_expr`]`!(args...))`
///
/// If only one argument is used, the macro just returns it as a result.
///
/// Use [`define_try_writer`] if you need to define a fallible writer.
/// This macro is used by [`define_printlike`], [`define_dbglike`] and [`define_flush`] macros.
///
/// # Examples
///
#[cfg_attr(feature = "alloc", doc = "```rust")]
#[cfg_attr(not(feature = "alloc"), doc = "```rust,compile_fail")]
/// use core::fmt::Write;
///
/// let mut expr_string = String::new();
/// let mut expr_writer = custom_print::define_writer!(&mut expr_string);
/// let mut concat_string = String::new();
/// let mut concat_writer = custom_print::define_writer!(concat, |value: &str| {
///     concat_string += value;
/// });
///
/// assert_eq!(writeln!(expr_writer, "first"), Ok(()));
/// assert_eq!(expr_string, "first\n");
/// assert_eq!(writeln!(concat_writer, "second"), Ok(()));
/// assert_eq!(concat_string, "second\n");
/// ```
///
/// [`ConcatWriter`]: struct.ConcatTryWriter.html
/// [`FmtWriter`]: struct.FmtTryWriter.html
/// [`IoWriter`]: struct.IoTryWriter.html
#[macro_export]
macro_rules! define_writer {
    ( concat, $($args:tt)* ) => {
        $crate::ConcatWriter::from_closure($crate::define_writer_expr!($($args)*))
    };
    ( fmt, $($args:tt)* ) => {
        $crate::FmtWriter::from_closure($crate::define_writer_expr!($($args)*))
    };
    ( io, $($args:tt)* ) => {
        $crate::IoWriter::from_closure($crate::define_writer_expr!($($args)*))
    };
    ( $expr:expr ) => {
        $expr
    };
}

/// Defines a fallible writer from writer expression arguments or from the provided one.
///
/// If more than one argument is used, the first argument specifies the writer type,
/// and the others are used to define the expression:
/// - `concat, args...`: [`ConcatTryWriter`]`::from_closure(`[`define_writer_expr`]`!(args...))`
/// - `fmt, args...`: [`FmtTryWriter`]`::from_closure(`[`define_writer_expr`]`!(args...))`
/// - `io, args...`: [`IoTryWriter`]`::from_closure(`[`define_writer_expr`]`!(args...))`
///
/// If only one argument is used, the macro just returns it as a result.
///
/// Use [`define_writer`] if you need to define a non-fallible writer.
/// This macro is used by [`define_printlike`], [`define_dbglike`] and [`define_try_flush`] macros.
///
/// # Examples
///
#[cfg_attr(feature = "alloc", doc = "```rust")]
#[cfg_attr(not(feature = "alloc"), doc = "```rust,compile_fail")]
/// use core::fmt::{self, Write};
///
/// let mut expr_string = String::new();
/// let mut expr_writer = custom_print::define_try_writer!(&mut expr_string);
/// let mut concat_string = String::new();
/// let mut concat_writer = custom_print::define_writer!(concat, |value: &str| {
///     concat_string += value;
/// });
/// let mut fallible_writer = custom_print::define_try_writer!(fmt, |_: &str| Err(fmt::Error));
///
/// assert_eq!(writeln!(expr_writer, "first"), Ok(()));
/// assert_eq!(expr_string, "first\n");
/// assert_eq!(writeln!(concat_writer, "second"), Ok(()));
/// assert_eq!(concat_string, "second\n");
/// assert_eq!(writeln!(fallible_writer, "third"), Err(fmt::Error));
/// ```
///
/// [`ConcatTryWriter`]: struct.ConcatTryWriter.html
/// [`FmtTryWriter`]: struct.FmtTryWriter.html
/// [`IoTryWriter`]: struct.IoTryWriter.html
#[macro_export]
macro_rules! define_try_writer {
    ( concat, $($args:tt)* ) => {
        $crate::ConcatTryWriter::from_closure($crate::define_writer_expr!($($args)*))
    };
    ( fmt, $($args:tt)* ) => {
        $crate::FmtTryWriter::from_closure($crate::define_writer_expr!($($args)*))
    };
    ( io, $($args:tt)* ) => {
        $crate::IoTryWriter::from_closure($crate::define_writer_expr!($($args)*))
    };
    ( $expr:expr ) => {
        $expr
    };
}
