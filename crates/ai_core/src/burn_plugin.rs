/// Backend-only: Burn wgpu device initialization.
///
/// Only compiled when the `backend` feature is active.
#[cfg(feature = "backend")]
pub mod backend {
    use bevy::prelude::*;
    use bevy::render::{
        RenderApp,
        renderer::{
            RenderAdapter, RenderAdapterInfo, RenderDevice, RenderInstance, RenderQueue,
            WgpuWrapper,
        },
    };
    use burn::backend::wgpu::{
        RuntimeOptions as BurnRuntimeOptions, WgpuDevice as BurnWgpuDevice,
        WgpuSetup as BurnWgpuSetup, init_device as init_burn_device,
    };

    /// The Burn wgpu device, initialized from Bevy's shared wgpu context.
    #[derive(Resource, Deref, DerefMut, Clone, Debug, Hash, PartialEq, Eq)]
    pub struct BurnDevice(pub BurnWgpuDevice);

    /// Bevy plugin that shares Bevy's wgpu device with Burn.
    pub struct BurnPlugin;

    impl Plugin for BurnPlugin {
        fn build(&self, _app: &mut App) {}

        fn finish(&self, app: &mut App) {
            let render_app = app
                .get_sub_app_mut(RenderApp)
                .expect("BurnPlugin: RenderApp not found");

            let bevy_adapter = render_app.world().resource::<RenderAdapter>();
            let wgpu_adapter = unwrap_wgpu_wrapper(&bevy_adapter.0);

            let bevy_device = render_app.world().resource::<RenderDevice>();
            let wgpu_device = bevy_device.wgpu_device().clone();

            let bevy_instance = render_app.world().resource::<RenderInstance>();
            let wgpu_instance = unwrap_wgpu_wrapper(&bevy_instance.0);

            let bevy_queue = render_app.world().resource::<RenderQueue>();
            let wgpu_queue = unwrap_wgpu_wrapper(&bevy_queue.0);

            let render_adapter_info = render_app.world().resource::<RenderAdapterInfo>();
            let wgpu_backend = render_adapter_info.backend;

            let wgpu_setup = BurnWgpuSetup {
                adapter: wgpu_adapter,
                device: wgpu_device,
                instance: wgpu_instance,
                queue: wgpu_queue,
                backend: wgpu_backend,
            };

            let burn_device = init_burn_device(wgpu_setup, BurnRuntimeOptions::default());
            app.insert_resource(BurnDevice(burn_device));
        }
    }

    fn unwrap_wgpu_wrapper<T: Clone>(wrapper: &WgpuWrapper<T>) -> T {
        <WgpuWrapper<T> as Clone>::clone(wrapper).into_inner()
    }
}
