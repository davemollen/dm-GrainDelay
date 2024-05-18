use std::f32::consts::{FRAC_PI_2, PI};

pub trait FloatExt {
  fn dbtoa(self) -> Self;
  fn scale(self, in_low: Self, in_high: Self, out_low: Self, out_high: Self) -> Self;
  fn mix(self, right: f32, factor: f32) -> Self;
  fn fast_atan1(self) -> Self;
  fn fast_atan2(self) -> Self;
  fn fast_tanh1(self) -> Self;
  fn fast_tanh2(self) -> Self;
  fn fast_tanh3(self) -> Self;
  fn fast_sin(self) -> Self;
  fn fast_cos(self) -> Self;
  fn fast_sin_bhaskara(self) -> Self;
  fn fast_cos_bhaskara(self) -> Self;
  fn fast_pow(self, exponent: Self) -> Self;
  fn fast_exp(self) -> Self;
  fn mstosamps(self, sample_rate: Self) -> Self;
  fn is_equal_to(self, other: Self) -> bool;
}

impl FloatExt for f32 {
  /// Converts decibels to a linear amplitude value
  fn dbtoa(self) -> Self {
    (10_f32).powf(self * 0.05)
  }

  fn scale(self, in_low: Self, in_high: Self, out_low: Self, out_high: Self) -> Self {
    let in_scale = 1. / (in_high - in_low);
    let out_range = out_high - out_low;
    let normalized_value = (self - in_low) * in_scale;
    normalized_value * out_range + out_low
  }

  fn mix(self, right: f32, factor: f32) -> Self {
    self * (1. - factor) + right * factor
  }

  /// This is an atan approximation
  fn fast_atan1(self) -> Self {
    let a1 = 0.99997726;
    let a3 = -0.33262347;
    let a5 = 0.19354346;
    let a7 = -0.11643287;
    let a9 = 0.05265332;
    let a11 = -0.01172120;
    let squared_self = self * self;
    self
      * (a1
        + squared_self
          * (a3
            + squared_self * (a5 + squared_self * (a7 + squared_self * (a9 + squared_self * a11)))))
  }

  /// This is an atan approximation, not atan2. This variant only amplifies the first harmonic instead of multiple.
  /// https://www.dsprelated.com/showarticle/1052.php
  fn fast_atan2(self) -> Self {
    let n1 = 0.97239411;
    let n2 = -0.19194795;
    (n1 + n2 * self * self) * self
  }

  /// This is a tanh approximation.
  fn fast_tanh1(self) -> Self {
    let squared_self = self * self;
    let a = self * (135135. + squared_self * (17325. + squared_self * (378. + squared_self)));
    let b = 135135. + squared_self * (62370. + squared_self * (3150. + squared_self * 28.));
    a / b
  }

  /// This is a tanh approximation. It's cheaper than fast_tanh1, but looses accuracy for higher input values (< -1 and > 1).
  fn fast_tanh2(self) -> Self {
    let x2 = self * self;
    let x3 = x2 * self;
    let x4 = x3 * self;
    (105. * self + 10. * x3) / (105. + 45. * x2 + x4)
  }

  /// This is a tanh approximation. For more accuracy (less aliasing) choose fast_tanh1 or fast_tanh2.
  fn fast_tanh3(self) -> Self {
    let a = self.abs();
    let b = 1.26175667589988239 + a * (-0.54699348440059470 + a * (2.66559097474027817));
    (b * self) / (b * a + 1.)
  }

  /// This is a sine approximation. Use this to safe processing power.
  fn fast_sin(self) -> Self {
    const TWOPI: f32 = 6.2831853071795865;
    const INVTWOPI: f32 = 0.15915494309189534;
    let k: u32 = (self * INVTWOPI) as u32;
    let half = if self < 0_f32 { -0.5_f32 } else { 0.5_f32 };
    let x = (half + (k as f32)) * TWOPI - self;
    sin_approx(x)
  }

