pub fn fibonacci(n: u32) -> u32 {
    let golden_ratio = 1 as f64 +(5 as f64).sqrt()/2 as f64;
    let conjugate = 1 as f64-(5 as f64).sqrt()/2 as f64;
    let answer = (golden_ratio.powf(n as f64) - conjugate.powf(n as f64)) / 5.0_f64.sqrt();
    answer as u32
 }


