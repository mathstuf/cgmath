// Copyright 2013 The CGMath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use array::Array;
use matrix::{Matrix2, Matrix3, Matrix4};
use point::{Point2, Point3};
use vector::{Vector2, Vector3, Vector4};

pub trait Ptr<T> {
    fn ptr<'a>(&'a self) -> &'a T;
}

impl<S: Clone> Ptr<S> for Vector2<S> { #[inline] fn ptr<'a>(&'a self) -> &'a S { self.i(0) } }
impl<S: Clone> Ptr<S> for Vector3<S> { #[inline] fn ptr<'a>(&'a self) -> &'a S { self.i(0) } }
impl<S: Clone> Ptr<S> for Vector4<S> { #[inline] fn ptr<'a>(&'a self) -> &'a S { self.i(0) } }

impl<S: Clone> Ptr<S> for Point2<S> { #[inline] fn ptr<'a>(&'a self) -> &'a S { self.i(0) } }
impl<S: Clone> Ptr<S> for Point3<S> { #[inline] fn ptr<'a>(&'a self) -> &'a S { self.i(0) } }

impl<S: Clone> Ptr<S> for Matrix2<S> { #[inline] fn ptr<'a>(&'a self) -> &'a S { self.i(0).i(0) } }
impl<S: Clone> Ptr<S> for Matrix3<S> { #[inline] fn ptr<'a>(&'a self) -> &'a S { self.i(0).i(0) } }
impl<S: Clone> Ptr<S> for Matrix4<S> { #[inline] fn ptr<'a>(&'a self) -> &'a S { self.i(0).i(0) } }

impl<'a, T, P: Ptr<T>> Ptr<T> for &'a [P] {
    #[inline] fn ptr<'a>(&'a self) -> &'a T { self[0].ptr() }
}
