use relm4::gtk::{gdk, glib};

#[derive(Debug)]
pub struct RGBA(gdk::RGBA);

impl From<gdk::RGBA> for RGBA {
    fn from(value: gdk::RGBA) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for RGBA {
    type Error = glib::error::BoolError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(gdk::RGBA::parse(value)?))
    }
}

impl RGBA {
    /// Converts to a hexadecimal string in the format #RRGGBBAA
    pub fn to_hex(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            (self.0.red() * 255.0) as usize,
            (self.0.green() * 255.0) as usize,
            (self.0.blue() * 255.0) as usize,
            (self.0.alpha() * 255.0) as usize,
        )
    }

    /// Converts to a hexadecimal string in the format #AARRGGBB (used by MPV)
    pub fn to_mpv_hex(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            (self.0.alpha() * 255.0) as usize,
            (self.0.red() * 255.0) as usize,
            (self.0.green() * 255.0) as usize,
            (self.0.blue() * 255.0) as usize,
        )
    }
}
