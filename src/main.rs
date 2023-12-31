use std::time::Duration;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, (setup_camera, setup_scene, spawn_player, spawn_boxes))
        .add_systems(Update, (character_horizontal_movement, character_jump, store_pos, ground_character))
        .add_systems(Update, (global_rewind.before(local_rewind), local_rewind.before(object_rewind), object_rewind.before(store_pos)))
        .run()
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
struct Past{
    pub is_rewinding: bool,
    pub is_rewinding_global: bool,
    pub transforms: Vec<Transform>,
    pub velocities: Vec<Velocity>,
    pub timer: Timer,
}


fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_scene(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material1 = materials.add(Color::rgb(0.5, 0.5, 1.0).into());

    //blue background
    commands
        .spawn(SpriteBundle{
            sprite: Sprite { color: Color::rgb(0.1, 0.4, 1.), custom_size: Some(Vec2::new(5000., 5000.)), ..default()},
            transform: Transform::from_xyz(0., 0., -10.),
            ..default()
        });


    //floors
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
        .insert(material1.clone());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 0., 0.),
                custom_size: Some(Vec2::new(500.0, 50.0)),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(250., 25.))
        .insert(TransformBundle::from(Transform::from_xyz(300., -50., 0.)))
        .insert(material1.clone());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 0., 0.),
                custom_size: Some(Vec2::new(500.0, 50.0)),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(250., 25.))
        .insert(TransformBundle::from(Transform::from_xyz(-300., -150., 0.)))
        .insert(material1.clone());

    // walls and ceiling
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 0., 0.),
                custom_size: Some(Vec2::new(400.0,1000.0)),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(200., 500.))
        .insert(TransformBundle::from(Transform::from_xyz(-850., 0., 0.)))
        .insert(material1.clone());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 0., 0.),
                custom_size: Some(Vec2::new(400., 1000.0)),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(200., 500.))
        .insert(TransformBundle::from(Transform::from_xyz(850., 0., 0.)))
        .insert(material1.clone());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 0., 0.),
                custom_size: Some(Vec2::new(2000.0, 200.0)),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(1000., 100.))
        .insert(TransformBundle::from(Transform::from_xyz(0., 400., 0.)))
        .insert(material1.clone());



}

fn spawn_player(
    mut commands: Commands, 
    // asset_server: Res<AssetServer>,
) {
    // let texture:Handle<Image>  = asset_server.load("box.png");
    
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(32., 32.))
        .insert(KinematicCharacterController{
            ..default()
        })
        .insert(Velocity{linvel: Vec2::new(0., 0.), angvel:0.})
        .insert(SpriteBundle {
            sprite: Sprite { color: Color::rgb(1., 1., 1.), custom_size: Some(Vec2::new(64., 64.)), ..default()},
            // texture,
            ..Default::default()
        })
        .insert(GravityScale(10.))
        .insert(TransformBundle::from(Transform::from_xyz(300.,-200., 0.)))
        .insert(Player{
            speed: 300.,
        })
        .insert(Past{
            is_rewinding: false,
            is_rewinding_global: false,
            transforms: Vec::new(),
            velocities: Vec::new(),
            timer: Timer::new(Duration::new(0, 100000), TimerMode::Repeating),
        });
}

fn spawn_boxes(
    mut commands: Commands, 
    // asset_server: Res<AssetServer>,
) {
    // let texture2:Handle<Image> = asset_server.load("rewind_box.png");
    for x in 0..10{
        commands
            .spawn(RigidBody::Dynamic)
            .insert(Collider::cuboid(32., 32.))
            .insert(Velocity{linvel: Vec2::new(0., 0.), angvel:0.})
            .insert(SpriteBundle {
            sprite: Sprite { color: Color::rgb(0.1, 0.2, 0.3), custom_size: Some(Vec2::new(64., 64.)), ..default()},
                // texture: texture2.clone(),
                ..Default::default()
            })
            .insert(GravityScale(10.))
            .insert(TransformBundle::from(Transform::from_xyz(x as f32 * 100. - 500.,200. , 0.)))
            .insert(Past{
                is_rewinding: false,
                is_rewinding_global: false,
                transforms: Vec::new(),
                velocities: Vec::new(),
                timer: Timer::new(Duration::new(0, 100000), TimerMode::Repeating),
            });

    }
}

