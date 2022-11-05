//Due to the immutable nature and some security features,
//functions and variables are by default private and immutable in rust.
// We utilize the pub keyword to make our function public.

pub fn run() {
    println!("Hello World!");
    //formatting values.
    //values have to be formatted using a placeholder "{}".
    println!("Number : {}", 1);
    println!("{} is from {}","Monis", "India" );
    println!("He is {} years {}",25,"old");


    //positional arguments.
    //formatting values can be done differently,
    //let me explain.
    
    println!("{0} doesn't like to {1}","Monis","code");
    println!("{0} would however {1} to join {2} for learning {3}","Monis","like","WBA", "Cosmos");
    //We have mentioned the position of the values inside the placeholders. Works like MAGIC, doesn't it ?

    //named arguments
    println!(
        "{name} likes to {activity} {field}",
        name = "Monis", activity = "Manage", field = "Projects"
    );
    //see what we did there?
    //declared variables inside the print function.
    //wow, i think i like this language.

    //placeholder traits
    println!("Binary {:b} Hex {:x} Octal {:o}", 10,10,10);

    //placeholder for debug trait
    println!("{:?}", ("Monis", "accepted", "for", "cohort", "?"));

    //lets do some basic math with placeholders,
    println!("Monis + WBA = {}", "Cosmos".to_owned() + " Engineer");
    //why did i use .to_owned?
    //to keep the string while adding two strings.

}