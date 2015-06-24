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
use super::vector3::Vector3;
use super::plane::Plane;
use super::quaternion:: Quaternion;

#[derive(Copy, Clone)]
pub struct Matrix {
    pub m11: f32, pub m12: f32, pub m13: f32, pub m14: f32,
    pub m21: f32, pub m22: f32, pub m23: f32, pub m24: f32,
    pub m31: f32, pub m32: f32, pub m33: f32, pub m34: f32,
    pub m41: f32, pub m42: f32, pub m43: f32, pub m44: f32
}

impl PartialEq<Matrix> for Matrix {
    fn eq(&self, other:&Matrix) -> bool {
        (self.m11 == other.m11) && (self.m12 == other.m12) && (self.m13 == other.m13) && (self.m14 == other.m14) && 
        (self.m21 == other.m21) && (self.m22 == other.m22) && (self.m23 == other.m23) && (self.m14 == other.m14) && 
        (self.m31 == other.m31) && (self.m32 == other.m32) && (self.m33 == other.m33) && (self.m14 == other.m14) && 
        (self.m41 == other.m41) && (self.m42 == other.m42) && (self.m43 == other.m43) && (self.m14 == other.m14)        
    }
}

impl Add<Matrix> for Matrix {
    type Output = Matrix;
    fn add(self, rhs:Matrix) -> Matrix {
        Matrix {
            m11: self.m11 + rhs.m11, m12: self.m12 + rhs.m12, m13: self.m13 + rhs.m13, m14: self.m14 + rhs.m14,
            m21: self.m21 + rhs.m21, m22: self.m22 + rhs.m22, m23: self.m23 + rhs.m23, m24: self.m24 + rhs.m24,
            m31: self.m31 + rhs.m31, m32: self.m32 + rhs.m32, m33: self.m33 + rhs.m33, m34: self.m34 + rhs.m34,
            m41: self.m41 + rhs.m41, m42: self.m42 + rhs.m42, m43: self.m43 + rhs.m43, m44: self.m44 + rhs.m44
        }
    }
}

impl Sub<Matrix> for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Matrix {
        Matrix {
            m11: self.m11 - rhs.m11, m12: self.m12 - rhs.m12, m13: self.m13 - rhs.m13, m14: self.m14 - rhs.m14, 
            m21: self.m21 - rhs.m21, m22: self.m22 - rhs.m22, m23: self.m23 - rhs.m23, m24: self.m24 - rhs.m24, 
            m31: self.m31 - rhs.m31, m32: self.m32 - rhs.m32, m33: self.m33 - rhs.m33, m34: self.m34 - rhs.m34,
            m41: self.m41 - rhs.m41, m42: self.m42 - rhs.m42, m43: self.m43 - rhs.m43, m44: self.m44 - rhs.m44
        }
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs:Matrix) -> Matrix {
        Matrix {
            m11: (((self.m11 * rhs.m11) + (self.m12 * rhs.m21)) + (self.m13 * rhs.m31)) + (self.m14 * rhs.m41),
            m12: (((self.m11 * rhs.m12) + (self.m12 * rhs.m22)) + (self.m13 * rhs.m32)) + (self.m14 * rhs.m42),
            m13: (((self.m11 * rhs.m13) + (self.m12 * rhs.m23)) + (self.m13 * rhs.m33)) + (self.m14 * rhs.m43),
            m14: (((self.m11 * rhs.m14) + (self.m12 * rhs.m24)) + (self.m13 * rhs.m34)) + (self.m14 * rhs.m44),
            m21: (((self.m21 * rhs.m11) + (self.m22 * rhs.m21)) + (self.m23 * rhs.m31)) + (self.m24 * rhs.m41),
            m22: (((self.m21 * rhs.m12) + (self.m22 * rhs.m22)) + (self.m23 * rhs.m32)) + (self.m24 * rhs.m42),
            m23: (((self.m21 * rhs.m13) + (self.m22 * rhs.m23)) + (self.m23 * rhs.m33)) + (self.m24 * rhs.m43),
            m24: (((self.m21 * rhs.m14) + (self.m22 * rhs.m24)) + (self.m23 * rhs.m34)) + (self.m24 * rhs.m44),
            m31: (((self.m31 * rhs.m11) + (self.m32 * rhs.m21)) + (self.m33 * rhs.m31)) + (self.m34 * rhs.m41),
            m32: (((self.m31 * rhs.m12) + (self.m32 * rhs.m22)) + (self.m33 * rhs.m32)) + (self.m34 * rhs.m42),
            m33: (((self.m31 * rhs.m13) + (self.m32 * rhs.m23)) + (self.m33 * rhs.m33)) + (self.m34 * rhs.m43),
            m34: (((self.m31 * rhs.m14) + (self.m32 * rhs.m24)) + (self.m33 * rhs.m34)) + (self.m34 * rhs.m44),
            m41: (((self.m41 * rhs.m11) + (self.m42 * rhs.m21)) + (self.m43 * rhs.m31)) + (self.m44 * rhs.m41),
            m42: (((self.m41 * rhs.m12) + (self.m42 * rhs.m22)) + (self.m43 * rhs.m32)) + (self.m44 * rhs.m42),
            m43: (((self.m41 * rhs.m13) + (self.m42 * rhs.m23)) + (self.m43 * rhs.m33)) + (self.m44 * rhs.m43),
            m44: (((self.m41 * rhs.m14) + (self.m42 * rhs.m24)) + (self.m43 * rhs.m34)) + (self.m44 * rhs.m44)
        }
    }
}

