// use hello::greet; // how to bring standard library to use
// rust std vec (example)
// crates.io rust package registry
#![allow(unused_variables, dead_code, unused_braces, unused_mut)]

use std::collections::HashMap;
use std::fs::File;
use std::thread;

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
        // println!("{}", num)
    }

    let array = [(1,2), (3, 4)];
    for (_x, _y) in array.iter() {
        // do stuff
    }

    for num in 0..=5 { // če dodaš = vključi zraven tudi 5
        // println!("{}", num); 
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

    // STRUCTS
    let fox = RedFox {
        enemy: true,
        life: 70,
    };

    let fox2 = RedFox::new(); // accesing the associative function of a struct
    let life_left = fox.life;
    // fox.enemy = false; // calling methods


    // let robot = Robot {};
    // robot.run(); 

    // VECTORS
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(4);
    v.push(6);
    let x = v.pop(); // x is 6
    println!("vector {}", v[1]); // prints 4

    let mut vv = vec![2, 4, 6];
    // insert, remove, split...

    // HASHMAPS (imagine dictionaries, lookup tables)
    // vmr ustni :(
    let mut h: HashMap<u8, bool> = HashMap::new(); // specify type of key and value - byte, bool
    h.insert(5, true);
    h.insert(6, false);
    let have_five = h.remove(&5).unwrap();
    println!("{}", have_five); // vrne vrednost true pri 5ki

    // other possibly useful collections
    // VecDeque, LinkedList, HashSet, BinaryHeap, BTreeMap, BTreeSet


    // ENUMS
    enum Color {
        Red, 
        Green,
        Blue,
    }

    let color = Color::Red;

    enum DispenserItem {
        Empty,
        Ammo(u8), 
        Things(String, i32), // tuple of data
        Place {x: i32, y: i32},
    }

    use DispenserItem::*; // item can be any of those things but just one at the time
    let item0 = Empty; 
    let item1 = Ammo(50); 
    let item2 = Things("hat".to_string(), 7); 
    let item3 = Place { x: 24, y: 48 }; 

    impl DispenserItem {
        fn display(&self) {}
    }

    // generic enum in the std lib
    // enum Option<MYTYPE> { // T as any type
    //     Some(MYTYPE), 
    //     None,
    // }

    // if let 
    // if let Some(x) = my_variable {
    //     println!("value is {}", x);
    // }

    let my_variable: Option<i32> = Some(42);
    // let my_variable: Option<i32> = None;
    // PATTERN MATCHING :)))))))))))
    match my_variable {
        Some(x) => {
            println!("value is {}", x);
        },
        None => {
            println!("no value");
        },
    }

    match my_variable {
        _ => {
            println!("who cares");
        },
    }

    let x = match my_variable {
        Some(x) => x + 1,
        None => 42,
    };

    // useful
    // my_variable.is_none();
    // my_variable.is_some();

    // HANDLING ERRORS
    // let res = File::open("foo");
    // let f = res.expect("error message");
    // if res.is_ok() { // .is_err
    //     let ff = res.unwrap();
    // }
    // match res {
    //     OK(f) => ...;
    //     Err(f) => ...;
    // }
    
    // CLOSURES |x, y| { x + y}
    let add = |x, y| { x + y};
    add(1, 2);

    let mut vvv = vec![2, 4, 6];
    
    vvv.iter().map(|x| x * 3);
    // vvv.iter().filter(|x| *x > 10); zakaj tu ne dela
    vvv.iter().fold(0, |acc, x| acc + x);

    // THREADS

    let handle = thread::spawn(move || {
        // do stuff in a child thread
    });


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

struct RedFox {
    enemy: bool,
    life: u8,
}

// typically you would implement an associative function
impl RedFox { // implementation block (takes the struct)
    fn new() -> Self { // associative functions podobno kot class methods
        Self { // self used in place of struct name
            enemy: true,
            life: 70,
        }
    }

    // methods
    // fn move(self) ...
    // fn borrow(&self) ...
    // fn mut_borrow(&mut self) ...
}

trait Noisy { // specifiest that a struct must have a method called get_noise that returner a borrowed string slice if it wants to be noisy
    fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
    fn get_noise(&self) -> &str {
        { "Meow" }
    }
}


// TRAITS
// trait Robot {
//     fn run(&self) {
//         println!("I'm running!");
//     }
// }

// struct Robot {}
// impl Run for Robot {}

