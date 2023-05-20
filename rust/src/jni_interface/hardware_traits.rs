#![allow(dead_code)]

pub trait Device {
    fn get_name(&self) -> &str;
    fn get_port_number(&self) -> i32;
}

pub mod actuators {
    use super::sensors::*;
    use uom::si::f64::{Angle, ElectricCurrent};
    use crate::utils::normalize;

    #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
    pub enum Direction {
        #[default]
        Forward,
        Reverse
    }

    pub trait BasicMotor {
        fn set_direction(&mut self, direction: Direction);
        fn get_direction(&self) -> Direction;

        fn set_power(&mut self, power: f64);
        fn get_power(&self) -> f64;
    }

    #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
    pub enum ZeroPowerBehavior {
        Brake,
        #[default]
        Float
    }

    pub trait DcMotor: BasicMotor + RawEncoder {
        fn set_zero_power_behavior(&mut self, zero_power_behavior: ZeroPowerBehavior);
        fn get_zero_power_behavior(&self) -> ZeroPowerBehavior;
    }

    pub trait DcMotorEx: DcMotor {
        fn set_motor_enabled(&mut self);
        fn set_motor_disabled(&mut self);
        fn is_motor_enabled(&self) -> bool;

        fn get_current(&self) -> ElectricCurrent;
        fn get_current_alert(&self) -> ElectricCurrent;
        fn set_current_alert(&mut self, alert_level: ElectricCurrent);
        fn is_over_current(&self) -> bool;
    }

    pub trait CrServo: BasicMotor {}

    pub trait Servo {
        fn set_direction(&mut self, direction: Direction);
        fn get_direction(&self) -> Direction;

        fn set_position(&mut self, position: f64);
        fn get_position(&self) -> f64;
    }

    pub trait AdvancedServo: Servo {
        fn set_upper_bound(&mut self, upper_bound: Angle);
        fn set_lower_bound(&mut self, lower_bound: Angle);
        fn get_upper_bound(&self) -> Angle;
        fn get_lower_bound(&self) -> Angle;

        fn set_range(&mut self, lower_bound: Angle, upper_bound: Angle) {
            self.set_lower_bound(lower_bound);
            self.set_upper_bound(upper_bound);
        }
        fn get_range(&self) -> (Angle, Angle) {
            (self.get_lower_bound(), self.get_upper_bound())
        }

        fn set_angle(&mut self, mut angle: Angle) {
            let (mut lower_bound, upper_bound): (Angle, Angle) = self.get_range();
            let mut bound_distance: Angle = upper_bound - lower_bound;
            if lower_bound >= upper_bound {
                bound_distance = Angle::FULL_TURN - bound_distance;
            }
            let rotation: Angle = lower_bound + bound_distance / 2.;

            lower_bound -= rotation;
            angle -= rotation;
            angle = normalize(angle);

            let angle_distance: Angle = angle - lower_bound;
            let interpolation = (angle_distance / bound_distance).value.clamp(0., 1.);
            
            self.set_position(interpolation);
        }
        fn get_angle(&self) -> Angle {
            let (lower_bound, upper_bound): (Angle, Angle) = self.get_range();
            let mut bound_distance: Angle = upper_bound - lower_bound;
            if lower_bound >= upper_bound {
                bound_distance = Angle::FULL_TURN - bound_distance;
            }

            normalize(lower_bound + bound_distance * self.get_position())
        }
    }
}

pub mod sensors {
    use uom::si::f64::{Angle, ElectricPotential};
    use uom::si::electric_potential::volt;

    pub trait RawEncoder {
        fn get_ticks(&self) -> i64;
    }

    pub trait Encoder {
        fn get_position(&self) -> Angle;
    }

    pub trait AnalogInput {
        fn get_voltage(&self) -> ElectricPotential;
        fn get_max_voltage(&self) -> ElectricPotential {
            ElectricPotential::new::<volt>(3.3)
        }
    }

    #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
    pub enum SwitchState {
        Triggered,
        #[default]
        Clear
    }

    pub trait LimitSwitch {
        fn get_state(&self) -> SwitchState;
    }

    pub trait TouchSensor : LimitSwitch {}
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DigitalState {
    High = 1,
    #[default]
    Low = 0
}

pub trait DigitalInput {
    fn get_state(&self) -> DigitalState;

    fn set_mode_output<T: DigitalOutput>(self) -> T;
}

pub trait DigitalOutput {
    fn set_state(&self, state: DigitalState);

    fn set_mode_input<T: DigitalInput>(self) -> T;
}