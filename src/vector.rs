// Copyright 2015 Markus Dittrich
// Licensed under BSD license, see LICENSE file for details

use std::ops::{Add,Sub};

/// Vec3 implements a simple 3D Vector
#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

/// Add overloads '+' operator for Vec3 references
impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
  type Output = Vec3;

  fn add(self, other: &'b Vec3) -> Vec3 {
    Vec3{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
  }
}

/// Sub overloads '-' operator for Vec3 references
impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
  type Output = Vec3;

  fn sub(self, other: &'b Vec3) -> Vec3 {
    Vec3{ x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
  }
}

/// Helper methods for Vec3
impl Vec3 {

  /// new returns a new Vec3 and is a convenience method
  pub fn new(v1: f64, v2: f64, v3: f64) -> Vec3 {
    Vec3{x: v1, y: v2, z: v3}
  }

  /// dot product
  pub fn dot(&self, other: &Vec3) -> f64 {
    (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
  }

  // cross product
  pub fn cross(&self, other: &Vec3) -> Vec3 {
    Vec3{ x: (self.y * other.z) - (self.z * other.y),
          y: (self.z * other.x) - (self.x * other.z),
          z: (self.x * other.y) - (self.y * other.x)}
  }

  /// scalar multiplication
  pub fn scalar(&self, other: f64) -> Vec3 {
    Vec3{ x: self.x * other, y: self.y * other, z: self.z * other}
  }

  // norm is the L2 norm of Vec3
  pub fn norm(&self) -> f64 {
    ((self).dot(self)).sqrt()
  }
}
