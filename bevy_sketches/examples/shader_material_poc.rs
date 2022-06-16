use bevy::prelude::*;
use bevy_sketches::base::sketch;
use bevy_web_extras::prelude::*;


pub fn main() {
    let webcfg = WebExtrasCfg {
        title: String::from("cellular"),
        match_element: Some(String::from("content")),
        match_clear_color: true,
        ..Default::default()
    };
    let mut app = sketch(webcfg);
    // app.insert_resource(ClearColor(CELL_CLEAR_CLR))
    //     .add_plugin(ShapePlugin)
    //     .add_startup_system(cell_setup)
    //     .add_system(redraw_cell)
    //     .add_system(mutate_cell.with_run_criteria(FixedTimestep::step(CELL_STEP)));
    app.insert_resource(ClearColor(Color::SALMON));

    app.run();
}
