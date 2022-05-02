
#[allow(unused_variables)] // this works

fn main() {
    println!("Hello, world!");
    let a = String::from("Howdy");
    let b = a;
    let c = 0;
    let _d = 0; // unused variable: `d`
                // help: if this is intentional, prefix it with an underscore: `_d`rustc(unused_variables)
    println!("{}", b); // OK
    // println!("{}", a); // error[E0382]: borrow of moved value: `a`

    // stack - used by default - for fixed size variables
    let stack_i8: i8 = 10;
    let stack_f32: f32 = 20.;
    let stack_bool: bool = true;
    let stack_char: char = 'a';
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // fixed-size array

    // collections and vectors cannot be stack variables
    // Strings are collections of u8s that can grow
    

    if stack_i8 == 3 { // curly brackets create new scope (new stack frame?)
        let inside_scope = 9;
    }
}
