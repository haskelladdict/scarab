
extern crate rand;

mod geometry;
mod mesh;
mod molecule;
mod random;
mod util;
mod vector;

fn main() {

  let v1 = vector::Vec3{ x: 1.0, y: 1.0, z: 1.0};
  let v2 = vector::Vec3{ x: 2.0, y: 1.5, z: 1.5};
  let v3 = vector::Vec3{ x: 0.5, y: 3.3, z: 3.1};
  let m = mesh::MeshElem::new(v1, v2, v3);
  //println!("{:?}", v1);

  let a_spec = molecule::Species{ D: 6e-6, ID: 0, name: String::from("foobar")};
  let m1 = molecule::Mol{ spec: &a_spec, r: vector::Vec3{x: 0.0, y: 0.0, z: 0.0}, t: 0.0};
/*
  let mut rng = random::Mersenne::new(1);
  for i in 1..100000 {
    println!("{}", rng.norm());
  }
*/

  let mut rng = random::rng::new(1);
  for i in 0..1000000 {
    println!("{}", rng.norm());
  }

}



