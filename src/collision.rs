use crate::entity_structs::Circle;

pub fn  circle_circle(c1: Circle, c2: Circle) -> bool {

    let dist_x = c1.origin.x - c2.origin.x;
    let dist_y = c1.origin.y - c2.origin.y;
    let distance = ((dist_x*dist_x) + (dist_y*dist_y)).sqrt();
  
    distance <= c1.radius + c2.radius
}