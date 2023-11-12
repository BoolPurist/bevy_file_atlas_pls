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
/// Positive scale value which is between 0 and 1.
/// Default value: 1
pub struct PosScaleFactor(f32);

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
    /// Returns a scale at zero.
    pub fn zero() -> Self {
        Self(0.0)
    }
    /// Tries to convert `value` as [`f32`] to a valid [`PosScaleFactor`] value.
    /// # Errors
    /// Returns an error if:
    ///
    /// - `value` is negative.
    /// - `value` is greater than 1
    pub fn new(value: f32) -> Result<Self, InvalidScaleValue> {
        if value < 0. {
            Err(InvalidScaleValue::Negative(value))
        } else if value > 1. {
            Err(InvalidScaleValue::OverOne(value))
        } else {
            Ok(Self(value))
        }
    }
    /// Tries to convert `value` as [`f32`] to a valid [`PosScaleFactor`] value.
    /// If `value` is negative then it is rounded up to zero.
    ///
    /// # Errors
    /// Returns an error if:
    ///
    /// - `value` is greater than 1
    pub fn at_least_zero(value: f32) -> Result<Self, InvalidScaleValue> {
        if value < 0. {
            Ok(Self(0.))
        } else {
            Self::new(value)
        }
    }
    /// Converts `value` as [`f32`] to a valid [`PosScaleFactor`] value.
    /// If `value` is negative then it is rounded up to zero.
    /// If `value` is greater than 1 then it is rounded capped at 1.
    pub fn clamp(value: f32) -> Self {
        Self::new(value).unwrap_or_else(|error| match error {
            InvalidScaleValue::Negative(_) => Self(0.),
            InvalidScaleValue::OverOne(_) => Self(1.),
        })
    }
    /// Returns scale at 1 as the maximum.
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
/// Represents an invalid value for a positive scale value.
/// An invalid positive scale value is not between 0 and 1.
pub enum InvalidScaleValue {
    #[error("Value {0} should not be negative")]
    Negative(f32),
    #[error("Value {0} should not be over 1")]
    OverOne(f32),
}
