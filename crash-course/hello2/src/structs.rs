// structs - used to create custom data types

// traditional struct
struct Colour {
  red: u8,
  green: u8,
  blue: u8
}

// tuple struct
struct ColourTuple (u8, u8, u8);

struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string() // rust extn couldn't autocomplete/find to_string()
    } // implicit return of struct Person

    // if Person {... ended with }; (semicolon), get:

    // mismatched types
    // expected struct `structs::Person`, found `()`rustc(E0308)
    // structs.rs(19, 6): implicitly returns `()` as its body has no tail or `return` expression
    // structs.rs(19, 38): expected struct `structs::Person`, found `()`
  }

  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name) // implicit return
  }

  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }

  fn name_to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() {
  let mut c = Colour {
    red: 255,
    green: 255,
    blue: 255,
  };

  c.red = 200;
  println!("Colour: {} {} {}", c.red, c.green, c.blue);
  
  let mut c = ColourTuple(255, 0, 0); // allows to redefine c?
  c.0 = 200;
  println!("Colour: {} {} {}", c.0, c.1, c.2);
  
  let mut p = Person::new("Joe", "Bloggs");
  println!("Person: {} {}", p.first_name, p.last_name);
  p.last_name = "Bender".to_string(); // this works! don't need setter... not private?
  println!("Person: {}", p.full_name());

  p.set_last_name("Billhooks");
  println!("Person: {}", p.full_name());
  
  println!("Person tuple: {:#?}", p.name_to_tuple());
  // help: the trait `std::fmt::Display` is not implemented for `(std::string::String, std::string::String)`
  // note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
  // note: required by `std::fmt::Display::fmt`rustc(E0277)
}

// https://youtu.be/zF34dRivLOw?t=5655 "Maybe it's a man who gets married, who knows these days..."