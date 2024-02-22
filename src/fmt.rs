use std::fmt;

use crate::{ShortUuid, ShortUuidCustom};

impl fmt::Display for ShortUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        String::from_utf8(self.0.to_vec()).unwrap().fmt(f)
    }
}

impl fmt::Display for ShortUuidCustom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        String::from_utf8(self.0.to_vec()).unwrap().fmt(f)
    }
}
