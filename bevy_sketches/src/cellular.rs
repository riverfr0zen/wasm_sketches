use super::base;
use bevy::prelude::*;


pub fn app() {
    let winsetup = base::WindowSetup {
        clear_color: Color::rgb(0.72, 0.81, 1.0),
        ..Default::default()
    };
    let mut app = base::sketch_factory(winsetup);
    app.run();
}
