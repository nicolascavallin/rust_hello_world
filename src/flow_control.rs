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

  // LOOP

  let mut counter = 0;

  loop {
    println!("Counter (loop): {}", counter);
    counter += 1;
    if counter == 10 {
      break;
    }
  }
  
  // WHILE

  while counter > 0 {
    println!("Counter (while): {}", counter);
    counter -= 1;
  }

  // FOR

  let lista = [1,2,3,4,5];

  for element in lista.iter() {
    println!("Element: {}", element);
  } 


}