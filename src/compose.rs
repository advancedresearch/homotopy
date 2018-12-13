use super::*;

/// Functional composition that is itself a homotopy.
#[derive(Copy, Clone)]
pub struct Compose<H1, H2, S1, S2> {
    h1: H1,
    h2: H2,
    _s1: PhantomData<S1>,
    _s2: PhantomData<S2>,
}

impl<H1, H2, S1, S2> Compose<H1, H2, S1, S2> {
    /// Creates a new composition of two homotopy maps.
    pub fn new(h1: H1, h2: H2) -> Self {
        Compose {
            h1, h2, _s1: PhantomData, _s2: PhantomData
        }
    }
}

impl<X, H1, H2> Homotopy<X, [f64; 2]> for Compose<H1, H2, f64, f64>
    where H1: Homotopy<X>, H2: Homotopy<H1::Y>
{
    type Y = H2::Y;

    fn f(&self, x: X) -> Self::Y {self.h2.f(self.h1.f(x))}
    fn g(&self, x: X) -> Self::Y {self.h2.g(self.h1.g(x))}
    fn h(&self, x: X, s: [f64; 2]) -> Self::Y {self.h2.h(self.h1.h(x, s[0]), s[1])}
}

impl<X, H1, H2> Homotopy<X, [f64; 3]> for Compose<H1, H2, [f64; 2], f64>
    where H1: Homotopy<X, [f64; 2]>, H2: Homotopy<H1::Y>
{
    type Y = H2::Y;

    fn f(&self, x: X) -> Self::Y {self.h2.f(self.h1.f(x))}
    fn g(&self, x: X) -> Self::Y {self.h2.g(self.h1.g(x))}
    fn h(&self, x: X, s: [f64; 3]) -> Self::Y {self.h2.h(self.h1.h(x, [s[0], s[1]]), s[2])}
}

impl<X, H1, H2> Homotopy<X, [f64; 3]> for Compose<H1, H2, f64, [f64; 2]>
    where H1: Homotopy<X>, H2: Homotopy<H1::Y, [f64; 2]>
{
    type Y = H2::Y;

    fn f(&self, x: X) -> Self::Y {self.h2.f(self.h1.f(x))}
    fn g(&self, x: X) -> Self::Y {self.h2.g(self.h1.g(x))}
    fn h(&self, x: X, s: [f64; 3]) -> Self::Y {self.h2.h(self.h1.h(x, s[0]), [s[1], s[2]])}
}

impl<X, H1, H2> Homotopy<X, [f64; 4]> for Compose<H1, H2, [f64; 2], [f64; 2]>
    where H1: Homotopy<X, [f64; 2]>, H2: Homotopy<H1::Y, [f64; 2]>
{
    type Y = H2::Y;

    fn f(&self, x: X) -> Self::Y {self.h2.f(self.h1.f(x))}
    fn g(&self, x: X) -> Self::Y {self.h2.g(self.h1.g(x))}
    fn h(&self, x: X, s: [f64; 4]) -> Self::Y {self.h2.h(self.h1.h(x, [s[0], s[1]]), [s[2], s[3]])}
}

impl<X, H1, H2> Homotopy<X, [f64; 4]> for Compose<H1, H2, [f64; 3], f64>
    where H1: Homotopy<X, [f64; 3]>, H2: Homotopy<H1::Y, f64>
{
    type Y = H2::Y;

    fn f(&self, x: X) -> Self::Y {self.h2.f(self.h1.f(x))}
    fn g(&self, x: X) -> Self::Y {self.h2.g(self.h1.g(x))}
    fn h(&self, x: X, s: [f64; 4]) -> Self::Y {self.h2.h(self.h1.h(x, [s[0], s[1], s[2]]), s[3])}
}

impl<X, H1, H2> Homotopy<X, [f64; 4]> for Compose<H1, H2, f64, [f64; 3]>
    where H1: Homotopy<X, f64>, H2: Homotopy<H1::Y, [f64; 3]>
{
    type Y = H2::Y;

    fn f(&self, x: X) -> Self::Y {self.h2.f(self.h1.f(x))}
    fn g(&self, x: X) -> Self::Y {self.h2.g(self.h1.g(x))}
    fn h(&self, x: X, s: [f64; 4]) -> Self::Y {self.h2.h(self.h1.h(x, s[0]), [s[1], s[2], s[3]])}
}

impl<X, H1, H2> Homotopy<X, [f64; 5]> for Compose<H1, H2, [f64; 4], f64>
    where H1: Homotopy<X, [f64; 4]>, H2: Homotopy<H1::Y, f64>
{
    type Y = H2::Y;

    fn f(&self, x: X) -> Self::Y {self.h2.f(self.h1.f(x))}
    fn g(&self, x: X) -> Self::Y {self.h2.g(self.h1.g(x))}
    fn h(&self, x: X, s: [f64; 5]) -> Self::Y {
        self.h2.h(self.h1.h(x, [s[0], s[1], s[2], s[3]]), s[4])
    }
}
