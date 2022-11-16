// use rand::random;
// struct User {
//     name: String,
//     age: u32,
//     city: String,
//     active: bool,
// }

// impl User {
//     fn is_happy(&self) -> bool {
//         return random::<bool>();
//     }
// }

// pub fn play_structs() {
//     println!("");
//     println!("This is structs");

//     let user1: User = make_user();

//     // copy with .. not working :(

//     // let user2 = User {
//     //   name: String::from("Jane"),
//     //   active: false,
//     //   ..user1
//     // };

//     // let user3 = User {
//     //   name: String::from("Jackie"),
//     //   age: 30,
//     //   ..user1
//     // };

//     println!(
//         "User 1: {:?}, {:?}, {:?}, {:?}, happy? {:?}",
//         user1.name,
//         user1.age,
//         user1.city,
//         user1.active,
//         user1.is_happy()
//     );
//     // println!("User 2: {:?}, {:?}, {:?}, {:?}, happy? {:?}", user2.name, user2.age, user2.city, user2.active, user2.is_happy() );
//     // println!("User 3: {:?}, {:?}, {:?}, {:?}, happy? {:?}", user3.name, user3.age, user3.city, user3.active, user3.is_happy() );
// }

// fn make_user() -> User {
//     return User {
//         name: String::from("John"),
//         age: 32,
//         city: String::from("New York"),
//         active: true,
//     };
// }
