//! A library for homotopy logic.

#![deny(missing_docs)]

use std::ops::{Add, Mul};
use std::marker::PhantomData;

pub use sides::*;

mod sides;

/// A continuous map between two functions.
pub trait Homotopy<X, Scalar=f64>: Sized {
    /// The output type.
    type Y;

    /// The function being mapped from.
    fn f(&self, X) -> Self::Y;
    /// The function being mapped to.
    fn g(&self, X) -> Self::Y;
    /// A continuous map such that `h(x, 0.0) == f(x)` and `h(x, 1.0) == g(x)`.
    fn h(&self, X, Scalar) -> Self::Y;

    /// Gets the inverse.
    fn inverse<'a>(&'a self) -> Inverse<&'a Self> {Inverse(self)}

    /// Gets the diagonal.
    fn diagonal<'a>(&'a self) -> Diagonal<&'a Self, Scalar>
        where Diagonal<&'a Self, Scalar>: Homotopy<X>
    {
        Diagonal::new(self)
    }

    /// Gets the left side.
    fn left<'a, S>(&'a self) -> Left<&'a Self>
        where Left<&'a Self>: Homotopy<X, S>
    {
        Left(self)
    }

    /// Gets the right side.
    fn right<'a, S>(&'a self) -> Right<&'a Self>
        where Right<&'a Self>: Homotopy<X, S>
    {
        Right(self)
    }

    /// Gets the top side.
    fn top<'a, S>(&'a self) -> Top<&'a Self>
        where Top<&'a Self>: Homotopy<X, S>
    {
        Top(self)
    }

    /// Gets the bottom side.
    fn bottom<'a, S>(&'a self) -> Bottom<&'a Self>
        where Bottom<&'a Self>: Homotopy<X, S>
    {
        Bottom(self)
    }

    /// Gets the front side.
    fn front<'a, S>(&'a self) -> Front<&'a Self>
        where Front<&'a Self>: Homotopy<X, S>
    {
        Front(self)
    }

    /// Gets the back side.
    fn back<'a, S>(&'a self) -> Back<&'a Self>
        where Back<&'a Self>: Homotopy<X, S>
    {
        Back(self)
    }

    /// Gets a left-right intersection, controlled by `s`.
    fn left_right<'a, S>(&'a self, s: f64) -> LeftRight<&'a Self>
        where LeftRight<&'a Self>: Homotopy<X, S>
    {
        LeftRight(self, s)
    }

    /// Gets a top-bottom intersection, controlled by `s`.
    fn top_bottom<'a, S>(&'a self, s: f64) -> TopBottom<&'a Self>
        where TopBottom<&'a Self>: Homotopy<X, S>
    {
        TopBottom(self, s)
    }

    /// Gets a front-back intersection, controlled by `s`.
    fn front_back<'a, S>(&'a self, s: f64) -> FrontBack<&'a Self>
        where FrontBack<&'a Self>: Homotopy<X, S>
    {
        FrontBack(self, s)
    }

    /// Gets a converter to and from vectors.
    fn as_vec<'a, S, VX>(&'a self) -> AsVec<&'a Self>
        where AsVec<&'a Self>: Homotopy<VX, S>
    {
        AsVec(self)
    }
}

impl<'a, X, T, S> Homotopy<X, S> for &'a T
    where T: Homotopy<X, S>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {T::f(self, x)}
    fn g(&self, x: X) -> Self::Y {T::g(self, x)}
    fn h(&self, x: X, s: S) -> Self::Y {T::h(self, x, s)}
}

/// Checks that the homotopy constraints hold for some input `x`.
#[must_use]
pub fn check<H, X>(h: &H, x: X) -> bool
    where H: Homotopy<X>,
          H::Y: PartialEq,
          X: Clone
{
    h.h(x.clone(), 0.0) == h.f(x.clone()) &&
    h.h(x.clone(), 1.0) == h.g(x)
}

/// Checks that the 2D homotopy constraints hold for some input `x`.
#[must_use]
pub fn check2<H, X>(h: &H, x: X) -> bool
    where H: Homotopy<X, [f64; 2]>,
          H::Y: PartialEq,
          X: Clone,
{
    let a = h.f(x.clone());
    let b = h.g(x.clone());
    h.h(x.clone(), [0.0, 0.0]) == a &&
    h.h(x.clone(), [1.0, 1.0]) == b &&
    check(&h.left(), x.clone()) &&
    check(&h.right(), x.clone()) &&
    check(&h.top(), x.clone()) &&
    check(&h.bottom(), x.clone())
}

