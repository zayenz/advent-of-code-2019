use nalgebra;
use num::clamp;
use std::cmp::Ordering;
use std::iter::FromIterator;
use std::ops::Index;

pub type Point2 = nalgebra::Point2<f64>;
pub type Vector2 = nalgebra::Vector2<f64>;

/// Given two non-NaN floats, compute the minimum and maximum
pub fn minmax(a: f64, b: f64) -> (f64, f64) {
    let a_notnan = decorum::N64::from(a);
    let b_notnan = decorum::N64::from(b);
    match a_notnan.cmp(&b_notnan) {
        Ordering::Less => (a, b),
        Ordering::Equal => (a, b),
        Ordering::Greater => (b, a),
    }
}

/// Simple bounds structure
pub struct Bounds {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
}

impl Bounds {
    /// The bounds of two points
    pub fn from_points(a: Point2, b: Point2) -> Bounds {
        let (min_x, max_x) = minmax(a.x, b.x);
        let (min_y, max_y) = minmax(a.y, b.y);
        Bounds::new(min_x, max_x, min_y, max_y)
    }

    /// Standard constructor
    pub fn new(left: f64, right: f64, top: f64, bottom: f64) -> Bounds {
        assert!(left <= right);
        assert!(top <= bottom);
        Bounds {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn clamp_x(&self, x: f64) -> f64 {
        clamp(x, self.left, self.right)
    }

    pub fn clamp_y(&self, y: f64) -> f64 {
        clamp(y, self.top, self.bottom)
    }

    pub fn width(&self) -> f64 {
        self.right - self.left
    }

    pub fn height(&self) -> f64 {
        self.bottom - self.top
    }

    /// Check if two bounds overlap each other.
    /// Check is done inclusive
    pub fn overlaps(&self, other: &Bounds) -> bool {
        // Logic is simpler checking for no-overlap and then inverting
        // Similar logic to the Side::try_from method
        !(
            // self is to the left of other
            self.right < other.left ||
                // other is to the left of self
                other.right < self.left ||
                // self is above other
                self.bottom < other.top ||
                // other is above self
                other.bottom < self.top
        )
    }

    /// Check if `point` is in the bounds
    /// Check is done inclusive.
    pub fn contains_point(&self, point: Point2) -> bool {
        self.left <= point.x
            && point.x <= self.right
            && self.top <= point.y
            && point.y <= self.bottom
    }
}

/// Which side enum.
pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

impl Side {
    /// If the position is at or off one side, return that side
    ///
    /// If the position is of multiple sides, choose one.
    pub fn try_from(bounds: &Bounds, position: Point2) -> Option<Side> {
        if position.x < bounds.left {
            Some(Side::Left)
        } else if position.x > bounds.right {
            Some(Side::Right)
        } else if position.y < bounds.top {
            Some(Side::Top)
        } else if position.y > bounds.bottom {
            Some(Side::Bottom)
        } else {
            None
        }
    }
}

/// Intersection results when the result may be 1, 2, or 3
#[derive(Debug, Copy, Clone)]
pub enum Intersections {
    None,
    One(Point2),
    Two(Point2, Point2),
}

impl Intersections {
    pub fn len(&self) -> usize {
        match self {
            Intersections::None => 0,
            Intersections::One(_) => 1,
            Intersections::Two(_, _) => 2,
        }
    }

    pub fn is_empty(&self) -> bool {
        *self == Intersections::None
    }
}

impl PartialEq for Intersections {
    fn eq(&self, other: &Self) -> bool {
        use decorum::N64;
        use Intersections::*;
        match (*self, *other) {
            (None, None) => true,
            (One(s1), One(o1)) => {
                N64::from(s1.x) == N64::from(o1.x) && N64::from(s1.y) == N64::from(o1.y)
            }
            (Two(s1, s2), Two(o1, o2)) => {
                N64::from(s1.x) == N64::from(o1.x)
                    && N64::from(s1.y) == N64::from(o1.y)
                    && N64::from(s2.x) == N64::from(o2.x)
                    && N64::from(s2.y) == N64::from(o2.y)
            }
            _ => false,
        }
    }
}

impl FromIterator<Point2> for Intersections {
    fn from_iter<T: IntoIterator<Item = Point2>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        if let Some(first) = iter.next() {
            if let Some(second) = iter.next() {
                Intersections::Two(first, second)
            } else {
                Intersections::One(first)
            }
        } else {
            Intersections::None
        }
    }
}

impl IntoIterator for Intersections {
    type Item = Point2;
    type IntoIter = IntersectionsIterator;

