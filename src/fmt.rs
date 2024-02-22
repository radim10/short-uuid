use std::fmt;

use crate::ShortUuid;

impl fmt::Display for ShortUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        String::from_utf8(self.0.to_vec()).unwrap().fmt(f)
    }
}