  /// This is a cosine approximation. Use this to safe processing power.
  fn fast_cos(self) -> Self {
    const TWOPI: f32 = 6.2831853071795865;
    const INVTWOPI: f32 = 0.15915494309189534;
    let x = self + FRAC_PI_2;
    let k: u32 = (x * INVTWOPI) as u32;
    let half = if x < 0_f32 { -0.5_f32 } else { 0.5_f32 };
    let x_new = (half + (k as f32)) * TWOPI - x;
    sin_approx(x_new)
  }

  /// This is the Bhaskara sine approximation. It returns a sine from 0 to 180 degrees.
  /// This function expects values between 0. and 1.
  fn fast_sin_bhaskara(self) -> Self {
    let x = self * FRAC_PI_2;
    let pi_squared = 9.869604401089358;
    let a = x * (PI - x);
    (16. * a) / (5. * pi_squared - 4. * a)
  }

  /// This is the Bhaskara cosine approximation. It returns a sine from 0 to 180 degrees.
  /// This function expects values between 0. and 1.
  fn fast_cos_bhaskara(self) -> Self {
    let x = self * FRAC_PI_2;
    let x_squared = x * x;
    let pi_squared = 9.869604401089358;
    (pi_squared - 4. * x_squared) / (pi_squared + x_squared)
  }

  fn fast_pow(self, exponent: Self) -> Self {
    pow2(exponent * log2(self))
  }

  /// Exponential function.
  fn fast_exp(self) -> Self {
    pow2(1.442695040_f32 * self)
  }

  /// Convert milliseconds to samples based on the samplerate.
  fn mstosamps(self, sample_rate: Self) -> Self {
    self * 0.001 * sample_rate
  }

  /// Check if one value is equal to the other
  fn is_equal_to(self, other: Self) -> bool {
    (if self > other {
      self - other
    } else {
      other - self
    }) <= Self::EPSILON
  }
}

#[cfg(test)]
mod tests {
  use super::FloatExt;
  use std::f32::consts::{FRAC_1_SQRT_2, PI};

  fn assert_approximately_eq(left: f32, right: f32) {
    assert_eq!((left * 100.).floor() / 100., (right * 100.).floor() / 100.)
  }

  #[test]
  fn dbtoa() {
    assert_eq!((-3f32).dbtoa(), 0.70794576);
    assert_eq!((-6f32).dbtoa(), 0.5011872);
    assert_eq!((-12f32).dbtoa(), 0.25118864);
  }

  #[test]
  fn scale() {
    assert_eq!((1f32).scale(1., 500., -6., -15.), -6.);
    assert_eq!((250f32).scale(1., 500., -6., -15.), -10.490982);
    assert_eq!((500f32).scale(1., 500., -6., -15.), -15.);
  }

  #[test]
  fn mix() {
    assert_eq!((1f32).mix(0., 0.), 1.);
    assert_eq!((1f32).mix(0., 0.5), 0.5);
    assert_eq!((1f32).mix(0., 1.), 0.);
  }

  #[test]
  fn fast_atan1() {
    assert_approximately_eq((0.5).fast_atan1(), (0.5f32).atan());
    assert_approximately_eq((-0.5).fast_atan1(), (-0.5f32).atan());
    assert_approximately_eq((1.).fast_atan1(), (1f32).atan());
    assert_approximately_eq((-1.).fast_atan1(), (-1f32).atan());
  }

  #[test]
  fn fast_atan2() {
    assert_approximately_eq((0.5).fast_atan2(), (0.5f32).atan());
    assert_approximately_eq((-0.5).fast_atan2(), (-0.5f32).atan());
    assert_approximately_eq((1.).fast_atan2(), (1f32).atan());
    assert_approximately_eq((-1.).fast_atan2(), (-1f32).atan());
  }

