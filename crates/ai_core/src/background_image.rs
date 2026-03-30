/// Platform-unified camera frame resource.
///
/// With `backend` feature on native: re-exports `bevy_nokhwa::BackgroundImage`
/// so the nokhwa render pipeline works unchanged.
///
/// With `backend` feature on wasm32: defines an equivalent resource filled
/// each frame by the web camera.
///
/// Without `backend` feature (dev mode): a plain stub resource so that systems
/// depending on `BackgroundImage` compile and can be fed mock data.
#[cfg(all(feature = "backend", not(target_arch = "wasm32")))]
pub use bevy_nokhwa::BackgroundImage;

#[cfg(all(feature = "backend", target_arch = "wasm32"))]
pub use self::web_impl::BackgroundImage;

#[cfg(not(feature = "backend"))]
pub use self::mock_impl::BackgroundImage;

#[cfg(all(feature = "backend", target_arch = "wasm32"))]
mod web_impl {
    use bevy::image::Image;
    use bevy::prelude::*;
    use bevy::render::extract_resource::ExtractResource;

    #[derive(Deref, DerefMut, Default, Resource, ExtractResource, Clone)]
    pub struct BackgroundImage(pub Image);
}

#[cfg(not(feature = "backend"))]
mod mock_impl {
    use bevy::image::Image;
    use bevy::prelude::*;
    use bevy::render::extract_resource::ExtractResource;

    /// Stub resource used during development (no `backend` feature).
    /// Fill this with mock image data to drive detection pipelines without
    /// a real camera or ML runtime.
    #[derive(Deref, DerefMut, Default, Resource, ExtractResource, Clone)]
    pub struct BackgroundImage(pub Image);
}
