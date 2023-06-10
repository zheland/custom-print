/// Defines `flush` macro that calls `flush` method of the specified writer.
///
/// Use [`define_try_flush`] if you need to define a fallible flush macro.
///
/// The macro is intentionally defined instead of a function
/// because of the custom result type specified by the writer.
///
/// # Examples
///
/// ```rust
/// use std::io::{self, LineWriter, Write};
/// use std::sync::Mutex;
///
/// let written: Mutex<Vec<u8>> = Mutex::default();
///
/// #[derive(Clone, Debug)]
/// struct CustomWriter<'a>(&'a Mutex<Vec<u8>>);
///
/// impl Write for CustomWriter<'_> {
///     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
///         let mut written = self.0.lock().unwrap();
///         written.extend_from_slice(buf);
///         Ok(buf.len())
///     }
///
///     fn flush(&mut self) -> io::Result<()> {
///         Ok(())
///     }
/// }
///
/// let custom_writer = CustomWriter(&written);
/// let mut line_writer = LineWriter::new(custom_writer);
///
/// custom_print::define_print!(cprint, line_writer);
/// custom_print::define_flush!(flush, line_writer);
///
/// assert_eq!(cprint!("first,"), ());
/// assert_eq!(*written.lock().unwrap(), b"");
/// assert_eq!(cprint!("second\nthird,"), ());
/// assert_eq!(*written.lock().unwrap(), b"first,second\n");
/// assert_eq!(flush!(), ());
/// assert_eq!(*written.lock().unwrap(), b"first,second\nthird,");
/// ```
///
/// [`define_try_flush`]: macro.define_try_flush.html
#[macro_export]
macro_rules! define_flush {
    ( $( #[$meta:meta] )* $name:ident, $($args:tt)* ) => {
        $( #[$meta] )*
        #[allow(unused_macros)]
        macro_rules! $name {
            () => {
                $crate::define_writer!($($args)*).flush().expect("failed flushing")
            };
        }
    };
}

/// Defines `try_flush` macro that calls `flush` method of the specified writer.
///
/// Use [`define_flush`] if you need to define a non-fallible flush macros.
///
/// The macro is intentionally defined instead of a function
/// because of the custom result type specified by the writer.
///
/// # Examples
///
/// ```rust
/// use std::io::{self, LineWriter, Write};
/// use std::sync::Mutex;
///
/// let written: Mutex<Vec<u8>> = Mutex::default();
///
/// #[derive(Clone, Debug)]
/// struct CustomWriter<'a>(&'a Mutex<Vec<u8>>);
///
/// impl Write for CustomWriter<'_> {
///     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
///         let mut written = self.0.lock().unwrap();
///         written.extend_from_slice(buf);
///         Ok(buf.len())
///     }
///
///     fn flush(&mut self) -> io::Result<()> {
///         Ok(())
///     }
/// }
///
/// let custom_writer = CustomWriter(&written);
/// let mut line_writer = LineWriter::new(custom_writer);
///
/// custom_print::define_try_print!(try_print, line_writer);
/// custom_print::define_try_flush!(try_flush, line_writer);
///
/// assert_eq!(try_print!("first,").ok(), Some(()));
/// assert_eq!(*written.lock().unwrap(), b"");
/// assert_eq!(try_print!("second\nthird,").ok(), Some(()));
/// assert_eq!(*written.lock().unwrap(), b"first,second\n");
/// assert_eq!(try_flush!().ok(), Some(()));
/// assert_eq!(*written.lock().unwrap(), b"first,second\nthird,");
/// ```
#[macro_export]
macro_rules! define_try_flush {
    ( $( #[$meta:meta] )* $name:ident, $($args:tt)* ) => {
        $( #[$meta] )*
        #[allow(unused_macros)]
        macro_rules! $name {
            () => {
                $crate::define_try_writer!($($args)*).flush()
            };
        }
    };
}
