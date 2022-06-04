#[allow(unused)]
fn main() 
{
    let a = 1.0;

    let c: i16 = (1 << 15);
    let b = a * ((1 << 15) as f64);

    println!("{}", b);
}