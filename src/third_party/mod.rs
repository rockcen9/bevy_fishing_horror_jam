mod bevy_intl_ext;
mod bevy_sprite_layer_ext;
mod camera_box_ext;
pub(crate) fn plugin(app: &mut bevy::app::App) {
    bevy_sprite_layer_ext::plugin(app);
    bevy_intl_ext::plugin(app);
    camera_box_ext::plugin(app);
}
