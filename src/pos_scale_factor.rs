use bevy::reflect::Reflect;
use derive_more::*;
use thiserror::Error;

#[cfg(feature = "bevy_inspect")]
use bevy_inspector_egui::prelude::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Into,
    Display,
    Add,
    AddAssign,
    Mul,
    MulAssign,
    Reflect,
)]
#[cfg_attr(
    feature = "bevy_inspect",
    derive(InspectorOptions),
    reflect(InspectorOptions)
)]
pub struct PosScaleFactor(f32);

impl Default for PosScaleFactor {
    fn default() -> Self {
        Self(1.)
    }
}

impl PosScaleFactor {
    pub fn new(value: f32) -> Result<Self, NegativeValueError> {
        if value < 0. {
            Err(NegativeValueError(value))
        } else {
            Ok(Self(value))
        }
    }
    pub fn at_least_zero(value: f32) -> Self {
        if value < 0. {
            Self(0.)
        } else {
            Self(value)
        }
    }

    pub fn to_f32(self) -> f32 {
        self.0
    }
}

impl std::ops::Sub for PosScaleFactor {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::at_least_zero(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign for PosScaleFactor {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

#[derive(Debug, Error)]
#[error("Value {0} should not be negative")]
pub struct NegativeValueError(f32);
