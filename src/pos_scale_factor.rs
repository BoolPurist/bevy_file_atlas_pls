use bevy::reflect::Reflect;
use derive_more::*;
use thiserror::Error;

#[cfg(feature = "bevy_inspect")]
use bevy_inspector_egui::prelude::*;

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Display, Add, AddAssign, Mul, MulAssign, Reflect,
)]
#[cfg_attr(
    feature = "bevy_inspect",
    derive(InspectorOptions),
    reflect(InspectorOptions)
)]
pub struct PosScaleFactor(pub f32);

impl From<PosScaleFactor> for f32 {
    fn from(value: PosScaleFactor) -> Self {
        value.to_f32()
    }
}

impl Default for PosScaleFactor {
    fn default() -> Self {
        Self(1.)
    }
}

impl PosScaleFactor {
    pub fn zero() -> Self {
        Self(0.0)
    }
    pub fn new(value: f32) -> Result<Self, NegativeValueError> {
        if value < 0. {
            Err(NegativeValueError::Negative(value))
        } else if value > 1. {
            Err(NegativeValueError::OverOne(value))
        } else {
            Ok(Self(value))
        }
    }
    pub fn at_least_zero(value: f32) -> Result<Self, NegativeValueError> {
        if value < 0. {
            Ok(Self(0.))
        } else {
            Self::new(value)
        }
    }
    pub fn clamp(value: f32) -> Self {
        Self::new(value).unwrap_or_else(|error| match error {
            NegativeValueError::Negative(_) => Self(0.),
            NegativeValueError::OverOne(_) => Self(1.),
        })
    }
    pub fn new_as_complete() -> Self {
        Self(1.0)
    }

    pub fn to_f32(self) -> f32 {
        self.0
    }
}

impl std::ops::Sub for PosScaleFactor {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::clamp(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign for PosScaleFactor {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

#[derive(Debug, Error)]
pub enum NegativeValueError {
    #[error("Value {0} should not be negative")]
    Negative(f32),
    #[error("Value {0} should not be over 1")]
    OverOne(f32),
}
