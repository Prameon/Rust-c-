fn main() {
    let frequency = osc_string(0.648, 32.0, 1118.0, 0.01);
    println!("{}", frequency)
}

fn osc_string(
    l:f64,
    f:f64,
    p:f64,
    s:f64) ->f64{
    (1.0 /(2.0 * l ))*(f64::sqrt(f /(p * s)))
}
