#[cfg(feature = "backend")]
mod detect;
mod idle_to_backward;
mod left_right;
#[cfg(not(feature = "backend"))]
mod mock_detect;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    #[cfg(feature = "backend")]
    detect::plugin(app);
    #[cfg(not(feature = "backend"))]
    mock_detect::plugin(app);
    left_right::plugin(app);
    idle_to_backward::plugin(app);
    // app.add_systems(Update, on_casting_sfx_event.in_set(SpineSet::OnEvent));
}

// fn on_casting_sfx_event(mut spine_events: MessageReader<SpineEvent>, mut commands: Commands) {
//     for event in spine_events.read() {
//         if let SpineEvent::Event { name, .. } = event {
//             if name == "forward_sfx" {
//                 commands.trigger(SFXEvent::new("casting"));
//             }
//         }
//     }
// }
