// Copyright 2015 Markus Dittrich
// Licensed under BSD license, see LICENSE file for details

extern crate rand;

use std::collections::HashMap;

mod diffuse;
mod geometry;
mod mesh;
mod molecule;
mod random;
mod util;
mod vector;

use diffuse::diffuse;
use vector::Vec3;


fn main() {

  // geometry
  let meshes = mesh::make_rect(Vec3{x: -1.0, y: -1.0, z: -1.0},
                               Vec3{x:  1.0, y:  1.0, z:  1.0});

  let mut specs = Vec::new();
  let mut mol_map : HashMap<usize, Vec<molecule::Mol>> = HashMap::new();

  let a_spec_id = 0;
  let a_spec = molecule::Species{ D: 600.0, ID: a_spec_id, name: String::from("foobar")};
  specs.push(a_spec);

  for _ in 0..22000 {
    let m1 = molecule::Mol{ ID: a_spec_id, r: vector::Vec3{x: 0.0, y: 0.0, z: 0.0}, t: 0.0};
    if mol_map.contains_key(&a_spec_id) {
      match mol_map.get_mut(&a_spec_id) {
        Some(mols) => mols.push(m1),
        None       => {},
      }
    } else {
      let m = Vec::new();
      mol_map.insert(a_spec_id, m);
    }
  }

  let dt = 1e-6;  // timestep
  let mut rng = random::rng::new(1);
  for iter in 0..1000 {
    println!("iter {}", iter);
    for (id, mols) in &mut mol_map {
      let d = specs[*id].D;
      for m in mols {
        diffuse(m, &meshes, d, dt, &mut rng);
      }
    }
  }
}