/// Checks that the 3D homotopy constraints hold for some input `x`.
#[must_use]
pub fn check3<H, X>(h: &H, x: X) -> bool
    where H: Homotopy<X, [f64; 3]>,
          H::Y: PartialEq,
          X: Clone,
{
    let a = h.f(x.clone());
    let b = h.g(x.clone());
    h.h(x.clone(), [0.0, 0.0, 0.0]) == a &&
    h.h(x.clone(), [1.0, 1.0, 1.0]) == b &&
    check2(&h.left(), x.clone()) &&
    check2(&h.right(), x.clone()) &&
    check2(&h.top(), x.clone()) &&
    check2(&h.bottom(), x.clone()) &&
    check2(&h.front(), x.clone()) &&
    check2(&h.back(), x.clone())
}

/// Identity homotopy.
///
/// `f`, `g` and `h` uses the identity function, so this is a homotopy.
#[derive(Copy, Clone)]
pub struct Id;

impl<X> Homotopy<X> for Id {
    type Y = X;

    fn f(&self, x: X) -> X {x}
    fn g(&self, x: X) -> X {x}
    fn h(&self, x: X, _: f64) -> X {x}
}

/// The Dirac function.
#[derive(Copy, Clone)]
pub struct Dirac;

impl Homotopy<()> for Dirac {
    type Y = f64;

    fn f(&self, _: ()) -> f64 {1.0}
    fn g(&self, _: ()) -> f64 {0.0}
    fn h(&self, _: (), s: f64) -> f64 {if s == 0.0 {1.0} else {0.0}}
}

/// Dirac From homotopy.
///
/// Define `h` to be `f` at 0.0 and `g` elsewhere.
/// Since `h` is `g` at 1.0, this is a homotopy.
#[derive(Copy, Clone)]
pub struct DiracFrom<X, Y, F, G>
    where F: Fn(X) -> Y, G: Fn(X) -> Y
{
    fx: F,
    gx: G,
    _x: PhantomData<X>,
    _y: PhantomData<Y>,
}

impl<X, Y, F, G> DiracFrom<X, Y, F, G>
    where F: Fn(X) -> Y, G: Fn(X) -> Y
{
    /// Creates a new `DiracFrom`.
    pub fn new(f: F, g: G) -> DiracFrom<X, Y, F, G> {
        DiracFrom {
            fx: f,
            gx: g,
            _x: PhantomData,
            _y: PhantomData,
        }
    }
}

impl<X, Y, F, G> Homotopy<X> for DiracFrom<X, Y, F, G>
    where Y: Clone,
          F: Fn(X) -> Y,
          G: Fn(X) -> Y
{
    type Y = Y;

    fn f(&self, x: X) -> Y {(self.fx)(x)}
    fn g(&self, x: X) -> Y {(self.gx)(x)}
    fn h(&self, x: X, s: f64) -> Y {
        if s == 0.0 {(self.fx)(x)}
        else {(self.gx)(x)}
    }
}

/// Linear interpolation homotopy.
///
/// `f` and `g` are functions mapping `()` to a value.
/// The scalar passed to `h` controls the linear map.
#[derive(Copy, Clone)]
pub struct Lerp<X>(pub X, pub X);

impl<Y> Homotopy<()> for Lerp<Y>
    where Y: Mul<f64, Output = Y> + Add<Output = Y> + Clone
{
    type Y = Y;

    fn f(&self, _: ()) -> Y {self.0.clone()}
    fn g(&self, _: ()) -> Y {self.1.clone()}
    fn h(&self, _: (), s: f64) -> Y {self.0.clone() * (1.0 - s) + self.1.clone() * s}
}

/// Quadratic Bezier homotopy.
///
/// Maps from point A to C using a point B as control point.
#[derive(Copy, Clone)]
pub struct QuadraticBezier<X>(pub X, pub X, pub X);