impl Mul<f32> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs:f32) -> Matrix {
        Matrix {
            m11: self.m11 * rhs, m12: self.m12 * rhs, m13: self.m13 * rhs, m14: self.m14 * rhs,
            m21: self.m21 * rhs, m22: self.m22 * rhs, m23: self.m23 * rhs, m24: self.m24 * rhs,
            m31: self.m31 * rhs, m32: self.m32 * rhs, m33: self.m33 * rhs, m34: self.m34 * rhs,
            m41: self.m41 * rhs, m42: self.m42 * rhs, m43: self.m43 * rhs, m44: self.m44 * rhs
        }
    }
}

impl Div<Matrix> for Matrix {
    type Output = Matrix;
    fn div(self, rhs:Matrix) -> Matrix {
        Matrix {
            m11: self.m11 / rhs.m11, m12: self.m12 / rhs.m12, m13: self.m13 / rhs.m13, m14: self.m14 / rhs.m14,
            m21: self.m21 / rhs.m21, m22: self.m22 / rhs.m22, m23: self.m23 / rhs.m23, m24: self.m24 / rhs.m24,
            m31: self.m31 / rhs.m31, m32: self.m32 / rhs.m32, m33: self.m33 / rhs.m33, m34: self.m34 / rhs.m34,
            m41: self.m41 / rhs.m41, m42: self.m42 / rhs.m42, m43: self.m43 / rhs.m43, m44: self.m44 / rhs.m44
        }
    }
}

impl Div<f32> for Matrix {
    type Output = Matrix;
    fn div(self, rhs:f32) -> Matrix {
        let num = 1f32 / rhs;
        Matrix {
            m11: self.m11 * num, m12: self.m12 * num, m13: self.m13 * num, m14: self.m14 * num,
            m21: self.m21 * num, m22: self.m22 * num, m23: self.m23 * num, m24: self.m24 * num,
            m31: self.m31 * num, m32: self.m32 * num, m33: self.m33 * num, m34: self.m34 * num,
            m41: self.m41 * num, m42: self.m42 * num, m43: self.m43 * num, m44: self.m44 * num
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {}, {}, {}, \n {}, {}, {}, {}, \n {}, {}, {}, {}, \n {}, {}, {}, {})", 
            self.m11, self.m12, self.m13, self.m14,
            self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34,
            self.m41, self.m42, self.m43, self.m44)
    }
}

impl Matrix {

