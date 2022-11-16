use rand::random;

pub fn play_enums() {
    println!("Welcome to Enums");
    // Playing with the Option enum

    let mut name: Option<String> = Some(String::from("John"));

    if random::<bool>() {
        name = None;
    }

    match name {
        Some(name) => println!("Your name is {}", name),
        None => println!("You don't have a name"),
    }
}
