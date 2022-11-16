
pub fn iterators() {
  println!("Welcome to Iterators");

  let products = [1,2,3,4,5,6,7,8,9,10];

  for x in products.iter(){
    println!("Product: {}", x);
  }
}