# LibEFB -- A bag of flight utilities

This library provides utilities to support the flight planning and air
navigation. The lib is written in Rust and intended to be integrated
into EFB applications via one of its language binding (see
`bindings/`).

> [!NOTE]
> This library is still in the early stage of development with the
> first milestone to allow a full pre-flight planning. For more,
> checkout the roadmap.

## Getting Started

The `efb` crate provides the core library. The center piece of this
library is the _Flight Management System_ (FMS). It integrates and
interfaces with all sub-systems that are used for flight planning and
execution. The following example just shows the very basics that we
can enter into the FMS to get started:

```rust
use std::path::Path;

use efb::error::Error;
use efb::fms::FMS;
use efb::nd::InputFormat;

fn main() -> Result<(), Error> {
    let mut fms = FMS::new();

    // Read a ARINC 424 file into the navigation data (ND). You can get
    // a dataset at e.g.: https://www.openflightmaps.org
    fms.nd()
        .read_file(Path::new("arinc_ed.pc"), InputFormat::Arinc424)?;

    // Decode a route from EDDH to EDHF with winds at 20 kt from 290° and
    // cruising speed of 107 kt and an altitude of 2500 ft.
    fms.decode("29020KT N0107 A0250 EDDH DHN2 DHN1 EDHF")?;

    // Now we could define an aircraft and continue with our planning
    // but for now we'll just print the route
    println!("{}", fms.print(40)); // the line length is set to 40 character

    Ok(())
}
```

Running this example with a proper ARINC 424 file, we get the following output:

```
----------------------------------------
-- ROUTE
----------------------------------------

TO          HDG          DIST      ETE
DHN2       354°M       3.2 NM     00:02

TO          HDG          DIST      ETE
DHN1       354°M       7.5 NM     00:04

TO          HDG          DIST      ETE
EDHF       298°M      19.6 NM     00:13

DIST                             30.3 NM
ETE                                00:20
```

Following we have the same example using the C binding that can be
used to write other language bindings using their
_foreign function interface_ (FFI):

```c
#include "efb.h"

int main(int argc, char *argv[]) {
    EfbFMS *fms = efb_fms_new();

    efb_fms_nd_read_file(fms, "arinc_ed.pc", Arinc424);

    efb_fms_decode(fms, "29020KT N0107 A0250 EDDH DHN2 DHN1 EDHF");

    char *printout = efb_fms_print(fms, 40);
    printf("%s", printout);
    efb_string_free(printout);

    efb_fms_free(fms);

    return 0;
}
```

For more on a specific binding and its usage, refer to the READMEs of
the bindings.

## Roadmap

In a first stage, the library should be extended to allow proper
flight planning by adding the following feature:

- [ ] Add runway analysis to planning
- [ ] Add missions with multiple flights
- [x] Add Python bindings to allow easy scripted planning
- [x] Add measurement trait and refactor core types
- [ ] Extend printer to render HTML
- [ ] Add AIXM parser
- [ ] Add NOTAMS to FMS
- [ ] Add vertical flight profile to FMS
- [ ] Add book to outline how the FMS and planning is used from the
      user perspective

Once those feature are available and flight planning can be conducted
with this library, the goal is to move on to add feature that will be
needed in flight:

- [ ] Create concept to provide geo-referenced VFR approach charts
- [ ] Add position input to FMS
- [ ] Add airspace alerts to FMS
- [ ] Add location info to FMS
- [ ] Create concept to gather weather data and METAR
- [ ] Add initial airport information with frequencies and RWYs
- [ ] Add landing field analysis for EMER
- [ ] Add wake turbulence predictor
