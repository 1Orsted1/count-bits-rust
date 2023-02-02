//challenge: count the 1ns in the selected number converted to binary

mod bit_counter;

fn main() {

    let perform_counting = bit_counter::perform_counting;

    println!("result: {}",perform_counting(0));
    println!("result: {}",perform_counting(4));
    println!("result: {}",perform_counting(7));
    println!("result: {}",perform_counting(9));
    println!("result: {}",perform_counting(10));
    println!("result: {}",perform_counting(1234));
    //
}

