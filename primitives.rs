fn main() {
    let b: bool = true;
    let f: f64 = 1.0;
    let i = 5i32;
    
    let default_f = 65.1;
    let default_i = 10;
    let mut infer_i = 7;
    infer_i = 9999999999999999999999999999;

    let mut mutable = 12;
    mutable = 19;
    
    let mut mutable = "test";
    println!("{} {} {}", b, f, i);
}
