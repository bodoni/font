macro_rules! deref {
    ($struct_name:ident::$field_name:ident => $target_name:ty) => (
        impl ::std::ops::Deref for $struct_name {
            type Target = $target_name;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field_name
            }
        }

        impl ::std::ops::DerefMut for $struct_name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field_name
            }
        }
    );
}

macro_rules! raise(
    ($message:expr) => (return Err(crate::Error::new(::std::io::ErrorKind::Other, $message)));
);

macro_rules! some(
    ($option:expr, $message:expr) => (
        match $option {
            Some(value) => value,
            _ => raise!($message),
        }
    );
);
