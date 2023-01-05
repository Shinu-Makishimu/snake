use std::collections::VecDeque;

use crate::random::random_range;



pub type Position = (usize,usize);


#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}


#[derive(Debug)]
pub struct GameSnake {
    pub width: usize,
    pub height: usize,
    pub snake: VecDeque<Position>, //hed firts, tail is end
    direction: Direction,
    next_direction: Direction,
    pub food: Position,
    loose: bool,
}


impl GameSnake {
    pub fn new(width: usize, height: usize) -> Self {
        Self { 
            width, 
            height, 
            snake: [((width-2).max(0), height/2)].into_iter().collect(), 
            direction: Direction::Left, 
            next_direction: Direction::Left, 
            food: (2.min(width - 1), height / 2), 
            loose: false,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if self.loose {
            return;
        }
        match (&self.direction, direction) {
            (Direction::Top, Direction::Right) => self.next_direction = Direction::Right,
            (Direction::Top, Direction::Left) => self.next_direction = Direction::Left,
            (Direction::Right, Direction::Top) => self.next_direction = Direction::Top,
            (Direction::Right, Direction::Bottom) => self.next_direction = Direction::Bottom,
            (Direction::Bottom, Direction::Right) => self.next_direction = Direction::Right,
            (Direction::Bottom, Direction::Left) => self.next_direction = Direction::Left,
            (Direction::Left, Direction::Top) => self.next_direction = Direction::Top,
            (Direction::Left, Direction::Bottom) => self.next_direction = Direction::Bottom,
            _ => {},
        }
    }

    pub fn is_valid(&self, (x,y): Position) -> bool {
        x < self.width && y < self.height 
    }

    pub fn tick(&mut self) {
        if self.loose || self.snake.len() == 0 {
            return;
        }

        self.direction = self.next_direction;

        let (x,y) = self.snake[0];

        let new_head = match &self.direction {
            Direction::Top => (x, y-1),
            Direction::Right => (x + 1, y),
            Direction::Bottom => (x, y + 1),
            Direction::Left => (x- 1, y),
        };


        if !self.is_valid(new_head) || self.snake.contains(&new_head) {
            self.loose = true;
        } else {
            if new_head != self.food {
                self.snake.pop_back();
            } else {
                let free_pos = (0..self.height)
                    .flat_map(|y| (0..self.width).map(move |x| (x,y)))
                    .filter(|pos| !self.snake.contains(pos))
                    .collect::<Vec<_>>();
                if free_pos.is_empty() {
                    self.loose = true;
                    return;
                }

                self.food  = free_pos[(random_range(0, free_pos.len()))]
            }
            self.snake.push_front(new_head);
        }
    }
}




#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_snake() {
        println!("{:?}", GameSnake::new(10,10));
    }
}