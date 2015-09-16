
// Third party packages
extern crate lodestone_core;
extern crate lodestone_polygon;

use lodestone_core::{wgs84};
use lodestone_polygon::FeaturePolygon;

pub type Position = Vec<f64>;

pub fn polygon_area(polygon: &FeaturePolygon) -> f64 {
  
  let rings = polygon.coordinates();
  let mut iter = rings.into_iter();
  let mut area = ring_area(iter.next().unwrap());
  
  loop {
    match iter.next() {
      Some(ring) => {
        area -= ring_area(ring);
      },
      None => break
    }
  };

  area
}

///
/// Calculate the approximate area of the polygon were it projected onto
///     the earth.  Note that this area will be positive if ring is oriented
///     clockwise, otherwise it will be negative.
///
/// Reference:
/// Robert. G. Chamberlain and William H. Duquette, "Some Algorithms for
///     Polygons on a Sphere", JPL Publication 07-03, Jet Propulsion
///     Laboratory, Pasadena, CA, June 2007 http://trs-new.jpl.nasa.gov/dspace/handle/2014/40409
pub fn ring_area(coords: &Vec<Position>) -> f64 {
  let mut area = 0.0;
  let mut iter = coords.into_iter().peekable();

  loop {
    let p1 = iter.next().unwrap();
    
    match iter.peek() {
      Some(p2) => {
        area += (p2[0] - p1[0]).to_radians() * 
                (2.0 + p1[1].to_radians().sin() + p2[1].to_radians().sin());
      },
      None => break
    }
  };

  area = area * wgs84::RADIUS * wgs84::RADIUS / 2.0;
  area
}

#[cfg(test)]
mod tests {
  use lodestone_polygon::FeaturePolygon;
  use super::{polygon_area, ring_area};

  #[test]
  fn test_ring_area() {
    let coords1 = vec![vec![1.0,1.0], vec![1.0,1.1], vec![1.1,1.1], vec![1.0,1.0]];
    let coords2 = vec![vec![1.0,1.0], vec![1.0,1.01], vec![1.01,1.01], vec![1.0,1.0]];

    assert_eq!(ring_area(&coords1), 61949733.21122331);
    assert_eq!(ring_area(&coords2), 619506.1364177351);
  }

  #[test]
  fn test_polygon_area() {
    let coords = vec![vec![vec![1.0,1.0], vec![1.0,1.1], vec![1.1,1.1], vec![1.0,1.0]]];
    let polygon = FeaturePolygon::new(coords);

    assert_eq!(polygon_area(&polygon), 61949733.21122331);
  }

  #[test]
  fn test_polygon_area_with_hole() {
    let coords = vec![
      vec![vec![1.0,1.0], vec![1.0,1.1], vec![1.1,1.1], vec![1.0,1.0]],
      vec![vec![1.0,1.0], vec![1.0,1.01], vec![1.01,1.01], vec![1.0,1.0]]
    ];
    let polygon = FeaturePolygon::new(coords);

    assert_eq!(polygon_area(&polygon), 61949733.21122331 - 619506.1364177351);
  }
}