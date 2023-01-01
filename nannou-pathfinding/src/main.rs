use std::collections::{HashMap, HashSet};

use nannou::prelude::*;
use rand;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

const DENSITY: f32 = 0.25;
const M: usize = 100;
const SIDE: f32 = 8.0;
type Field = Vec<Vec<Cell>>;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    Wall,
}

impl Distribution<Cell> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        if rng.gen::<f32>() < DENSITY {
            Cell::Wall
        } else {
            Cell::Empty
        }
    }
}

enum SearchStrategy {
    BreadthFirst,
    DepthFirst,
    AStar,
    Beam,
}

struct Search {
    strategy: SearchStrategy,
    open: HashSet<(usize, usize)>,
    closed: HashSet<(usize, usize)>,
    came_from: HashMap<(usize, usize), (usize, usize)>,
    g_score: HashMap<(usize, usize), i32>,
    current: (usize, usize),
}

impl Search {
    fn A_star() -> Self {
        Self {
            strategy: SearchStrategy::AStar,
            open: HashSet::new(),
            closed: HashSet::new(),
            came_from: HashMap::new(),
            g_score: HashMap::new(),
            current: (0, 0),
        }
    }

    fn init(&mut self) {
        match self.strategy {
            SearchStrategy::AStar => self.init_a_star(),
            _ => (),
        }
    }

    fn update(&mut self, cells: &Field) {
        match self.strategy {
            SearchStrategy::AStar => self.update_a_star(cells),
            _ => (),
        }
    }

    fn init_a_star(&mut self) {
        let start = (0, 0);
        self.open.insert(start);
        self.g_score.insert(start, 0);
    }

    fn update_a_star(&mut self, cells: &Field) {
        let end = (M - 1, M - 1);
        if self.open.is_empty() {
            println!("Open set is empty. Terminated!");
            return;
        }
        let current = *self
            .open
            .iter()
            .min_by_key(|(x, y)| {
                let g_score = *self.g_score.get(&(*x, *y)).unwrap();
                let g_score = 0;
                let h_score = (end.0 as i32 - *x as i32).abs() + (end.1 as i32 - *y as i32).abs();
                // let h_score = (*x - end.0) * (*y - end.1);
                g_score + h_score as i32
            })
            .unwrap();
        self.current = current;
        if current == end {
            println!("Found path!");
            return;
        }
        self.open.remove(&current);
        self.closed.insert(current);
        let neighbors = vec![
            (current.0 as i32 + 1, current.1 as i32),
            // (current.0 as i32 + 1, current.1 as i32 - 1),
            // (current.0 as i32 + 1, current.1 as i32 + 1),
            (current.0 as i32 - 1, current.1 as i32),
            // (current.0 as i32 - 1, current.1 as i32 - 1),
            // (current.0 as i32 - 1, current.1 as i32 + 1),
            (current.0 as i32, current.1 as i32 + 1),
            (current.0 as i32, current.1 as i32 - 1),
        ];
        for neighbor in neighbors {
            if neighbor.0 < 0 || neighbor.0 >= M as i32 || neighbor.1 < 0 || neighbor.1 >= M as i32
            {
                continue;
            }
            let neighbor = (neighbor.0 as usize, neighbor.1 as usize);
            if cells[neighbor.0][neighbor.1] == Cell::Wall {
                continue;
            }
            if self.closed.contains(&neighbor) {
                continue;
            }
            let tentative_g_score = self.g_score.get(&current).unwrap() + 1;
            if !self.open.contains(&neighbor) {
                self.open.insert(neighbor);
            } else if tentative_g_score >= *self.g_score.get(&neighbor).unwrap() {
                continue;
            }
            self.came_from.insert(neighbor, current);
            self.g_score.insert(neighbor, tentative_g_score);
        }
    }

    fn draw(&self, draw: &Draw) {
        for &(x, y) in self.open.iter() {
            let cell_xy = pt2(
                SIDE * x as f32 - 400.0 + SIDE / 2.0,
                SIDE * y as f32 - 400.0 + SIDE / 2.0,
            );
            let cell_wh = vec2(SIDE, SIDE);
            draw.rect().xy(cell_xy).wh(cell_wh).color(LIGHTGREEN);
        }
        for &(x, y) in self.closed.iter() {
            let cell_xy = pt2(
                SIDE * x as f32 - 400.0 + SIDE / 2.0,
                SIDE * y as f32 - 400.0 + SIDE / 2.0,
            );
            let cell_wh = vec2(SIDE, SIDE);
            draw.rect().xy(cell_xy).wh(cell_wh).color(LIGHTGREEN);
        }
        let reconstructed_path = self.reconstruct_path();
        for &(x, y) in reconstructed_path.iter() {
            let cell_xy = pt2(
                SIDE * x as f32 - 400.0 + SIDE / 2.0,
                SIDE * y as f32 - 400.0 + SIDE / 2.0,
            );
            let cell_wh = vec2(SIDE, SIDE);
            draw.rect().xy(cell_xy).wh(cell_wh).color(GREEN);
        }
    }

    fn reconstruct_path(&self) -> Vec<(usize, usize)> {
        let mut current = self.current;
        let mut path = vec![current];
        while self.came_from.contains_key(&current) {
            current = *self.came_from.get(&current).unwrap();
            path.push(current);
        }
        // println!("Path: {:?}", path);
        path
    }
}

struct Model {
    cells: Field,
    a_star: Search,
}

impl Model {
    fn new() -> Self {
        let mut cells = vec![];
        for _ in 0..M {
            let mut row = vec![];
            for _ in 0..M {
                row.push(rand::random());
            }
            cells.push(row);
        }
        // Set the start and end cells to empty
        cells[0][0] = Cell::Empty;
        cells[M - 1][M - 1] = Cell::Empty;
        let mut a_star = Search::A_star();
        a_star.init();
        Self { cells, a_star }
    }

    fn draw(&self, draw: &Draw) {
        for (x, row) in self.cells.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                let cell_xy = pt2(
                    SIDE * x as f32 - 400.0 + SIDE / 2.0,
                    SIDE * y as f32 - 400.0 + SIDE / 2.0,
                );
                let cell_wh = vec2(SIDE, SIDE);
                match cell {
                    Cell::Empty => draw.rect().xy(cell_xy).wh(cell_wh).color(WHITE),
                    Cell::Wall => draw.rect().xy(cell_xy).wh(cell_wh).color(BLACK),
                };
            }
        }
        self.a_star.draw(draw);
    }

    fn update(&mut self) {
        self.a_star.update(&self.cells);
    }
}

fn model(app: &App) -> Model {
    let _ = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(800, 800)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model::new()
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => model.update(),
        _ => (),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(LIGHTSLATEGRAY);
    model.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
