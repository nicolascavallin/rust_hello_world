pub fn main() {
    println!("Hello, generics!");
    /*
     * Option<T> -> Se define el T en tiempo de ejecución, no en compilación
     */

    let pointA: Point<i32> = Point { x: 1, y: 2 };
    let pointB: Point<f32> = Point { x: 1.5, y: 2.5 };

    // No es tan flexible, va a ser f o i para los dos valores, por ende tendría que usar X.0 para enteros, o presentar una solución como la siguiente

    let pointC: PointFlex<i32, f32> = PointFlex { x: 1, y: 2.5 };

    // También puedo NO definir los tipos

    let pointD = PointFlex { x: "Hola", y: true };
}

struct Point<T> {
    x: T,
    y: T,
}

struct PointFlex<T, V> {
    x: T,
    y: V,
}
