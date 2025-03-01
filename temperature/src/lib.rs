pub fn f_to_c(n: f64) -> f64 {
    let c = (n - 32 as f64) / 1.8 as f64;
    c
}

pub fn c_to_f(n: f64) -> f64 {
   let f = (n * 1.8 as f64) + 32 as f64;
   f
}
