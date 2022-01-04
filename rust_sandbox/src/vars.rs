pub fn run() {
    let name = "Chen";
    let mut age = 32;
    println!("My name is {}, and I am {}", name, age);
    age = 33;
    println!("My name is {}, and I am {}", name, age);

    // Define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Xu Chen", 32);
    println!("{} is {}", my_name, my_age);
}