    pub fn up       (&self) -> Vector3 { Vector3::new( self.m21,  self.m22,  self.m23) }
    pub fn down     (&self) -> Vector3 { Vector3::new(-self.m21, -self.m22, -self.m23) }
    pub fn right    (&self) -> Vector3 { Vector3::new( self.m11,  self.m12,  self.m13) }
    pub fn left     (&self) -> Vector3 { Vector3::new(-self.m11, -self.m12, -self.m13) }
    pub fn forward  (&self) -> Vector3 { Vector3::new(-self.m31, -self.m32, -self.m33) }
    pub fn backward (&self) -> Vector3 { Vector3::new( self.m31,  self.m32,  self.m33) }

    pub fn new (m11: f32, m12: f32, m13: f32, m14: f32,
                m21: f32, m22: f32, m23: f32, m24: f32,
                m31: f32, m32: f32, m33: f32, m34: f32,
                m41: f32, m42: f32, m43: f32, m44: f32) -> Matrix {
        Matrix {
            m11: m11, m12: m12, m13: m13, m14: m14,
            m21: m21, m22: m22, m23: m23, m24: m24,
            m31: m31, m32: m32, m33: m33, m34: m34,
            m41: m41, m42: m42, m43: m43, m44: m44
        }
    }

    pub fn identity() -> Matrix {
        Matrix {
            m11: 1f32, m12: 0f32, m13: 0f32, m14: 0f32,
            m21: 0f32, m22: 1f32, m23: 0f32, m24: 0f32,
            m31: 0f32, m32: 0f32, m33: 1f32, m34: 0f32,
            m41: 0f32, m42: 0f32, m43: 0f32, m44: 1f32
        }
    }

    pub fn zero() -> Matrix {
        Matrix {
            m11: 0f32, m12: 0f32, m13: 0f32, m14: 0f32,
            m21: 0f32, m22: 0f32, m23: 0f32, m24: 0f32,
            m31: 0f32, m32: 0f32, m33: 0f32, m34: 0f32,
            m41: 0f32, m42: 0f32, m43: 0f32, m44: 0f32
        }
    }
    
    pub fn one() -> Matrix {
        Matrix {
            m11: 1f32, m12: 1f32, m13: 1f32, m14: 1f32,
            m21: 1f32, m22: 1f32, m23: 1f32, m24: 1f32,
            m31: 1f32, m32: 1f32, m33: 1f32, m34: 1f32,
            m41: 1f32, m42: 1f32, m43: 1f32, m44: 1f32
        }
    }

    pub fn translation(v: Vector3) -> Matrix {
        Matrix {
            m11: 1f32, m12: 0f32, m13: 0f32, m14: 0f32,
            m21: 0f32, m22: 1f32, m23: 0f32, m24: 0f32,
            m31: 0f32, m32: 0f32, m33: 1f32, m34: 0f32,
            m41: v.x,  m42: v.y,  m43: v.z,  m44: 1f32
        }
    }

    pub fn scale(v: Vector3) -> Matrix {
        Matrix {
            m11: v.x , m12: 0f32, m13: 0f32, m14: 0f32,
            m21: 0f32, m22: v.y , m23: 0f32, m24: 0f32,
            m31: 0f32, m32: 0f32, m33: v.z,  m34: 0f32,
            m41: 0f32, m42: 0f32, m43: 0f32, m44: 1f32
        }
    }

    pub fn rotation_x(radian: f32) -> Matrix {
        let num2 = radian.cos();
        let num  = radian.sin();
        Matrix {
            m11: 1f32, m12: 0f32, m13: 0f32, m14: 0f32,
            m21: 0f32, m22: num2, m23: num,  m24: 0f32,
            m31: 0f32, m32: -num, m33: num2, m34: 0f32,
            m41: 0f32, m42: 0f32, m43: 0f32, m44: 1f32
        }
    }

    pub fn rotation_y(radian: f32) -> Matrix {
        let num2 = radian.cos();
        let num = radian.sin();
        Matrix {
            m11: 1f32, m12: 0f32, m13: 0f32, m14: 0f32,
            m21: 0f32, m22: num2, m23: num,  m24: 0f32,
            m31: 0f32, m32: -num, m33: num2, m34: 0f32,
            m41: 0f32, m42: 0f32, m43: 0f32, m44: 1f32
        }
    }

