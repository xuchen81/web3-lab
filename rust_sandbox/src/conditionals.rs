pub fn run() {
    let age: i8 = 22;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: what to drink");
    } else if age < 21 && check_id {
        println!("Sorry, leave");
    } else  {
        println!("Please show id");
    }

    // Shorthand If
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}
