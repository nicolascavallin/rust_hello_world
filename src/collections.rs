use std::collections::HashSet;

pub fn main() {
  println!("Hello, Collections!");

  // El tamaño de la memoria no tiene que conocerse en tiempo de compilación

  // --------------------------------------------------------------------------------

  // VECTOR: todos los valores tienen que ser del mismo tipo de dato

  // Inicialización tradicional
  let mut my_vector: Vec<i32> = Vec::new();

  // Macro para inicializar vector menos verboso
  let mut vector_2 = vec![1, 2];

  // Agregar elementos
  my_vector.push(1);
  my_vector.push(2);
  my_vector.push(3);
  my_vector.push(4);

  vector_2.push(5);
  
  // Una vez que termina el scope, es liberado de memoria

  // Acceder a un elemento
  let third = my_vector[2];
  println!("El tercer elemento es: {}", third);

  // Es más recomendable usar el método .get que devuelve un option, si el indice está fuera de rango se rompe

  let third_2 = vector_2.get(2);
  if third_2.is_some() {
    println!("El tercer elemento es: {}", third_2.unwrap());
  }

  // --------------------------------------------------------------------------------

  // STRING & STRING-SLICE

  // Son una colección de caracteres, específicamente UTF-8 (Vec<u8>)

  let string_1 = String::from("Hola"); // Tiene su espacio en memoria y puede crecer
  let string_2 = "Chau"; // Está hardcodeado en el binario, no puede cambiar.

  // --------------------------------------------------------------------------------

  // HASHSET

  // Es un wrapper de Hashmap, no permite duplicados (simil SET JS)

  let mut ids_1 = HashSet::new();

  ids_1.insert(100);
  ids_1.insert(150);
  ids_1.insert(200);
  ids_1.insert(200);

  for id in ids_1.iter() {
    println!("ID: {}", id);
  }

  // union: devuelve un nuevo hashset con los elementos de ambos
  // intersection: devuelve un nuevo hashset con los elementos que se repiten en ambos
  // difference: devuelve un nuevo hashset con los elementos que no se repiten en ambos
  // symetric_difference: devuelve un nuevo hashset con los elementos que no se repiten en ambos

  // let ids_2: HashSet<u32> = vec![1, 2, 3, 4, 5];

  // HASHMAPS

  // key-value

}