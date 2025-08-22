# LibEFB Language Bindings

LibEFB provides language bindings to make the flight planning library
accessible from multiple programming languages. Each binding exposes
the core functionality of the `efb` library while following the idioms
and conventions of the target language.

## Available Bindings

### C Bindings (`c/`)

**Target:** Native applications, other language FFI integration

**Example Usage:**
```c
EfbFMS *fms = efb_fms_new();
efb_fms_nd_read_file(fms, "arinc_ed.pc", Arinc424);
efb_fms_decode(fms, "29020KT N0107 A0250 EDDH DHN2 DHN1 EDHF");
char *printout = efb_fms_print(fms, 40);
printf("%s", printout);
efb_string_free(printout);
efb_fms_free(fms);
```

### Python Bindings (`python/`)

**Target:** Data analysis, scripting, scientific computing

**Example Usage:**
```python
from efb import *

fms = FMS()
fms.nd_read(arinc_data, InputFormat.ARINC_424)
fms.decode("26007KT N0107 A0220 EDDH DHD HLL EDHL")
print(fms.print(40))
```

### WebAssembly Bindings (`wasm/`)

**Target:** Web applications, browser-based flight planning

**Example Usage:**
```javascript
import { FMS } from './pkg/efb_wasm.js';

const fms = new FMS();
fms.nd().read(arincData, "arinc424");
fms.decode("29020KT N0107 A0250 EDDH DHN2 DHN1 EDHF");
console.log(fms.print(40));
```

### Swift Bindings (`swift/`)

**Target:** iOS/macOS applications, Apple ecosystem

**Example Usage:**
```swift
let fms = FMS()
fms.read(data: arincData, format: .arinc424)
fms.decode(route: "29020KT N0107 A0250 EDDH DHN2 DHN1 EDHF")
```

## Common Functionality

All bindings expose the following core capabilities:

### Flight Management System (FMS)
- **Navigation Data**: Load ARINC 424 and OpenAir format files
- **Route Decoding**: Parse route strings with wind, speed, and altitude
- **Flight Planning**: Generate flight plans with fuel calculations

### Aircraft Configuration
- **Mass & Balance**: Define aircraft with stations, fuel tanks, and CG envelope
- **Performance Data**: Configure engine performance at different altitudes
- **Fuel Planning**: Calculate required fuel with reserves and taxi allowances

## Build Instructions

Each binding contains specific build instructions and examples. Generally:

- **C**: `cargo build` generates static library and headers
- **Python**: `maturin develop` for development, `maturin build` for wheel
- **WASM**: `wasm-pack build` generates JavaScript/TypeScript packages
- **Swift**: `swift build` or open Package.swift in Xcode

## Design Philosophy

The bindings follow these principles:

- **Close to Core**: Maintain API similarity across languages while respecting language idioms
- **State Management**: Applications handle state; bindings provide stateless operations where possible
- **Memory Safety**: Proper resource management with language-appropriate patterns
- **Type Safety**: Leverage each language's type system for measurement units
