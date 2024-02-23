use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_pancam::PanCam;
use bevy_pancam::PanCamPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PanCamPlugin, EguiPlugin))
        .init_resource::<InputString>()
        .add_systems(Startup, setup)
        .add_systems(Update, egui_window)
        .run();
}

#[derive(Default, Resource)]
struct InputString {
    input_string: String,
}

fn egui_window(mut contexts: EguiContexts, mut input_string: ResMut<InputString>) {
    egui::Window::new("Transcriber").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Enter a Phrase:");
            //Take in user text input
            let re = ui.text_edit_singleline(&mut input_string.input_string);
            if re.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {}
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.set_min_height(25.0);
            if ui.button("Reset").clicked() {}
        });
    });
}

const X_EXTENT: f32 = 600.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());

    let shapes = [
        Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
        Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        ))),
    ];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT to +X_EXTENT.
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                0.0,
                0.0,
            ),
            ..default()
        });
    }
}
