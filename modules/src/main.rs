// this line is saying to _include_ the garden module, not define it.
// `garden` is discovered by the compiler by checking for `garden.rs` or `garden/mod.rs`.
// importing garden will implicitly reveal all public elements of the garden module, including the `vegetables` submodule.
pub mod garden;

// It is this line which brings the public struct `Asparagus` into scope, not the importing of the module.
use crate::garden::vegetables::Asparagus;
use crate::garden::Garden;

fn main() {
    let garden = Garden { has_grass: true };
    let plant = Asparagus {};
    println!("Does the garden have grass? {:?}!", garden);
    println!("I'm growing {:?}!", plant);
}