impl<X> QuadraticBezier<X> {
    /// Creates a quadratic bezier that is identical to linear interpolation.
    pub fn from_linear(a: X, b: X) -> QuadraticBezier<X>
        where X: Mul<f64, Output = X> + Add<Output = X> + Clone
    {
        QuadraticBezier(a.clone(), a * 0.5 + b.clone() * 0.5, b)
    }
}

impl<X> From<Lerp<X>> for QuadraticBezier<X>
    where X: Mul<f64, Output = X> + Add<Output = X> + Clone
{
    fn from(lerp: Lerp<X>) -> QuadraticBezier<X> {
        QuadraticBezier::from_linear(lerp.0, lerp.1)
    }
}

impl<Y> Homotopy<()> for QuadraticBezier<Y>
    where Y: Mul<f64, Output = Y> + Add<Output = Y> + Clone
{
    type Y = Y;

    fn f(&self, _: ()) -> Y {self.0.clone()}
    fn g(&self, _: ()) -> Y {self.2.clone()}
    fn h(&self, _: (), s: f64) -> Y {
        let a = Lerp(self.0.clone(), self.1.clone()).h((), s);
        let b = Lerp(self.1.clone(), self.2.clone()).h((), s);
        Lerp(a, b).h((), s)
    }
}

/// Cubic Bezier homotopy.
///
/// Maps from point A to D using point B and C as control points.
#[derive(Copy, Clone)]
pub struct CubicBezier<X>(pub X, pub X, pub X, pub X);

impl<X> CubicBezier<X> {
    /// Creates a cubic bezier that is identical to quadratic bezier.
    pub fn from_quadratic(a: X, b: X, c: X) -> CubicBezier<X>
        where X: Clone
    {
        CubicBezier(a, b.clone(), b, c)
    }
}

impl<X> From<QuadraticBezier<X>> for CubicBezier<X>
    where X: Clone
{
    fn from(QuadraticBezier(a, b, c): QuadraticBezier<X>) -> CubicBezier<X> {
        CubicBezier::from_quadratic(a, b, c)
    }
}

impl<Y> Homotopy<()> for CubicBezier<Y>
    where Y: Mul<f64, Output = Y> + Add<Output = Y> + Clone
{
    type Y = Y;

    fn f(&self, _: ()) -> Y {self.0.clone()}
    fn g(&self, _: ()) -> Y {self.3.clone()}
    fn h(&self, _: (), s: f64) -> Y {
        let a = Lerp(self.0.clone(), self.1.clone()).h((), s);
        let b = Lerp(self.2.clone(), self.3.clone()).h((), s);
        Lerp(a, b).h((), s)
    }
}

/// Functional composition that is itself a homotopy.
#[derive(Copy, Clone)]
pub struct Compose<X, H1, H2>
    where H1: Homotopy<X>, H2: Homotopy<H1::Y>
{
    h1: H1,
    h2: H2,
    _x: PhantomData<X>,
}

impl<X, H1, H2> Compose<X, H1, H2>
    where H1: Homotopy<X>, H2: Homotopy<H1::Y>
{
    /// Creates a new composition of two homotopy maps.
    pub fn new(h1: H1, h2: H2) -> Self {
        Compose {
            h1, h2, _x: PhantomData
        }
    }
}

impl<X, H1, H2> Homotopy<X> for Compose<X, H1, H2>
    where H1: Homotopy<X>, H2: Homotopy<H1::Y>
{
    type Y = H2::Y;

    fn f(&self, x: X) -> Self::Y {self.h2.f(self.h1.f(x))}
    fn g(&self, x: X) -> Self::Y {self.h2.g(self.h1.g(x))}
    fn h(&self, x: X, s: f64) -> Self::Y {self.h2.h(self.h1.h(x, s), s)}
}

/// Takes the square of two homotopy maps and produces a 2D homotopy.
#[derive(Copy, Clone)]
pub struct Square<X1, X2, H1, H2>
    where H1: Homotopy<X1>, H2: Homotopy<X2>
{
    h1: H1,
    h2: H2,
    _x1: PhantomData<X1>,
    _x2: PhantomData<X2>,
}

impl<X1, X2, H1, H2> Square<X1, X2, H1, H2>
    where H1: Homotopy<X1>, H2: Homotopy<X2>
{
    /// Creates a square of two homotopy maps.
    pub fn new(h1: H1, h2: H2) -> Self {
        Square {h1, h2, _x1: PhantomData, _x2: PhantomData}
    }
}

