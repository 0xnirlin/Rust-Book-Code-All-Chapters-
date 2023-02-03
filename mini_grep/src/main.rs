fn main() {
    let a = 10;
    let b:i32 = 29;
    let c = 10_i32;
    let d = 25i32;

    let e = add(add(a,b),add(c,d));
    println!("(a+b) + (c+d) = {}",e)

    //numbers can have functions attached to them in rust like:
    // 24.5_f32.round() rather than (round(24.5_f32)).

}

fn add(a:i32,b:i32) -> i32 {
    a+b
}