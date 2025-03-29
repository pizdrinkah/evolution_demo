use rand::{thread_rng, Rng};
use raylib::{color::{self, Color}, prelude::{RaylibDraw, RaylibDrawHandle, RaylibMode2D}};

use crate::{collision::circle_circle, data_structs::{PointQuadTree, PosContainer}};

const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.;

#[derive(Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32
}

#[derive(Clone, Copy)]
pub struct Circle {
    pub origin: Vector,
    pub radius: f32
}

#[derive(Clone, Copy)]
pub struct Creature {
    pub pos: Vector,
    pub vel: Vector,
    pub dir: f32,
    pub size: f32,
    pub speed: f32,
    pub smell: f32,
    pub energy: f32,
    pub colour: Color,
    pub food_requirement: f32,
    pub food_count: f32,
}

impl Creature {
    
    pub fn update(&mut self, foods: &mut PointQuadTree<Food>) {
        let mut quadrant: u8 = 0;
        
        quadrant += match self.pos.x as u32 > foods.quadrant_x {
            true  => 1,
            false => 0
        };
        quadrant += match self.pos.y as u32 > foods.quadrant_y {
            true  => 2,
            false => 0
        };

        let foods_vec: Vec<PosContainer<Food>> = match quadrant {
            0 => (*foods.q1).to_vec(),
            1 => (*foods.q2).to_vec(),
            2 => (*foods.q3).to_vec(),
            3 => (*foods.q4).to_vec(),
            _ => panic!(),
        };
        
        if foods_vec.len() > 0 {
            let foods_clone: Vec<PosContainer<Food>> = foods_vec.clone();
            let mut food_target: usize = 0;
            
            for (index, food) in foods_vec.clone().into_iter().enumerate() {
                if ((self.pos.x - food.c.pos.x).powf(2.) + (self.pos.y - food.c.pos.y).powf(2.)) < 
                   ((self.pos.x - foods_clone[food_target].c.pos.x).powf(2.) + (self.pos.y - foods_clone[food_target].c.pos.y).powf(2.)) {
                    food_target = index;
                }
            }

            if circle_circle(Circle {origin: self.pos, radius: (self.smell * (self.size / 4.))}, 
                             Circle {origin: foods_vec[food_target].c.pos, radius: 4.}) {
                if        ((foods_vec[food_target].c.pos.x) - (self.pos.x)).atan2((foods_vec[food_target].c.pos.y) - (self.pos.y)) < self.dir{
                    self.dir -= 5.4 * DEG_TO_RAD;
                } else if ((foods_vec[food_target].c.pos.x) - (self.pos.x)).atan2((foods_vec[food_target].c.pos.y) - (self.pos.y)) > self.dir{
                    self.dir += 5.4 * DEG_TO_RAD;
                }
            } else {
                self.dir += thread_rng().gen_range((-5.4 * DEG_TO_RAD)..(5.4 * DEG_TO_RAD));
            }

            if circle_circle(Circle {origin: self.pos, radius: self.size},
                             Circle {origin: foods_vec[food_target].c.pos, radius: 4.})
                            && !foods_vec[food_target].c.is_eaten {
                match quadrant {
                    0 => foods.q1[food_target].c.is_eaten = true,
                    1 => foods.q2[food_target].c.is_eaten = true,
                    2 => foods.q3[food_target].c.is_eaten = true,
                    3 => foods.q4[food_target].c.is_eaten = true,
                    _ => panic!()
                }
                self.energy += 350. * self.size;
                self.food_count += 1.;
            }
        }
        
        self.vel = Vector {
            x: self.dir.sin() * (self.speed * (self.size / 2.)),
            y: self.dir.cos() * (self.speed * (self.size / 2.)),
        };
        
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        self.energy -= ((self.speed * 5.  ) * (f32::sqrt(self.size) / 2.)) +
                       ((self.smell / 50. ) * (f32::sqrt(self.size) / 2.));
    
    }

    pub fn multiply(&mut self) -> Creature {
        let size = self.size + thread_rng().gen_range(-0.2..0.2);
        
        Creature {
            pos: self.pos,
            vel: self.vel,
            dir: self.dir,
            size,
            speed: self.speed + thread_rng().gen_range(-0.05..0.05),
            smell: self.smell + thread_rng().gen_range(-10.0..10.0),
            energy: 8000.,
            colour: color::rcolor((self.speed * 64.) as u8, (self.size * 16.) as u8, (self.smell / 2.) as u8, 255),
            food_requirement: f32::sqrt(size) / 4.,
            food_count: 0.,
        }
    }
    
    pub fn render(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle<'_>> ) {
        d.draw_circle(self.pos.x as i32, self.pos.y as i32, self.size, self.colour);
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Food {
    pub pos: Vector,
    pub is_eaten: bool,
}

impl Food {
    pub fn render(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle<'_>>,) {
        d.draw_circle(self.pos.x as i32, self.pos.y as i32, 4., color::rcolor(100, 200, 100, 255));
    }
    pub fn posify(&self) -> PosContainer<Food> {
        PosContainer { 
            c: *self, 
            x: self.pos.x as u32, 
            y: self.pos.y as u32, 
        }
    }
}

pub fn food(w: i32, h: i32) -> Food {
    Food {
        pos: Vector {
            x: thread_rng().gen_range(((w * -1) as f32)..(w as f32)),
            y: thread_rng().gen_range(((h * -1) as f32)..(h as f32)),
        },
        is_eaten: false
    }
}

pub fn creature(w: i32, h: i32) -> Creature{
    Creature {
        pos: Vector {
            x: thread_rng().gen_range(((w * -1) as f32)..(w as f32)),
            y: thread_rng().gen_range(((h * -1) as f32)..(h as f32)),
        },
        vel: Vector {x:0., y:0.},
        dir: 0.,
        size: 6.,
        food_requirement: 1.,
        speed: 2.0,
        smell: 180.,
        energy: 24000.,
        colour: color::rcolor(128, 128, 128, 255),
        food_count: 0.,
    }
}