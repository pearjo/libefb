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

//! Flight Planning.
use crate::aircraft::Aircraft;
use crate::error::Error;
use crate::fp::*;
use crate::measurements::{Mass, Temperature};
use crate::nd::RunwayConditionCode;
use crate::route::Route;
use crate::{Fuel, Wind};

use super::{SubSystem, SubSystemBuilder};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct FlightPlanningBuilder {
    aircraft: Option<Aircraft>,
    mass: Option<Vec<Mass>>,
    policy: Option<FuelPolicy>,
    taxi: Option<Fuel>,
    reserve: Option<Reserve>,
    perf: Option<Performance>,
    takeoff_perf: Option<TakeoffLandingPerformance>,
    takeoff_factors: Option<AlteringFactors>,
    origin_rwycc: Option<RunwayConditionCode>,
    origin_wind: Option<Wind>,
    origin_temperature: Option<Temperature>,
    landing_perf: Option<TakeoffLandingPerformance>,
    landing_factors: Option<AlteringFactors>,
    destination_rwycc: Option<RunwayConditionCode>,
    destination_wind: Option<Wind>,
    destination_temperature: Option<Temperature>,
}

impl FlightPlanningBuilder {
    pub fn new() -> FlightPlanningBuilder {
        Self {
            aircraft: None,
            mass: None,
            policy: None,
            taxi: None,
            reserve: None,
            perf: None,
            takeoff_perf: None,
            takeoff_factors: None,
            origin_rwycc: None,
            origin_wind: None,
            origin_temperature: None,
            landing_perf: None,
            landing_factors: None,
            destination_rwycc: None,
            destination_wind: None,
            destination_temperature: None,
        }
    }

    pub fn aircraft(&self) -> Option<&Aircraft> {
        self.aircraft.as_ref()
    }

    pub fn set_aircraft(&mut self, aircraft: Aircraft) -> &mut FlightPlanningBuilder {
        self.aircraft = Some(aircraft);
        self
    }

    pub fn mass(&mut self) -> Option<&mut Vec<Mass>> {
        self.mass.as_mut()
    }

    pub fn set_mass(&mut self, mass: Vec<Mass>) -> &mut FlightPlanningBuilder {
        self.mass = Some(mass);
        self
    }

    pub fn policy(&self) -> Option<&FuelPolicy> {
        self.policy.as_ref()
    }

    pub fn set_policy(&mut self, policy: FuelPolicy) -> &mut FlightPlanningBuilder {
        self.policy = Some(policy);
        self
    }

    pub fn taxi(&self) -> Option<&Fuel> {
        self.taxi.as_ref()
    }

    pub fn set_taxi(&mut self, taxi: Fuel) -> &mut FlightPlanningBuilder {
        self.taxi = Some(taxi);
        self
    }

    pub fn reserve(&self) -> Option<&Reserve> {
        self.reserve.as_ref()
    }

    pub fn set_reserve(&mut self, reserve: Reserve) -> &mut FlightPlanningBuilder {
        self.reserve = Some(reserve);
        self
    }

    pub fn perf(&self) -> Option<&Performance> {
        self.perf.as_ref()
    }

    pub fn set_perf(&mut self, perf: Performance) -> &mut FlightPlanningBuilder {
        self.perf = Some(perf);
        self
    }

    pub fn takeoff_perf(&self) -> Option<&TakeoffLandingPerformance> {
        self.takeoff_perf.as_ref()
    }

    pub fn set_takeoff_perf(
        &mut self,
        perf: TakeoffLandingPerformance,
    ) -> &mut FlightPlanningBuilder {
        self.takeoff_perf = Some(perf);
        self
    }

    pub fn takeoff_factors(&self) -> Option<&AlteringFactors> {
        self.takeoff_factors.as_ref()
    }

    pub fn set_takeoff_factors(&mut self, factors: AlteringFactors) -> &mut FlightPlanningBuilder {
        self.takeoff_factors = Some(factors);
        self
    }

    pub fn origin_rwycc(&self) -> Option<&RunwayConditionCode> {
        self.origin_rwycc.as_ref()
    }

    pub fn set_origin_rwycc(&mut self, rwycc: RunwayConditionCode) -> &mut FlightPlanningBuilder {
        self.origin_rwycc = Some(rwycc);
        self
    }

    pub fn origin_wind(&self) -> Option<&Wind> {
        self.origin_wind.as_ref()
    }

    pub fn set_origin_wind(&mut self, wind: Wind) -> &mut FlightPlanningBuilder {
        self.origin_wind = Some(wind);
        self
    }

    pub fn origin_temperature(&self) -> Option<&Temperature> {
        self.origin_temperature.as_ref()
    }

    pub fn set_origin_temperature(
        &mut self,
        temperature: Temperature,
    ) -> &mut FlightPlanningBuilder {
        self.origin_temperature = Some(temperature);
        self
    }

    pub fn landing_perf(&self) -> Option<&TakeoffLandingPerformance> {
        self.landing_perf.as_ref()
    }

