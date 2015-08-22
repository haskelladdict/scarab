//mod random;
mod geometry;
mod mesh;
mod util;
mod vector;

fn main() {

  let v1 = vector::Vec3{ x: 1.0, y: 1.0, z: 1.0};
  let v2 = vector::Vec3{ x: 2.0, y: 1.5, z: 1.5};
  let v3 = vector::Vec3{ x: 0.5, y: 3.3, z: 3.1};
  let m = mesh::MeshElem::new(v1, v2, v3);
  println!("{:?}", v1);
}



