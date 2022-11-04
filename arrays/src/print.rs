
//Arrays are lists with same data type elements.
//using mem from std lib for printing out the size of array.
use std::mem;

pub fn run() {
    let students_in_cohort: [i32; 12] = [1,2,3,4,5,6,7,8,9,10,11,12];
    println!("{:?}",students_in_cohort);

    //get single value
    println!("one of the students is at {}.", students_in_cohort[2]);

    //get array length
    println!("Array length {}",students_in_cohort.len());

    //arrays are stack allocated. In layman terms, size of array in memory.
    println!("This array occupies {} bytes. Isn't that amazing?", mem::size_of_val(&students_in_cohort));

    //time to get some slices of this array.
    let slice: &[i32] = &students_in_cohort[2..7];
    println!("Slice: {:?}", slice);
}