  #[test]
  fn fast_tanh1() {
    assert_approximately_eq((0.5).fast_tanh1(), (0.5f32).tanh());
    assert_approximately_eq((-0.5).fast_tanh1(), (-0.5f32).tanh());
    assert_approximately_eq((1.).fast_tanh1(), (1f32).tanh());
    assert_approximately_eq((-1.).fast_tanh1(), (-1f32).tanh());
    assert_approximately_eq((1.5).fast_tanh1(), (1.5f32).tanh());
    assert_approximately_eq((-1.5).fast_tanh1(), (-1.5f32).tanh());
  }

  #[test]
  fn fast_tanh2() {
    assert_approximately_eq((0.5).fast_tanh2(), (0.5f32).tanh());
    assert_approximately_eq((-0.5).fast_tanh2(), (-0.5f32).tanh());
    assert_approximately_eq((1.).fast_tanh2(), (1f32).tanh());
    assert_approximately_eq((-1.).fast_tanh2(), (-1f32).tanh());
    assert_approximately_eq((1.5).fast_tanh2(), (1.5f32).tanh());
    assert_approximately_eq((-1.5).fast_tanh2(), (-1.5f32).tanh());
  }

  #[test]
  fn fast_tanh3() {
    assert_approximately_eq((0.5).fast_tanh2(), (0.5f32).tanh());
    assert_approximately_eq((-0.5).fast_tanh2(), (-0.5f32).tanh());
    assert_approximately_eq((1.).fast_tanh2(), (1f32).tanh());
    assert_approximately_eq((-1.).fast_tanh2(), (-1f32).tanh());
    assert_approximately_eq((1.5).fast_tanh2(), (1.5f32).tanh());
    assert_approximately_eq((-1.5).fast_tanh2(), (-1.5f32).tanh());
  }

  #[test]
  fn fast_sin() {
    assert_approximately_eq((0.1).fast_sin(), (0.1f32).sin());
    assert_approximately_eq((PI * 1.5).fast_sin(), (PI * 1.5).sin());
    assert_approximately_eq((PI * -1.9).fast_sin(), (PI * -1.9).sin());
  }

  #[test]
  fn fast_cos() {
    assert_approximately_eq((0.1).fast_cos(), (0.1f32).cos());
    assert_approximately_eq((PI * 1.5).fast_cos(), (PI * 1.5).cos());
    assert_approximately_eq((PI * 1.9).fast_cos(), (PI * 1.9).cos());
  }

  #[test]
  fn fast_bhaskara() {
    assert_approximately_eq((0.).fast_sin_bhaskara(), 0.);
    assert_approximately_eq((0.5).fast_sin_bhaskara(), FRAC_1_SQRT_2);
    assert_approximately_eq((1.).fast_sin_bhaskara(), 1.);
    assert_approximately_eq((0.).fast_cos_bhaskara(), 1.);
    assert_approximately_eq((0.5).fast_cos_bhaskara(), FRAC_1_SQRT_2);
    assert_approximately_eq((0.5).fast_cos_bhaskara(), (0.5).fast_sin_bhaskara());
    assert_approximately_eq((0.).fast_cos_bhaskara(), (1.).fast_sin_bhaskara());
  }
}

fn log2(x: f32) -> f32 {
  let mut y = x.to_bits() as f32;
  y *= 1.1920928955078125e-7_f32;
  y - 126.94269504_f32
}

fn pow2(p: f32) -> f32 {
  let clipp = if p < -126.0 { -126.0_f32 } else { p };
  let v = ((1 << 23) as f32 * (clipp + 126.94269504_f32)) as u32;
  f32::from_bits(v)
}

fn sin_approx(x: f32) -> f32 {
  const FOUROVERPI: f32 = 1.2732395447351627;
  const FOUROVERPISQ: f32 = 0.40528473456935109;
  const Q: f32 = 0.77633023248007499;

  let mut p = 0.22308510060189463_f32.to_bits();
  let mut v = x.to_bits();

  let sign: u32 = v & 0x80000000;
  v &= 0x7FFFFFFF;

  let qpprox = FOUROVERPI * x - FOUROVERPISQ * x * f32::from_bits(v);

  p |= sign;

  qpprox * (Q + f32::from_bits(p) * qpprox)
}
