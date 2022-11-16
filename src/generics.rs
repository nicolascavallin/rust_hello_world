// pub fn main() {
//     println!("Hello, generics!");
//     /*
//      * Option<T> -> Se define el T en tiempo de ejecución, no en compilación
//      */

//     let point_a: Point<i32> = Point { x: 1, y: 2 };
//     let point_b: Point<f32> = Point { x: 1.5, y: 2.5 };

//     // No es tan flexible, va a ser f o i para los dos valores, por ende tendría que usar X.0 para enteros, o presentar una solución como la siguiente

//     let point_c: PointFlex<i32, f32> = PointFlex { x: 1, y: 2.5 };

//     // También puedo NO definir los tipos

//     let point_d = PointFlex { x: "Hola", y: true };

//     print(point_a);
//     print(point_b);
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// struct PointFlex<T, V> {
//     x: T,
//     y: V,
// }

// fn print<T>(data: Point<T>) {
//   // match data {
//   //   Point { x, y } => println!("x: {}, y: {}", x, y),
//   //   _ => println!("Hey")
//   // }
// }