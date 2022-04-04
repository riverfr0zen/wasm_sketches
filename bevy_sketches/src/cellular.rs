use crate::base::sketch;
use bevy::prelude::*;
use bevy_web_extras::prelude::*;


pub fn app() {
    let webcfg = WebExtrasCfg::default();
    let mut app = sketch(webcfg);
    app.insert_resource(ClearColor(Color::rgb(0.72, 0.81, 1.0)));
    app.run();
}
