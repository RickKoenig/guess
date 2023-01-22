#![allow(unused_imports)]
pub mod main1;
pub mod main10;
pub mod main2;
pub mod main3;
pub mod main4;
pub mod main5;
pub mod main6;
pub mod main7;

use main10::main10;
use main5::main5;
use main6::main6;
use main7::main7;

fn main() {
    //main1::main1(); // guessing game
    //main2::main2(); // types
    //main3::main3(); // ownership
    //main4::main4(); // struct
    //main5(); // enums
    //main6(); // collections
    main7(); // error handling
    main10(); // genric types, traits, and lifetimes ( TODO: number chapters right)
}
