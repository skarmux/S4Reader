use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use s4::ground::{Ground, GroundType};
//use rand::prelude::*;
use bevy::input::mouse::*;
use bevy::render::mesh::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(Map::new(20,20))
    .add_startup_system(spawn_camera)
    //.add_startup_system(spawn_map)
    .add_startup_system(spawn_settlers)
    .add_startup_system(spawn_player)
    .add_system(settler_movement)
    .add_system(camera_zoom)
    .add_system(camera_move)
    .add_system(place_star_on_click)
    .add_event::<MouseWheel>()
    .run();
}

#[derive(Resource)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<s4::ground::Ground>>
}
impl Map {
    fn new(width: usize, height: usize) -> Self {
        let mut grid = Vec::new();
        for _ in 0..height {
            grid.push(vec![Ground { 
                height: 0,
                ground_type: GroundType::Grass,
                flags: 0,
            }; width]);
        }
        Map {
            width,
            height,
            grid
        }
    }
}

#[derive(Component)]
pub struct Settler;

#[derive(Component)]
pub struct Star;

#[derive(Component)]
pub struct Health(u8);

#[derive(Component)]
pub struct MainCamera;

fn create_triangle() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
    );
    //mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![[0.0, 0.0, 0.0, 1.0]; 3]);
    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));
    mesh
}

pub fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<Map>,
) {
    for (x, y) in (0..map.width).flat_map(move |x| (0..map.height).map(move |y| (x, y))) {

    }


    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(create_triangle()).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..Default::default()
    });
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            },
            Settler {}
        )
    );
}

pub fn spawn_settlers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
) {
    //let mut rng = rand::thread_rng();

    for row in 0..map.height {
        for col in 0..map.width {
            
            commands.spawn(
                (
                    SpriteBundle {
                        transform: Transform::from_xyz(col as f32 * 64.0 + (row as f32 * 32.0), row as f32 * 64.0, -1.0),
                        texture: asset_server.load("sprites/ball_red_large.png"),
                        ..default()
                    },
                    Settler {}
                )
            );
        }
    }
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        MainCamera,
        Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
        }
    ));
}

pub fn camera_move(
    mut motion_evr: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    if mouse_buttons.pressed(MouseButton::Right) {
        if let Ok(mut transform) = camera_query.get_single_mut() {
            for ev in motion_evr.iter() {
                println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
                transform.translation += Vec3::new(-ev.delta.x, ev.delta.y, 0.0);
            }
        }   
    }
}

pub fn camera_zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    let mut projection = camera_query.single_mut();

    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                println!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
                if ev.y < 0.0 {
                    projection.scale *= 4.0;
                } else {
                    projection.scale /= 4.0;
                }
                
            }
            MouseScrollUnit::Pixel => {
                println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
                projection.scale *= ev.y;
            }
        }
    }

    projection.scale = projection.scale.clamp(1.0, 64.0);
}

pub const MOVEMENT_SPEED: f32 = 500.0;

fn snap_to_grid(x: f32, y: f32, map: Res<Map>) {
    
}

pub fn place_star_on_click(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_buttons: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        // get the camera info and transform
        // assuming there is exactly one main camera entity, so query::single() is OK
        let (camera, camera_transform) = camera_query.single();

        // get the window that the camera is displaying to (or the primary window)
        let window = window_query.get_single().unwrap();

        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            eprintln!("World coords: {}/{}", world_position.x, world_position.y);

            if world_position.x < (map.width as f32 * 64.0) && world_position.y < (map.height as f32 * 64.0) {
                eprintln!("New Star");
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz( world_position.x,  world_position.y, -1.0),
                        texture: asset_server.load("sprites/star.png"),
                        ..default()
                    },
                    Star
                ));
            }
        }
    }
    
}

pub fn settler_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Settler>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * MOVEMENT_SPEED * time.delta_seconds();
    }
}
