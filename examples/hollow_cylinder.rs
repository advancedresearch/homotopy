extern crate homotopy;

use homotopy::*;

fn main() {
    let inner_circle = Circle {center: [0.0, 0.0], radius: 1.0};
    let outer_circle = Circle {center: [0.0, 0.0], radius: 2.0};
    let inner_and_outer = Square::new(inner_circle, outer_circle);
    let sweep = inner_and_outer.diagonal();
    let hollow_cylinder = Square::new(sweep, Lerp(0.0, 10.0));
    let map = hollow_cylinder.smap(|((a, b), c), s| [
        a[0] + (b[0] - a[0]) * s,
        a[1] + (b[1] - a[1]) * s,
        c
    ]);

    assert_eq!(map.hu([0.0, 0.0, 0.0]), [1.0, 0.0, 0.0]);
    assert_eq!(map.hu([0.0, 0.0, 1.0]), [2.0, 0.0, 0.0]);
    assert_eq!(map.hu([0.25, 0.5, 0.5]), [0.0, 1.5, 5.0]);
}