    fn into_iter(self) -> Self::IntoIter {
        IntersectionsIterator {
            intersections: self,
            produced1: false,
            produced2: false,
        }
    }
}

impl Index<usize> for Intersections {
    type Output = Point2;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Intersections::None => panic!("Out of bounds access!"),
            Intersections::One(point) => {
                assert_eq!(index, 0, "Out of bounds access!");
                point
            }
            Intersections::Two(point1, point2) => match index {
                0 => point1,
                1 => point2,
                _ => panic!("Out of bounds access!"),
            },
        }
    }
}

pub struct IntersectionsIterator {
    intersections: Intersections,
    produced1: bool,
    produced2: bool,
}

impl Iterator for IntersectionsIterator {
    type Item = Point2;

    fn next(&mut self) -> Option<Self::Item> {
        match self.intersections {
            Intersections::None => None,
            Intersections::One(point) => {
                if self.produced1 {
                    None
                } else {
                    self.produced1 = true;
                    Some(point)
                }
            }
            Intersections::Two(point1, point2) => match (self.produced1, self.produced2) {
                (false, _) => {
                    self.produced1 = true;
                    Some(point1)
                }
                (true, false) => {
                    self.produced2 = true;
                    Some(point2)
                }
                (true, true) => None,
            },
        }
    }
}

/// Line segment between two points
#[derive(Debug, Copy, Clone)]
pub struct LineSegment {
    pub start: Point2,
    pub end: Point2,
    pub vector: Vector2,
}

// The epsilon is chosen by manual testing to not have rays intersect with the ship.
// At 1e-8 they intersect, not at 1e-9, and at 1e-10 we have sufficient margin or error
// to be confident rays do not intersect the ship, while still keeping some fuzziness
// in computations.
pub const EPSILON: f64 = 1e-10;

impl LineSegment {
    pub fn new(start: Point2, end: Point2) -> LineSegment {
        LineSegment {
            start,
            end,
            vector: end - start,
        }
    }

    /// The bounds of this segment
    pub fn bounds(&self) -> Bounds {
        Bounds::from_points(self.start, self.end)
    }

    pub fn angle(&self) -> f64 {
        let heading = nalgebra::Rotation2::rotation_between(&Vector2::x(), &self.vector); //.expect("Angle must exist for simple vectors in 2D");
        heading.angle()
    }

    /// Find the point on this segment that is closest to `point`
    pub fn closest_point_on_segment(&self, point: Point2) -> Point2 {
        // Source for math: http://paulbourke.net/geometry/pointlineplane/

        //If the line segment is degenerate, just use an endpoint
        if self.start == self.end {
            return self.start;
        }
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;

        let t = ((point.x - self.start.x) * dx + (point.y - self.start.y) * dy)
            / self.vector.norm_squared();

        if t <= 0.0 {
            self.start
        } else if t >= 1.0 {
            self.end
        } else {
            let x = self.start.x + t * dx;
            let y = self.start.y + t * dy;
            Point2::new(x, y)
        }
    }

    /// Check if the point is on the segment
    /// Check is done inclusive.
    pub fn point_on_segment(&self, point: Point2) -> bool {
        // We use EPSILON as is here, instead of squaring it, since we want the test for
        // on-segment to allow more values to be on the segment, than the computation in
        // circle_intersections does
        let epsilon = EPSILON as f64;

        // Use cheap bounding box check first, filters out some cases and is very cheap to do
        if self.bounds().contains_point(point) {
            self.distance_to_segment_squared(point) < epsilon
        } else {
            false
        }
    }

    /// Find the intersection(s), if any, between this segment and a circle
    pub fn circle_intersections(&self, center: Point2, radius: f64) -> Intersections {
        // f64 is used for computations to keep precision in intermediate results
        // Clippy warns on the casts if the base types change, which will not be the case here
        #![allow(clippy::cast_lossless)]

        // Uses description from
        // http://csharphelper.com/blog/2014/09/determine-where-a-line-intersects-a-circle-in-c/

        let epsilon = EPSILON;

        // Values sx, sy, ex, ey are adjusted from center not being (0,0)
        let cx = center.x;
        let cy = center.y;
        let sx = (self.start.x - cx) as f64;
        let sy = (self.start.y - cy) as f64;
        let ex = (self.end.x - cx) as f64;
        let ey = (self.end.y - cy) as f64;

        let radius_sq = (radius * radius) as f64;
        let d_x = ex - sx;
        let d_y = ey - sy;
        let a = (d_x * d_x) + (d_y * d_y);
        let b = 2.0 * (d_x * sx + d_y * sy);
        let c = sx * sx + sy * sy - radius_sq;
        let det = b * b - 4.0 * a * c;

        let line_intersections = if a < epsilon || det < -epsilon {
            // No solutions
            Intersections::None
        } else if det.abs() < epsilon {
            let t = -b / (2.0 * a);
            Intersections::One(Point2::new(
                (self.start.x as f64 + t * d_x) as f64,
                (self.start.y as f64 + t * d_y) as f64,
            ))
        } else {
            let det_sqrt = det.sqrt();
            let t1 = (-b + det_sqrt) / (2.0 * a);
            let t2 = (-b - det_sqrt) / (2.0 * a);
            Intersections::Two(
                Point2::new(
                    (self.start.x as f64 + t1 * d_x) as f64,
                    (self.start.y as f64 + t1 * d_y) as f64,
                ),
                Point2::new(
                    (self.start.x as f64 + t2 * d_x) as f64,
                    (self.start.y as f64 + t2 * d_y) as f64,
                ),
            )
        };

        line_intersections
            .into_iter()
            .filter(|point| self.point_on_segment(*point))
            .collect()
    }

