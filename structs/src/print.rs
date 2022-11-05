// structs used to create custom data types.

// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }

//tuple struct
// struct Color(u8, u8, u8);

struct Person {
    firstname: String,
    lastname: String
}

impl Person {
    //construct a person
    fn new(first: &str, last: &str) -> Person {
        Person {
            firstname: first.to_string(),
            lastname: last.to_string()
        }
    }

    //get full name
    fn fullname(&self) -> String {
        format!("{} {}", self.firstname,self.lastname)

    }
    //get lastname
    fn getlast(&mut self, last: &str){
        self.lastname = last.to_string();
    }
    //name to tuple    
    fn to_tuple(self) -> (String, String) {
        (self.firstname, self.lastname)
    }
}

pub fn run() {

    // let mut c = Color{
    //     red: 255,
    //     green: 10,
    //     blue: 110
    // };   

    // c.red = 200; 

    // println!("Color: {} {} {}",c.red, c.green, c.blue );

    // let mut a = Color(239, 67, 87);
    // a.1 = 200;
    // println!("Color : {} {} {}", a.0, a.1,a.2);

    let mut p = Person::new("Monis", "Tahir");
    p.getlast("Taher");
    println!("Person : {}", p.fullname());

    println!("Person : {:?}", p.to_tuple());

}