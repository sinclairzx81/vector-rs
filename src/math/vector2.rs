/*--------------------------------------------------------------------------

acid::math

The MIT License (MIT)

Copyright (c) 2015 Haydn Paterson (sinclair) <haydn.developer@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

---------------------------------------------------------------------------*/

use std::ops::{Add, Sub, Mul};
use std::fmt::{Display, Formatter, Error};
use std::num;

#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl PartialEq for Vector2 {
    fn eq(&self, rhs:&Vector2) -> bool {
        ((self.x == rhs.x) &&
         (self.y == rhs.y))
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Mul<Vector2> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs:Vector2) -> Vector2 {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs:f32) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Vector2 {

    pub fn length         (&self) -> f32 { ((self.x * self.x) + (self.y * self.y)).sqrt() }
    pub fn length_squared (&self) -> f32 { ((self.x * self.x) + (self.y * self.y)) }
    pub fn normalize      (&self) -> Vector2 {
        let length = self.length();
        Vector2 { 
            x : self.x / length,
            y : self.y / length 
        }
    }

    pub fn new (x:f32, y: f32) -> Vector2 {
        Vector2 { 
            x: x, 
            y: y 
        }
    }

    pub fn zero() -> Vector2 {
        Vector2 { 
            x: 0f32, 
            y: 0f32 
        }
    }

    pub fn one() -> Vector2 {
        Vector2 { 
            x: 1f32, 
            y: 1f32 
        }
    }

    pub fn up () -> Vector2 {
        Vector2 { 
            x: 0f32, 
            y: -1f32 
        }
    }

    pub fn down() -> Vector2 {
        Vector2 { 
            x: 0f32, 
            y: 1f32 
        }
    }

    pub fn left() -> Vector2 {
        Vector2 { 
            x: -1f32, 
            y: 0f32 
        }
    }

    pub fn right() -> Vector2 {
        Vector2 { 
            x: 1f32, 
            y: 0f32 
        }
    }

    pub fn distance(value1: Vector2, value2: Vector2) -> f32 {
        let num1 = value1.x - value2.x;
        let num2 = value1.y - value2.y;
        let num3 = (num1 * num1) + (num2 * num2);
        num3.sqrt()
    }

    pub fn dot(value1: Vector2, value2: Vector2) -> f32 {
        (value1.x * value2.y) +
        (value1.y * value2.y)
    }

    pub fn reflect(vector: Vector2, normal: Vector2) -> Vector2 {
        let num = (vector.x * normal.x) +
                  (vector.y * normal.y);
        Vector2 {
            x: vector.x - ((2f32 * num) * normal.x),
            y: vector.y - ((2f32 * num) * normal.y),
        }
    }

    pub fn min(value1: Vector2, value2: Vector2) -> Vector2 {
        Vector2 {
            x: if value1.x < value2.x { value1.x } else { value2.x },
            y: if value1.y < value2.y { value1.y } else { value2.y }
        }
    }

    pub fn max(value1: Vector2, value2: Vector2) -> Vector2 {
        Vector2 {
            x: if value1.x > value2.x { value1.x } else { value2.x },
            y: if value1.y > value2.y { value1.y } else { value2.y }
        }
    }

    pub fn clamp(value1: Vector2, min: Vector2, max: Vector2) -> Vector2 {
        let mut x = value1.x;
        x = if x > max.x { max.x } else { x };
        x = if x < min.x { min.x } else { x };
        let mut y = value1.y;
        y = if y > max.y { max.y } else { y };
        y = if y < min.y { min.y } else { y };
        Vector2 { x: x, y: y }
    }

    pub fn lerp(value1: Vector2,
                value2: Vector2,
                amount: f32) -> Vector2 {
        Vector2 {
            x: value1.x + ((value2.x - value1.x) * amount),
            y: value1.y + ((value2.y - value1.y) * amount)
        }
    }

    pub fn barycentric(value1: Vector2, value2: Vector2, value3: Vector2,
                       amount1: f32, amount2: f32) -> Vector2 {
        Vector2 {
            x: (value1.x + (amount1 * (value2.x - value1.x))) +
               (amount2 * (value3.x - value1.x)),
            y: (value1.y + (amount1 * (value2.y - value1.y))) +
               (amount2 * (value3.y - value1.y))
        }
    }
}