impl<X1, X2, H1, H2> Homotopy<(X1, X2), [f64; 2]> for Square<X1, X2, H1, H2>
    where H1: Homotopy<X1>, H2: Homotopy<X2>
{
    type Y = (H1::Y, H2::Y);

    fn f(&self, x: (X1, X2)) -> Self::Y {(self.h1.f(x.0), self.h2.f(x.1))}
    fn g(&self, x: (X1, X2)) -> Self::Y {(self.h1.g(x.0), self.h2.g(x.1))}
    fn h(&self, x: (X1, X2), s: [f64; 2]) -> Self::Y {(self.h1.h(x.0, s[0]), self.h2.h(x.1, s[1]))}
}

/// Takes the cube of three homotopy maps and produces a 3D homotopy.
#[derive(Copy, Clone)]
pub struct Cube<X1, X2, X3, H1, H2, H3>
    where H1: Homotopy<X1>, H2: Homotopy<X2>, H3: Homotopy<X3>
{
    h1: H1,
    h2: H2,
    h3: H3,
    _x1: PhantomData<X1>,
    _x2: PhantomData<X2>,
    _x3: PhantomData<X3>,
}

impl<X1, X2, X3, H1, H2, H3> Cube<X1, X2, X3, H1, H2, H3>
    where H1: Homotopy<X1>, H2: Homotopy<X2>, H3: Homotopy<X3>
{
    /// Creates a new cube to three homotopy maps.
    pub fn new(h1: H1, h2: H2, h3: H3) -> Self {
        Cube {h1, h2, h3, _x1: PhantomData, _x2: PhantomData, _x3: PhantomData}
    }
}

impl<X1, X2, X3, H1, H2, H3> Homotopy<(X1, X2, X3), [f64; 3]> for Cube<X1, X2, X3, H1, H2, H3>
    where H1: Homotopy<X1>, H2: Homotopy<X2>, H3: Homotopy<X3>
{
    type Y = (H1::Y, H2::Y, H3::Y);

    fn f(&self, x: (X1, X2, X3)) -> Self::Y {(self.h1.f(x.0), self.h2.f(x.1), self.h3.f(x.2))}
    fn g(&self, x: (X1, X2, X3)) -> Self::Y {(self.h1.g(x.0), self.h2.g(x.1), self.h3.g(x.2))}
    fn h(&self, x: (X1, X2, X3), s: [f64; 3]) -> Self::Y {
        (self.h1.h(x.0, s[0]), self.h2.h(x.1, s[1]), self.h3.h(x.2, s[2]))
    }
}

/// Inverts the direction of a homotopy.
pub struct Inverse<T>(pub T);

impl<X, T> Homotopy<X> for Inverse<T>
    where T: Homotopy<X>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.g(x)}
    fn g(&self, x: X) -> Self::Y {self.0.f(x)}
    fn h(&self, x: X, s: f64) -> Self::Y {self.0.h(x, 1.0 - s)}
}

/// Converts to and from vectors.
#[derive(Copy, Clone)]
pub struct AsVec<T>(pub T);

impl<X, Y, S, T> Homotopy<[X; 2], S> for AsVec<T>
    where T: Homotopy<(X, X), S, Y = (Y, Y)>, X: Copy
{
    type Y = [Y; 2];

    fn f(&self, x: [X; 2]) -> Self::Y {
        let (a, b) = self.0.f((x[0], x[1]));
        [a, b]
    }
    fn g(&self, x: [X; 2]) -> Self::Y {
        let (a, b) = self.0.g((x[0], x[1]));
        [a, b]
    }
    fn h(&self, x: [X; 2], s: S) -> Self::Y {
        let (a, b) = self.0.h((x[0], x[1]), s);
        [a, b]
    }
}

