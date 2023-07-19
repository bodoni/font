use std::rc::Rc;

use opentype::truetype::character_mapping::CharacterMapping;

/// Characters.
pub type Characters = Rc<CharacterMapping>;
