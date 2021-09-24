use std::fmt;

#[derive(Clone)]
pub enum Colours {
    Red,
    Green,
    Blue,
    Custom(i16, i16, i16)
}

impl fmt::Display for Colours {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Colours::Red => write!(f, "red"),
            &Colours::Green => write!(f, "green"),
            &Colours::Blue => write!(f, "blue"),
            &Colours::Custom(r, g, b) => write!(f, "custom ({}, {}, {})", r, g, b),
        }
    }
}

/// Doc comment that ends up in online docs
///
/// # Arguments
///
/// * `colour` - Enum colour choice to return rgb values for
// regular comment that does not appear in docs
fn get_rgb(colour: Colours) -> (i16, i16, i16) {
    let (r, g, b) = match colour {
        Colours::Red => (255, 0, 0),
        Colours::Green => (0, 255, 0),
        Colours::Blue => (0, 0, 255),
        Colours::Custom(r, g, b) => (r, g, b),
    };
    return (r, g, b);
}

/// Sample Generics example
///
/// # Arguments
///
/// * `name` - Value to say hello to
fn say_hi<T: fmt::Display>(name: T) {
    println!("Hello {}!", name)
}

fn main() {
    let colour = Colours::Custom(100, 23, 64);
    let (r, g, b) = get_rgb(colour.clone());
    println!("r: {}, g: {}, b: {}", r, g, b);
    println!("********************");
    say_hi(colour);
    say_hi(1);
    say_hi("Bob");
    say_hi("Bob".to_owned());
}
