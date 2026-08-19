

const SIGMA: f64 = 5.67e-8;                                                                     // Stefan-Boltzmann constant
const EARTH_IRRADIANCE: f64 = 1361.0;                                                           // Solar constant
const EARTH_AGE: f64 = 4.543e9;                                                                 // Age of the Earth

pub struct InitOptions {                                                                         // Default parameter initialisation is weird in Rust :p
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

impl Default for InitOptions {
    fn default() -> Self {                                                                       // Creates emoty values for all the stuff above
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

fn check_albedo_validity(albedo: f64) -> bool {
    if albedo < 0.0 || albedo > 1.0 {
        println!("Albedo value {} must be in range [0, 1]", albedo);
        return false;
    }
    true
}

fn irradiance_over_time(time: f64, solar_fraction: f64) -> f64 {
  let mut dimensionless_time = (time - EARTH_AGE) / EARTH_AGE + 1;
  let mut dimensionless_irradiance: f64 = solar_fraction + (1-solar_fraction) * dimensionless_time; 

  return dimensionless_irradiance * EARTH_IRRADIANCE;
}

pub fn daisy_world(config: InitOptions) {
  check_albedo_validity(config.black_albedo);
  check_albedo_validity(config.white_albedo);
  check_albedo_validity(config.grey_albedo);

}