use std::cell::RefCell;
use std::io::Result;
use std::ops::DerefMut;
use std::rc::Rc;

use opentype;
use typeface::Tape;

use crate::formats::opentype::characters::Mapping;
use crate::formats::opentype::metrics::Metrics;

macro_rules! cache(
    ($(($field:ident -> $try_field:ident($($argument:tt)*), $type:ty, $name:literal,),)+) => (
        cache!(@define $($field, $type),+);

        impl<T: Tape> Cache<T> {
            #[inline]
            pub fn new(tape: Rc<RefCell<T>>, backend: opentype::Font) -> Self {
                Self {
                    tape,
                    backend,

                    mapping: Default::default(),
                    metrics: Default::default(),

                    $($field: Default::default(),)+
                }
            }

            $(cache!(@implement $field -> $try_field($($argument)*), $type, $name);)+
        }
    );
    (@define $($field:ident, $type:ty),+) => (
        pub struct Cache<T> {
            pub tape: Rc<RefCell<T>>,
            pub backend: opentype::Font,

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
                $(
                    let $argument = match self.$argument()? {
                        Some(argument) => argument.clone(),
                        _ => return Ok(None),
                    };
                )+
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

dereference! { Cache<T>::backend => opentype::Font }

cache! {
    (
        character_mapping -> try_character_mapping(),
        opentype::truetype::tables::CharacterMapping,
        "the character-to-glyph mapping",
    ),
    (
        font_header -> try_font_header(),
        opentype::truetype::tables::FontHeader,
        "the font header",
    ),
    (
        font_set -> try_font_set(),
        opentype::postscript::compact1::FontSet,
        "the font set",
    ),
    (
        font_variations -> try_font_variations(),
        opentype::tables::font_variations::FontVariations,
        "the font variations",
    ),
    (
        glyph_data -> try_glyph_data(try_glyph_mapping),
        opentype::truetype::tables::GlyphData,
        "the glyph data",
    ),
    (
        glyph_mapping -> try_glyph_mapping(try_font_header, try_maximum_profile),
        opentype::truetype::tables::GlyphMapping,
        "the glyph-to-location mapping",
    ),
    (
        glyph_positioning -> try_glyph_positioning(),
        opentype::tables::glyph_positioning::GlyphPositioning,
        "the glyph positioning",
    ),
    (
        glyph_substitution -> try_glyph_substitution(),
        opentype::tables::glyph_substitution::GlyphSubstitution,
        "the glyph substitution",
    ),
    (
        horizontal_header -> try_horizontal_header(),
        opentype::truetype::tables::HorizontalHeader,
        "the horizontal header",
    ),
    (
        horizontal_metrics -> try_horizontal_metrics(try_horizontal_header, try_maximum_profile),
        opentype::truetype::tables::HorizontalMetrics,
        "the horizontal metrics",
    ),
    (
        maximum_profile -> try_maximum_profile(),
        opentype::truetype::tables::MaximumProfile,
        "the maximum profile",
    ),
    (
        naming_table -> try_naming_table(),
        opentype::truetype::tables::Names,
        "the naming table",
    ),
    (
        postscript -> try_postscript(),
        opentype::truetype::tables::PostScript,
        "the PostScript information",
    ),
    (
        windows_metrics -> try_windows_metrics(),
        opentype::truetype::tables::WindowsMetrics,
        "the OS/2 and Windows metrics",
    ),
}

impl<T: Tape> Cache<T> {
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
