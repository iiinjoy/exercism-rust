use std::ops::Add;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Default + Copy + PartialEq + Add<Output = T> + PartialOrd,
{
    pub fn build(sides: [T; 3]) -> Option<Self> {
        if sides[0] + sides[1] >= sides[2]
            && sides[1] + sides[2] >= sides[0]
            && sides[2] + sides[0] >= sides[1]
            && sides.iter().all(|&s| s > T::default())
        {
            Some(Triangle { sides })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[1] == self.sides[2]
            || self.sides[2] == self.sides[0]
    }
}
