use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

// This is a simple example of a camera that flies around.
// There's an included example of a system that toggles the "enabled"
// property of the fly camera with "T"

fn init(
	commands: &mut Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands
		.spawn(LightBundle {
			transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
			..Default::default()
		})
		.spawn(Camera3dBundle {
			..Default::default()
		})
		.with(FlyCamera::default());

	let box_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.25 }));
	let box_material = materials.add(Color::rgb(1.0, 0.2, 0.3).into());

	const AMOUNT: i32 = 6;
	for x in -(AMOUNT / 2)..(AMOUNT / 2) {
		for y in -(AMOUNT / 2)..(AMOUNT / 2) {
			for z in -(AMOUNT / 2)..(AMOUNT / 2) {
				commands.spawn(PbrBundle {
					mesh: box_mesh.clone(),
					material: box_material.clone(),
					transform: Transform::from_translation(Vec3::new(
						x as f32, y as f32, z as f32,
					)),
					..Default::default()
				});
			}
		}
	}

	println!("Started example!");
}

// Press "T" to toggle keyboard+mouse control over the camera
fn toggle_button_system(
	input: Res<Input<KeyCode>>,
	mut query: Query<(Entity, &mut FlyCamera)>
) {
	if input.just_pressed(KeyCode::T) {
		println!("Toggled FlyCamera enabled!");
		for (entity, mut options) in query.iter_mut() {
			options.enabled = !options.enabled;
		}
	}

}

fn main() {
	App::build()
		.add_resource(Msaa { samples: 4 })
		.add_plugins(DefaultPlugins)
		.add_startup_system(init.system())
		.add_plugin(FlyCameraPlugin)
		.add_system(toggle_button_system.system())
		.run();
}
