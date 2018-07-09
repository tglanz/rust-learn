use std::ops::{Add};

const Up: Vec2 = Vec2 { x: 0, y: 1 };
const Right: Vec2 = Vec2 { x: 1, y: 0 };
const Bottom: Vec2 = Vec2 { x: 0, y: -1 };
const Left: Vec2 = Vec2 { x: -1, y: 0 };

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Vec2 {
    x: i16,
    y: i16,
}

impl Vec2 {
    pub fn new(x: i16, y: i16) -> Vec2 {
        Vec2 {
            x, y
        }
    }

    pub fn x(&self) -> &i16 {
        &self.x
    }
    
    pub fn y(&self) -> &i16 {
        &self.y
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec2_add() {
        assert_eq!(
            Vec2::new(1, 1) + Vec2::new(2, 2),
            Vec2::new(3, 3)
        )
    }

    fn vec2_directions(){
        assert_eq!(Up, Vec2::new(0, 1));
        assert_eq!(Right, Vec2::new(1, 0));
        assert_eq!(Bottom, Vec2::new(0, -1));
        assert_eq!(Left, Vec2::new(-1, 0));
    }
}