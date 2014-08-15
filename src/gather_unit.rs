// Robigo Luculenta -- Proof of concept spectral path tracer in Rust
// Copyright (C) 2014 Ruud van Asseldonk
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

pub struct GatherUnit {
    /// The buffer of tristimulus values.
    pub tristimulus_buffer: Vec<f32>
}

impl GatherUnit {
    /// Constructs a new GatherUnit that will gather a canvas
    /// of the specified size.
    pub fn new(width: uint, height: uint) -> GatherUnit {
        GatherUnit {
            tristimulus_buffer: Vec::from_elem(width * height * 3, 0.0)
        }
    }
}