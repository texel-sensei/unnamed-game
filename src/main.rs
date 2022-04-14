//! Basic hello world example.
use bevy::{input::system::exit_on_esc_system, prelude::*};

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

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Splash,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(keyboard_input_system)
        .add_system(exit_on_esc_system)
        .add_system(state_change_system)
        .add_state(GameState::Splash)
        .add_plugin(splash::SplashPlugin)
        // .add_plugin(game::GamePlugin)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("awesome-square.png");
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            texture: texture_handle,
            transform: Transform::from_xyz(300.1, 10.0, 10.0),
            ..Default::default()
        })
        .insert(Player);
}

fn state_change_system(
    mut game_state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::U) {
        if game_state.current() == &GameState::Game {
            game_state.set(GameState::Splash).unwrap();
        } else {
            game_state.set(GameState::Game).unwrap();
        }
        dbg!(game_state);
    }
}

/// This system moves the square
fn keyboard_input_system(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
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

/// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_all_with_component<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

mod splash {
    use super::*;

    pub struct SplashPlugin;

    #[derive(Component)]
    struct OnSplashScreen;

    impl Plugin for SplashPlugin {
        fn build(&self, app: &mut App) {
            app.add_system_set(SystemSet::on_enter(GameState::Splash).with_system(splash_setup))
                .add_system_set(
                    SystemSet::on_exit(GameState::Splash)
                        .with_system(despawn_all_with_component::<OnSplashScreen>),
                );
        }
    }

    /// Return a TextBundle
    fn create_text(
        asset_server: &Res<AssetServer>,
        text: &str,
        font_size: f32,
        color: Color,
    ) -> TextBundle {
        TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                text,
                TextStyle {
                    font: asset_server.load("LiberationMono-Regular.ttf"),
                    font_size,
                    color,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        }
    }

    fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        println!("Entered splash screen!");
        commands
            // Layout whole screen
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(OnSplashScreen)
            // Add title
            .with_children(|parent| {
                let mut title = create_text(&asset_server, "Unnamed Game", 100.0, Color::BLACK);
                let padding = Rect::all(Val::Px(40.0));
                title.style.margin = padding;

                parent.spawn_bundle(title);
                parent
                    // Add layout for main buttons
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            align_items: AlignItems::Stretch,
                            justify_content: JustifyContent::SpaceEvenly,
                            flex_direction: FlexDirection::ColumnReverse,
                            ..Default::default()
                        },
                        color: Color::BLACK.into(),
                        ..Default::default()
                    })
                    // Add main buttons
                    .with_children(|parent| {
                        let solo = create_text(&asset_server, "Solo", 50.0, Color::WHITE);
                        let multi = create_text(&asset_server, "Multiplayer", 50.0, Color::GOLD);
                        let quit = create_text(&asset_server, "Quit", 50.0, Color::ANTIQUE_WHITE);
                        let padding = Rect::all(Val::Px(15.0));
                        for mut bundle in vec![solo, multi, quit] {
                            bundle.style.margin = padding;
                            parent.spawn_bundle(bundle);
                        }
                    });
                parent
                    // Layout for settings and about buttons
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            align_self: AlignSelf::Stretch,
                            align_items: AlignItems::Stretch,
                            justify_content: JustifyContent::SpaceBetween,
                            flex_direction: FlexDirection::Row,
                            ..Default::default()
                        },
                        color: Color::BLACK.into(),
                        ..Default::default()
                    })
                    // Add settings and about buttons
                    .with_children(|parent| {
                        let about = create_text(&asset_server, "?", 50.0, Color::WHITE);
                        let settings = create_text(&asset_server, "\u{2699}", 50.0, Color::GOLD);
                        let padding = Rect::all(Val::Px(15.0));
                        for mut bundle in vec![about, settings] {
                            bundle.style.margin = padding;
                            parent.spawn_bundle(bundle);
                        }
                    });
            });
    }
}
