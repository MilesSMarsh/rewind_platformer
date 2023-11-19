use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_ground, spawn_player))
        .add_systems(Update, character_movement)
        .run()
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_ground(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(20., 20.))
        .insert(TransformBundle::from(Transform::from_xyz(0., -50., 0.)))
        .insert(material);

    // Rectangle
    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::rgb(0.25, 0.25, 0.75),
    //         custom_size: Some(Vec2::new(300.0, 100.0)),
    //         ..default()
    //     },
    //     transform: Transform::from_translation(Vec3::new(-50., -50., 0.)),
    //     ..default()
    // });

}

fn spawn_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    let texture:Handle<Image>  = asset_server.load("box.png");
    
    commands
        .spawn(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(25., 25.))
        .insert(KinematicCharacterController{
            ..default()
        })
        .insert(Player{speed: 100.})
        .insert(SpriteBundle {
            global_transform: Transform::from_xyz(0., 0., 0.).into(),
            texture,
            ..Default::default()
        });
}


fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for( mut transform, player) in &mut characters{
        let movement_amount = player.speed * time.delta_seconds();
        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_amount;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_amount;
        }
    }
}