    pub fn rotation_z(radian: f32) -> Matrix {
        let num2 = radian.cos();
        let num = radian.sin();
        Matrix {
            m11: num2, m12: 0f32, m13: -num, m14: 0f32,
            m21: 0f32, m22: 1f32, m23: 0f32, m24: 0f32,
            m31: num,  m32: 0f32, m33: num2, m34: 0f32,
            m41: 0f32, m42: 0f32, m43: 0f32, m44: 1f32            
        }
    }

    pub fn rotation_axis(axis: Vector3, radian: f32) -> Matrix {
        let x     = axis.x;
        let y     = axis.y;
        let z     = axis.z;
        let num2  = radian.sin();
        let num   = radian.cos();
        let num11 = x * x;
        let num10 = y * y;
        let num9  = z * z;
        let num8  = x * y;
        let num7  = x * z;
        let num6  = y * z;
        Matrix {
            m11: num11 + (num * (1f32 - num11)),
            m12: (num8 - (num * num8)) + (num2 * z),
            m13: (num7 - (num * num7)) - (num2 * y),
            m14: 0f32,
            m21: (num8 - (num * num8)) - (num2 * z),
            m22: num10 + (num * (1f32 - num10)),
            m23: (num6 - (num * num6)) + (num2 * x),
            m24: 0f32,
            m31: (num7 - (num * num7)) + (num2 * y),
            m32: (num6 - (num * num6)) - (num2 * x),
            m33: num9 + (num * (1f32 - num9)),
            m34: 0f32,
            m41: 0f32,
            m42: 0f32,
            m43: 0f32,
            m44: 1f32
        }
    }

    pub fn perspective(width: f32, height: f32, near_plane: f32, far_plane: f32) -> Matrix {
        Matrix {
            m11: (2f32 * near_plane) / width,
            m12: 0f32,
            m13: 0f32,
            m14: 0f32,
            m21: 0f32,
            m22: (2f32 * near_plane) / height,
            m23: 0f32,
            m24: 0f32,
            m31: 0f32,
            m32: 0f32,
            m33: far_plane / (near_plane - far_plane),
            m34: -1f32,
            m41: 0f32,
            m42: 0f32,
            m43: (near_plane * far_plane) / (near_plane - far_plane),
            m44: 0f32
        }
    }

    pub fn perspective_fov(fov: f32, aspect: f32, near_plane: f32, far_plane: f32) -> Matrix {
        let num  = 1f32 / (fov * 0.5f32).tan();
        let num9 = num / aspect;
        Matrix {
            m11: num9,
            m12: 0f32,
            m13: 0f32, 
            m14: 0f32,
            m21: 0f32,
            m22: num,
            m23: 0f32,
            m24: 0f32,
            m31: 0f32,
            m32: 0f32,
            m33: far_plane / (near_plane - far_plane),
            m34: -1f32,
            m41: 0f32,
            m42: 0f32,
            m43: (near_plane * far_plane) / (near_plane - far_plane),
            m44: 0f32
        }
    }

    pub fn perspective_off_center(left: f32, right: f32, bottom: f32, top: f32, near_plane: f32, far_plane: f32) -> Matrix {
        Matrix {
            m11: (2f32 * near_plane) / (right - left),
            m12: 0f32,
            m13: 0f32,
            m14: 0f32,
            m21: 0f32,
            m22: (2f32 * near_plane) / (top - bottom),
            m23: 0f32,
            m24: 0f32,
            m31: (left + right) / (right - left),
            m32: (top + bottom) / (top - bottom),
            m33: far_plane / (near_plane - far_plane),
            m34: -1f32,
            m41: 0f32,
            m42: 0f32,
            m43: (near_plane * far_plane) / (near_plane - far_plane),
            m44: 0f32
        }
    }

    pub fn orthographic(width: f32, height: f32, near_plane: f32, far_plane: f32) -> Matrix {
        Matrix {
            m11: 2f32 / width,
            m12: 0f32,
            m13: 0f32,
            m14: 0f32,
            m21: 0f32,
            m22: 2f32 / height,
            m23: 0f32,
            m24: 0f32,
            m31: 0f32,
            m32: 0f32,
            m33: 1f32 / (near_plane - far_plane),
            m34: 0f32,
            m41: 0f32,
            m42: 0f32,
            m43: near_plane / (near_plane - far_plane),
            m44: 1f32
        }
    }

