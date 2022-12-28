use std::io::{Read, Seek};
use std::ops::DerefMut;
use std::{cell::RefCell, rc::Rc};

use opentype;

use super::mapping::Mapping;
use super::metrics::Metrics;
use crate::Result;

macro_rules! cache(
    ($(($field:ident -> $try_field:ident($($argument:tt)*), $type:ty, $name:literal),)+) => (
        cache!(@define $($field, $type),+);

        impl<T: Read + Seek> Cache<T> {
            #[inline]
            pub fn new(tape: Rc<RefCell<T>>, backend: opentype::Font) -> Self {
                Self {
                    tape: tape,
                    backend: backend,

                    mapping: Default::default(),
                    metrics: Default::default(),

                    $($field: Default::default(),)+
                }
            }

            $(cache!(@implement $field -> $try_field($($argument)*), $type, $name);)+
        }
    );
    (@define $($field:ident, $type:ty),+) => (
        pub struct Cache<T: Read + Seek> {
            tape: Rc<RefCell<T>>,
            backend: opentype::Font,

            mapping: Option<Rc<Mapping>>,
            metrics: Option<Rc<Metrics>>,

            $($field: Option<Rc<$type>>,)+
        }
    );
    (@implement $field:ident -> $try_field:ident(), $type:ty, $name:literal) => (
        #[allow(dead_code)]
        pub fn $field(&mut self) -> Result<&Rc<$type>> {
            match self.$try_field()? {
                Some(table) => Ok(table),
                _ => raise!(concat!("cannot find ", $name)),
            }
        }

        #[allow(dead_code)]
        pub fn $try_field(&mut self) -> Result<Option<&Rc<$type>>> {
            if self.$field.is_none() {
                self.$field = match self.backend.take::<_, $type>(
                    self.tape.borrow_mut().deref_mut(),
                )? {
                    Some(value) => Some(Rc::new(value)),
                    _ => None,
                };
            }
            Ok(self.$field.as_ref())
        }
    );
    (@implement $field:ident -> $try_field:ident($($argument:ident),+), $type:ty, $name:literal) => (
        #[allow(dead_code)]
        pub fn $field(&mut self) -> Result<&Rc<$type>> {
            match self.$try_field()? {
                Some(table) => Ok(table),
                _ => raise!(concat!("cannot find ", $name)),
            }
        }

        #[allow(dead_code)]
        pub fn $try_field(&mut self) -> Result<Option<&Rc<$type>>> {
            if self.$field.is_none() {
                $(let $argument = self.$argument()?.clone();)+
                self.$field = match self.backend.take_given::<_, $type>(
                    self.tape.borrow_mut().deref_mut(),
                    ($(&$argument),+)
                )? {
                    Some(value) => Some(Rc::new(value)),
                    _ => None,
                };
            }
            Ok(self.$field.as_ref())
        }
    );
);

cache! {
    (character_mapping -> try_character_mapping(), ::truetype::CharacterMapping, "the character-to-glyph mapping"),
    (font_header -> try_font_header(), ::truetype::FontHeader, "the font header"),
    (font_set -> try_font_set(), ::postscript::compact1::FontSet, "the font set"),
    (glyph_data -> try_glyph_data(glyph_mapping), ::truetype::GlyphData, "the glyph data"),
    (glyph_mapping -> try_glyph_mapping(font_header, maximum_profile), ::truetype::GlyphMapping, "the glyph mapping"),
    (horizontal_header -> try_horizontal_header(), ::truetype::HorizontalHeader, "the horizontal header"),
    (horizontal_metrics -> try_horizontal_metrics(horizontal_header, maximum_profile), ::truetype::HorizontalMetrics, "the horizontal metrics"),
    (maximum_profile -> try_maximum_profile(), ::truetype::MaximumProfile, "the maximum profile"),
    (windows_metrics -> try_windows_metrics(), ::truetype::WindowsMetrics, "the OS/2 and Windows metrics"),
}

impl<T: Read + Seek> Cache<T> {
    pub fn mapping(&mut self) -> Result<&Rc<Mapping>> {
        if self.mapping.is_none() {
            self.mapping = Some(Rc::new(Mapping::new(self.character_mapping()?)?));
        }
        Ok(self.mapping.as_ref().unwrap())
    }

    pub fn metrics(&mut self) -> Result<&Rc<Metrics>> {
        if self.metrics.is_none() {
            self.metrics = Some(Rc::new(Metrics::new(self.horizontal_metrics()?.clone())));
        }
        Ok(self.metrics.as_ref().unwrap())
    }
}
