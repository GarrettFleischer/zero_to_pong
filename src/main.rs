use std::ops::RangeFrom;

use bevy::{prelude::*, window::WindowResolution};
use rand::Rng;

const WINDOW_WIDTH: f32 = 700.;
const WINDOW_HEIGHT: f32 = 500.;

const BALL_SIZE: f32 = 25.;

const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 150.;

const PADDLE_SPEED: f32 = 200.;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            resizable: false,
            ..Default::default()
        }),
        ..Default::default()
    }));

    app.add_systems(
        Startup,
        (spawn_background, spawn_players, spawn_ball, spawn_camera).chain(),
    );

    app.add_systems(
        Update,
        (move_paddle, move_ball, ball_outside, ball_collide).chain(),
    );
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Paddle {
    move_up: KeyCode,
    move_down: KeyCode,
}

fn spawn_players(mut commands: Commands) {
    spawn_player(
        &mut commands,
        -WINDOW_WIDTH * 0.5 + 20.,
        Paddle {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
        },
    );

    spawn_player(
        &mut commands,
        WINDOW_WIDTH * 0.5 - 20.,
        Paddle {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
        },
    )
}

fn spawn_player(commands: &mut Commands, x: f32, paddle: Paddle) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(x, 0., 0.)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        },
        paddle,
    ));
}

fn spawn_background(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        if input.pressed(settings.move_up) {
            pos.translation.y += PADDLE_SPEED * time.delta_seconds();
        }

        if input.pressed(settings.move_down) {
            pos.translation.y -= PADDLE_SPEED * time.delta_seconds();
        }

        pos.translation.y = pos.translation.y.clamp(
            -WINDOW_HEIGHT * 0.5 + PADDLE_HEIGHT * 0.5,
            WINDOW_HEIGHT * 0.5 - PADDLE_HEIGHT * 0.5,
        )
    }
}

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

fn spawn_ball(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(25., 25.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Ball {
            velocity: Vec2::new(-100., 0.),
        },
    ));
}

fn move_ball(mut balls: Query<(&mut Transform, &Ball)>, time: Res<Time>) {
    for (mut pos, ball) in &mut balls {
        pos.translation += ball.velocity.extend(0.) * time.delta_seconds();

        // pos.translation.y = pos
        //     .translation
        //     .y
        //     .clamp(-WINDOW_HEIGHT * 0.5 + 75., WINDOW_HEIGHT * 0.5 - 75.)
    }
}

fn ball_collide(
    mut balls: Query<(&Transform, &mut Ball)>,
    paddles: Query<&Transform, With<Paddle>>,
) {
    for (pos, mut ball) in &mut balls {
        for paddle in &paddles {
            if pos.translation.x - BALL_SIZE * 0.5 < paddle.translation.x + PADDLE_WIDTH * 0.5
                && pos.translation.y - BALL_SIZE * 0.5 < paddle.translation.y + PADDLE_HEIGHT * 0.5
                && pos.translation.x + BALL_SIZE * 0.5 > paddle.translation.x - PADDLE_WIDTH * 0.5
                && pos.translation.y + BALL_SIZE * 0.5 > paddle.translation.x - PADDLE_HEIGHT * 0.5
            {
                ball.velocity *= -1.;
                ball.velocity.y = rand::thread_rng().gen_range(-100.0..100.0);
            }
        }
    }
}

fn ball_outside(mut balls: Query<(&Transform, &mut Ball)>) {
    for (pos, mut ball) in &mut balls {
        if pos.translation.y.abs() + BALL_SIZE * 0.5 > WINDOW_HEIGHT * 0.5 {
            ball.velocity.y *= -1.;
        }
    }
}
