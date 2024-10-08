use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::system::events::ChangeGameStatus;
use crate::common::status_type::GameStatusType;
use crate::system::resource::GameContext;

const MAIN_MENU_WIDTH: f32 = 200.;
const MAIN_MENU_HEIGHT: f32 = 200.;
pub const POP_UP_MENU_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

#[derive(Component)]
struct MainMenuHandle;

pub fn show_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<ChangeGameStatus>,
    mut game_context: ResMut<GameContext>
) {
    for event in events.read() {
        match event.0 {
            GameStatusType::MainMenu => {
                println!("Main Menu Selected!");
                let mesh = Mesh::from(Rectangle::new(MAIN_MENU_WIDTH, MAIN_MENU_HEIGHT));
                let material = ColorMaterial::from(Color::rgb(0., 1., 0.));
            
                let mesh_handle = meshes.add(mesh);
                let material_handle = materials.add(material);
            
                commands.spawn((
                    MainMenuHandle,
                    MaterialMesh2dBundle {
                        mesh: mesh_handle.into(),
                        material: material_handle,
                        ..default()
                    },
                ));
            },
            GameStatusType::Rank => {
                println!("Rank Menu Selected!")
            }
            GameStatusType::Game => {
                println!("Game Menu Selected!");
                game_context.status = GameStatusType::Game;
            }
        }
    }
}

fn spawn_main_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    println!("Main Menu Selected!");
    let mesh = Mesh::from(Rectangle::new(MAIN_MENU_WIDTH, MAIN_MENU_HEIGHT));
    let material = ColorMaterial::from(Color::rgb(0., 1., 0.));

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    commands.spawn((
        MainMenuHandle,
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
    ));
}