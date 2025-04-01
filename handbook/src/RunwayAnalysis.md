# Runway Analysis

Am analysis is run for each runway of the origin and destination
airport. The analysis considers different penalties. Currently, only
recommendations base on the _Flugsicherheitsmitteilung FSM 3/75_ of
the German LBA are implemented.

The analysis is done either for a takeoff or landing runway and has
the following result:

- The wind direction for each runway
- The head- and crosswind speed
- The ground roll and total distance over 50 ft obstacle
- The remaining runway available if any

## Penalties based on FSM 3/75

The analysis considers the following penalties:

- Pressure altitude of the aerodrome
- Temperature at aerodrome
- Slope of runway
- Runway surface
- Runway condition