    /// Distance between `point` and this segment squared
    pub fn distance_to_segment_squared(&self, point: Point2) -> f64 {
        (self.closest_point_on_segment(point) - point).norm_squared()
    }
}

#[cfg(test)]
mod test {
    use crate::geometry::{Bounds, Intersections, LineSegment, Point2, Vector2};

    #[test]
    fn test_bounds_overlap() {
        // Just test a few common cases in one direction
        let b1212 = Bounds::new(1.0, 2.0, 1.0, 2.0);
        let b1312 = Bounds::new(1.0, 3.0, 1.0, 2.0);
        let b2312 = Bounds::new(2.0, 3.0, 1.0, 2.0);
        let b3412 = Bounds::new(3.0, 4.0, 1.0, 2.0);
        let b2412 = Bounds::new(2.0, 4.0, 1.0, 2.0);

        assert!(b1212.overlaps(&b2312));
        assert!(!b1212.overlaps(&b3412));
        assert!(b1312.overlaps(&b2412));
    }

    #[test]
    fn test_closest_point_on_segment() {
        let origo = Point2::new(0.0, 0.0);
        let p1010 = Point2::new(10.0, 10.0);

        let pneg = Point2::new(-2.0, -1.0);
        let pbig = Point2::new(20.456456, 1000.0);
        let pmid = Point2::new(5.0, 5.0);
        let p010 = Point2::new(0.0, 10.0);

        let segment = LineSegment::new(origo, p1010);

        // Closest point to points on segment is the point itself
        let close_p1010 = segment.closest_point_on_segment(p1010);
        assert_eq!(close_p1010, p1010);
        let close_origo = segment.closest_point_on_segment(origo);
        assert_eq!(close_origo, origo);
        let close_pmid = segment.closest_point_on_segment(pmid);
        assert_eq!(close_pmid, pmid);

        // Closest point to a point off the end is the end-point
        let close_pneg = segment.closest_point_on_segment(pneg);
        assert_eq!(close_pneg, origo);
        let close_pbig = segment.closest_point_on_segment(pbig);
        assert_eq!(close_pbig, p1010);

        // Standard closest point variant
        let close_p010 = segment.closest_point_on_segment(p010);
        assert_eq!(close_p010, pmid);
    }

    #[test]
    fn test_segment_circle_intersection() {
        // Some very basic tests
        let origo = Point2::new(0.0, 0.0);
        let p1010 = Point2::new(10.0, 10.0);
        let segment = LineSegment::new(origo, p1010);

        let p010 = Point2::new(0.0, 10.0);
        let pmid = Point2::new(5.0, 5.0);
        let distance = (pmid - p010).norm();

        assert_eq!(segment.circle_intersections(p010, distance - 1.0).len(), 0);
        assert_eq!(segment.circle_intersections(p010, distance)[0], pmid);
        let intersections = segment.circle_intersections(p010, distance + 1.0);
        assert_eq!(intersections.len(), 2);
        assert_eq!(
            segment.closest_point_on_segment(intersections[0]),
            intersections[0]
        );
        assert_eq!(
            segment.closest_point_on_segment(intersections[1]),
            intersections[1]
        );
    }

    #[test]
    fn test_segment_circle_intersection_failures() {
        // These tests are collected from failing cases
        let cases = [(
            LineSegment::new(
                Point2::new(-1.1943204, 0.32567957),
                Point2::new(-71.83069, -70.310684),
            ),
            Point2::new(-1.12, 0.000000014901161),
            0.08,
        )];

        for (segment, center, radius) in &cases {
            assert_eq!(
                segment.circle_intersections(*center, *radius),
                Intersections::None
            );
        }
    }
}
