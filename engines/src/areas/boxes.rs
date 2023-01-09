use core::cmp::{min, max};
use physics::{Zero, Space, space};
use physics::space::ObservableSpace;

#[derive(Copy, Clone)]
pub struct NonRotatingBox<S: Space> {
    ltop: S,
    rbottom: S
}

impl<S: ObservableSpace<2>> NonRotatingBox<S> {
    pub fn width(&self) -> S {
        let [width, _] = (self.rbottom - self.ltop).components();
        width
    }

    pub fn height(&self) -> S {
        let [_, height] = (self.rbottom - self.ltop).components();
        height
    }
}

impl<S: ObservableSpace<2>> space::Area<S> for NonRotatingBox<S> {
    fn amount_of_space(&self) -> S {
        let dist_comps = (self.rbottom - self.ltop).components();
        let mut sum = S::new(&[S::Base::zero(); 2]);

        for i in 0..dist_comps.len() {
            let j = (i + 1) % dist_comps.len();
            sum = sum + (dist_comps[i] * dist_comps[j]);
        }

        sum
    }
}

impl<S: ObservableSpace<2>> space::AreaIntersection<S, Self> for NonRotatingBox<S> {
    fn area_intersection(&self, other: &Self) -> Self {
        let ltop_a = self.ltop.components();
        let rbottom_a = self.rbottom.components();
        let ltop_b = other.ltop.components();
        let rbottom_b = other.rbottom.components();

        let left_x = max(ltop_a[0], ltop_b[0]);
        let right_x = min(rbottom_a[0], rbottom_b[0]);
        let top_y = max(ltop_a[1], ltop_b[1]);
        let bottom_y = min(rbottom_a[1], rbottom_b[1]);

        if right_x > left_x && bottom_y > top_y {
            Self {
                ltop: left_x + top_y,
                rbottom: right_x + bottom_y
            }
        } else {
            Self {
                ltop: self.ltop,
                rbottom: self.ltop
            }
        }
    }
}