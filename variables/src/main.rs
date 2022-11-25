const STARTING_MISSILES : i32 = 8;
const READY_AMOUNT : i32 = 2;

fn main() {
    // println!("Hello, world!");

    // Part 1
    let missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);

    // Part 2
    let missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
