// Copyright 2015 Markus Dittrich
// Licensed under BSD license, see LICENSE file for details

use geometry::{intersect, geom_epsilon, geom_epsilon_2};
use mesh;
use molecule;
use random;
use vector;

/// diffuse propagates molecule mol for time dt
pub fn diffuse(mol: &mut molecule::Mol, meshes: &mesh::Mesh, d: f64, dt: f64,
  rng: &mut random::rng) {

  // compute displacement
  let scale = (4.0 * d * dt).sqrt();
  let mut disp = vector::Vec3{x: scale * rng.norm(), y: scale * rng.norm(), z: scale * rng.norm() };

  // diffuse and collide until we're at the end of our diffusion step
  loop {
    let (done, disp) = collide(mol, meshes, disp);
    if done {
      break;
    }
  }
  let pos = mol.r;
  mol.move_to(&pos + &disp);
}


/// collide propagates mol until the next hit with a wall until the motion
/// is completely done. In the first case it returns (false, remaining_displacement)
/// or (true, null_displacement) otherwise.
fn collide(mol: &mut molecule::Mol, meshes: &mesh::Mesh, mut disp: vector::Vec3)
  -> (bool, vector::Vec3) {

  for m in meshes {
    let (status, mut hit) = intersect(&mol.r, &disp, m);
    if (status != 0) {
      continue
    }
    let disp_rem = &hit - &mol.r;

    // reflect: Rr = Ri - 2 N (Ri * N)
    disp = &disp_rem - &(m.nn.scalar(2.0*disp_rem.dot(&m.nn)));

    // move slightly away from the triangle along the reflected ray.
    // If we happen to end our ray at hitpoint we move along the triangle
    // normal instead.
    if disp.norm2() > geom_epsilon_2 {
      let n = disp.norm();
      let disp_n = disp.scalar(1.0/n);
      hit = &hit + &disp_n.scalar(geom_epsilon);
      disp = disp_n.scalar(n - geom_epsilon);
    } else {
      let mut side = 1;
      if disp_rem.dot(&m.nn) > 0.0 {
        side = -1;
      }
      hit = &hit + &m.nn.scalar((side as f64) * geom_epsilon);
    }
    mol.move_to(hit);
    return (false, disp);
  }
  (true, vector::Vec3{x:0.0, y:0.0, z:0.0})
}

/*

fn collide(const State& state, VolMol& mol, Vector3D& disp) {
  for (const auto& mesh : state.get_Meshes()) {
    for (const auto& m : mesh->get_meshElements()) {
      Vector3D hitPoint;
      if (intersect(mol.pos(), disp, m.get(), &hitPoint) == 0) {
        Vector3D disp_rem = hitPoint - mol.pos();

        // reflect: Rr = Ri - 2 N (Ri * N)
        disp = disp_rem - (2 * (disp_rem * m->n_norm())) * m->n_norm();

        // move slightly away from the triangle along the reflected ray.
        // If we happen to end our ray at hitpoint we move along the triangle
        // normal instead.
        if (disp.norm2() > GEOM_EPSILON_2) {
          double n = disp.norm();
          auto disp_n = (1.0 / n) * disp;
          hitPoint += GEOM_EPSILON * disp_n;
          disp = (n - GEOM_EPSILON) * disp_n;
        } else {
          std::cout << "foo" << std::endl;
          double side = (disp_rem * m->n_norm()) > 0 ? -1 : 1;
          hitPoint += side * GEOM_EPSILON * m->n_norm();
        }
        mol.moveTo(hitPoint);
        return 1;
      }
    }
  }
  return 0;
}
*/
