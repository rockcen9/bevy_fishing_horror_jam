//! A high-level way to load collections of asset handles as resources.

use std::collections::VecDeque;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ResourceHandles>();
    app.add_systems(PreUpdate, poll_and_promote_loaded_assets);
}


/// A function that inserts a loaded resource.
type InsertLoadedResourceFn = fn(&mut World, &UntypedHandle);

#[derive(Resource, Default)]
pub(crate) struct ResourceHandles {
    // Use a queue for waiting assets so they can be cycled through and moved to
    // `finished` one at a time.
    waiting: VecDeque<(UntypedHandle, InsertLoadedResourceFn)>,
    finished: Vec<UntypedHandle>,
}

impl ResourceHandles {
    /// Track an already-loaded handle so the loading screen waits for it.
    pub(crate) fn track(&mut self, handle: UntypedHandle) {
        self.waiting.push_back((handle, |_world, _handle| {}));
    }

    /// Returns true if all requested [`Asset`]s have finished loading and are available as [`Resource`]s.
    pub(crate) fn is_all_done(&self) -> bool {
        self.waiting.is_empty()
    }

    pub(crate) fn total_count(&self) -> usize {
        self.waiting.len() + self.finished.len()
    }

    pub(crate) fn finished_count(&self) -> usize {
        self.finished.len()
    }
}

fn poll_and_promote_loaded_assets(world: &mut World) {
    world.resource_scope(|world, mut resource_handles: Mut<ResourceHandles>| {
        world.resource_scope(|world, assets: Mut<AssetServer>| {
            for _ in 0..resource_handles.waiting.len() {
                let (handle, insert_fn) = resource_handles.waiting.pop_front().unwrap();
                if assets.is_loaded_with_dependencies(&handle) {
                    insert_fn(world, &handle);
                    resource_handles.finished.push(handle);
                } else {
                    resource_handles.waiting.push_back((handle, insert_fn));
                }
            }
        });
    });
}
