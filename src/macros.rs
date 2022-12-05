macro_rules! some(
    ($option:expr, $message:expr) => (
        match $option {
            Some(value) => value,
            _ => raise!($message),
        }
    );
);
