// Copyright 2015 Markus Dittrich
// Licensed under BSD license, see LICENSE file for details

use std::f64;

/// same compares two floating point values
/// NOTE: This is a hack and needs to be made more robust
pub fn same(x: f64, y: f64) -> bool {
  return (x - y).abs() < f64::EPSILON
}
