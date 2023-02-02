pub fn perform_counting(n: i64) -> u32 {
    //BEST SOLUTION:
    //n.count_ones();

    //SECOND BEST SOLUTION:
    //format!("{:b}", n).matches('1').count() as u32
    
    //MY CAVEMAN SOLUTION:
    let mut quotient = n;
    let mut remainders: Vec<u32> = Vec::new();
    while quotient > 0 {
        remainders.push(match quotient {
            1 => 1,
            _ => (quotient % 2) as u32,
        });
        quotient = quotient / 2;
    }
    let result: u32 = remainders.iter().sum();
    return result;
}
