use crate::prelude::*;
// #[derive(Component)]
// #[require(Visibility, Name::new("MainMesh"))]
// pub struct MainMesh;

#[derive(Component, Reflect, Debug)]
#[relationship(relationship_target = MeshRoot)]
pub struct MeshOf(pub Entity);

#[derive(Component, Deref, Default, Reflect)]
#[relationship_target(relationship = MeshOf)]
pub struct MeshRoot(Vec<Entity>);

// pub(crate) fn plugin(app: &mut App) {
//     app.add_plugins(Material2dPlugin::<CustomMesh2dMaterial>::default());
//     app.add_observer(setup_mesh);
//     app.add_systems(Update, (y_sorting_system, update_effect_visuals));
// }

// fn setup_mesh(
//     trigger: On<Add, MeshUnit>,
//     q_mesh: Query<(&MeshUnit, &Faction)>,
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<CustomMesh2dMaterial>>,
//     server: Res<AssetServer>,
// ) {
//     let root = trigger.entity;
//     let Ok((mesh_actor, faction)) = q_mesh.get(root) else {
//         return;
//     };

//     let texture = server.load(mesh_actor.sprite_file.to_string());
//     let material = match faction {
//         Faction::Player => CustomMesh2dMaterial::player(texture),
//         Faction::Enemy => CustomMesh2dMaterial::enemy(texture),
//     };

//     commands
//         .entity(root)
//         .insert(ModelRoot::default())
//         .insert(MeshRoot::default());

//     commands.spawn((
//         Model,
//         ChildOf(root),
//         ModelOf(root),
//         children![(
//             MainMesh,
//             Mesh2d(meshes.add(Rectangle::new(64.0, 64.0))),
//             MeshMaterial2d(materials.add(material)),
//             Transform::default(),
//             MeshOf(root),
//         )],
//     ));
// }
