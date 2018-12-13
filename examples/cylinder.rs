extern crate homotopy;

use homotopy::*;

fn main() {
    let a = Circle {center: [0.0, 0.0], radius: 1.0};
    let b = Lerp(0.0, 10.0);
    let c = Square::new(a, b);

    assert!(check2(&c, Default::default()));

    let d = c.map(|(xy, z)| [xy[0], xy[1], z]);
    assert!(check2(&d, Default::default()));

    let left = d.left();
    assert_eq!(left.hu(0.0), [1.0, 0.0, 0.0]);
    assert_eq!(left.hu(0.5), [1.0, 0.0, 5.0]);
    assert_eq!(left.hu(1.0), [1.0, 0.0, 10.0]);

    let right = d.right();
    assert_eq!(right.hu(0.0), [1.0, 0.0, 0.0]);
    assert_eq!(right.hu(0.5), [1.0, 0.0, 5.0]);
    assert_eq!(right.hu(1.0), [1.0, 0.0, 10.0]);

    let top = d.top();
    assert_eq!(top.hu(0.0), [1.0, 0.0, 0.0]);
    assert_eq!(top.hu(0.5), [-1.0, 0.0, 0.0]);
    assert_eq!(top.hu(1.0), [1.0, 0.0, 0.0]);

    let bottom = d.bottom();
    assert_eq!(bottom.hu(0.0), [1.0, 0.0, 10.0]);
    assert_eq!(bottom.hu(0.5), [-1.0, 0.0, 10.0]);
    assert_eq!(bottom.hu(1.0), [1.0, 0.0, 10.0]);

    let middle = d.top_bottom(0.5);
    assert_eq!(middle.hu(0.0), [1.0, 0.0, 5.0]);
    assert_eq!(middle.hu(0.5), [-1.0, 0.0, 5.0]);
    assert_eq!(middle.hu(1.0), [1.0, 0.0, 5.0]);
}
