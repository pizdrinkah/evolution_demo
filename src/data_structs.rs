
#[derive(Clone, Copy)]
pub struct PosContainer<T> {
    pub c: T,
    pub x: u32,
    pub y: u32
}

#[derive(Clone)]
pub struct PointQuadTree<T> {
    pub quadrant_x: u32,
    pub quadrant_y: u32,
    pub q1: Vec<PosContainer<T>>,
    pub q2: Vec<PosContainer<T>>,
    pub q3: Vec<PosContainer<T>>,
    pub q4: Vec<PosContainer<T>>,
}

impl<T> PointQuadTree<T> {
    /// creates an empty PointQuadTree of the specified size
    pub fn new(size: u32) -> PointQuadTree<T> {
        PointQuadTree { 
            quadrant_x: size / 2, 
            quadrant_y: size / 2, 
            q1: vec![], 
            q2: vec![], 
            q3: vec![], 
            q4: vec![] 
        }
    }
    
    /// adds an element to the quad tree, pushing it to the appropriate quadrant.
    pub fn push(&mut self, to_push: PosContainer<T>) {
        let mut quadrant: u8 = 0;
        
        quadrant += match to_push.x as u32 > self.quadrant_x {
            true  => 1,
            false => 0
        };
        quadrant += match to_push.y as u32 > self.quadrant_y {
            true  => 2,
            false => 0
        };

        match quadrant {
            0 => self.q1.push(to_push),
            1 => self.q2.push(to_push),
            2 => self.q3.push(to_push),
            3 => self.q4.push(to_push),
            _ => panic!(),
        }
    }

    /// implementation of Vec.retain() for PointQuadTree
    pub fn retain<F>(&mut self, mut f: F) 
    where 
    F: FnMut(&T) -> bool {
        self.q1.retain_mut(|elem| f(&elem.c));
        self.q2.retain_mut(|elem| f(&elem.c));
        self.q3.retain_mut(|elem| f(&elem.c));
        self.q4.retain_mut(|elem| f(&elem.c));
    }

    /// implementation of vec.into_iter().for_each() for PointQuadTree.
    /// note that iter() operates on the datatype contained within PosContainer and not the PosContainer itself.
    pub fn iter<F>(&mut self, mut f: F) 
    where 
    F: FnMut(&T) -> () {
        let quad1 = &self.q1;
        let quad2 = &self.q2;
        let quad3 = &self.q3;
        let quad4 = &self.q4;
        quad1.into_iter().for_each(|elem| f(&elem.c));
        quad2.into_iter().for_each(|elem| f(&elem.c));
        quad3.into_iter().for_each(|elem| f(&elem.c));
        quad4.into_iter().for_each(|elem| f(&elem.c));
    }

    /// gets the total length of all 4 quadrants of the quad tree.
    pub fn len(&mut self) -> usize {
        self.q1.len() +
        self.q2.len() +
        self.q3.len() +
        self.q4.len()
    }
}