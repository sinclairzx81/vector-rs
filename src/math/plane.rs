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
use super::vector4::Vector4;
use super::quaternion::Quaternion;
use super::matrix::Matrix;

pub enum IntersectionType {
    front, 
    back,
    intersecting    
}

#[derive(Copy, Clone)]
pub struct Plane {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32
}

impl Plane {
    
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Plane {
        Plane{
            a: a, 
            b: b,
            c: c,
            d: d
        }
    }

    pub fn normalize(plane: Plane) -> Plane {
        let num2 = ((plane.a * plane.a) + (plane.b * plane.b)) + (plane.c * plane.c);
        let num = 1f32 / num2.sqrt();
        Plane {
            a: plane.a * num,
            b: plane.b * num,
            c: plane.c * num,
            d: plane.d * num
        }
    }

    pub fn dot(plane: Plane, vector: Vector4) -> f32 {
        ((((plane.a * vector.x)  + 
           (plane.b * vector.y)) + 
           (plane.c * vector.z)) + 
           (plane.d * vector.w))
    }

    pub fn dot_coordinate(plane: Plane, vector: Vector3) -> f32 {
        (((plane.a * vector.x)  + 
          (plane.b * vector.y)) + 
          (plane.c * vector.z)) + plane.d
    }

    pub fn dot_normal(plane: Plane, vector: Vector3) -> f32 {
        (((plane.a * vector.x) + 
          (plane.b * vector.y)) + 
          (plane.c * vector.z))
    }

    pub fn transform(plane: Plane, matrix: Matrix) -> Plane {
        let m = Matrix::invert(matrix);
        Plane {
            a: (((plane.a * m.m11) + (plane.b * m.m12)) + (plane.c * m.m13)) + (plane.d * m.m14),
            b: (((plane.a * m.m21) + (plane.b * m.m22)) + (plane.c * m.m23)) + (plane.d * m.m24),
            c: (((plane.a * m.m31) + (plane.b * m.m32)) + (plane.c * m.m33)) + (plane.d * m.m34),
            d: (((plane.a * m.m41) + (plane.b * m.m42)) + (plane.c * m.m43)) + (plane.d * m.m44)
        }
    }

    pub fn transform_quaternion(plane: Plane, rotation: Quaternion) -> Plane {
        let num15 = rotation.x + rotation.x;
        let num5  = rotation.y + rotation.y;
        let num   = rotation.z + rotation.z;
        let num14 = rotation.w * num15;
        let num13 = rotation.w * num5;
        let num12 = rotation.w * num;
        let num11 = rotation.x * num15;
        let num10 = rotation.x * num5;
        let num9  = rotation.x * num;
        let num8  = rotation.y * num5;
        let num7  = rotation.y * num;
        let num6  = rotation.z * num;
        let num24 = (1f32 - num8) - num6;
        let num23 = num10 - num12;
        let num22 = num9 + num13;
        let num21 = num10 + num12;
        let num20 = (1f32 - num11) - num6;
        let num19 = num7 - num14;
        let num18 = num9 - num13;
        let num17 = num7 + num14;
        let num16 = (1f32 - num11) - num8;
        let x     = plane.a;
        let y     = plane.b;
        let z     = plane.c;
        Plane {
            a: ((plane.a * num24) + (plane.b * num23)) + (plane.c * num22),
            b: ((plane.a * num21) + (plane.b * num20)) + (plane.c * num19),
            c: ((plane.a * num18) + (plane.b * num17)) + (plane.c * num16),
            d: plane.d 
        }
    }
}
