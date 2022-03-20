use crate::base::prelude::*;
use bevy::prelude::*;


pub fn app() {
    let winsetup = WindowSetup {
        clear_color: Color::rgb(0.72, 0.81, 1.0),
        ..Default::default()
    };
    let mut app = sketch_factory(winsetup);
    app.run();
}
