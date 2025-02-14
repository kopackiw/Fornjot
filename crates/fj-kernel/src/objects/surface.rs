use fj_math::{Line, Point, Vector};

use crate::path::GlobalPath;

/// A two-dimensional shape
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Surface {
    u: GlobalPath,
    v: Vector<3>,
}

impl Surface {
    /// Construct a `Surface` from two paths that define its coordinate system
    pub fn new(u: GlobalPath, v: impl Into<Vector<3>>) -> Self {
        let v = v.into();
        Self { u, v }
    }

    /// Construct a plane from 3 points
    pub fn plane_from_points(points: [impl Into<Point<3>>; 3]) -> Self {
        let [a, b, c] = points.map(Into::into);

        let u = GlobalPath::Line(Line::from_points([a, b]));
        let v = c - a;

        Self { u, v }
    }

    /// Access the path that defines the u-coordinate of this surface
    pub fn u(&self) -> GlobalPath {
        self.u
    }

    /// Access the path that defines the v-coordinate of this surface
    pub fn v(&self) -> Vector<3> {
        self.v
    }

    /// Convert a point in surface coordinates to model coordinates
    pub fn point_from_surface_coords(
        &self,
        point: impl Into<Point<2>>,
    ) -> Point<3> {
        let point = point.into();
        self.u.point_from_path_coords([point.u])
            + self.path_to_line().vector_from_line_coords([point.v])
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn vector_from_surface_coords(
        &self,
        vector: impl Into<Vector<2>>,
    ) -> Vector<3> {
        let vector = vector.into();
        self.u.vector_from_path_coords([vector.u])
            + self.path_to_line().vector_from_line_coords([vector.v])
    }

    fn path_to_line(&self) -> Line<3> {
        Line::from_origin_and_direction(self.u.origin(), self.v)
    }
}

#[cfg(test)]
mod tests {
    use fj_math::{Line, Point, Vector};
    use pretty_assertions::assert_eq;

    use crate::path::GlobalPath;

    use super::Surface;

    #[test]
    fn point_from_surface_coords() {
        let swept = Surface {
            u: GlobalPath::Line(Line::from_origin_and_direction(
                Point::from([1., 1., 1.]),
                Vector::from([0., 2., 0.]),
            )),
            v: Vector::from([0., 0., 2.]),
        };

        assert_eq!(
            swept.point_from_surface_coords([2., 4.]),
            Point::from([1., 5., 9.]),
        );
    }

    #[test]
    fn vector_from_surface_coords() {
        let swept = Surface {
            u: GlobalPath::Line(Line::from_origin_and_direction(
                Point::from([1., 0., 0.]),
                Vector::from([0., 2., 0.]),
            )),
            v: Vector::from([0., 0., 2.]),
        };

        assert_eq!(
            swept.vector_from_surface_coords([2., 4.]),
            Vector::from([0., 4., 8.]),
        );
    }
}
