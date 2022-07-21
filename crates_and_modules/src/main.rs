mod garden;

use crate::garden::vegetables::Asparagus;

fn main() {
    garden::hello();

    let plant = Asparagus {};

    println!("{:?}", plant)
}
