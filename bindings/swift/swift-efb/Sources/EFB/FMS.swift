// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 Joe Pearson
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

import efb

public enum InputFormat {
    case arinc424
    case openAir

    func efbInputFormat() -> EfbInputFormat {
        switch self {
        case .arinc424:
            return Arinc424
        case .openAir:
            return OpenAir
        }
    }
}

public class FMS {
    let fms: OpaquePointer!

    public init() {
        fms = efb_fms_new()
    }

    public func read(data: String, format: InputFormat) {
        efb_fms_nd_read(fms, data, format.efbInputFormat())
    }

    public func readFile(path: String, format: InputFormat) {
        efb_fms_nd_read_file(fms, path, format.efbInputFormat())
    }

    public func decode(route: String) {
        efb_fms_decode(fms, route)
    }

    public func route() -> Route {
        Route(route: efb_fms_route_ref(fms))
    }

    deinit {
        efb_fms_free(fms)
    }
}
