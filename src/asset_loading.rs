//! Loads all game assets from `asset_manifest.json` at startup.
//!
//! Run `cargo run -p asset_scanner` first to regenerate the manifest
//! whenever assets are added or removed.

use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use serde::Deserialize;

use bevy::platform::collections::HashMap;

#[cfg(feature = "backend")]
use bevy_kira_audio::AudioSource;

use crate::asset_tracking::ResourceHandles;

const MANIFEST_PATH: &str = "asset_manifest.json";

pub fn plugin(app: &mut App) {
    app.add_plugins(JsonAssetPlugin::<AssetManifest>::new(&[
        "asset_manifest.json",
    ]));
    app.add_plugins(JsonAssetPlugin::<GenericJsonAsset>::new(&["json"]));
    app.init_resource::<GameAssets>();
    app.add_systems(Startup, start_loading_asset_manifest);
    app.add_systems(
        Update,
        start_loading_assets.run_if(not(resource_exists::<AssetManifestLoaded>)),
    );
}

// ── Universal JSON asset ──────────────────────────────────────────────────────

/// Generic JSON asset — used to preload any `.json` file without needing its
/// specific typed loader. The typed loaders (e.g. `RawItemDataList`) load the
/// same path as their own type separately; this handle just keeps the bytes
/// resident so there is no disk read stutter at gameplay time.
#[derive(Deserialize, Asset, TypePath)]
pub struct GenericJsonAsset(#[allow(dead_code)] serde_json::Value);

// ── Manifest ─────────────────────────────────────────────────────────────────

#[derive(Deserialize, Asset, TypePath, Debug, Clone)]
pub struct AssetManifest {
    pub images: Vec<String>,
    #[cfg_attr(not(feature = "backend"), allow(dead_code))]
    pub audio: Vec<String>,
    pub fonts: Vec<String>,
    pub shaders: Vec<String>,
    pub data: Vec<String>,
}

#[derive(Resource)]
struct AssetManifestHandle(Handle<AssetManifest>);

/// Marker resource inserted once the manifest has been processed.
#[derive(Resource)]
pub struct AssetManifestLoaded;

// ── Game Assets resource ──────────────────────────────────────────────────────

/// Holds preloaded asset handles. Keeping handles alive prevents Bevy from
/// unloading the assets from memory.
#[derive(Resource, Default)]
pub struct GameAssets {
    pub images: HashMap<String, Handle<Image>>,
    pub fonts: HashMap<String, Handle<Font>>,
    #[cfg(feature = "backend")]
    pub audio: HashMap<String, Handle<AudioSource>>,
    shaders: Vec<UntypedHandle>,
    json: Vec<Handle<GenericJsonAsset>>,
}

// ── Systems ───────────────────────────────────────────────────────────────────

fn start_loading_asset_manifest(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut resource_handles: ResMut<ResourceHandles>,
) {
    let handle: Handle<AssetManifest> = asset_server.load(MANIFEST_PATH);
    resource_handles.track(handle.clone().untyped());
    commands.insert_resource(AssetManifestHandle(handle));
}

fn start_loading_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    handle: Option<Res<AssetManifestHandle>>,
    manifests: Res<Assets<AssetManifest>>,
    mut game_assets: ResMut<GameAssets>,
    mut resource_handles: ResMut<ResourceHandles>,
) {
    let Some(handle) = handle else { return };
    let Some(manifest) = manifests.get(&handle.0) else {
        return;
    };

    for path in &manifest.images {
        let h: Handle<Image> = asset_server.load(path.clone());
        resource_handles.track(h.clone().untyped());
        game_assets.images.insert(path.clone(), h);
    }

    for path in &manifest.fonts {
        let h: Handle<Font> = asset_server.load(path.clone());
        resource_handles.track(h.clone().untyped());
        game_assets.fonts.insert(path.clone(), h);
    }

    #[cfg(feature = "backend")]
    for path in &manifest.audio {
        let h: Handle<AudioSource> = asset_server.load(path.clone());
        resource_handles.track(h.clone().untyped());
        game_assets.audio.insert(path.clone(), h);
    }

    for path in &manifest.shaders {
        let h = asset_server.load_untyped(path.clone());
        resource_handles.track(h.clone().untyped());
        game_assets.shaders.push(h.untyped());
    }

    for path in &manifest.data {
        let h: Handle<GenericJsonAsset> = asset_server.load(path.clone());
        resource_handles.track(h.clone().untyped());
        game_assets.json.push(h);
    }

    #[cfg(feature = "backend")]
    let audio_count = manifest.audio.len();
    #[cfg(not(feature = "backend"))]
    let audio_count = 0usize;

    let total = manifest.images.len()
        + audio_count
        + manifest.fonts.len()
        + manifest.shaders.len()
        + manifest.data.len();
    info!("GameAssets: preloading {total} assets from manifest");

    commands.remove_resource::<AssetManifestHandle>();
    commands.insert_resource(AssetManifestLoaded);
}
