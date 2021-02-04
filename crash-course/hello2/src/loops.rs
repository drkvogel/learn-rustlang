
// loops - used to iterate until condition is met

pub fn test() {
  println!("This is a test")
}

pub fn run() {
  let mut count = 0;

  // infinite loop
  loop {
    count += 1;
    println!("Number: {}", count);
    
    if count == 20 {
      break;
    }
  }
  
  // guarded return - suggested by linter - doesn't finish!
  // loop {
  //   count += 1;
  //   println!("Number: {}", count);
    
  //   if count != 20 {
  //       continue;
  //   }
  // break;
  // }

  count = 0;

  // while loop - fizzbuzz
  while count <= 100 {
    if count % 15 == 0 {
      println!("FizzBuzz");
    } else if count % 3 == 0 {
      println!("Fizz");
    } else if count % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}", count);
    }
    count += 1;
  }

  // for range
  for x in 0..100 { // increments automatically
    if x % 15 == 0 {
      println!("FizzBuzz");
    } else if x % 3 == 0 {
      println!("Fizz");
    } else if x % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}", x);
    }
    // x += 1;    
  }

}