    pub fn orthographic_off_center(left: f32, right: f32, bottom: f32, top: f32, near_plane: f32, far_plane: f32) -> Matrix {
        Matrix {
            m11: 2f32 / (right - left),
            m12: 0f32,
            m13: 0f32,
            m14: 0f32,
            m21: 0f32,
            m22: 2f32 / (top - bottom),
            m23: 0f32,
            m24: 0f32,
            m31: 0f32,
            m32: 0f32,
            m33: 1f32 / (near_plane - far_plane),
            m34: 0f32,
            m41: (left + right) / (left - right),
            m42: (top + bottom) / (bottom - top),
            m43: near_plane / (near_plane - far_plane),
            m44: 1f32
        }
    }

    pub fn look_at(position: Vector3, target: Vector3, up: Vector3) -> Matrix {
        let vector  = (position - target).normalize();
        let vector2 = Vector3::cross(up, vector).normalize();
        let vector3 = Vector3::cross(vector, vector2);
        Matrix {
            m11: vector2.x,
            m12: vector3.x,
            m13: vector.x,
            m14: 0f32,
            m21: vector2.y,
            m22: vector3.y,
            m23: vector.y,
            m24: 0f32,
            m31: vector2.z,
            m32: vector3.z,
            m33: vector.z,
            m34: 0f32,
            m41: -Vector3::dot(vector2, position),
            m42: -Vector3::dot(vector3, position),
            m43: -Vector3::dot(vector, position),
            m44: 1f32
        }
    }
    
    pub fn world(position: Vector3, forward: Vector3, up: Vector3) -> Matrix {
        let vector = (position - forward).normalize();
        let vector2 = Vector3::cross(up, vector).normalize();
        let vector3 = Vector3::cross(vector, vector2);
        Matrix {
            m11: vector2.x,
            m12: vector2.y,
            m13: vector2.z,
            m14: -Vector3::dot(vector2, position),
            m21: vector3.x,
            m22: vector3.y,
            m23: vector3.z,
            m24: -Vector3::dot(vector3, position),
            m31: vector.x,
            m32: vector.y,
            m33: vector.z,
            m34: -Vector3::dot(vector, position),
            m41: 0f32,
            m42: 0f32,
            m43: 0f32,
            m44: 0f32
        }
    }

    pub fn from_quaternion(q0: Quaternion) -> Matrix {
        let num9 = q0.x * q0.x;
        let num8 = q0.y * q0.y;
        let num7 = q0.z * q0.z;
        let num6 = q0.x * q0.y;
        let num5 = q0.z * q0.w;
        let num4 = q0.z * q0.x;
        let num3 = q0.y * q0.w;
        let num2 = q0.y * q0.z;
        let num =  q0.x * q0.w;
        Matrix {
            m11: 1f32 - (2f32 * (num8 + num7)),
            m12: 2f32 * (num6 + num5),
            m13: 2f32 * (num4 - num3),
            m14: 0f32,
            m21: 2f32 * (num6 - num5),
            m22: 1f32 - (2f32 * (num7 + num9)),
            m23: 2f32 * (num2 + num),
            m24: 0f32,
            m31: 2f32 * (num4 + num3),
            m32: 2f32 * (num2 - num),
            m33: 1f32 - (2f32 * (num8 + num9)),
            m34: 0f32,
            m41: 0f32,
            m42: 0f32,
            m43: 0f32,
            m44: 1f32           
        } 
    }

    pub fn from_yaw_pitch_roll(yaw: f32, pitch: f32, roll: f32) -> Matrix {
        Matrix::identity()
    }

    pub fn shadow(lightDirection: Vector3, plane: Plane) -> Matrix {
        Matrix::identity()
    }

    pub fn reflection(plane: Plane) -> Matrix {
        Matrix::identity()
    }

