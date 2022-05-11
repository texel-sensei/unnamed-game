//! Basic hello world example.
use common_macros::hash_map;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use input::ActionQueue;

mod input;

// Kuhter code
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
    About,
    Error,
    Game,
    Lobby,
    Settings,
    Splash,
}

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(keyboard_input_system)
        .add_system(square_move_system)
        .add_system(exit_on_esc_system)
        .add_system(state_change_system)
        .add_state(GameState::Splash)
        .add_plugin(splash::SplashPlugin)
        // .add_plugin(game::GamePlugin)
        ;

    for state in vec![About, Error, Game, Lobby, Settings, Splash] {
        app.add_system_set(SystemSet::on_enter(state).with_system(print_state_system));
    }

    use GameState::*;
    for state in vec![About, Lobby, Settings] {
        app.add_system_set(SystemSet::on_enter(state).with_system(not_implemented_system));
    }

    app.add_system_set(SystemSet::on_enter(GameState::Error).with_system(show_error_system));
    app.run();
}

fn not_implemented_system(mut game_state: ResMut<State<GameState>>) {
    println!("Before: {:?}", game_state);
    game_state.replace(GameState::Error).unwrap();
    dbg!(game_state);
}

fn print_state_system(game_state: Res<State<GameState>>) {
    println!("Now in state {:?}", game_state);
}

fn show_error_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(create_text(
        &asset_server,
        "Something went terribly wrong!",
        42.6913374711,
        Color::RED,
    ));
}

#[derive(Component)]
struct Player(u32);

#[derive(Component)]
struct Velocity(Vec3);


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("awesome-square.png");
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            texture: texture_handle.clone(),
            transform: Transform::from_xyz(300.1, 10.0, 10.0),
            ..Default::default()
        })
        .insert(Player(0))
        .insert(ActionQueue::new())
        ;
    commands
        .spawn_bundle(SpriteBundle {
            texture: texture_handle.clone(),
            transform: Transform::from_xyz(310.1, 10.0, 10.0),
            ..Default::default()
        })
        .insert(Player(1))
        .insert(ActionQueue::new())
        ;
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
    mut query: Query<(&Player, &mut ActionQueue)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    use input::Action::*;

    let keybindings = vec![
        hash_map! {
            Up => KeyCode::W,
            Left => KeyCode::A,
            Down => KeyCode::S,
            Right => KeyCode::D,
        },
        hash_map! {
            Up => KeyCode::Up,
            Left => KeyCode::Left,
            Down => KeyCode::Down,
            Right => KeyCode::Right,
        },
    ];

    let all_actions = vec![Left, Right, Up, Down];

    for (player, mut actions) in query.iter_mut() {
        let bindings = &keybindings[player.0 as usize];

        let pressed_actions = all_actions.iter()
            .filter(|a| keyboard_input.pressed(bindings[a]))
            ;

        actions.update(pressed_actions);
    }
}

/// This is the thing that does that our square moves (#7)
fn square_move_system(
    mut query: Query<(&mut Transform, &ActionQueue), With<Player>>
) {
    for (mut transform, actions) in query.iter_mut() {
        let mut delta = Vec3::default();
        let speed = 20.0;

        use input::Action::*;

        if actions.is_pressed(Up) {
            delta.y += speed;
        }

        if actions.is_pressed(Left) {
            delta.x -= speed;
        }

        if actions.is_pressed(Down) {
            delta.y -= speed;
        }

        if actions.is_pressed(Right) {
            delta.x += speed;
        }

        transform.translation += delta;
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

mod splash {
    use bevy::app::AppExit;

    use super::*;

    pub struct SplashPlugin;

    #[derive(Component)]
    struct OnSplashScreen;

    #[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
    enum ButtonID {
        Solo,
        Multiplayer,
        Quit,
        About,
        Settings,
    }

    impl Plugin for SplashPlugin {
        fn build(&self, app: &mut App) {
            app.add_system_set(SystemSet::on_enter(GameState::Splash).with_system(splash_setup))
                .add_system_set(SystemSet::on_update(GameState::Splash).with_system(button_system))
                .add_system_set(
                    SystemSet::on_exit(GameState::Splash)
                        .with_system(despawn_all_with_component::<OnSplashScreen>),
                );
        }
    }

    fn create_button<T>(commands: &mut ChildBuilder, id: ButtonID, child: T)
    where
        T: Bundle,
    {
        commands
            .spawn_bundle(ButtonBundle {
                color: UiColor(Color::BLACK),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(child);
            })
            .insert(id);
    }

    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &ButtonID, &mut UiColor, Changed<Interaction>),
            With<Button>,
        >,
        mut game_state: ResMut<State<GameState>>,
        mut app_exit_events: EventWriter<AppExit>,
    ) {
        for (interaction, id, mut color, changed) in interaction_query.iter_mut() {
            if !changed {
                continue;
            }
            println!("Interaction on {:?} {:?}", id, interaction);

            match *interaction {
                Interaction::Clicked => {
                    use ButtonID::*;
                    match *id {
                        Solo => game_state.set(GameState::Game).unwrap(),
                        Multiplayer => game_state.set(GameState::Lobby).unwrap(),
                        Settings => game_state.set(GameState::Settings).unwrap(),
                        About => game_state.set(GameState::About).unwrap(),
                        Quit => app_exit_events.send(AppExit),
                    }
                }
                Interaction::Hovered => {
                    *color = UiColor(Color::GRAY);
                }
                Interaction::None => {
                    *color = UiColor(Color::BLACK);
                }
            }
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
                        ..Default::default()
                    })
                    // Add main buttons
                    .with_children(|parent| {
                        let solo = create_text(&asset_server, "Solo", 50.0, Color::WHITE);
                        let multi = create_text(&asset_server, "Multiplayer", 50.0, Color::GOLD);
                        let quit = create_text(&asset_server, "Quit", 50.0, Color::ANTIQUE_WHITE);
                        let padding = Rect::all(Val::Px(15.0));
                        use ButtonID::*;
                        for (mut bundle, id) in
                            vec![(solo, Solo), (multi, Multiplayer), (quit, Quit)]
                        {
                            bundle.style.margin = padding;
                            create_button(parent, id, bundle);
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
                        use ButtonID::*;
                        for (mut bundle, id) in vec![(about, About), (settings, Settings)] {
                            bundle.style.margin = padding;
                            create_button(parent, id, bundle);
                        }
                    });
            });
    }
}
