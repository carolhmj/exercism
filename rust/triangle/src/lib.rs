use std::ops::Add;

#[derive(Debug)]
pub struct Triangle<T>(T,T,T);

impl<T> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> 
        where T : PartialOrd + From<i8> + Copy + Add<Output=T> {
        //Check for the triangle inequality
        let x = sides[0];
        let y = sides[1];
        let z = sides[2];
        if x > 0i8.into() &&
           y > 0i8.into() &&
           z > 0i8.into() &&
           x + y >= z && 
           x + z >= y &&
           y + z >= x {
            Some(Triangle(x, y, z))
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool 
        where T : PartialEq {
        self.0 == self.1 && self.1 == self.2
    }

    pub fn is_scalene(&self) -> bool 
        where T : PartialEq {
        self.0 != self.1 && self.1 != self.2 && self.0 != self.2
    }

    pub fn is_isosceles(&self) -> bool 
        where T : PartialEq {
        self.0 == self.1 || self.1 == self.2 || self.0 == self.2
    }
}
