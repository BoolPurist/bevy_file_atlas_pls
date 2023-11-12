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
    /// Returns an error if the `value` is negative.
    pub fn new(value: f32) -> Result<Self, InvalidPosScaleValue> {
        if value < 0. {
            Err(InvalidPosScaleValue(value))
        } else {
            Ok(Self(value))
        }
    }
    /// Tries to convert `value` as [`f32`] to a valid [`PosScaleFactor`] value.
    /// If `value` is negative then it is rounded up to zero.
    pub fn at_least_zero(value: f32) -> Self {
        if value < 0. {
            Self(0.)
        } else {
            Self(value)
        }
    }
    /// Converts `value` as [`f32`] to a valid [`PosScaleFactor`] value.
    /// If `value` is negative then it is rounded up to zero.
    pub fn clamp(value: f32) -> Self {
        Self::new(value).unwrap_or(Self(0.))
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
#[error("Value {0} should not be negative")]
pub struct InvalidPosScaleValue(f32);
