
pub fn main () {
  println!("Hello, Closures!");

  let suma = |nro1, nro2| {
    nro1 + nro2
  };

  println!("Suma: {}", suma(1, 2));

  let mut original = 5;

  println!("Original: {}", original);
  
  original += 1;
  
  println!("Original + 1: {}", original);
  
  let original_borrowed = &original;
  
  println!("Borrowed: {}", original_borrowed);
  
  // a partir de aquí no puedo modificar la original porque está "prestada"
  
}