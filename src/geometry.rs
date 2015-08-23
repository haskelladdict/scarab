// Copyright 2015 Markus Dittrich
// Licensed under BSD license, see LICENSE file for details

use mesh;
use vector;
use util;

const GEOM_EPSILON : f64 = 1e-12;

/// intersect tests for ray MeshElem intersections. This function returns the
/// hit point and intersection status which can be one of:
///  0: triangle and ray segment intersect, in this case hitPoint contains the
///     location of the intersection point
///  1: triangle and ray intersect but ray segment does not reach the triangle
///  2: triangle and ray do not intersect
///  3: ray and triangle are co-planar
///  4: triangle is degenerate
///
/// NOTE: This function was adapted from Dan Sunday
/// <http://geomalgorithms.com/a06-_intersect-2.html#intersect3D_RayTriangle()>
pub fn intersect(start: &vector::Vec3, disp: &vector::Vec3, m: &mesh::MeshElem)
  -> (i8, vector::Vec3) {

  // if the normal vector is zero, triangle is degenerate
  if util::same(m.n.x, 0.0) && util::same(m.n.y, 0.0) && util::same(m.n.z, 0.0) {
    return (4, vector::Vec3::new(0.0, 0.0, 0.0));
  }

  // compute intersection of ray from start along disp with plane in which
  // m is located
  let w0 = start - &m.a;
  let a = -m.n.dot(&w0);
  let b = m.n.dot(&disp);
  if b.abs() < GEOM_EPSILON {  // our ray is parallel to triangle plane
    if util::same(a, 0.0) {    // our ray is coplanar with the triangle
      return (3, vector::Vec3::new(0.0, 0.0, 0.0));
    } else {
      return (2, vector::Vec3::new(0.0, 0.0, 0.0));
    }
  }

  let r = a/b;
  // if ray points away from triangle plane we won't hit it
  if r < 0.0 {
    return (2, vector::Vec3::new(0.0, 0.0, 0.0));
  // if the ray segment doesn't reach the plane we won't hit it
  } else if r > 1.0 {
    return (1, vector::Vec3::new(0.0, 0.0, 0.0));
  }

  // now test that hitPoint is within the triangle
  // we use local variable for efficiency
  let hitPoint = start + &disp.scalar(r);
  let w = &hitPoint - &m.a;
  let uu = m.u.dot(&m.u);
  let uv = m.u.dot(&m.v);
  let vv = m.v.dot(&m.v);
  let wu = w.dot(&m.u);
  let wv = w.dot(&m.v);
  let D = uv * uv - uu * vv;

  // compute and test parametric coords
  let s = (uv * wv - vv * wu) / D;
  // hitPoint is outside m
  if s < 0.0 || s > 1.0 {
    return (2, hitPoint);
  }

  let t = (uv * wu - uu * wv) / D;
  // hitPoint is outside m
  if t < 0.0 || (s + t) > 1.0 {
    return (2, hitPoint);
  }

  // hitPoint is in m
  (0, hitPoint)
 }
