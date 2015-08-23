// Copyright 2015 Markus Dittrich
// Licensed under BSD license, see LICENSE file for details

use vector;

/// Species describes a type of molecule
#[derive(Debug)]
pub struct Species {
  pub D   : f64,     // diffusion coefficient
  pub ID  : usize,   // species ID
  pub name: String,  // species name
}

// Mol are molecules of type species
#[derive(Debug)]
pub struct Mol {
  pub ID : usize,        // what species are we
  pub r  : vector::Vec3, // 3D position
  pub t  : f64,          // birthday
}

impl Mol {

  pub fn move_to(&mut self, new_r: vector::Vec3) {
    self.r = new_r
  }
}