    pub fn transform(value: Matrix, rotation: Quaternion) -> Matrix {
        let num21 = rotation.x + rotation.x;
        let num11 = rotation.y + rotation.y;
        let num10 = rotation.z + rotation.z;
        let num20 = rotation.w * num21;
        let num19 = rotation.w * num11;
        let num18 = rotation.w * num10;
        let num17 = rotation.x * num21;
        let num16 = rotation.x * num11;
        let num15 = rotation.x * num10;
        let num14 = rotation.y * num11;
        let num13 = rotation.y * num10;
        let num12 = rotation.z * num10;
        let num9 = (1f32 - num14) - num12;
        let num8 = num16 - num18;
        let num7 = num15 + num19;
        let num6 = num16 + num18;
        let num5 = (1f32 - num17) - num12;
        let num4 = num13 - num20;
        let num3 = num15 - num19;
        let num2 = num13 + num20;
        let num = (1f32 - num17) - num14;
        Matrix {
            m11: ((value.m11 * num9) + (value.m12 * num8)) + (value.m13 * num7),
            m12: ((value.m11 * num6) + (value.m12 * num5)) + (value.m13 * num4),
            m13: ((value.m11 * num3) + (value.m12 * num2)) + (value.m13 * num),
            m14: value.m14,
            m21: ((value.m21 * num9) + (value.m22 * num8)) + (value.m23 * num7),
            m22: ((value.m21 * num6) + (value.m22 * num5)) + (value.m23 * num4),
            m23: ((value.m21 * num3) + (value.m22 * num2)) + (value.m23 * num),
            m24: value.m24,
            m31: ((value.m31 * num9) + (value.m32 * num8)) + (value.m33 * num7),
            m32: ((value.m31 * num6) + (value.m32 * num5)) + (value.m33 * num4),
            m33: ((value.m31 * num3) + (value.m32 * num2)) + (value.m33 * num),
            m34: value.m34,
            m41: ((value.m41 * num9) + (value.m42 * num8)) + (value.m43 * num7),
            m42: ((value.m41 * num6) + (value.m42 * num5)) + (value.m43 * num4),
            m43: ((value.m41 * num3) + (value.m42 * num2)) + (value.m43 * num),
            m44: value.m44
        }
    }

    pub fn transpose(matrix: Matrix) -> Matrix {
        Matrix {
            m11: matrix.m11, m12: matrix.m21, m13: matrix.m31, m14: matrix.m41,
            m21: matrix.m12, m22: matrix.m22, m23: matrix.m32, m24: matrix.m42,
            m31: matrix.m13, m32: matrix.m23, m33: matrix.m33, m34: matrix.m43,
            m41: matrix.m14, m42: matrix.m24, m43: matrix.m34, m44: matrix.m44
        }
    }

    pub fn determinant(matrix: Matrix) -> f32 {
        let num22 = matrix.m11;
        let num21 = matrix.m12;
        let num20 = matrix.m13;
        let num19 = matrix.m14;
        let num12 = matrix.m21;
        let num11 = matrix.m22;
        let num10 = matrix.m23;
        let num9 = matrix.m24;
        let num8 = matrix.m31;
        let num7 = matrix.m32;
        let num6 = matrix.m33;
        let num5 = matrix.m34;
        let num4 = matrix.m41;
        let num3 = matrix.m42;
        let num2 = matrix.m43;
        let num   = matrix.m44;
        let num18 = (num6 * num)  - (num5 * num2);
        let num17 = (num7 * num)  - (num5 * num3);
        let num16 = (num7 * num2) - (num6 * num3);
        let num15 = (num8 * num)  - (num5 * num4);
        let num14 = (num8 * num2) - (num6 * num4);
        let num13 = (num8 * num3) - (num7 * num4);
        ((((num22 * 
          (((num11 * num18) - (num10 * num17)) + 
           (num9 * num16))) - (num21 * (((num12 * num18) - (num10 * num15)) + 
           (num9 * num14)))) + (num20 * (((num12 * num17) - (num11 * num15)) + 
           (num9 * num13)))) - (num19 * (((num12 * num16) - (num11 * num14)) + 
           (num10 * num13))))
    }

