use bevy::prelude::*;
use bevy::window::{PresentMode, WindowId};
use bevy::winit::WinitWindows;
use std::io::Cursor;
use winit::window::Icon;

pub struct BevyConfigPlugin;

impl Plugin for BevyConfigPlugin {
    fn build(&self, app: &mut App) {
        let default_plugins = DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.,
                height: 600.,
                title: "Foxtrot".to_string(), // ToDo
                canvas: Some("#bevy".to_owned()),
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        });
        #[cfg(feature = "dev")]
        let default_plugins = default_plugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        });
        app.insert_resource(Msaa { samples: 4 })
            .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
            .add_plugins(default_plugins)
            .add_startup_system(set_window_icon);
    }
}

// Sets the icon on windows and X11
fn set_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
