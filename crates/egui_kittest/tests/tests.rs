use egui::Modifiers;
use egui_kittest::Harness;
use kittest::{Key, Queryable};

#[test]
fn test_shrink() {
    let mut harness = Harness::new_ui(|ui| {
        ui.label("Hello, world!");
        ui.separator();
        ui.label("This is a test");
    });

    harness.fit_contents();

    #[cfg(all(feature = "snapshot", feature = "wgpu"))]
    harness.snapshot("test_shrink");
}

#[test]
fn test_modifiers() {
    #[derive(Default)]
    struct State {
        cmd_clicked: bool,
        cmd_z_pressed: bool,
    }
    let mut harness = Harness::new_ui_state(
        |ui, state| {
            if ui.button("Click me").clicked() && ui.input(|i| i.modifiers.command) {
                state.cmd_clicked = true;
            }
            if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::Z)) {
                state.cmd_z_pressed = true;
            }
        },
        State::default(),
    );

    harness.get_by_label("Click me").key_down(Key::Command);
    // This run isn't necessary, but allows us to test whether modifiers are remembered between frames
    harness.run();
    harness.get_by_label("Click me").click();
    // TODO(lucasmerlin): Right now the key_up needs to happen on a separate frame or it won't register.
    // This should be more intuitive
    harness.run();
    harness.get_by_label("Click me").key_up(Key::Command);

    harness.run();

    harness.press_key_modifiers(Modifiers::COMMAND, egui::Key::Z);
    // TODO(lucasmerlin): This should also work (Same problem as above)
    // harness.node().key_combination(&[Key::Command, Key::Z]);

    let state = harness.state();
    assert!(state.cmd_clicked, "The button wasn't command-clicked");
    assert!(state.cmd_z_pressed, "Cmd+Z wasn't pressed");
}
