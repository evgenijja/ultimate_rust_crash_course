use hello::greet; // how to bring standard library to use
// rust std vec (example)
// crates.io rust package registry

fn main() {

    // VARIABLES -----------------------------
    // println!("Hello, world!");
    // let bunnies = 16;
    // let (a, b) = (1, 2);
    let x = 5;
    {
        let y = 99;
        let x = 50; // povozi tist x od prej
        println!("{},{}", x, y);
        // variables used in a specific scope
    }
    println!("{},{}", x, x);

    // FUNCTIONS -------------------------------
    let x = do_stuff(2.0, 12.5);
    println!("{}", x);


    // MODULE SYSTEM ----------------------------

    hello::greet();
    let x = rand::thread_rng().gen_range(0,100);


}       


fn do_stuff(qty: f64, oz: f64) -> f64 {
    println!("{} {}-oz sarsaparilla(s)", qty, oz);
    qty*oz
}