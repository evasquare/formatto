#[macro_export]
macro_rules! console_log {
    ($($arg:tt)*) => ($crate::log(&format!($($arg)*)));
}

#[macro_export]
macro_rules! console_error {
    ($($arg:tt)*) => ($crate::error(&format!($($arg)*)));
}
