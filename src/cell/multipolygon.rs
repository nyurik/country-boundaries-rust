use crate::cell::point::Point;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Multipolygon {
    pub outer: Vec<Vec<Point>>,
    pub inner: Vec<Vec<Point>>
}

impl Multipolygon {
    pub fn covers(&self, point: &Point) -> bool {
        let mut insides = 0;
        for area in self.outer.iter() {
            if is_point_in_polygon(point, area.as_slice()) {
                insides += 1;
            }
        }
        for area in self.inner.iter() {
            if is_point_in_polygon(point, area.as_slice()) {
                insides -= 1;
            }
        }
        insides > 0
    }
}

// modified from:
// Copyright 2000 softSurfer, 2012 Dan Sunday
// This code may be freely used and modified for any purpose
// providing that this copyright notice is included with it.
// SoftSurfer makes no warranty for this code, and cannot be held
// liable for any real or imagined damage resulting from its use.
// Users of this code must verify correctness for their application.
// http://geomalgorithms.com/a03-_inclusion.html

fn is_point_in_polygon(p: &Point, v: &[Point]) -> bool {
    let mut wn = 0;
    let mut i = v.len() - 1;
    for j in 0 .. v.len() {
        if v[i].y <= p.y {
            if v[j].y > p.y {
                if is_left(&v[i], &v[j], p) > 0 {
                    wn += 1;
                }
            }
        } else {
            if v[j].y <= p.y {
                if is_left(&v[i], &v[j], p) < 0 {
                    wn -= 1;
                }
            }
        }
        i = j;
    }
    wn != 0
}

fn is_left(p0: &Point, p1: &Point, p: &Point) -> i64 {
    // must cast to 64 because otherwise there could be an integer overflow
    (p1.x as i64 - p0.x as i64) * (p.y as i64 - p0.y as i64)
    - (p.x as i64 - p0.x as i64) * (p1.y as i64 - p0.y as i64)
}

#[cfg(test)]
mod tests {
    use super::*;


    fn big_square() -> Vec<Point> { vec![p(0, 0), p(0, 10), p(10, 10), p(10, 0)] }
    fn hole() -> Vec<Point> { vec![p(2, 2), p(2, 8), p(8, 8), p(8, 2)] }
    fn small_square() -> Vec<Point> { vec![p(4, 4), p(4, 6), p(6, 6), p(6, 4)] }

    #[test]
    fn simple_point_in_polygon() {
        assert!(is_point_in_polygon(&p(5, 5), big_square().as_slice()));
    }

    #[test]
    fn covers_simple_polygon() {
        assert!(Multipolygon { outer: vec![big_square()], inner: vec![] }
            .covers(&p(5, 5))
        );
    }

    #[test]
    fn does_not_cover_hole() {
        assert!(!Multipolygon { outer: vec![big_square()], inner: vec![hole()] }
            .covers(&p(5, 5))
        );
    }

    #[test]
    fn does_cover_polygon_in_hole() {
        assert!(Multipolygon { outer: vec![big_square(), small_square()], inner: vec![hole()] }
            .covers(&p(5, 5))
        );
    }

    #[test]
    fn only_upper_left_edge_counts_as_inside() {
        let polygon = Multipolygon { outer: vec![big_square()], inner: vec![] };

        assert!(polygon.covers(&p(0, 0)));
        assert!(polygon.covers(&p(5, 0)));
        assert!(polygon.covers(&p(0, 5)));
        assert!(!polygon.covers(&p(0, 10)));
        assert!(!polygon.covers(&p(10, 0)));
        assert!(!polygon.covers(&p(5, 10)));
        assert!(!polygon.covers(&p(10, 5)));
        assert!(!polygon.covers(&p(10, 10)));
    }

    fn p(x: u16, y: u16) -> Point {
        Point { x, y }
    }
}
