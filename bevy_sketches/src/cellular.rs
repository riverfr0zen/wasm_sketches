use bevy::prelude::*;
use bevy_web_extras::prelude::*;


pub fn app() {
    let winsetup = WindowSetup::default();
    let mut app = web_app(winsetup);
    app.insert_resource(ClearColor(Color::rgb(0.72, 0.81, 1.0)));
    app.run();
}
