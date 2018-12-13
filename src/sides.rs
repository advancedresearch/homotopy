use super::*;

/// The diagonal of an N-dimensional homotopy, resulting in a 1D homotopy.
///
/// This interpolates along all dimensions at once.
#[derive(Copy, Clone)]
pub struct Diagonal<'a, T, S> {
    shape: &'a T,
    _s: PhantomData<S>,
}

impl<'a, T, S> Diagonal<'a, T, S> {
    /// Creates a new diagonal.
    pub fn new(shape: &'a T) -> Self {
        Diagonal {shape, _s: PhantomData}
    }
}

impl<'a, X, T> Homotopy<X> for Diagonal<'a, T, [f64; 2]>
    where T: Homotopy<X, [f64; 2]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.shape.f(x)}
    fn g(&self, x: X) -> Self::Y {self.shape.g(x)}
    fn h(&self, x: X, s: f64) -> Self::Y {self.shape.h(x, [s; 2])}
}

impl<'a, X, T> Homotopy<X> for Diagonal<'a, T, [f64; 3]>
    where T: Homotopy<X, [f64; 3]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.shape.f(x)}
    fn g(&self, x: X) -> Self::Y {self.shape.g(x)}
    fn h(&self, x: X, s: f64) -> Self::Y {self.shape.h(x, [s; 3])}
}

/// The left side of an N-dimensional homotopy, resulting in a N-1 homotopy.
#[derive(Copy, Clone)]
pub struct Left<'a, T>(pub &'a T);

impl<'a, X, T> Homotopy<X> for Left<'a, T>
    where T: Homotopy<X, [f64; 2]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.f(x)}
    fn g(&self, x: X) -> Self::Y {self.0.h(x, [0.0, 1.0])}
    fn h(&self, x: X, s: f64) -> Self::Y {self.0.h(x, [0.0, s])}
}

impl<'a, X, T> Homotopy<X, [f64; 2]> for Left<'a, T>
    where T: Homotopy<X, [f64; 3]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.f(x)}
    fn g(&self, x: X) -> Self::Y {self.0.h(x, [0.0, 1.0, 1.0])}
    fn h(&self, x: X, s: [f64; 2]) -> Self::Y {self.0.h(x, [0.0, s[0], s[1]])}
}

/// The right side of an N-dimensional homotopy, resulting in a N-1 homotopy.
#[derive(Copy, Clone)]
pub struct Right<'a, T>(pub &'a T);

impl<'a, X, T> Homotopy<X> for Right<'a, T>
    where T: Homotopy<X, [f64; 2]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.h(x, [1.0, 0.0])}
    fn g(&self, x: X) -> Self::Y {self.0.g(x)}
    fn h(&self, x: X, s: f64) -> Self::Y {self.0.h(x, [1.0, s])}
}

impl<'a, X, T> Homotopy<X, [f64; 2]> for Right<'a, T>
    where T: Homotopy<X, [f64; 3]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.h(x, [1.0, 0.0, 0.0])}
    fn g(&self, x: X) -> Self::Y {self.0.g(x)}
    fn h(&self, x: X, s: [f64; 2]) -> Self::Y {self.0.h(x, [1.0, s[0], s[1]])}
}

/// The top side of an N-dimensional homotopy, resulting in a N-1 homotopy.
#[derive(Copy, Clone)]
pub struct Top<'a, T>(pub &'a T);

impl<'a, X, T> Homotopy<X> for Top<'a, T>
    where T: Homotopy<X, [f64; 2]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.f(x)}
    fn g(&self, x: X) -> Self::Y {self.0.h(x, [1.0, 0.0])}
    fn h(&self, x: X, s: f64) -> Self::Y {self.0.h(x, [s, 0.0])}
}

impl<'a, X, T> Homotopy<X, [f64; 2]> for Top<'a, T>
    where T: Homotopy<X, [f64; 3]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.f(x)}
    fn g(&self, x: X) -> Self::Y {self.0.h(x, [1.0, 0.0, 1.0])}
    fn h(&self, x: X, s: [f64; 2]) -> Self::Y {self.0.h(x, [s[0], 0.0, s[1]])}
}

/// The bottom side of an N-dimensional homotopy, resulting in a N-1 homotopy.
#[derive(Copy, Clone)]
pub struct Bottom<'a, T>(pub &'a T);

impl<'a, X, T> Homotopy<X> for Bottom<'a, T>
    where T: Homotopy<X, [f64; 2]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.h(x, [0.0, 1.0])}
    fn g(&self, x: X) -> Self::Y {self.0.g(x)}
    fn h(&self, x: X, s: f64) -> Self::Y {self.0.h(x, [s, 1.0])}
}

impl<'a, X, T> Homotopy<X, [f64; 2]> for Bottom<'a, T>
    where T: Homotopy<X, [f64; 3]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.h(x, [0.0, 1.0, 0.0])}
    fn g(&self, x: X) -> Self::Y {self.0.g(x)}
    fn h(&self, x: X, s: [f64; 2]) -> Self::Y {self.0.h(x, [s[0], 1.0, s[1]])}
}

/// The front side of an N-dimensional homotopy, resulting in a N-1 homotopy.
#[derive(Copy, Clone)]
pub struct Front<'a, T>(pub &'a T);

impl<'a, X, T> Homotopy<X, [f64; 2]> for Front<'a, T>
    where T: Homotopy<X, [f64; 3]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.f(x)}
    fn g(&self, x: X) -> Self::Y {self.0.h(x, [1.0, 1.0, 0.0])}
    fn h(&self, x: X, s: [f64; 2]) -> Self::Y {self.0.h(x, [s[0], s[1], 0.0])}
}

/// The back side of an N-dimensional homotopy, resulting in a N-1 homotopy.
#[derive(Copy, Clone)]
pub struct Back<'a, T>(pub &'a T);

impl<'a, X, T> Homotopy<X, [f64; 2]> for Back<'a, T>
    where T: Homotopy<X, [f64; 3]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.h(x, [0.0, 0.0, 1.0])}
    fn g(&self, x: X) -> Self::Y {self.0.g(x)}
    fn h(&self, x: X, s: [f64; 2]) -> Self::Y {self.0.h(x, [s[0], s[1], 1.0])}
}
