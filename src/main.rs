use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_ticker)
        .run();
}

#[derive(Component)]
struct Ticker {
    timer: Timer,
}

fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // This is needed otherwise the invalid operation will not fire.
    cmd.spawn_bundle(Camera2dBundle {
        ..Default::default()
    });

    cmd.spawn()
        .insert(Ticker {
            timer: Timer::new(Duration::from_secs(5), false),
        })
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        });
}

// delete the entity after 5 seconds.
fn update_ticker(mut cmd: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Ticker)>) {
    for (e, mut t) in q.iter_mut() {
        t.timer.tick(time.delta());
        if t.timer.just_finished() {
            cmd.entity(e).despawn();
        }
    }
}
