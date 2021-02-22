use super::*;

use utils::point::Point;

#[test]
fn point_ops() {
    assert_eq!(3.0 * Point::new(2.0, 3.0), Point::new(6.0, 9.0));
    assert_eq!(Point::new(2.0, 3.0) * 3.0, Point::new(6.0, 9.0));
    assert_eq!(Point::new(2.0, 3.0) * 0.5, Point::new(1.0, 1.5));
    assert_eq!(Point::new(2.0, 3.0) / 3.0, Point::new(2.0 / 3.0, 1.0));
    assert_eq!(Point::new(2.0, 3.0) + Point::new(3.0, 4.0), Point::new(5.0, 7.0));
    assert_eq!(Point::new(2.0, 3.0) - Point::new(3.0, 1.0), Point::new(-1.0, 2.0));

    assert_eq!(Point::from_polar(0.0, 1.0), Point::new(1.0, 0.0));
    assert_eq!(Point::from_polar(0.0, 2.0), Point::new(2.0, 0.0));
    assert_eq!(Point::from_polar(std::f64::consts::PI, 1.0).angle(), std::f64::consts::PI);

    assert_eq!({
        let mut p = Point::new(2.0, 0.0);
        p.norm();
        p
    }, Point::new(1.0, 0.0));

    assert_eq!(Point::new(3.0, 4.0).maq(), 5.0);
}

#[test]
fn utils_fn() {
    assert_eq!(utils::remap(5.0, 0.0, 10.0, 0.0, 5.0), 2.5);
    assert_eq!(utils::remap(-5.0, 0.0, 10.0, 0.0, 5.0), -2.5);
    assert_eq!(utils::remap(0.5, 0.0, 1.0, 0.0, 360.0), 180.0);

    assert_eq!(utils::lerp(0.25, -1.0, 3.0), 0.0);
    assert_eq!(utils::lerp(0.5, 0.0, 3.0), 1.5);

    assert_eq!(utils::bezier(0.0, 2.0, 4.0, 6.0), 2.0);
    assert_eq!(utils::bezier(0.5, 0.0, 4.0, 6.0), 4.0);
    assert_eq!(utils::bezier(1.0, 2.0, 4.0, 6.0), 6.0);
}