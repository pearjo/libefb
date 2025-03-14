# LibEFB Python Binding

This package provides a native Python binding to the LibEFB. It
intends to provide the basics to conduct a scripted flight planning.

## Installation

To install the binding into your active Python environment simply run:

```
pip install .
```

To build the documentation, change into the `docs` directory, install
the requirements and build it via make:

```sh
cd docs
pip install -r requirements.txt
make html
```

## Usage

With the package installed, we can define a simple route:

```python
import efb

fms = efb.FMS()

# Read a ARINC 424 file into the navigation data (ND). You can get
# a dataset at e.g.: https://www.openflightmaps.org
fms.nd_read_file("arinc_ed.pc", efb.InputFormat.ARINC_424)

# Decode a route from EDDH to EDHF with winds at 20 kt from 290° and
# cruising speed of 107 kt and an altitude of 2500 ft.
fms.decode("29020KT N0107 A0250 EDDH DHN2 DHN1 EDHF")

# Now we could define an aircraft and continue with our planning
# but for now we'll just print the route
print(fms.print(40))  # the line length is set to 40 character
```

For more, checkout the user guide and API reference.