fn ground_character(
    mut controllers: Query<&mut KinematicCharacterController>
){
    for mut controller in controllers.iter_mut(){
        controller.translation = Some(Vec2 { x: 0., y: -1. })
    }
}

fn character_horizontal_movement(
    mut characters: Query<(&Player, &Past, &mut Velocity)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for(player, past, mut velocity) in characters.iter_mut(){
        if input.pressed(KeyCode::D) && !past.is_rewinding{
            let right_speed = player.speed*time.delta_seconds();
            velocity.linvel.x = right_speed * 30.;
        } else if input.pressed(KeyCode::A) && !past.is_rewinding{
            let left_speed = -player.speed*time.delta_seconds();
            velocity.linvel.x = left_speed*30.;
        }
    }
}


fn character_jump(
    mut characters: Query<(&Player, &mut KinematicCharacterControllerOutput, &mut Velocity, &Past)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for(player, output, mut velocity, past) in characters.iter_mut(){
        let movement_amount = player.speed * time.delta_seconds();
        if output.grounded && input.pressed(KeyCode::W) && !past.is_rewinding && !past.is_rewinding_global{
            velocity.linvel = Vec2::new(0.,movement_amount * 120.);
        }
    }
}

fn store_pos(
    mut objects_with_past: Query<(&mut Past, &Transform, &Velocity)>,
) {
    for (mut past, transform, velocity) in objects_with_past.iter_mut(){
        if past.timer.finished() && !past.is_rewinding && !past.is_rewinding_global{
            past.transforms.push(*transform);
            past.velocities.push(*velocity);
        }
    }
}


// add checks to be able to rewind locall and objects at same time and to not call both global and either other rewind at the same time 

fn global_rewind(
    mut objects: Query<(&mut Past, &mut Transform, &mut GravityScale, &mut Velocity)>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
){
    for(mut past, mut transform, mut gravity, mut velocity) in objects.iter_mut(){
        past.timer.tick(time.delta());
        if input.pressed(KeyCode::Space) && past.transforms.len() > 0 && !past.is_rewinding{
            past.is_rewinding_global = true;
            if past.timer.finished(){
                let this_transform = past.transforms.pop().unwrap();
                let this_velocity = past.velocities.pop().unwrap();
                transform.translation = this_transform.translation;
                transform.rotation = this_transform.rotation;
                velocity.linvel = this_velocity.linvel;
                velocity.angvel = this_velocity.angvel;
                gravity.0 = 0.;
            }
        }
        else {
            past.is_rewinding_global = false;
            gravity.0 = 10.;
        }
    }
}

fn local_rewind(
    mut objects: Query<(&mut Past, &mut Transform, &mut GravityScale, &mut Velocity), With<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
){
    for(mut past, mut transform, mut gravity, mut velocity) in objects.iter_mut(){
        past.timer.tick(time.delta());
        if input.pressed(KeyCode::Q) && past.transforms.len() > 0 && !past.is_rewinding_global{
            past.is_rewinding = true;
            if past.timer.finished(){
                let this_transform = past.transforms.pop().unwrap();
                let this_velocity = past.velocities.pop().unwrap();
                transform.translation = this_transform.translation;
                transform.rotation = this_transform.rotation;
                velocity.linvel = this_velocity.linvel;
                velocity.angvel = this_velocity.angvel;
                gravity.0 = 0.;
            }
        }
        else {
            past.is_rewinding = false;
            gravity.0 = 10.;
        }
    }
}


fn object_rewind(
    mut objects: Query<(&mut Past, &mut Transform, &mut GravityScale, &mut Velocity), Without<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
){
    for(mut past, mut transform, mut gravity, mut velocity) in objects.iter_mut(){
        past.timer.tick(time.delta());
        if input.pressed(KeyCode::E) && past.transforms.len() > 0 && !past.is_rewinding_global{
            past.is_rewinding = true;
            if past.timer.finished(){
                let this_transform = past.transforms.pop().unwrap();
                let this_velocity = past.velocities.pop().unwrap();
                transform.translation = this_transform.translation;
                transform.rotation = this_transform.rotation;
                velocity.linvel = this_velocity.linvel;
                velocity.angvel = this_velocity.angvel;
                gravity.0 = 0.;
            }
        }
        else {
            past.is_rewinding = false;
            gravity.0 = 10.;
        }
    }
}

