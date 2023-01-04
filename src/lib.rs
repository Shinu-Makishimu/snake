mod random;

use std::collections::VecDeque;

use random::random_range;



pub type Position = (usize,usize);


#[derive(Debug)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}


#[derive(Debug)]
pub struct Snake {
    width: usize,
    height: usize,
    snake: VecDeque<Position>, //hed firts, tail is end
    direction: Direction,
    food: Position,
    loose: bool,
}


impl Snake {
    pub fn new(width: usize, height: usize) -> Self {
        Self { 
            width, 
            height, 
            snake: [((width-2).max(0), height/2)].into_iter().collect(), 
            direction: Direction::Left, 
            food: (2.min(width - 1), height / 2), 
            loose: false,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match (&self.direction, direction) {
            (Direction::Top, Direction::Right) => self.direction = Direction::Right,
            (Direction::Top, Direction::Left) => self.direction = Direction::Left,
            (Direction::Right, Direction::Top) => self.direction = Direction::Top,
            (Direction::Right, Direction::Bottom) => self.direction = Direction::Bottom,
            (Direction::Bottom, Direction::Right) => self.direction = Direction::Right,
            (Direction::Bottom, Direction::Left) => self.direction = Direction::Left,
            (Direction::Left, Direction::Top) => self.direction = Direction::Top,
            (Direction::Left, Direction::Bottom) => self.direction = Direction::Bottom,
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
    use crate::Snake;

    #[test]
    fn test_snake() {
        println!("{:?}", Snake::new(10,10));
    }
}