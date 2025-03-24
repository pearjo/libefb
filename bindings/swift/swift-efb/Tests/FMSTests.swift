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

import EFB
import Testing

@Suite struct FMSTests {
    let fms: FMS

    private init() {
        fms = FMS()

        let data = """
            SEURP EDDHEDA        0        N N53374900E009591762E002000053                   P    MWGE    HAMBURG                       356462409
            SEURP EDHFEDA        0        N N53593300E009343600E000000082                   P    MWGE    ITZEHOE/HUNGRIGER WOLF        320782409
            """

        fms.read(data: data, format: InputFormat.arinc424)
    }

    @Test func decodeRoute() {
        fms.decode(route: "21010KT N0107 A0250 EDDH EDHF EDDH")
        let route = fms.route
        #expect(route.legs().count == 2)
    }
}
