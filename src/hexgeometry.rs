use std::ops::Sub;
use std::ops::Add;

use std::collections::HashMap;

use direction::Direction;



#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Hexpoint {
    pub x: i32,
    pub y: i32,
    z: i32,
}

impl Hexpoint {
    
    pub fn new(x: i32, y: i32) -> Hexpoint {
        Hexpoint {
            x,
            y,
            z: -x-y,
        }
    }
    
    pub fn neighbor(&self,direction: Direction) -> Hexpoint{
        match direction {
            Direction::E  => Hexpoint::new(self.x+1,self.y  ),
            Direction::SE => Hexpoint::new(self.x  ,self.y+1),
            Direction::SW => Hexpoint::new(self.x-1,self.y+1),
            Direction::W  => Hexpoint::new(self.x-1,self.y  ),
            Direction::NW => Hexpoint::new(self.x  ,self.y-1),
            Direction::NE => Hexpoint::new(self.x+1,self.y-1),
        }
    }

    pub fn neighbors(&self) -> Vec<Hexpoint>{
        vec![Hexpoint::new(self.x+1,self.y  ),
             Hexpoint::new(self.x  ,self.y+1),
             Hexpoint::new(self.x-1,self.y+1),
             Hexpoint::new(self.x-1,self.y  ),
             Hexpoint::new(self.x  ,self.y-1),
             Hexpoint::new(self.x+1,self.y-1)]
    }

    pub fn inward_neighbors(&self) -> Vec<Hexpoint>{
    
        // This could be optimized in many ways, but 
        // I don't think it's a bottleneck anywhere...
        let mut v=Vec::new();
        for n in self.neighbors() {
            if n.ring_number() == self.ring_number() -1 {
                v.push(n);
            }
        }
        v
    }
    
    pub fn ring_number(&self) -> i32 {
	self.x.abs().max(self.y.abs().max(self.z.abs()))
    }
    
    pub fn ring(&self) -> Vec<Hexpoint> {
        let mut v = Vec::new();
        let r = self.ring_number();
        
        // top
        for i in 0..r {
            v.push(Hexpoint::new(i,-r));
        } 
        // top-right
        for i in 0..r {
            v.push(Hexpoint::new(r,-r+i));
        }
        // bottom-right
        for i in 0..r {
            v.push(Hexpoint::new(r-i,i));
        }
        // bottom
        for i in 0..r {
            v.push(Hexpoint::new(-i,r));
        } 
        // bottom-left
        for i in 0..r {
            v.push(Hexpoint::new(-r,r-i));
        }
        // top-left
        for i in 0..r {
            v.push(Hexpoint::new(i-r,-i));
        }
        v
    }
    
    pub fn direction_to(&self, other: Hexpoint) -> Direction {
        match other-*self {
            Hexpoint {x:  1, y:  0, z: -1} => Direction::E,
            Hexpoint {x:  0, y:  1, z: -1} => Direction::SE,
            Hexpoint {x: -1, y:  1, z:  0} => Direction::SW,
            Hexpoint {x: -1, y:  0, z:  1} => Direction::W,
            Hexpoint {x:  0, y: -1, z:  1} => Direction::NW,
            Hexpoint {x:  1, y: -1, z:  0} => Direction::NE,
            _ => unreachable!(),
        }
    }

}

impl Sub for Hexpoint {
    type Output = Hexpoint;

    fn sub(self, other: Hexpoint) -> Hexpoint {
        Hexpoint {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Hexpoint {
    type Output = Hexpoint;
    
    fn add(self, other: Hexpoint) -> Hexpoint {
        Hexpoint {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug)]
pub struct Hexmap<'a> {
    size: i32,
    // FIXME: this ends up making function call types
    //  *ridiculously* complicated. Need to encapsulate all that.
    pub hexes: HashMap<Hexpoint,Vec<&'a str>>,
    pub decor: HashMap<Hexpoint,Vec<&'a str>>,
}

impl<'a> Hexmap<'a> {

    pub fn new(size: i32) -> Hexmap<'a> {
        Hexmap {
            size,
            hexes: HashMap::new(),
            decor: HashMap::new()
        }
    }
    
    pub fn get_ranked(&self, orientation: Direction) -> Vec<((i32,i32),Option<&Vec<&str>>,Option<&Vec<&str>>)> {
        match orientation {
            Direction::E  => self.get_ranked_horizontal(1),
            Direction::SE => self.get_ranked_diagonal(1),
            Direction::SW => self.get_ranked_vertical(1),
            Direction::W  => self.get_ranked_horizontal(-1),
            Direction::NW => self.get_ranked_diagonal(-1),
            Direction::NE => self.get_ranked_vertical(-1),
        }    
    }
    
    fn get_ranked_horizontal(&self,flip: i32) -> Vec<((	i32,i32),Option<&Vec<&str>>,Option<&Vec<&str>>)> {
    
        let mut v: Vec<((i32,i32),Option<&Vec<&str>>,Option<&Vec<&str>>)> = Vec::new();

        // This looks super-complicated but basically it's
        // https://www.redblobgames.com/grids/hexagons/#map-storage
        // for orientation East (top-left corner to bottom-right corner)
        // or West (flip = -1)
        
        for y in 0..self.size {
            let r=flip*(y-(self.size/2));
            for x in 0..self.size {
                let q=flip*(x-(self.size/2));
                let offset=(((x-(self.size/2))*2+(y-(self.size/2))),y-(self.size/2));
                v.push((offset,self.hexes.get(&Hexpoint::new(q,r)),self.decor.get(&Hexpoint::new(q,r))));
            }
        }
        v
    }

    fn get_ranked_vertical(&self,flip: i32) -> Vec<((i32,i32),Option<&Vec<&str>>,Option<&Vec<&str>>)> {
    
        let mut v: Vec<((i32,i32),Option<&Vec<&str>>,Option<&Vec<&str>>)> = Vec::new();

        // Same as above, but we're going through columns
        // first instead of rows (effectively a 90° rotation from
        // the other function
        for y in 0..self.size {
            let q=flip*(y-(self.size/2));
            for x in 0..self.size {
                let r=flip*(x-(self.size/2));
                let offset=(-1*((x-(self.size/2))*2+(y-(self.size/2))),y-(self.size/2));
                v.push((offset,self.hexes.get(&Hexpoint::new(q,r)),self.decor.get(&Hexpoint::new(q,r))));
            }
        }
                
        v
    }

    fn get_ranked_diagonal(&self,flip: i32) -> Vec<((i32,i32),Option<&Vec<&str>>,Option<&Vec<&str>>)> {
    
        let mut v: Vec<((i32,i32),Option<&Vec<&str>>,Option<&Vec<&str>>)> = Vec::new();

        // for orientation SouthEast, top row down
        // flip for NW.
        for y in 0..self.size {
            // start pointy, get broad
            for x in 1..y {
                let r=flip*(y-self.size/2-x);
                let q=flip*(y-self.size-flip*r);
                let offset=(x*2-y,y-self.size);
                v.push((offset,self.hexes.get(&Hexpoint::new(q,r)),self.decor.get(&Hexpoint::new(q,r))));
            }
        }
        for y in self.size..self.size*2+1 {
            // broad in the middle, get pointy
            for x in 1..(self.size*2-y) {
                let r=flip*(self.size/2-x);
                let q=flip*(y-self.size-flip*r);
                let offset=(x*2-(self.size*2-y),y-self.size);
                v.push((offset,self.hexes.get(&Hexpoint::new(q,r)),self.decor.get(&Hexpoint::new(q,r))));
            }
        }
        v
    }

}
