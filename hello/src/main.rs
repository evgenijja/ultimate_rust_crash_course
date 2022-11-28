// use hello::greet; // how to bring standard library to use
// rust std vec (example)
// crates.io rust package registry
#![allow(unused_variables)]

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

    // hello::greet();
    // let x = rand::thread_rng().gen_range(0,100);

    // CONTROL FLOW
    let (a, b, c) = (true, 2, 3);
    let num = if a { b } else { c };
    println!("{}", num);

    loop {
        break;
    }

    'bob: loop {
        loop {
            loop {
                break 'bob;
            }
        }
    }

    // while condition() {
    //     // do stuff
    // }

    for num in [7, 8, 9].iter() {
        // do stuff
        println!("{}", num)
    }

    let array = [(1,2), (3, 4)];
    for (_x, _y) in array.iter() {
        // do stuff
    }

    for num in 0..=5 { // če dodaš = vključi zraven tudi 5
        println!("{}", num); 
    }

    // STRINGS ----------------------------------------------
    let _msg = "neki".to_string();


    // OWNERSHIP
    let s1 = String::from("abc");
    // let s2 = s1;
    // println!("{}", s2);

    // REFERENCES
    do_stuff2(&s1); // the reference goes into the function

    let mut s11 = String::from("abc");
    do_stuff3(&mut s11);

}       


fn do_stuff(qty: f64, oz: f64) -> f64 {
    println!("{} {}-oz sarsaparilla(s)", qty, oz);
    qty*oz
}

fn do_stuff2(s: &String) { // dollar sign je za refernece
    // do stuff
}

fn do_stuff3(s: &mut String) {
    s.insert_str(0, "Hi, ");
}