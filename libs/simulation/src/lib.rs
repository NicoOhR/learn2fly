use nalgebra as na;
use rand::{Rng, RngCore};

pub struct Simulation {
    world: World,
}

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}
#[derive(Debug)]
pub struct Animal {
    position: na::Point2<f32>,
    velocity: na::Vector2<f32>,
}

#[derive(Debug)]
pub struct Food {
    position: na::Point2<f32>,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }
    pub fn world(&self) -> &World {
        &self.world
    }
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40).map(|_| Animal::random(rng)).collect();
        let foods = (0..40).map(|_| Food::random(rng)).collect();
        Self { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            velocity: rng.gen(),
        }
    }

    pub fn pos(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn vel(&self) -> na::Vector2<f32> {
        self.velocity
    }
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }
    pub fn pos(&self) -> na::Point2<f32> {
        self.position
    }
}
