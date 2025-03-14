.. _userguide:

User Guide
==========

Getting Started
---------------

The get started, we plan a very short flight from Hamburg ``EDDH`` to Lübeck
``EDHL``. First, we load the :py:mod:`efb` module::

  >>> from efb import *

Now we need to instantiate our Flight Management System :py:class:`FMS
<efb.FMS>` and load some navigation data. A good source to download the latest
data in ARINC 424 format is `open flightmaps
<https://www.openflightmaps.org/>`_::

  >>> fms = FMS()
  >>> # read the ARINC 424 data we downloaded
  >>> fms.nd_read_file("/path/to/arinc_ed.pc", InputFormat.ARINC_424)

Entering a Route
----------------

With the navigation data loaded into the FMS we can enter the route. For details
on all input possibilities, please refer to the Rust documentation of this
library.

1. We start by adding the wind in Hamburg with 7 knots from 260° in the format
   we get from the METAR ``26007KT``.
2. We want to cruise at 2200 feet at a speed of 107 knots. Speed and level are
   entered in the format used in the ICAO flight plan.
3. We leave Hamburg's CTR via the Delta routing. Visual reporting points are
   identified by the last two character of the airport's ICAO code (omitting the
   region identified) followed by the reporting point identifier e.g. ``D`` for
   Delta or ``N2`` for November 2.
4. We plan here to enter the CTR of Lübeck via Lima.

This gives use the following route to enter into the FMS::

  >>> fms.decode("26007KT N0107 A0220 EDDH DHD HLL EDHL")

To check the plausibility of the entered we can print it::

  >>> print(fms.print(40))
  ----------------------------------------
  -- ROUTE
  ----------------------------------------

  TO          HDG          DIST      ETE
  DHD        124°M       7.4 NM     00:04

  TO          HDG          DIST      ETE
  HLL        060°M      18.5 NM     00:10

  TO          HDG          DIST      ETE
  EDHL       016°M       7.8 NM     00:04

  DIST                             33.7 NM
  ETE                                00:18


Defining an Aircraft
--------------------

Defining an aircraft is straight forward by entering the data you get from your
pilot's operating handbook (POH)::

  >>> d_eabc = Aircraft(
  ...     registration="D-EABC",
  ...     # the stations where payload can be loaded to
  ...     stations=[
  ...         Station(Meter(0.94), "the front seats"),
  ...         Station(Meter(1.85), "the back seats"),
  ...         Station(Meter(2.41), "the first cargo compartment"),
  ...         Station(Meter(3.12), "the second cargo compartment")
  ...     ],
  ...     empty_mass=Kilogram(807.0),
  ...     empty_balance=Meter(1.0),
  ...     fuel_type=FuelType.DIESEL,
  ...     # the wing tanks are combined as one
  ...     tanks=[FuelTank(Liter(168.8), Meter(1.22))],
  ...     # this defines the limits of our Center of Gravity envelope
  ...     cg_envelope=CGEnvelope([
  ...         (Kilogram(0.0), Meter(0.89)),
  ...         (Kilogram(885.0), Meter(0.89)),
  ...         (Kilogram(1111.0), Meter(1.02)),
  ...         (Kilogram(1111.0), Meter(1.20)),
  ...         (Kilogram(0.0), Meter(1.20))
  ...     ]),
  ...     notes=""
  ... )

Build a Flight Planning
-----------------------

The FMS builds a flight planning for the entered route by using a
:py:class:`FlightPlanningBuilder <efb.FlightPlanningBuilder>`.

The builder takes an aircraft, defines how the aircraft's stations are loaded
and sets our fuel constrains::

  >>> flight_planning = FlightPlanningBuilder(
  ...     aircraft=d_eabc,
  ...     # we're sitting alone in the front seat
  ...     mass=[Kilogram(80), Kilogram(0), Kilogram(0), Kilogram(0)],
  ...     policy=ManualFuel(Diesel(Liter(80))),
  ...     # for taxiing we add a buffer and plan with 10 liters
  ...     taxi=Diesel(Liter(10)),
  ...     # we want a fuel reserve of 30 minutes
  ...     reserve=ManualReserve(Duration(0, 30, 0)),
  ...     perf=Performance([
  ...         # this is a very incomplete performance table
  ...         (Altitude(1000), Knots(107), PerHour(Diesel(Liter(21))))
  ...     ]),
  ... )

Now all left to do is to add the builder to the FMS and print our plan::

  >>> fms.build_flight_planning(flight_planning)
  >>> print(fms.print(40))
  ----------------------------------------
  -- ROUTE
  ----------------------------------------

  TO          HDG          DIST      ETE
  DHD        124°M       7.4 NM     00:04

  TO          HDG          DIST      ETE
  HLL        060°M      18.5 NM     00:10

  TO          HDG          DIST      ETE
  EDHL       016°M       7.8 NM     00:04

  DIST                             33.7 NM
  ETE                                00:18

  ----------------------------------------
  -- FUEL
  ----------------------------------------

  TRIP                                 6 L
  TAXI                                10 L
  RESERVE                             10 L
  MINIMUM                             27 L
  EXTRA                               53 L
  TOTAL                               80 L

  ----------------------------------------
  -- MASS & BALANCE
  ----------------------------------------

                    MASS       BALANCE
        ON RAMP    954 kg       1.0 m
  AFTER LANDING    940 kg       1.0 m

  BALANCED                             YES
