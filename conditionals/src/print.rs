pub fn run() {
    let age: u8 = 28;
    let check_id: bool = true;
    let knows_person_of_age: bool = true;

    if age >= 21 && check_id || knows_person_of_age{
        println!("Bartender says lets get wasted.");
    } else if age < 21 && check_id{
        println!("Bartender serves you juice.");
    } else {
        println!("I'll need to see ID.");
    }

    //shorthand if
    let is_of_age = if age >= 21 {true} else {false};
    println!("is of age : {}", is_of_age);

    //different variables , same concept
    let name = "Monis";
    let knows_japarjam : bool = true;
    let is_developer : bool = true;

    if name == "Monis" && knows_japarjam && is_developer{
        println!("You've been shortlisted for the cohort.");
    } else if name != "Monis" && knows_japarjam && is_developer {
            println!("Let's fill out the form and schedule a meet.");
    } else {
        println!("Please fill out the form.");
    }

    let is_a_potential_candidate = if is_developer {true} else {false};
    println!("is a potential candidate {}.", is_a_potential_candidate);
}
