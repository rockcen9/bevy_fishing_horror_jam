use crate::prelude::*;

mod description_panel;
pub(crate) use description_panel::{
    DescriptionPanel, DescriptionSubPanel, DespawnDescriptionPanelEvent, SpawnDescriptionPanelEvent,
};

pub(crate) fn plugin(app: &mut App) {
    description_panel::plugin(app);
}
