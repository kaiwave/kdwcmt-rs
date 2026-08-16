

let sigma = 5.67e-8;                                                                     // Stefan-Boltzmann constant
let earth_irradiance = 1361.0;                                                           // Solar constant
let earth_age = 4.543e9;                                                                 // Age of the Earth

#[derive(Default)]                                                                       // Default parameter initialisation is weird in Rust :p
struct initOptions {
	pub grey_albedo: f64,
  pub white_albedo: f64,
  pub black_albedo: f64,
  pub white_ideal_temperature: f64,
  pub black_ideal_temperature: f64,
  pub tolerance: f64,
  pub max_growth_rate: f64,
  pub death_rate: f64,
  pub initial_area: f64,
  pub solar_fraction: f64,
}

impl Default for initOptions {
    fn default() -> Self {                                                               // Creates emoty values for all the stuff above
        Self {
            grey_albedo: 0.5,
            white_albedo: 0.75,
            black_albedo: 0.25,
            white_ideal_temperature: 280.0,
            black_ideal_temperature: 270.0,
            tolerance: 0.003,
            max_growth_rate: 1e-6,
            death_rate: 3e-7,
            initial_area: 0.0001,
            solar_fraction: 2.0 / 3.0,
        }
    }
}

pub fn daisyWorld() {


}