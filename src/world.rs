// Copyright 2018 Steven Sheffey
// This file is part of heroesoftheswarm.
//
// heroesoftheswarm is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// heroesoftheswarm is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with heroesoftheswarm.  If not, see <http://www.gnu.org/licenses/>.
extern crate serde_json;
use entity::{Bullet, Swarm};
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};
/// Represents the state of the game's world
#[derive(Clone, Debug)]
pub struct World {
    /// The width of the world
    pub width: f32,
    /// The height of the world
    pub height: f32,
    /// Each swarm in the world
    /// Map of player ID to swarm
    swarms: HashMap<usize, Swarm>,
    /// Each bullet in the world
    /// TODO: vec and element swap
    bullets: Vec<Bullet>,
}
/// Functions for the world
impl World {
    /// Constructor
    /// width: the width of the world
    /// height: the height of the world
    pub fn new(width: f32, height: f32) -> Self {
        World {
            width: width,
            height: height,
            swarms: HashMap::new(),
            bullets: Vec::new(),
        }
    }
    /// Capacity constructor
    /// width: the width of the world
    /// height: the height of the world
    /// capacity: the number players to allocate space for
    /// Space is allocated for 100x the number of bullets
    pub fn with_capacity(width: f32, height: f32, capacity: usize) -> Self {
        World {
            width: width,
            height: height,
            swarms: HashMap::with_capacity(capacity),
            bullets: Vec::with_capacity(capacity * 10),
        }
    }
    /// Adds a player to the server with the given ID
    pub fn add_player(&mut self, id: usize) {
        info!("Adding player {} to the server", id);
        // Get a random position
        let (x, y) = self.random_position();
        // Get a random color
        let color = World::random_color();
        self.swarms.insert(id, Swarm::new(x, y).with_color(color));
    }

    /// Removes a player to the server with the given ID
    pub fn remove_player(&mut self, id: usize) {
        info!("Removing player {} from the server", id);
        match self.swarms.remove(&id) {
            _ => {}
        }
    }
    /// Generates a random position
    fn random_position(&self) -> (f32, f32) {
        // Get the rng
        let mut rng = thread_rng();
        // Defines a margin
        // TODO: make this an associated const
        let margin: f32 = 50.0;
        // Generate the position
        (
            rng.gen_range(margin, self.width - margin),
            rng.gen_range(margin, self.height - margin),
        )
    }
    /// Generates a random color
    fn random_color() -> (u8, u8, u8) {
        // Get the RNG
        let mut rng = thread_rng();
        // Generate the color
        (rng.gen(), rng.gen(), rng.gen())
    }
    /// Performs one "tick" of the world
    /// return: The amount of time elapsed during the tick
    /// Executes each swarm's program on itself
    /// Moves bullets
    /// Does bullet collision
    pub fn update(&mut self) -> Duration {
        // Record time at beginning of update
        let start_time = Instant::now();
        // Update each member of the swarm with its own program
        for (_id, swarm) in self.swarms.iter_mut() {
            swarm.update(self.width, self.height)
        }
        // Update each bullet
        // TODO: different logic for this as a bullet could be destroyed
        for bullet in self.bullets.iter_mut() {
            bullet.update()
        }
        // Record time at end of update and return the time elapsed
        Instant::now().duration_since(start_time)
    }
    /// Returns the world in byte representation
    /// Used to render the world on a client
    pub fn get_state(&self) -> WorldState {
        WorldState {
            swarms: self.swarms.clone(),
            bullets: self.bullets.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct WorldState {
    swarms: HashMap<usize, Swarm>,
    bullets: Vec<Bullet>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initialize_world() {
        let world = World::new(1000.0, 1000.0);
    }
}
