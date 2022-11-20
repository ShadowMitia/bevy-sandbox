use bevy::prelude::*;

#[derive(Resource)]
struct GameOfLifeConfig {
    alive_color: Color,
    dead_color: Color,
}

#[derive(Debug, Clone, Copy)]
struct GridCoordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CellState {
    ALIVE,
    DEAD,
}

#[derive(Component, Debug, Clone, Copy)]
struct Cell {
    coordinate: GridCoordinate,
    state: CellState,
}

#[derive(Debug, Resource)]
struct Cells {
    cells: Vec<CellState>,
    width: usize,
    height: usize,
}

#[derive(Component, Debug)]
struct CellNeighbors {
    neighbors: Vec<GridCoordinate>,
}

fn create_cell(position: Vec2, size: Vec2, color: Color) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            custom_size: Some(size),
            color,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
        ..Default::default()
    }
}

fn setup_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    _materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title("Game Of Life".to_string());

    commands.spawn(Camera2dBundle::default());

    let alive_color = Color::GREEN;
    let dead_color = Color::BLACK;

    commands.insert_resource(GameOfLifeConfig {
        alive_color,
        dead_color,
    });

    let window_width = window.width();
    let window_height = window.height();

    let width = 20;
    let height = 20;

    let size = Vec2::new(window_width / width as f32, window_height / height as f32);

    let width = width + 2;
    let height = height + 2;

    let mut cells = Vec::with_capacity(width * height);
    cells.resize_with(width * height, || CellState::DEAD);

    for x in 1..width - 1 {
        for y in 1..height - 1 {
            let cell = Cell {
                coordinate: GridCoordinate { x, y },
                state: if rand::random() {
                    CellState::ALIVE
                } else {
                    CellState::DEAD
                },
            };

            cells[y * width + x] = cell.state;

            let x = x as isize;
            let y = y as isize;

            let neighbors = vec![
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
                (x - 1, y),
                // (x,     y),
                (x + 1, y),
                (x - 1, y + 1),
                (x, y + 1),
                (x + 1, y + 1),
            ];

            let neighbors = neighbors
                .iter()
                .map(|n| GridCoordinate {
                    x: n.0 as usize,
                    y: n.1 as usize,
                })
                .collect();

            commands.spawn((
                create_cell(
                    Vec2::new(
                        size.x * x as f32 - window_width / 2.0 - size.x / 2.0,
                        size.y * y as f32 - window_height / 2.0 - size.y / 2.0,
                    ),
                    size,
                    match cell.state {
                        CellState::ALIVE => alive_color,
                        CellState::DEAD => dead_color,
                    },
                ),
                cell,
                CellNeighbors { neighbors },
            ));
        }
    }

    commands.insert_resource(Cells {
        cells,
        width: width as usize,
        height: height as usize,
    });
}

fn count_neighbors(cells: &Cells, neighbors: &CellNeighbors) -> usize {
    neighbors.neighbors.iter().fold(0, |mut state, coordinate| {
        let x = coordinate.x;
        let y = coordinate.y;
        let cell_state = cells.cells[y * cells.width + x];
        if cell_state == CellState::ALIVE {
            state += 1;
        };
        state
    })
}

fn game_of_life_update_system(
    time: Res<Time>,
    mut update_timer: ResMut<UpdateTimer>,
    config: Res<GameOfLifeConfig>,
    mut cells: ResMut<Cells>,
    query_cells: Query<(&Cell, &CellNeighbors)>,
    mut query_materials: Query<(&Cell, &mut Sprite)>,
) {
    update_timer.0.tick(time.delta());

    if update_timer.0.just_finished() {
        let mut new_materials = Vec::new();

        for (cell, neighbors) in query_cells.iter() {
            let count = count_neighbors(&cells, neighbors);

            let x = cell.coordinate.x;
            let y = cell.coordinate.y;

            let width = cells.width;

            let cell_state = cells.cells[y * cells.width + x];

            let new_state = match (count, cell_state) {
                (2, CellState::ALIVE) => CellState::ALIVE,
                (3, _) => CellState::ALIVE,
                _ => CellState::DEAD,
            };

            cells.cells[y * width + x] = new_state;

            let material = match new_state {
                CellState::ALIVE => config.alive_color,
                CellState::DEAD => config.dead_color,
            };

            new_materials.push(material);
        }

        query_materials
            .iter_mut()
            .zip(new_materials)
            .for_each(|((_, mut material), mat)| material.color = mat);
    }
}

#[derive(Resource)]
struct UpdateTimer(Timer);

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(UpdateTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_system(game_of_life_update_system)
        // .add_system(mouse_events_system.system())
        .run();
}
