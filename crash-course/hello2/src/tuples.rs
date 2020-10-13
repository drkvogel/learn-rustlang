
// tuples
// group of (different) types


pub fn run() {
  let person: (&str, &str, i8) = ("Chris", "Faringdon", 45);
  println!("{} is from {} and is {} years old", person.0, person.1, person.2);
    // Chris is from Faringdon and is 45 years old
}