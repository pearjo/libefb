# `efb-c` -- C Wrapper of the EFB library

This crate provides a C-ABI compatible wrapper of the `efb`
crate. Despite some core types, most types are opaque and manipulated
by functions. Thus, the implementation of the underlying Rust library
can change without effecting the C-API directly.

## Ownership

Where ever possible, the `efb` types store there data and return
either a borrowed reference `&T` or an optional reference
`Option<&T>`which both translate to `const T *` in C with the later
returning a `NULL` pointer if the option is `None`. By sticking to
this pattern, we don't need to ref, unref or free any data we pass
from Rust over the FFI boundary. Things that can be created by the
C-API are put on the heap as a Box and declare some safety rules in
the documentation.

## Example

```C
#include <stdio.h>
#include "efb.h"

int main(int argc, char *argv[]) {
    // we create a new FMS which we need to free after using it
    EfbFMS *fms = efb_fms_new();

    // here we read an ARINC 424 file we got from www.openflightmaps.org
    efb_fms_nd_read_file(fms, "arinc_ed.pc", Arinc424);

    // now we crate a simple route from Hamburg to Lübeck. The wind
    // blows with 10 knots from the east (90°) and we have a cruise
    // speed of 107 knuts and altitude of 2500 feet.
    efb_fms_decode(fms, "09010KT N0107 A0250 EDDH EDHL");

    /* now we can build our flight planning and check if we are good to go */

    // when all is done, we need to free all things we either created
    // by `_new` or referenced by `_ref`
    efb_fms_free(fms);

    return 0;
}
```
