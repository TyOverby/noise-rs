// Copyright 2015 The Noise-rs Developers.
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

//! An example of using simplex noise

#![feature(core)]
#![feature(path)]

extern crate noise;

use noise::{open_simplex2, open_simplex3, Seed, Point2};

mod debug;

fn main() {
    debug::render_png("open_simplex2.png", &Seed::new(0), 1024, 1024, scaled_open_simplex2);
    debug::render_png("open_simplex3.png", &Seed::new(0), 1024, 1024, scaled_open_simplex3);
    println!("\nGenerated open_simplex2.png and open_simplex3.png");
}

fn scaled_open_simplex2(seed: &Seed, point: &Point2<f32>) -> f32 {
    open_simplex2(seed, &[point[0] / 16.0, point[1] / 16.0])
}

fn scaled_open_simplex3(seed: &Seed, point: &Point2<f32>) -> f32 {
    open_simplex3(seed, &[point[0] / 16.0, point[1] / 16.0, 0.0])
}
