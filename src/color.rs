pub trait RGBColorTextExt {
    fn rgb(self, r: u8, g: u8, b: u8) -> String;
    fn default(self) -> String;
}

impl RGBColorTextExt for char {
    fn rgb(self, r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;2;{r};{g};{b}m{self}")
    }

    fn default(self) -> String {
        format!("\x1b[39m{self}")
    }
}
