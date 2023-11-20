use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup_camera, setup_ground, spawn_player))
        .add_systems(Update, (character_horizontal_movement, character_jump))
        .run()
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    // pub past_positions: Array,
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
        .spawn(SpriteBundle{
            sprite: Sprite { color: Color::rgb(0.1, 0.4, 1.), custom_size: Some(Vec2::new(5000., 5000.)), ..default()},
            transform: Transform::from_xyz(0., 0., -10.),
            ..default()
        });

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 0., 0.),
                custom_size: Some(Vec2::new(2000.0, 250.0)),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(1000., 125.))
        .insert(TransformBundle::from(Transform::from_xyz(0., -400., 0.)))
        .insert(material);

}

fn spawn_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    let texture:Handle<Image>  = asset_server.load("box.png");
    
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(32., 32.))
        .insert(KinematicCharacterController{
            ..default()
        })
        .insert(Velocity{linvel: Vec2::new(0., 0.), angvel:0.})
        .insert(GravityScale(10.))
        .insert(Player{speed: 300.})
        .insert(SpriteBundle {
            global_transform: Transform::from_xyz(0., 100., 0.).into(),
            texture,
            ..Default::default()
        });
}

fn character_horizontal_movement(
    mut characters: Query<(&mut KinematicCharacterController, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for(mut controller, player) in characters.iter_mut(){
        if input.pressed(KeyCode::D) {
            let right_speed = player.speed*time.delta_seconds();
            controller.translation = match controller.translation{
                Some(mut v) => {
                    v.x = right_speed;
                    Some(v)
                }
                None =>{
                    Some(Vec2::new(right_speed, -1.0))
                }
            }
        } else if input.pressed(KeyCode::A) {
            let left_speed = -player.speed*time.delta_seconds();
            controller.translation = match controller.translation{
                Some(mut v) => {
                    v.x = left_speed;
                    Some(v)
                }
                None =>{
                    Some(Vec2::new(left_speed, -1.0))
                }
            }
        } else {
            controller.translation = match controller.translation {
                Some(mut v) => {
                    v.x = 0.;
                    Some(v)
                }
                None => Some(Vec2::new(0., -1.)),
            };
        }
    }
}


fn character_jump(
    mut characters: Query<(&Player, &mut KinematicCharacterControllerOutput, &mut Velocity)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for(player, output, mut velocity) in characters.iter_mut(){
        let movement_amount = player.speed * time.delta_seconds();
        if output.grounded && input.pressed(KeyCode::W) {
            velocity.linvel = Vec2::new(0.,movement_amount * 120.);
        }
    }
}

// fn store_pos(
//     time: Res<Time>,
//     player: Query<&Player>
// ) {

// }

