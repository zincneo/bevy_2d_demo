use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // 预防精灵图模糊
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .run()
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

/**
# Sprite

这个系统负责初始化：
1. 创建精灵表布局（TextureAtlasLayout）- 定义精灵图如何划分为网格
2. 加载精灵表图像
3. 配置动画组件
4. 设置摄像头

## 主要的新 API：

### TextureAtlasLayout::from_grid
- **用途**: 将一个大的精灵图按网格划分成多个单元格（每个单元格是一帧）
- **参数**:
  - `cell_size`: 每个单元格的大小 (UVec2::splat(24) = 24x24 像素)
  - `columns`: 列数 (7 列)
  - `rows`: 行数 (1 行，这里是 7 帧的水平排列)
  - `padding`: 单元格之间的填充
  - `offset`: 偏移量

### Sprite::from_atlas_image
- **用途**: 创建一个精灵体，使用精灵表而不是完整的图像
- **参数**:
  - `texture`: 精灵表的纹理
  - `TextureAtlas { layout, index }`:
    - `layout`: 精灵表布局（定义如何划分）
    - `index`: 初始显示的帧索引

### TextureAtlas 组件
- **用途**: 存储在 Sprite 中，指示当前显示的帧
- `index`: 当前显示的帧编号（0 开始）
*/
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // 加载精灵表图像
    let texture = asset_server.load("gabe-idle-run.png");

    // 创建布局：将 24x24 的精灵表按 7 列 1 行 划分
    // 这样得到 7 个 24x24 的动画帧
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);

    // 将布局加入资源库，获得句柄
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // 定义动画范围：第 1 帧到第 6 帧
    let animation_indices = AnimationIndices { first: 1, last: 6 };

    // 创建摄像头
    commands.spawn(Camera2d);

    // 创建动画精灵实体
    // 包含：精灵本体、变换、动画信息、计时器
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_scale(Vec3::splat(6.0)), // 放大 6 倍
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
