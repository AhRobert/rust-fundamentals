// using the loop keyword is useful to avoid having to define the condition upfront
// or if the condition is met in the middle of the loop
// It is also useful when you want to loop without knowing exactly when to stop

fn main() {
    let mut x: i64= 1;
    // continue looping until x > 10
    loop {
        println!("x is {}", x);
        x += x*2;
        if x > 50 {
            break;
        }
    }
}
