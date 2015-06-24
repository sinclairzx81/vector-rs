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

mod vector2;
mod vector3;
mod vector4;
mod quaternion;
mod matrix;
mod plane;
mod bounding_box;
mod bounding_sphere;
mod bounding_frustum;

pub use self::vector2::Vector2;
pub use self::vector3::Vector3;
pub use self::vector4::Vector4;
pub use self::quaternion::Quaternion;
pub use self::matrix::Matrix;
pub use self::plane::Plane;
pub use self::bounding_box::BoundingBox;
pub use self::bounding_sphere::BoundingSphere;
pub use self::bounding_frustum::BoundingFrustum;
