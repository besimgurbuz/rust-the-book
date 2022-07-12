mod fibonacci;
mod temperature_converter;

fn main() {
    let one_fah_in_cels = temperature_converter::convert_to_celsius(1.0);
    let one_cels_in_fah = temperature_converter::convert_to_fahrenheit(1.0);
    let twentieth_fibonacci = fibonacci::generate_nth_fibonacci_number(20);

    println!("1 fahrenheit = {one_fah_in_cels} celsius");
    println!("1 celsius = {one_cels_in_fah} fahrenheit");
    println!("20th fibonacci is: {twentieth_fibonacci}");
}
