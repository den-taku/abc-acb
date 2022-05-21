fn main() {
    let a: f32 = {
        let mut a: f32 = 1.0;
        for _ in 0..23 {
            a /= 2.0;
        }
        a
    };
    let b = a;
    let c: f32 = 2.0;

    println!("a + b + c = {:.9} but", a + b + c);
    println!("a + c + b = {:.9}.", a + c + b);

    println!("hint:");

    let a_bit = unsafe { *(&a as *const f32 as *const i32) };
    println!("1/2^23 = {a_bit:032b}");

    let c_bit = unsafe { *(&c as *const f32 as *const i32) };
    println!("2.0    = {c_bit:032b}");

    let a_b_bit = unsafe { *(&(a + b) as *const f32 as *const i32) };
    println!("a + b  = {a_b_bit:032b}");

    let a_c_bit = unsafe { *(&(a + c) as *const f32 as *const i32) };
    println!("a + c  = {a_c_bit:032b}");

    let a_b_c_bit = unsafe { *(&(a + b + c) as *const f32 as *const i32) };
    println!("a+b+c  = {a_b_c_bit:032b}");
    let a_c_b_bit = unsafe { *(&(a + c + b) as *const f32 as *const i32) };
    println!("a+c+b  = {a_c_b_bit:032b}");
}