    pub fn invert(m0: Matrix) -> Matrix {
        let num5 = m0.m11;
        let num4 = m0.m12;
        let num3 = m0.m13;
        let num2 = m0.m14;
        let num9 = m0.m21;
        let num8 = m0.m22;
        let num7 = m0.m23;
        let num6 = m0.m24;
        let num17 = m0.m31;
        let num16 = m0.m32;
        let num15 = m0.m33;
        let num14 = m0.m34;
        let num13 = m0.m41;
        let num12 = m0.m42;
        let num11 = m0.m43;
        let num10 = m0.m44;
        let num23 = (num15 * num10) - (num14 * num11);
        let num22 = (num16 * num10) - (num14 * num12);
        let num21 = (num16 * num11) - (num15 * num12);
        let num20 = (num17 * num10) - (num14 * num13);
        let num19 = (num17 * num11) - (num15 * num13);
        let num18 = (num17 * num12) - (num16 * num13);
        let num39 = ((num8 * num23) - (num7 * num22)) + (num6 * num21);
        let num38 = -(((num9 * num23) - (num7 * num20)) + (num6 * num19));
        let num37 = ((num9 * num22) - (num8 * num20)) + (num6 * num18);
        let num36 = -(((num9 * num21) - (num8 * num19)) + (num7 * num18));
        let num   = 1f32 / ((((num5 * num39) + (num4 * num38)) + (num3 * num37)) + (num2 * num36));
        let num35 = (num7 * num10) - (num6 * num11);
        let num34 = (num8 * num10) - (num6 * num12);
        let num33 = (num8 * num11) - (num7 * num12);
        let num32 = (num9 * num10) - (num6 * num13);
        let num31 = (num9 * num11) - (num7 * num13);
        let num30 = (num9 * num12) - (num8 * num13);
        let num29 = (num7 * num14) - (num6 * num15);
        let num28 = (num8 * num14) - (num6 * num16);
        let num27 = (num8 * num15) - (num7 * num16);
        let num26 = (num9 * num14) - (num6 * num17);
        let num25 = (num9 * num15) - (num7 * num17);
        let num24 = (num9 * num16) - (num8 * num17);
        Matrix {
            m11:  num39 * num,
            m21:  num38 * num,
            m31:  num37 * num,
            m41:  num36 * num,
            m12: -(((num4 * num23) - (num3 * num22)) + (num2 * num21)) * num,
            m22:  (((num5 * num23) - (num3 * num20)) + (num2 * num19)) * num,
            m32: -(((num5 * num22) - (num4 * num20)) + (num2 * num18)) * num,
            m42:  (((num5 * num21) - (num4 * num19)) + (num3 * num18)) * num,
            m13:  (((num4 * num35) - (num3 * num34)) + (num2 * num33)) * num,
            m23: -(((num5 * num35) - (num3 * num32)) + (num2 * num31)) * num,
            m33:  (((num5 * num34) - (num4 * num32)) + (num2 * num30)) * num,
            m43: -(((num5 * num33) - (num4 * num31)) + (num3 * num30)) * num,
            m14: -(((num4 * num29) - (num3 * num28)) + (num2 * num27)) * num,
            m24:  (((num5 * num29) - (num3 * num26)) + (num2 * num25)) * num,
            m34: -(((num5 * num28) - (num4 * num26)) + (num2 * num24)) * num,
            m44:  (((num5 * num27) - (num4 * num25)) + (num3 * num24)) * num
        }
    }

