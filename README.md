# LibEFB -- A bag of flight utilities

This library provides utilities to support the flight planning and air
navigation. The lib is written in Rust and provides bindings to other
languages (see `bindings/`).

## Getting Started

The `efb` crate provides the core library. The center piece of this
library is the _Flight Management System_ (FMS). It integrates and
interfaces with all sub-systems that are used for flight planning and
execution. The following example just shows the very basics that we
can enter into the FMS to get started:

```rust
use efb::error::Error;
use efb::fms::FMS;
use efb::nd::InputFormat;

fn main() -> Result<(), Error> {
    let mut fms = FMS::new();

    // Read a ARINC 424 file into the navigation data (ND). You can get
    // a dataset at e.g.: https://www.openflightmaps.org
    fms.nd().read("arinc_ed.pc", InputFormat::Arinc424)?;

    // Decode a route from EDDH to EDHF with winds at 20 kt from 290° and
    // cruising speed of 107 kt and an altitude of 2500 ft.
    fms.decode("29020KT N0107 A0250 EDDH DHN2 DHN1 EDHF")?;

    Ok(())
}
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

    efb_fms_free(fms);

    return 0;
}
```

For more on a specific binding and its usage, refer to the READMEs of
the bindings.
