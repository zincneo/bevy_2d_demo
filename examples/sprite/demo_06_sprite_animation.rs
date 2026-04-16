use std::time::Duration;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // 预防精灵图模糊
        .add_systems(Startup, setup)
        .add_systems(Update, execute_animations)
        .add_systems(
            Update,
            (
                trigger_animation::<LeftSprite>.run_if(input_just_pressed(KeyCode::ArrowLeft)),
                trigger_animation::<RightSprite>.run_if(input_just_pressed(KeyCode::ArrowRight)),
            ),
        )
        .run()
}

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / fps as f32), TimerMode::Once)
    }
}

#[derive(Component)]
struct LeftSprite;

#[derive(Component)]
struct RightSprite;

fn trigger_animation<S: Component>(mut animation: Single<&mut AnimationConfig, With<S>>) {
    animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
}

/**
# Sprite
通过Sprite和按键输入来制作动画
*/
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
    // 1. 加载贴图
    let texture = asset_server.load("gabe-idle-run.png");
    // 2. 切分
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // 3. 创建动画配置
    let animation_config_1 = AnimationConfig::new(1, 6, 10);
    commands.spawn((
        Sprite {
            image: texture.clone(),
            flip_x: true,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config_1.first_sprite_index,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(-70.0, 0.0, 0.0)),
        LeftSprite,
        animation_config_1,
    ));

    let animation_config_2 = AnimationConfig::new(1, 6, 20);

    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config_2.first_sprite_index,
            }),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(70.0, 0.0, 0.0)),
        RightSprite,
        animation_config_2,
    ));
}

fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    query.iter_mut().for_each(|(mut config, mut sprite)| {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if atlas.index == config.last_sprite_index {
                // 最后一帧移动回第一帧
                atlas.index = config.first_sprite_index;
            } else {
                // 下一帧
                atlas.index += 1;
                // 重新设置计时器
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    });
}
