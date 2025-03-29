use std::ops::Div;

use data_structs::PointQuadTree;
use entity_structs::*;
use raylib::{ffi::GetMouseWheelMove, prelude::*};
mod collision;

mod entity_structs;
mod data_structs;
fn main() {

    let world_size: i32 = 8192;
    

    let mut camx: f32 = 0.;
    let mut camy: f32 = 0.; 
    let mut camzoom: f32 = 0.5;


    let mut creatures: Vec<Creature> = vec![];
    let mut foods: PointQuadTree<Food> = PointQuadTree::new(world_size as u32);


    for _i in 0..100 {
        let creature = creature(world_size, world_size);
        creatures.push(creature);
    }

    for _i in 0..5 {
        let food: Food = food(world_size, world_size);
        foods.push(food.posify());
    }
    

    let (mut rl, thread) = raylib::init()
        .size(1920, 1080)
        .title("Evolution Tech Demo")
        .build();
    
    let mut food_timer = 0;
    
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        let cam:  Camera2D = Camera2D {
            offset: Vector2 {x:0., y:0.},
            target: Vector2 {x:camx, y:camy},
            rotation: 0.,
            zoom: camzoom
        };
        
        if rl.is_key_down(KeyboardKey::KEY_W) {
            camy += (-1920. * (0.5 / camzoom)) * dt;
        }
        
        if rl.is_key_down(KeyboardKey::KEY_S) {
            camy += ( 1920. * (0.5 / camzoom)) * dt;
        }
        
        if rl.is_key_down(KeyboardKey::KEY_D) {
            camx += ( 1920. * (0.5 / camzoom)) * dt;
        }
        
        if rl.is_key_down(KeyboardKey::KEY_A) {
            camx += (-1920. * (0.5 / camzoom)) * dt;
        }
        camzoom *= 1. + (unsafe {
            GetMouseWheelMove() / 10.
        });
        
        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        {
            let mut cd: RaylibMode2D<'_, RaylibDrawHandle<'_>> = d.begin_mode2D(cam);

            for creature in 0..creatures.len() {
                creatures[creature].update(&mut foods);
                creatures[creature].render(&mut cd);
                if creatures[creature].food_count >= creatures[creature].food_requirement {
                    creatures[creature].food_count -= creatures[creature].food_requirement;
                    creatures.push(creatures[creature].clone().multiply());
                }
            }

            creatures.retain(|&i| i.energy > 0.);

            foods.retain(|&i| !i.is_eaten);
            
            foods.iter(|f| f.render(&mut cd));

            if foods.len() < 640 {
                food_timer += 1;
                if food_timer == 2 {
                    let food: Food = food(world_size, world_size);
                    foods.push(food.posify());
                    food_timer = 0;
                }
                
            }
        }
        
        let creature_count = creatures.len();
        
        let mut av_size = 0.;
        creatures.clone().into_iter().for_each(|c| av_size += c.size);
        av_size = av_size.div(creatures.len() as f32);
        
        let mut av_speed = 0.;
        creatures.clone().into_iter().for_each(|c| av_speed += c.speed);
        av_speed = av_speed.div(creatures.len() as f32);
        
        let mut av_smell = 0.;
        creatures.clone().into_iter().for_each(|c| av_smell += c.smell);
        av_smell = av_smell.div(creatures.len() as f32);
        
        d.draw_text(&creature_count.to_string(), 10, 10, 20, Color::WHITE);
        d.draw_text(&av_size       .to_string(), 10, 35, 20, Color::WHITE);
        d.draw_text(&av_speed      .to_string(), 10, 60, 20, Color::WHITE);
        d.draw_text(&av_smell      .to_string(), 10, 85, 20, Color::WHITE);
        
        d.draw_fps(10, 110);
    }
}