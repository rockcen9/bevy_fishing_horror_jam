use extol_sprite_layer::LayerIndex;

use crate::prelude::*;

#[derive(Debug, Clone, Component, Hash, PartialEq, Eq, Reflect)]
pub enum SpriteLayer {
    Background,
    Pole,
    Monster,
    Bobber,
    Backpack,
    BackpackContainer,
    // BackpackContainer is hardcoded at z=20 (children use relative z offsets up to +2)
    Item,
    Tutorial,
    RightHand,
    LeftHand,
    LoadingBar,
    ChargeArrow,
    QteDial,
    DescriptionPanel,
    VFX,
}

impl LayerIndex for SpriteLayer {
    fn as_z_coordinate(&self) -> f32 {
        match *self {
            Self::Background => -100.0,
            Self::Pole => 1.0,
            Self::Monster => 3.0,
            Self::Bobber => 5.0,
            Self::Backpack => 10.0,
            Self::BackpackContainer => 15.0,
            Self::DescriptionPanel => 20.0,
            Self::Item => 30.0,
            Self::Tutorial => 33.0,
            Self::RightHand | Self::LeftHand => 40.0,
            Self::LoadingBar => 45.0,
            Self::ChargeArrow => 47.0,
            Self::QteDial => 50.0,
            Self::VFX => 1000.0,
        }
    }
}