    pub fn set_landing_perf(
        &mut self,
        perf: TakeoffLandingPerformance,
    ) -> &mut FlightPlanningBuilder {
        self.landing_perf = Some(perf);
        self
    }

    pub fn landing_factors(&self) -> Option<&AlteringFactors> {
        self.landing_factors.as_ref()
    }

    pub fn set_landing_factors(&mut self, factors: AlteringFactors) -> &mut FlightPlanningBuilder {
        self.landing_factors = Some(factors);
        self
    }

    pub fn destination_rwycc(&self) -> Option<&RunwayConditionCode> {
        self.destination_rwycc.as_ref()
    }

    pub fn set_destination_rwycc(
        &mut self,
        rwycc: RunwayConditionCode,
    ) -> &mut FlightPlanningBuilder {
        self.destination_rwycc = Some(rwycc);
        self
    }

    pub fn destination_wind(&self) -> Option<&Wind> {
        self.destination_wind.as_ref()
    }

    pub fn set_destination_wind(&mut self, wind: Wind) -> &mut FlightPlanningBuilder {
        self.destination_wind = Some(wind);
        self
    }

    pub fn destination_temperature(&self) -> Option<&Temperature> {
        self.destination_temperature.as_ref()
    }

    pub fn set_destination_temperature(
        &mut self,
        temperature: Temperature,
    ) -> &mut FlightPlanningBuilder {
        self.destination_temperature = Some(temperature);
        self
    }
}

impl SubSystemBuilder for FlightPlanningBuilder {
    type SubSystem = FlightPlanning;

    fn build(&self, route: &Route) -> Result<FlightPlanning, Error> {
        let fuel_planning = match (
            &self.aircraft,
            &self.policy,
            self.taxi,
            &self.reserve,
            &self.perf,
        ) {
            (Some(aircraft), Some(policy), Some(taxi), Some(reserve), Some(perf)) => {
                FuelPlanning::new(aircraft, policy, taxi, route, reserve, perf)
            }
            _ => None,
        };

        let mb = match (&self.aircraft, &self.mass, &fuel_planning) {
            (Some(aircraft), Some(mass), Some(fuel_planning)) => {
                Some(aircraft.mb_from_const_mass_and_equally_distributed_fuel(
                    mass,
                    fuel_planning.on_ramp(),
                    fuel_planning.after_landing(),
                )?)
            }
            _ => None,
        };

        let takeoff_rwy_analysis: Option<RunwayAnalysis> = match (
            &route.takeoff_rwy(),
            self.origin_rwycc,
            &self
                .origin_wind
                .or(route.legs().first().and_then(|leg| leg.wind()).cloned()),
            self.origin_temperature,
            &mb,
            &self.takeoff_perf,
        ) {
            (Some(rwy), Some(rwycc), Some(wind), Some(temperature), Some(mb), Some(perf)) => {
                Some(RunwayAnalysis::takeoff(
                    rwy,
                    rwycc,
                    wind,
                    temperature,
                    mb,
                    perf,
                    self.takeoff_factors.as_ref(),
                ))
            }
            _ => None,
        };

        let landing_rwy_analysis: Option<RunwayAnalysis> = match (
            &route.landing_rwy(),
            self.destination_rwycc,
            &self
                .destination_wind
                .or(route.legs().last().and_then(|leg| leg.wind()).cloned()),
            self.destination_temperature,
            &mb,
            &self.landing_perf,
        ) {
            (Some(rwy), Some(rwycc), Some(wind), Some(temperature), Some(mb), Some(perf)) => {
                Some(RunwayAnalysis::landing(
                    rwy,
                    rwycc,
                    wind,
                    temperature,
                    mb,
                    perf,
                    self.landing_factors.as_ref(),
                ))
            }
            _ => None,
        };

        Ok(FlightPlanning {
            aircraft: self.aircraft.clone(),
            fuel_planning,
            mb,
            takeoff_rwy_analysis,
            landing_rwy_analysis,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct FlightPlanning {
    aircraft: Option<Aircraft>,
    fuel_planning: Option<FuelPlanning>,
    mb: Option<MassAndBalance>,
    takeoff_rwy_analysis: Option<RunwayAnalysis>,
    landing_rwy_analysis: Option<RunwayAnalysis>,
}

impl FlightPlanning {
    pub fn fuel_planning(&self) -> Option<&FuelPlanning> {
        self.fuel_planning.as_ref()
    }

    pub fn mb(&self) -> Option<&MassAndBalance> {
        self.mb.as_ref()
    }

    pub fn is_balanced(&self) -> Option<bool> {
        match (self.aircraft.as_ref(), self.mb.as_ref()) {
            (Some(ac), Some(mb)) => Some(ac.is_balanced(mb)),
            _ => None,
        }
    }

    pub fn takeoff_rwy_analysis(&self) -> Option<&RunwayAnalysis> {
        self.takeoff_rwy_analysis.as_ref()
    }

    pub fn landing_rwy_analysis(&self) -> Option<&RunwayAnalysis> {
        self.landing_rwy_analysis.as_ref()
    }
}

impl SubSystem for FlightPlanning {
    type Builder = FlightPlanningBuilder;
}
