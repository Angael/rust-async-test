// Custom data types
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }

    fn contrast(&self, other: &Color) -> u8 {
        (self.red as i16 - other.red as i16).abs() as u8
            + (self.green as i16 - other.green as i16).abs() as u8
            + (self.blue as i16 - other.blue as i16).abs() as u8
    }
}

pub fn run() {
    println!("[Custom structs]");
}
