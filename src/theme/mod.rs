//! Reusable UI widgets & theming.

// Unused utilities may trigger this lints undesirably.
#![allow(dead_code)]

pub(crate) mod interaction;
pub(crate) mod palette;
mod widgets;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use super::{
        interaction::InteractionPalette,
        palette as ui_palette,
        widgets::{Containers as _, Widgets as _},
    };
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}
