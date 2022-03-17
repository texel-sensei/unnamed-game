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
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("tiles.png");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: texture_handle,
        ..Default::default()
    });
}
