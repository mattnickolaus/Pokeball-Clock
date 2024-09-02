
use bevy::{prelude::*};
use chrono::{Datelike, Timelike};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (clock_face, text_update_time, text_update_date))
        .run()
}

#[derive(Component)]
struct Datetext;

#[derive(Component)]
struct Timetext;

fn setup(
    mut commands: Commands,
    mut gizmo_conf: ResMut<GizmoConfig>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    let background_image = asset_server.load("Still-pokemon-background-image.PNG");
    commands.spawn(SpriteBundle{
        texture: background_image,
        transform: Transform::from_scale(Vec3::new(1.5, 1.5, 0.0)),
        ..default()
    });

    commands.spawn((TextBundle::from_section(
            "date",
            TextStyle {
                color: Color::WHITE,
                font: asset_server.load("Roboto-Medium.ttf"),
                font_size: 20.0,
                ..default()
            },
        )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Percent(15.0),
                right: Val::Percent(25.0),
                ..default()
            }),
            Datetext,
    ));

    commands.spawn((TextBundle::from_section(
            "time",
            TextStyle {
                color: Color::WHITE,
                font: asset_server.load("Roboto-Medium.ttf"),
                font_size: 150.0,
                ..default()
            },
        )
           .with_text_alignment(TextAlignment::Center)
           .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Percent(40.0),
                right: Val::Percent(37.0),
                ..default()
           }),
        Timetext,
    ));


    gizmo_conf.line_width = 30.0;
}


fn clock_face(mut gizmos: Gizmos) {
    let now = chrono::Local::now();

    let hour = now.hour() as f32;
    let minute = now.minute() as f32;
    let second = now.second() as f32;

    let second_angle = ((360.0 / 60.0) * second).to_radians();
    let minute_angle = ((360.0 / 60.0) * minute).to_radians();
    let hour_angle = ((360.0 / 24.0) * hour).to_radians();

    gizmos.arc_2d(Vec2::ZERO, second_angle / 2.0, second_angle, 70., Color::BLACK)
        .segments(360*3);

    gizmos.arc_2d(Vec2::ZERO, minute_angle/ 2.0, minute_angle, 230., Color::RED)
        .segments(360*3);

    gizmos.arc_2d(Vec2::ZERO, hour_angle/ 2.0, hour_angle, 260., Color::WHITE)
        .segments(360*3);

    gizmos.line_2d(Vec2::new(70.0, 0.0), Vec2::new(220.0, 0.0), Color::BLACK);
    gizmos.line_2d(Vec2::new(-70.0, 0.0), Vec2::new(-220.0, 0.0), Color::BLACK);


    // println!("{hour}:{minute}:{second}");
    // println!("{date}");
}


fn text_update_time(mut query: Query<&mut Text, With<Timetext>>,) {
    let now = chrono::Local::now();
    let hour = now.hour() as f32;
    let minute = now.minute() as f32;
    // let time_output  = format!("{}:{:02}", hour, minute);

    for mut text in &mut query {
        text.sections[0].value = format!("{:02}:{:02}", hour, minute);
    }
}

fn text_update_date(mut query: Query<&mut Text, With<Datetext>>,) {
    let now = chrono::Local::now();
    // let date = now.date_naive();
    let day = now.day();
    let month = now.month();
    let year = now.year();

    for mut text in &mut query {
        text.sections[0].value = format!("{}/{}/{}", month, day, year);
    }
}
