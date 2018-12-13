//! A library for homotopy logic.

#![deny(missing_docs)]

use std::ops::{Add, Mul};
use std::marker::PhantomData;

/// A continuous map between two functions.
pub trait Homotopy<X, Scalar=f64> {
    /// The output type.
    type Y;

    /// The function being mapped from.
    fn f(&self, X) -> Self::Y;
    /// The function being mapped to.
    fn g(&self, X) -> Self::Y;
    /// A continuous map such that `h(x, 0.0) == f(x)` and `h(x, 1.0) == g(x)`.
    fn h(&self, X, Scalar) -> Self::Y;
}

/// Checks that the homotopy constraints hold for some input `x`.
pub fn check<H, X>(h: &H, x: X) -> bool
    where H: Homotopy<X>,
          H::Y: PartialEq,
          X: Clone
{
    h.h(x.clone(), 0.0) == h.f(x.clone()) &&
    h.h(x.clone(), 1.0) == h.g(x)
}

/// Identity homotopy.
///
/// `f`, `g` and `h` uses the identity function, so this is a homotopy.
pub struct Id;

impl<X> Homotopy<X> for Id {
    type Y = X;

    fn f(&self, x: X) -> X {x}
    fn g(&self, x: X) -> X {x}
    fn h(&self, x: X, _: f64) -> X {x}
}

/// The Dirac function.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_id() {
        check(&Id, 0.0 as f64);
        check(&Id, 1.0 as f64);
        check(&Id, true);
        check(&Id, false);
    }

    #[test]
    fn check_dirac() {
        check(&Dirac, ());
    }

    #[test]
    fn check_dirac_from() {
        let ft = DiracFrom::new(|()| 1.0, |()| 0.0);
        check(&ft, ());
    }

    #[test]
    fn check_lerp() {
        let lerp = Lerp(1.2, 1.3);
        check(&lerp, ());
    }

    #[test]
    fn check_quadratic_bezier() {
        let qb = QuadraticBezier(0.3, 0.7, 0.9);
        check(&qb, ());
    }

    #[test]
    fn check_cubic_bezier() {
        let cb = CubicBezier(0.3, 0.7, 0.8, 0.9);
        check(&cb, ());
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
        check(&c, ());
        assert_eq!(c.h((), 0.0), 1.0);
        assert_eq!(c.h((), 0.0000000000000001), 5.0);
        assert_eq!(c.h((), 0.5), 8.5);
        assert_eq!(c.h((), 1.0), 12.0);
    }
}
