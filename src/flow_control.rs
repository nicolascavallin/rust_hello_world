use rand::random;

pub fn main() {

  println!("Hello, Flow Control!");

  // IF - ELSE

  let mut number = 10;

  if random::<bool>() {
    number = 0;
  }

  if number == 10 {
    println!("Number is 10");
  } else if number == 0 {
    println!("Number is 0");
  } else {
    println!("Number is not 10 nor 0");
  }

  let new_number = if number == 10 { 50 } else { 100 };

  println!("New number: {}", new_number);


}