//! Basic hello world example.
use bevy::prelude::*;

//                                      /;    ;\
//                                  __  \\____//
//                                 /{_\_/   `'\____
//                                 \___   (o)  (o  }
//      _____________________________/          :--'
//  ,-,'`@@@@@@@@       @@@@@@         \_    `__\
//  ;:(  @@@@@@@@@        @@@             \___(o'o)
//  :: )  @@@@          @@@@@@        ,'@@(  `===='
//  :: : @@@@@:          @@@@         `@@@:
//  :: \  @@@@@:       @@@@@@@)    (  '@@@'
//  ;; /\      /`,    @@@@@@@@@\   :@@@@@)
//  ::/  )    {_----------------:  :~`,~~;
// ;;'`; :   )   |  |            :  / `; ;
// ;;;; : :   ;  |  |            :  ;  ; :
// `'`' / :  :   |  |            :  :  : :
//     )_ \__;   |  |            :_ ;  \_\      `,','
//     :__\  \   \__/            \  \  :  \   *  8`;'*
//         `^'                    `^'  `-^-'   \v/ :

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(keyboard_input_system)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("tiles.png");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: texture_handle,
        transform: Transform::from_xyz(300.1,10.0,10.0),
        ..Default::default()
    }).insert(Player);
}

/// This system prints 'A' key state
fn keyboard_input_system(mut query: Query<&mut Transform, With<Player>>, keyboard_input: Res<Input<KeyCode>>) {
    let mut delta = Vec3::default();
    let mut size_delta = Vec3::default();
    let speed = 20.0;

    if keyboard_input.pressed(KeyCode::W) {
        delta.y += speed;
    }

    if keyboard_input.pressed(KeyCode::A) {
        delta.x -= speed;
    }

    if keyboard_input.pressed(KeyCode::S) {
        delta.y -= speed;
    }

    if keyboard_input.pressed(KeyCode::D) {
        delta.x += speed;
    }

    if keyboard_input.pressed(KeyCode::Q) {
        size_delta += 1.0;
    }

    if keyboard_input.pressed(KeyCode::E) {
        size_delta -= 1.0;
    }

    for mut transform in query.iter_mut() {
        transform.translation += delta;
        transform.scale += size_delta;
    }
}
