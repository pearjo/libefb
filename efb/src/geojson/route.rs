// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Joe Pearson
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

use geojson::{Feature, GeoJson, Geometry, Value};

use crate::geom::{BBox, Coordinate};
use crate::nd::Fix;
use crate::route::Route;

impl Route {
    /// Returns the route's legs as GeoJSON with a line string geometry.
    #[cfg_attr(docsrs, doc(cfg(feature = "geojson")))]
    pub fn to_geojson(&self) -> GeoJson {
        let legs = self.legs();
        let mut coords: Vec<Coordinate> = Vec::with_capacity(legs.len());
        let mut line_string: Vec<Vec<f64>> = Vec::with_capacity(legs.len());

        if let Some(origin) = legs.first() {
            let coord = origin.from().coordinate();
            coords.push(coord);
            line_string.push(vec![coord.longitude as f64, coord.latitude as f64])
        }

        for leg in legs {
            let coord = leg.to().coordinate();
            coords.push(coord);
            line_string.push(vec![coord.longitude as f64, coord.latitude as f64])
        }

        let geometry = Geometry::new(Value::LineString(line_string));

        GeoJson::Feature(Feature {
            bbox: BBox::new(&coords).map(|bbox| bbox.into()),
            geometry: Some(geometry),
            id: None,
            properties: None,
            foreign_members: None,
        })
    }
}
