use std::{env, fs, path::Path};

const MAX_PITCH: f32 = 24.;
const MAX_DRIFT: f32 = 1.;
const MIN_FREQUENCY: f32 = 1.;

fn get_max_grain_delay_time() -> f32 {
  (1. + 2f32.powf((MAX_PITCH + MAX_DRIFT) / 12.)) * MIN_FREQUENCY
}

fn main() {
  let max_grain_delay_time = get_max_grain_delay_time();

  let constants_content = format!("pub const MAX_GRAIN_DELAY_TIME: f32 = {max_grain_delay_time};");

  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("constants.rs");
  fs::write(&dest_path, constants_content).unwrap();
}
