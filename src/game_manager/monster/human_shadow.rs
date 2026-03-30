use crate::prelude::*;

/// Fixed Y position of the shadow (near the bottom of the screen).
const SHADOW_Y: f32 = -(crate::GAME_HEIGHT / 2.0) + 100.0;

/// Width/height of the shadow sprite in pixels.
const SHADOW_SIZE: Vec2 = Vec2::new(600.0, 300.0);

#[derive(Component)]
struct BottomShadow;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Monster), spawn_shadow);
    app.add_systems(OnExit(GameState::Monster), despawn_shadow);
    app.add_systems(Update, track_head_y.run_if(in_state(GameState::Monster)));
    app.add_systems(
        Update,
        warn_shadow_wrong_side.run_if(in_state(MonsterState::Shadow)),
    );
    app.add_systems(OnExit(MonsterState::Shadow), reset_shadow_color);
}

fn spawn_shadow(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("textures/shadow.png");
    let material = materials.add(ColorMaterial {
        texture: Some(texture),
        ..default()
    });
    let mesh = meshes.add(Rectangle::new(SHADOW_SIZE.x, SHADOW_SIZE.y));

    commands.spawn((
        Name::new("Bottom Shadow"),
        BottomShadow,
        SpriteLayer::Monster,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, SHADOW_Y, 0.0),
    ));
}

fn despawn_shadow(query: Query<Entity, With<BottomShadow>>, mut commands: Commands) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn warn_shadow_wrong_side(
    head: Res<PlayerHeadPosition>,
    side: Option<Res<super::charge_arrow::ChargeArrowSide>>,
    query: Query<&MeshMaterial2d<ColorMaterial>, With<BottomShadow>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    palette: Res<crate::theme::palette::ColorPalette>,
) {
    let Some(side) = side else { return };
    let Ok(mat_handle) = query.single() else {
        return;
    };
    let Some(mut mat) = materials.get_mut(&mat_handle.0) else {
        return;
    };

    let head_is_right = head.position.x >= 0.0;
    let arrow_is_right = side.0;

    if head_is_right != arrow_is_right {
        mat.color = palette.rust_red;
    } else {
        mat.color = Color::WHITE;
    }
}

fn reset_shadow_color(
    query: Query<&MeshMaterial2d<ColorMaterial>, With<BottomShadow>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok(mat_handle) = query.single() else {
        return;
    };
    let Some(mut mat) = materials.get_mut(&mat_handle.0) else {
        return;
    };
    mat.color = Color::WHITE;
}

fn track_head_y(
    head: Res<PlayerHeadPosition>,
    mut query: Query<&mut Transform, With<BottomShadow>>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };
    transform.translation.x = head.position.x;
}
