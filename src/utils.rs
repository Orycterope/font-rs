// Copyright 2018 Google Inc. All rights reserved.
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

#[inline]
#[cfg(not(feature = "std"))]
pub fn abs(x: f32) -> f32 {
    unsafe { core::intrinsics::fabsf32(x) }
}

#[inline]
#[cfg(feature = "std")]
pub fn abs(x: f32) -> f32 {
    x.abs()
}

#[inline]
#[cfg(all(target_env = "msvc", not(feature = "std")))]
fn ceil64(x: f64) -> f64 {
    unsafe { core::intrinsics::ceilf64(x) }
}

#[inline]
#[cfg(not(feature = "std"))]
pub fn ceil(x: f32) -> f32 {
    // see notes above in `floor`
    // TODO
    #[cfg(target_env = "msvc")]
    return ceil64(x as f64) as f32;
    #[cfg(not(target_env = "msvc"))]
    return unsafe { core::intrinsics::ceilf32(x) };
}

#[inline]
#[cfg(feature = "std")]
pub fn ceil(x: f32) -> f32 {
    x.ceil()
}

#[inline]
#[cfg(all(target_env = "msvc", not(feature = "std")))]
fn floor64(x: f64) -> f64 {
    unsafe { core::intrinsics::floorf64(x) }
}

#[inline]
#[cfg(not(feature = "std"))]
pub fn floor(x: f32) -> f32 {
    // On MSVC LLVM will lower many math intrinsics to a call to the
    // corresponding function. On MSVC, however, many of these functions
    // aren't actually available as symbols to call, but rather they are all
    // `static inline` functions in header files. This means that from a C
    // perspective it's "compatible", but not so much from an ABI
    // perspective (which we're worried about).
    //
    // The inline header functions always just cast to a f64 and do their
    // operation, so we do that here as well, but only for MSVC targets.
    //
    // Note that there are many MSVC-specific float operations which
    // redirect to this comment, so `floorf` is just one case of a missing
    // function on MSVC, but there are many others elsewhere.
    #[cfg(target_env = "msvc")]
    return floor64(x as f64) as f32;
    #[cfg(not(target_env = "msvc"))]
    return unsafe { core::intrinsics::floorf32(x) };
}

#[inline]
#[cfg(feature = "std")]
pub fn floor(x: f32) -> f32 {
    x.floor()
}

#[inline]
#[cfg(not(feature = "std"))]
pub fn sqrt(x: f32) -> f32 {
    if x < 0.0 {
        core::f32::NAN
    } else {
        unsafe { core::intrinsics::sqrtf32(x) }
    }
}

#[inline]
#[cfg(feature = "std")]
pub fn sqrt(x: f32) -> f32 {
    x.sqrt()
}