impl<X, Y, S, T> Homotopy<[X; 3], S> for AsVec<T>
    where T: Homotopy<(X, X, X), S, Y = (Y, Y, Y)>, X: Copy
{
    type Y = [Y; 3];

    fn f(&self, x: [X; 3]) -> Self::Y {
        let (a, b, c) = self.0.f((x[0], x[1], x[2]));
        [a, b, c]
    }
    fn g(&self, x: [X; 3]) -> Self::Y {
        let (a, b, c) = self.0.g((x[0], x[1], x[2]));
        [a, b, c]
    }
    fn h(&self, x: [X; 3], s: S) -> Self::Y {
        let (a, b, c) = self.0.h((x[0], x[1], x[2]), s);
        [a, b, c]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_id() {
        assert!(check(&Id, 0.0 as f64));
        assert!(check(&Id, 1.0 as f64));
        assert!(check(&Id, true));
        assert!(check(&Id, false));
    }

    #[test]
    fn check_dirac() {
        assert!(check(&Dirac, ()));
    }

    #[test]
    fn check_dirac_from() {
        let ft = DiracFrom::new(|()| 1.0, |()| 0.0);
        assert!(check(&ft, ()));
    }

    #[test]
    fn check_lerp() {
        let lerp = Lerp(1.2, 1.3);
        assert!(check(&lerp, ()));
    }

    #[test]
    fn check_quadratic_bezier() {
        let qb = QuadraticBezier(0.3, 0.7, 0.9);
        assert!(check(&qb, ()));
    }

    #[test]
    fn check_cubic_bezier() {
        let cb = CubicBezier(0.3, 0.7, 0.8, 0.9);
        assert!(check(&cb, ()));
    }

    #[test]
    fn check_reduced_quadratic_bezier_equals_lerp() {
        let qb = QuadraticBezier::from_linear(0.0, 1.0);
        let l = Lerp(0.0, 1.0);
        let mut s = 0.0;
        loop {
            assert!((qb.h((), s) - l.h((), s)).abs() < 0.000001);
            s += 0.1;
            if s > 1.0 {break}
        }
    }

    #[test]
    fn check_reduced_cubic_bezier_equals_quadratic_bezier() {
        let cb = CubicBezier::from_quadratic(0.0, 0.3, 0.9);
        let qb = QuadraticBezier(0.0, 0.3, 0.9);
        let mut s = 0.0;
        loop {
            assert_eq!(cb.h((), s), qb.h((), s));
            s += 0.1;
            if s > 1.0 {break}
        }
    }

    #[test]
    fn check_composition() {
        // Create a linear interpolation.
        let a = Lerp(3.0, 10.0);
        assert_eq!(a.h((), 0.0), 3.0);
        assert_eq!(a.h((), 0.5), 6.5);
        assert_eq!(a.h((), 1.0), 10.0);
        // Compose with a Dirac From that seperates the start of the line
        // from the rest of the line.
        let b = DiracFrom::new(|x| x - 2.0, |x| x + 2.0);
        let c = Compose::new(a, b);
        assert!(check(&c, ()));
        assert_eq!(c.h((), 0.0), 1.0);
        assert_eq!(c.h((), 0.0000000000000001), 5.0);
        assert_eq!(c.h((), 0.5), 8.5);
        assert_eq!(c.h((), 1.0), 12.0);
    }

    #[test]
    fn check_square() {
        let a = Lerp(1.0, 5.0);
        let b = Lerp(11.0, 15.0);
        let c = Square::new(a, b);
        let unit = ((), ());
        assert!(check2(&c, unit));
        assert!(check(&c.diagonal(), unit));
        assert!(check2(&c.as_vec(), [(); 2]));
        assert!(check(&c.left_right(0.5), unit));
        assert!(check(&c.top_bottom(0.5), unit));
    }

    #[test]
    fn check_cube() {
        let a = Lerp(1.0, 2.0);
        let b = Lerp(3.0, 4.0);
        let c = Lerp(5.0, 6.0);
        let c = Cube::new(a, b, c);
        let unit = ((), (), ());
        assert!(check3(&c, unit));
        assert!(check(&c.diagonal(), unit));
        assert!(check3(&c.as_vec(), [(); 3]));
        assert!(check2(&c.left_right(0.5), unit));
        assert!(check2(&c.top_bottom(0.5), unit));
        assert!(check2(&c.front_back(0.5), unit));
    }

    #[test]
    fn check_invert() {
        let a = Lerp(2.0, 4.0);
        let b = a.inverse();
        assert!(check(&b, ()));
    }
}
