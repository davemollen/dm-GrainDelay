use std::{env, fs, path::Path};

const MAX_PITCH: f32 = 24.;
const MAX_DRIFT: f32 = 2.;
const MIN_FREQUENCY: f32 = 1.;
const MAX_SPRAY: f32 = 0.5;

fn get_max_grain_delay_time() -> f32 {
  ((1. + 2f32.powf((MAX_PITCH + MAX_DRIFT) / 12.)) * MIN_FREQUENCY + MAX_SPRAY).ceil()
}

fn main() {
  let max_grain_delay_time = format!(
    "pub const MAX_GRAIN_DELAY_TIME: f32 = {:.1};",
    get_max_grain_delay_time()
  );
  let max_drift = format!("\npub const MAX_DRIFT: f32 = {MAX_DRIFT:.1};");
  let constants_content = format!("{max_grain_delay_time}{max_drift}");

  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("constants.rs");
  fs::write(&dest_path, constants_content).unwrap();
}
