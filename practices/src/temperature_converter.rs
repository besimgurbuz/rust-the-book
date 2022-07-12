pub fn convert_to_fahrenheit(celsius: f64) -> f64 {
  ((celsius * 9.0) / 5.0) + 32.0
}

pub fn convert_to_celsius(fahrenheit: f64) -> f64 {
  ((fahrenheit - 32.0) * 5.0) / 9.0
}
