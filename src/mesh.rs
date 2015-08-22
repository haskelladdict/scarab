// Copyright 2015 Markus Dittrich
// Licensed under BSD license, see LICENSE file for details

use vector;


#[derive(Debug, Default)]
/// MeshElem describes a single triangular mesh element which may be part of
/// a larger mesh object
pub struct MeshElem {
  pub a: vector::Vec3,  // vertex 1
  pub b: vector::Vec3,  // vertex 2
  pub c: vector::Vec3,  // vertex 3
  pub u: vector::Vec3,  // triangle vectors
  pub v: vector::Vec3,
  pub n: vector::Vec3,  // normal vector
}

/// methods for MeshElem
impl MeshElem {

  /// new returns a new MeshElement
  pub fn new(v1: vector::Vec3, v2: vector::Vec3, v3: vector::Vec3) -> MeshElem {
    let u_ = &v2 - &v1;
    let v_ = &v3 - &v1;
    let n_  = u_.cross(&v_);
    MeshElem{a: v1, b: v2, c: v3, u: u_, v: v_, n: n_ }
  }
}

/// Mesh is a vector of MeshElem
pub type Mesh = Vec<MeshElem>;

/// make_rect is a helper function for creating a rectangular mesh bounded by
/// a Vec3 to the lower left corner and upper right corner
pub fn make_rect(llc: &vector::Vec3, urc: &vector::Vec3) -> Mesh {
  assert!(llc.x < urc.x && llc.y < urc.y && llc.z < urc.z);

  // compute 8 corners
  let diag = urc - llc;
  let c0 = *llc;
  let c1 = llc + &vector::Vec3::new(diag.x, 0.0, 0.0);
  let c2 = llc + &vector::Vec3::new(0.0, diag.y, 0.0);
  let c3 = llc + &vector::Vec3::new(0.0, 0.0, diag.z);
  let c4 = llc + &vector::Vec3::new(diag.x, diag.y, 0.0);
  let c5 = llc + &vector::Vec3::new(diag.x, 0.0, diag.z);
  let c6 = llc + &vector::Vec3::new(0.0, diag.y, diag.z);
  let c7 = *urc;

  vec![
    MeshElem::new(c0, c1, c5),
    MeshElem::new(c0, c5, c3),
    MeshElem::new(c1, c4, c7),
    MeshElem::new(c1, c7, c5),
    MeshElem::new(c4, c2, c6),
    MeshElem::new(c4, c6, c7),
    MeshElem::new(c2, c0, c3),
    MeshElem::new(c2, c3, c6),
    MeshElem::new(c5, c7, c6),
    MeshElem::new(c5, c6, c3),
    MeshElem::new(c0, c2, c1),
    MeshElem::new(c1, c2, c4),
  ]
}

