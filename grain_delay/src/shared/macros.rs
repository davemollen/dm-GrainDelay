#[macro_export]
macro_rules! assert_approximately_eq {
  ($left:expr, $right:expr, $digits:expr $(,)?) => {{
    let tol = 10f32.powi(-($digits as i32));
    let diff = ($left - $right).abs();
    assert!(
      diff <= tol,
      "Values are not approximately equal: left={}, right={}, diff={}, tol={}",
      $left,
      $right,
      diff,
      tol
    );
  }};
}