    pub fn lerp(m0: Matrix, m1: Matrix, amount: f32) -> Matrix {
        Matrix{
            m11: m0.m11 + ((m1.m11 - m0.m11) * amount),
            m12: m0.m12 + ((m1.m12 - m0.m12) * amount),
            m13: m0.m13 + ((m1.m13 - m0.m13) * amount),
            m14: m0.m14 + ((m1.m14 - m0.m14) * amount),
            m21: m0.m21 + ((m1.m21 - m0.m21) * amount),
            m22: m0.m22 + ((m1.m22 - m0.m22) * amount),
            m23: m0.m23 + ((m1.m23 - m0.m23) * amount),
            m24: m0.m24 + ((m1.m24 - m0.m24) * amount),
            m31: m0.m31 + ((m1.m31 - m0.m31) * amount),
            m32: m0.m32 + ((m1.m32 - m0.m32) * amount),
            m33: m0.m33 + ((m1.m33 - m0.m33) * amount),
            m34: m0.m34 + ((m1.m34 - m0.m34) * amount),
            m41: m0.m41 + ((m1.m41 - m0.m41) * amount),
            m42: m0.m42 + ((m1.m42 - m0.m42) * amount),
            m43: m0.m43 + ((m1.m43 - m0.m43) * amount),
            m44: m0.m44 + ((m1.m44 - m0.m44) * amount)
        }
    }

    pub fn negate(m0: Matrix) -> Matrix {
        Matrix {
            m11: -m0.m11, m12: -m0.m12, m13: -m0.m13, m14: -m0.m14,
            m21: -m0.m21, m22: -m0.m22, m23: -m0.m23, m24: -m0.m24,
            m31: -m0.m31, m32: -m0.m32, m33: -m0.m33, m34: -m0.m34,
            m41: -m0.m41, m42: -m0.m42, m43: -m0.m43, m44: -m0.m44
        }
    }

    pub fn mul (m0: Matrix, m1: Matrix) -> Matrix {
        Matrix {
            m11: (((m0.m11 * m1.m11) + (m0.m12 * m1.m21)) + (m0.m13 * m1.m31)) + (m0.m14 * m1.m41),
            m12: (((m0.m11 * m1.m12) + (m0.m12 * m1.m22)) + (m0.m13 * m1.m32)) + (m0.m14 * m1.m42),
            m13: (((m0.m11 * m1.m13) + (m0.m12 * m1.m23)) + (m0.m13 * m1.m33)) + (m0.m14 * m1.m43),
            m14: (((m0.m11 * m1.m14) + (m0.m12 * m1.m24)) + (m0.m13 * m1.m34)) + (m0.m14 * m1.m44),
            m21: (((m0.m21 * m1.m11) + (m0.m22 * m1.m21)) + (m0.m23 * m1.m31)) + (m0.m24 * m1.m41),
            m22: (((m0.m21 * m1.m12) + (m0.m22 * m1.m22)) + (m0.m23 * m1.m32)) + (m0.m24 * m1.m42),
            m23: (((m0.m21 * m1.m13) + (m0.m22 * m1.m23)) + (m0.m23 * m1.m33)) + (m0.m24 * m1.m43),
            m24: (((m0.m21 * m1.m14) + (m0.m22 * m1.m24)) + (m0.m23 * m1.m34)) + (m0.m24 * m1.m44),
            m31: (((m0.m31 * m1.m11) + (m0.m32 * m1.m21)) + (m0.m33 * m1.m31)) + (m0.m34 * m1.m41),
            m32: (((m0.m31 * m1.m12) + (m0.m32 * m1.m22)) + (m0.m33 * m1.m32)) + (m0.m34 * m1.m42),
            m33: (((m0.m31 * m1.m13) + (m0.m32 * m1.m23)) + (m0.m33 * m1.m33)) + (m0.m34 * m1.m43),
            m34: (((m0.m31 * m1.m14) + (m0.m32 * m1.m24)) + (m0.m33 * m1.m34)) + (m0.m34 * m1.m44),
            m41: (((m0.m41 * m1.m11) + (m0.m42 * m1.m21)) + (m0.m43 * m1.m31)) + (m0.m44 * m1.m41),
            m42: (((m0.m41 * m1.m12) + (m0.m42 * m1.m22)) + (m0.m43 * m1.m32)) + (m0.m44 * m1.m42),
            m43: (((m0.m41 * m1.m13) + (m0.m42 * m1.m23)) + (m0.m43 * m1.m33)) + (m0.m44 * m1.m43),
            m44: (((m0.m41 * m1.m14) + (m0.m42 * m1.m24)) + (m0.m43 * m1.m34)) + (m0.m44 * m1.m44)
        }
    }
}
