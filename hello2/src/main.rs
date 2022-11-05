// Introducing the package manager of Rust, CARGO !!
// This similar to our last hello world program but,
// it uses a cargo anf imports functions from other files as well.
// We intialize the program by navigating to the 
// target file and type in "cargo init".
// This initializes the structure of the program
// and segregates the files.
// Navigating to the src folder, we can see the main.rs file.
// Let's create another file by the name of print.rs.
// We'll be using this file to import a function into main.rs.


//importing a function as a module.
mod print;

fn main() {
    print::run();
}
