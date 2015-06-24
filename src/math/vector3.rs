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

use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Display, Formatter, Error};
use std::num;
use super::matrix::Matrix;
use super::quaternion::Quaternion;

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs:Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs:Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs:f32) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

impl Div<f32> for Vector3 {
	type Output = Vector3;
	fn div(self, rhs: f32) -> Vector3 {
		let num = 1f32 / rhs;
		Vector3 {
			x: self.x / num,
			y: self.y / num,
			z: self.z / num
		}
	}
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Vector3 {

    pub fn equals(&self, other:Vector3) -> bool {
        ((self.x == other.x) &&
         (self.y == other.y) &&
         (self.z == other.z))
    }

    pub fn length(&self) -> f32 {
        ((self.x * self.x) +
         (self.y * self.y) +
         (self.z * self.z)).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        ((self.x * self.x) +
         (self.y * self.y) +
         (self.z * self.z))
    }

    pub fn normalize(&self) -> Vector3 {
        let length = self.length();
        Vector3 { x: self.x / length,
                  y: self.y / length,
                  z: self.z / length }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn one() -> Vector3 {
        Vector3 { 
            x: 1f32, 
            y: 1f32, 
            z: 1f32 
        }
    }

    pub fn zero() -> Vector3 {
        Vector3 { 
            x: 0f32, 
            y: 0f32, 
            z: 0f32 
        }
    }

    pub fn up() -> Vector3 {
        Vector3 { 
            x: 0f32, 
            y: -1f32, 
            z: 0f32 
        }
    }

    pub fn down() -> Vector3 {
        Vector3 { 
            x: 0f32, 
            y: 1f32, 
            z: 0f32 
        }
    }

    pub fn left() -> Vector3 {
        Vector3 { 
            x: -1f32, 
            y: 0f32, 
            z: 0f32 
        }
    }

    pub fn right() -> Vector3 {
        Vector3 { 
            x: 1f32, 
            y: 0f32,
            z: 0f32 
        }
    }

    pub fn forward() -> Vector3 {
        Vector3 { 
            x: 0f32, 
            y: 0f32, 
            z: 1f32 
        }
    }

    pub fn backward() -> Vector3 {
        Vector3 { 
            x: 0f32, 
            y: 0f32, 
            z: -1f32 
        }
    }

    pub fn distance(value1: Vector3, value2: Vector3) -> f32 {
        let num3 = value1.x - value2.x;
        let num2 = value1.y - value2.y;
        let num1 = value1.z - value2.z;
        ((num3 * num3) +
         (num2 * num2) +
         (num1 * num1)).sqrt()
    }

    pub fn dot(value1: Vector3, value2: Vector3) -> f32 {
        (((value1.x * value2.x)  +
          (value1.y * value2.y)) +
          (value1.z * value2.z))
    }

    pub fn cross(value1: Vector3, value2: Vector3) -> Vector3 {
        Vector3 {
            x: (value1.y * value2.z) - (value1.z * value2.y),
            y: (value1.z * value2.x) - (value1.x * value2.z),
            z: (value1.x * value2.y) - (value1.y * value2.x)
        }
    }

    pub fn reflect(vector: Vector3, normal: Vector3) -> Vector3 {
        let num = ((vector.x * normal.x) +
                   (vector.y * normal.y)) +
                   (vector.z * normal.z);
        Vector3 {
            x: vector.x - ((2f32 * num) * normal.x),
            y: vector.y - ((2f32 * num) * normal.y),
            z: vector.z - ((2f32 * num) * normal.z)
        }
    }

    pub fn negate(value: Vector3) -> Vector3 {
        Vector3 {
            x: -value.x,
            y: -value.y,
            z: -value.z
        }
    }

    pub fn min(value1: Vector3, value2: Vector3) -> Vector3 {
        Vector3 {
            x: if value1.x < value2.x { value1.x } else { value2.x },
            y: if value1.y < value2.y { value1.y } else { value2.y },
            z: if value1.z < value2.z { value1.z } else { value2.z }
        }
    }

    pub fn max(value1: Vector3, value2: Vector3) -> Vector3 {
        Vector3 {
            x: if value1.x > value2.x { value1.x } else { value2.x },
            y: if value1.y > value2.y { value1.y } else { value2.y },
            z: if value1.z > value2.z { value1.z } else { value2.z }
        }
    }

    pub fn clamp(value1: Vector3,
                 min: Vector3,
                 max: Vector3) -> Vector3 {
        let mut x = value1.x;
        x = if x > max.x { max.x } else { x };
        x = if x < min.x { min.x } else { x };
        let mut y = value1.y;
        y = if y > max.y { max.y } else { y };
        y = if y < min.y { min.y } else { y };
        let mut z = value1.z;
        z = if z > max.z { max.z } else { z };
        z = if z < min.z { min.z } else { z };
        Vector3 { x: x, y: y, z: z }
    }

    pub fn barycentric(value1: Vector3,
                       value2: Vector3,
                       value3: Vector3,
                       amount1: f32,
                       amount2: f32) -> Vector3 {
        Vector3 {
            x: (value1.x + (amount1 * (value2.x - value1.x))) + (amount2 * (value3.x - value1.x)),
            y: (value1.y + (amount1 * (value2.y - value1.y))) + (amount2 * (value3.y - value1.y)),
            z: (value1.z + (amount1 * (value2.z - value1.z))) + (amount2 * (value3.z - value1.z))
        }
    }

    pub fn smooth_step(value1: Vector3,
                       value2: Vector3,
                       amount: f32) -> Vector3 {
        let mut _amount: f32 = if amount > 1f32 { 1f32 } else {
            if amount < 0f32 { 0f32 } else { amount }
        };
        _amount = (_amount * _amount) * (3f32 - (2f32 * _amount));
        Vector3 {
            x: value1.x + ((value2.x - value1.x) * _amount),
            y: value1.y + ((value2.y - value1.y) * _amount),
            z: value1.z + ((value2.z - value1.z) * _amount)
        }
    }

    pub fn catmull_rom(value1: Vector3,
                       value2: Vector3,
                       value3: Vector3,
                       value4: Vector3,
                       amount: f32) -> Vector3 {
        let num = amount * amount;
        let num2 = amount * num;
        Vector3 {
            x:  0.5f32 * ((((2f32 * value2.x) + ((-value1.x + value3.x) * amount)) + (((((2f32 * value1.x) - (5f32 * value2.x)) + (4f32 * value3.x)) - value4.x) * num)) + ((((-value1.x + (3f32 * value2.x)) - (3f32 * value3.x)) + value4.x) * num2)),
            y:  0.5f32 * ((((2f32 * value2.y) + ((-value1.y + value3.y) * amount)) + (((((2f32 * value1.y) - (5f32 * value2.y)) + (4f32 * value3.y)) - value4.y) * num)) + ((((-value1.y + (3f32 * value2.y)) - (3f32 * value3.y)) + value4.y) * num2)),
            z:  0.5f32 * ((((2f32 * value2.z) + ((-value1.z + value3.z) * amount)) + (((((2f32 * value1.z) - (5f32 * value2.z)) + (4f32 * value3.z)) - value4.z) * num)) + ((((-value1.z + (3f32 * value2.z)) - (3f32 * value3.z)) + value4.z) * num2))
        }
    }

    pub fn hermite(value1: Vector3, tangent1: Vector3, value2: Vector3, tangent2: Vector3, amount: f32) -> Vector3 {
        let num = amount * amount;
        let num2 = amount * num;
        let num6 = ((2f32 * num2) - (3f32 * num)) + 1f32;
        let num5 = (-2f32 * num2) + (3f32 * num);
        let num4 = (num2 - (2f32 * num)) + amount;
        let num3 = num2 - num;
        Vector3 {
            x: (((value1.x * num6) + (value2.x * num5)) + (tangent1.x * num4)) + (tangent2.x * num3),
            y: (((value1.y * num6) + (value2.y * num5)) + (tangent1.y * num4)) + (tangent2.y * num3),
            z: (((value1.z * num6) + (value2.z * num5)) + (tangent1.z * num4)) + (tangent2.z * num3)
        }
    }

    pub fn transform(position: Vector3, matrix: Matrix) -> Vector3 {
        Vector3 {
            x: (((position.x * matrix.m11) + (position.y * matrix.m21)) + (position.z * matrix.m31)) + matrix.m41,
            y: (((position.x * matrix.m12) + (position.y * matrix.m22)) + (position.z * matrix.m32)) + matrix.m42,
            z: (((position.x * matrix.m13) + (position.y * matrix.m23)) + (position.z * matrix.m33)) + matrix.m43
        }
    }

    pub fn transform_normal(normal: Vector3, matrix: Matrix) -> Vector3 {
        Vector3 {
            x: ((normal.x * matrix.m11) + (normal.y * matrix.m21)) + (normal.z * matrix.m31),
            y: ((normal.x * matrix.m12) + (normal.y * matrix.m22)) + (normal.z * matrix.m32),
            z: ((normal.x * matrix.m13) + (normal.y * matrix.m23)) + (normal.z * matrix.m33)
        }
    }

    pub fn transform_quaternion(value: Vector3, rotation: Quaternion) -> Vector3 {
        let num12 = rotation.x + rotation.x;
        let num2  = rotation.y + rotation.y;
        let num   = rotation.z + rotation.z;
        let num11 = rotation.w * num12;
        let num10 = rotation.w * num2;
        let num9  = rotation.w * num;
        let num8  = rotation.x * num12;
        let num7  = rotation.x * num2;
        let num6  = rotation.x * num;
        let num5  = rotation.y * num2;
        let num4  = rotation.y * num;
        let num3  = rotation.z * num;
        Vector3 {
            x: ((value.x * ((1f32 - num5) - num3)) + (value.y * (num7 - num9))) + (value.z * (num6 + num10)),
            y: ((value.x * (num7 + num9)) + (value.y * ((1f32 - num8) - num3))) + (value.z * (num4 - num11)),
            z: ((value.x * (num6 - num10)) + (value.y * (num4 + num11))) + (value.z * ((1f32 - num8) - num5))
        }
    }
}
