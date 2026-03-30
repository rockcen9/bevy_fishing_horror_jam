use crate::prelude::*;
#[derive(Component, Default)]
#[require(ModelRoot, Transform, Visibility)]
pub struct Actor;

#[derive(Component, Reflect, Debug)]
#[relationship(relationship_target = ModelRoot)]
pub struct ModelOf(pub Entity);

#[derive(Component, Deref, Default, Reflect)]
#[relationship_target(relationship = ModelOf)]
pub struct ModelRoot(Vec<Entity>);

#[derive(Component)]
#[require(Transform, Visibility, Name::new("Model"))]
pub struct Model;
