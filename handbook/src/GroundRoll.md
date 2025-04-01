# Ground Roll

## POH's Takeoff and Landing Performance

The aircraft's ground roll varies from manufacturer to manufacturer
and is influenced by many different factors. Those factors and the
distances are described in many different ways in the Pilot Operating
Handbooks (POH). Some manufacturers provide plain tables or diagrams
to read the distances for different elevations, temperatures or
takeoff weights and provide other diagrams to in- or decrease the
distances based on factors like wind, runway condition or slope.

## Aircraft Configuration

### Aircraft Configuration Overview

To configure the aircraft's takeoff and landing distance as provided
by the POH, the following steps need to be followed:

1. Configure the distances for a temperature and elevation. This can
   be either done by table entries of by entering a linear or
   quadratic function that takes the temperature and elevation as
   input and returns the distance.
2. Add a list of factors that influence the ground roll as described
   in the POH. Those factors can be also configured either for
   discrete steps e.g. winds, ranges or as linear or quadratic
   functions.

### Ground Roll Factors

The following factors can be add to the list of factors and are
applied to the ground roll distance in the order as configured:

- Decrease for headwind
- Increase for tailwind
- Increase for dry grass
- Increase for wet grass
- Increase for a runway condition code
- In- or decrease for runway slope
- In- or decrease for weight if e.g. the base configuration is for
  Maximum Takeoff Weight (MTOW)

## Additional factors in Flight Planning

Besides the influencing factors described in the POH, further factors
can be defined in the flight planning. Those factors can implement
national recommendations like the _Flugsicherheitsmitteilung (FSM) 3/75_
in Germany. Those factors are add to the aircraft's influenced ground
roll again in the order of configuration.
