use bevy::prelude::*;
use bevy_light_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Light2dPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    const LIGHT_INTENSITY: f32 = 4.0;
    const LIGHT_RADIUS: f32 = 240.0;
    const LIGHT_FALLOFF: f32 = 120.0;

    let mut projection = OrthographicProjection::default_2d();
    projection.scale = 0.5;
    commands.spawn((
        Camera2d,
        Projection::Orthographic(projection),
        Light2d {
            ambient_light: AmbientLight2d {
                brightness: 0.01,
                ..default()
            },
        },
    ));

    commands.spawn((
        PointLight2d {
            // `intensity` 是整体亮度倍率，值越大，整个光照范围内都会更亮。
            intensity: LIGHT_INTENSITY,
            // `radius` 是照亮范围上限
            radius: LIGHT_RADIUS,
            // `falloff` 控制从中心到边缘的衰减速度。
            falloff: LIGHT_FALLOFF,
            color: Color::Srgba(Srgba::WHITE),
            ..default()
        },
        Transform::from_xyz(0., 80., 0.),
    ));
    let texture = asset_server.load("gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform {
            translation: vec3(0., 0., 0.),
            scale: Vec3::splat(6.0),
            ..Default::default()
        }, // 放大 6 倍
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)), // 每 0.1 秒切换一帧
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
