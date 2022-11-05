// enums are types with few definitive values

enum Movement{
    //variants
    Up,
    Down,
    Left,
    Right
}

fn move_ava(m: Movement) {
    //perform action depending on info
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    }
}

pub fn run() {
    let ava1 = Movement::Left;
    let ava2 = Movement::Up;
    let ava3 = Movement::Right;
    let ava4 = Movement::Down;

    move_ava(ava1);
    move_ava(ava2);
    move_ava(ava3);
    move_ava(ava4);

}