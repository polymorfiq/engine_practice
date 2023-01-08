use core::cmp::{min, max};
use physics::{Zero, Space};
use physics::space::{Observable, Quantifiable, Intersects, Intersectable};

#[derive(Copy, Clone)]
pub struct NonRotatingBox<const D: usize, S: Space<D>> {
    ltop: S,
    rbottom: S
}

impl<S: Space<2>> NonRotatingBox<2, S> {
    pub fn width(&self) -> S {
        let [width, _] = self.ltop.distance(&self.rbottom).components();
        width
    }

    pub fn height(&self) -> S {
        let [_, height] = self.ltop.distance(&self.rbottom).components();
        height
    }
}

impl<S: Space<2>> Intersectable<2, S> for NonRotatingBox<2, S> {}
impl<S: Space<2> + Observable<2, S>> Intersects<2, S, Self> for NonRotatingBox<2, S> {
    type Intersection = Self;

    fn intersection(&self, other: &Self) -> Self::Intersection {
        let ltop_a = self.ltop.components();
        let rbottom_a = self.rbottom.components();
        let ltop_b = other.ltop.components();
        let rbottom_b = other.rbottom.components();

        let left_x = max(ltop_a[0], ltop_b[0]);
        let right_x = min(rbottom_a[0], rbottom_b[0]);
        let top_y = max(ltop_a[1], ltop_b[1]);
        let bottom_y = min(rbottom_a[1], rbottom_b[1]);

        Self {
            ltop: left_x.offset(&top_y),
            rbottom: right_x.offset(&bottom_y)
        }
    }

    fn distance_until_intersection(&self, _other: &Self) -> S {
        S::new(&[S::Base::zero(); 2])
    }
}

impl<const D: usize, S: Space<D> + Observable<D, S>> Quantifiable<D, S> for NonRotatingBox<D, S> {
    fn area(&self) -> S {
        let dist_comps = self.ltop.distance(&self.rbottom).components();
        let mut sum = S::new(&[S::Base::zero(); D]);

        for i in 0..dist_comps.len() {
            let j = (i + 1) % dist_comps.len();
            sum = sum.offset(&dist_comps[i].scale(&dist_comps[j]));
        }

        sum.area()
    }
}