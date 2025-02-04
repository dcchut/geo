use super::Contains;
use crate::geometry::{Coordinate, Geometry, GeometryCollection, Point};
use crate::geometry_delegate_impl;
use crate::GeoNum;

// ┌──────────────────────────────┐
// │ Implementations for Geometry │
// └──────────────────────────────┘

impl<T> Contains<Coordinate<T>> for Geometry<T>
where
    T: GeoNum,
{
    fn contains(&self, coord: &Coordinate<T>) -> bool {
        self.contains(&Point::from(*coord))
    }
}

impl<T> Contains<Point<T>> for Geometry<T>
where
    T: GeoNum,
{
    geometry_delegate_impl! {
        fn contains(&self, point: &Point<T>) -> bool;
    }
}

// ┌────────────────────────────────────────┐
// │ Implementations for GeometryCollection │
// └────────────────────────────────────────┘

impl<T> Contains<Coordinate<T>> for GeometryCollection<T>
where
    T: GeoNum,
{
    fn contains(&self, coord: &Coordinate<T>) -> bool {
        self.iter().any(|geometry| geometry.contains(coord))
    }
}

impl<T> Contains<Point<T>> for GeometryCollection<T>
where
    T: GeoNum,
{
    fn contains(&self, point: &Point<T>) -> bool {
        self.contains(&point.0)
    }
}
