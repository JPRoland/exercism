extern crate num_traits;

use num_traits::Num;

pub struct Triangle<T>
where
    T: Num + PartialOrd + Copy,
{
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T>
where
    T: Num + PartialOrd + Copy,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let mut sides: Vec<T> = sides.to_vec();
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if sides[0] + sides[1] <= sides[2] {
            return None;
        }

        Some(Triangle {
            a: sides[0],
            b: sides[1],
            c: sides[2],
        })
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.a == self.c
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c
    }
}
