use std::rc::Rc;

use opentype::truetype::NamingTable;

/// Names.
pub type Names = Rc<NamingTable>;
