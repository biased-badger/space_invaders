use crate::game::app::*;
use bevy::color::palettes::basic::WHITE;
use bevy::color::Color;
use bevy::prelude::*;
use std::ops::Range;

const ENEMY_WIDTH: f32 = 30.;
const ENEMY_HEIGHT: f32 = 20.;

const ENEMY_GRID_GAP: f32 = 30.;

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle(SpriteBundle, Enemy);

impl EnemyBundle {
    pub fn new(position: Vec2, color: Color) -> Self {
        EnemyBundle(
            SpriteBundle {
                transform: Transform {
                    translation: position.extend(0.),
                    scale: Vec3::new(ENEMY_WIDTH, ENEMY_HEIGHT, 1.),
                    ..default()
                },
                sprite: Sprite { color, ..default() },
                ..default()
            },
            Enemy,
        )
    }
}

#[derive(Default)]
struct Grid {
    rows: f32,
    cols: f32,
    row_padding: f32,
    col_padding: f32,
}

impl Grid {
    pub fn default() -> Self {
        Self::new(
            Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT),
            Vec2::splat(ENEMY_GRID_GAP),
        )
    }

    pub fn new(area: Vec2, object: Vec2, gap: Vec2) -> Self {
        let available_height = (area.y / 2.) - gap.y;

        let rows = (available_height / (object.y + gap.y)).floor();
        let row_padding = available_height % (object.y + gap.y) / 2.;

        let available_width = area.x - gap.x * 3.;

        let cols = (available_width / (object.x + gap.x)).floor();
        let col_padding = available_width % (object.x + gap.x) / 2.;

        Grid {
            rows,
            cols,
            row_padding,
            col_padding,
        }
    }

    pub fn iter_rows(&self) -> Range<i16> {
        // TODO: find a safer way to do this
        let i16_rows = self.rows as i16;

        0..i16_rows
    }

    pub fn iter_cols(&self) -> Range<i16> {
        // TODO: find a safer way to do this
        let i16_cols = self.cols as i16;

        0..i16_cols
    }
}

pub fn setup_enemy(mut commands: Commands) {
    let grid = Grid::default();
    info!(
        "\nEnemy grid:\nrows: {:?}\ncols: {:?}\nrow_padding: {:?}\ncol_padding: {:?}",
        grid.rows, grid.cols, grid.row_padding, grid.col_padding
    );

    for row in grid.iter_rows() {
        let pos_y: f32 = WINDOW_TOP
            - ENEMY_GRID_GAP
            - grid.row_padding
            - ENEMY_HEIGHT / 2.
            - f32::from(row) * (ENEMY_HEIGHT + ENEMY_GRID_GAP);

        for col in grid.iter_cols() {
            let pos_x = WINDOW_LEFT
                + ENEMY_GRID_GAP * 2.
                + grid.col_padding
                + ENEMY_WIDTH / 2.
                + f32::from(col) * (ENEMY_WIDTH + ENEMY_GRID_GAP);

            // info!("\nNew enemy:\nx: {:?}\ny: {:?}", pos_x, pos_y);

            commands.spawn(EnemyBundle::new(
                Vec2::new(pos_x, pos_y),
                Color::from(WHITE),
            ));
        }
    }
}
