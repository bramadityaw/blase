//! Convenience macros.

/// Appends formatted string to a `String`.
#[macro_export]
macro_rules! format_to {
    ($buf:expr) => ();
    ($buf:expr, $lit:literal $($arg:tt)*) => {
        {
            use ::std::fmt::Write as _;
            // We can't do ::std::fmt::Write::write_fmt($buf, format_args!($lit $($arg)*))
            // unfortunately, as that loses out on autoref behavior.
            _ = $buf.write_fmt(format_args!($lit $($arg)*))
        }
    };
}
