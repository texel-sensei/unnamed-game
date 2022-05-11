use std::borrow::Borrow;

use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Left = 0x01,
    Right = 0x02,
    Up = 0x04,
    Down = 0x08,
}

#[derive(Component)]
pub struct ActionQueue {
    current_frame: u8,
    last_frame: u8,
}

fn is_pressed(bitset: u8, action: Action) -> bool {
    bitset & (action as u8) == (action as u8)
}

impl ActionQueue {
    pub fn new() -> Self {
        Self {
            current_frame: 0,
            last_frame: 0,
        }
    }

    /// Add the input for a new frame to the ActionQueue.
    pub fn update<I, E>(&mut self, currently_pressed: I)
    where
        I: IntoIterator<Item = E>,
        E: Borrow<Action>,
    {
        self.last_frame = self.current_frame;
        self.current_frame = currently_pressed
            .into_iter()
            .map(|a| *a.borrow() as u8)
            .sum();
    }

    pub fn just_pressed(&self, action: Action) -> bool {
        is_pressed(self.current_frame, action) && !is_pressed(self.last_frame, action)
    }

    pub fn is_pressed(&self, action: Action) -> bool {
        is_pressed(self.current_frame, action)
    }

    pub fn just_released(&self, action: Action) -> bool {
        !is_pressed(self.current_frame, action) && is_pressed(self.last_frame, action)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_queue_has_nothing_pressed() {
        let queue = ActionQueue::new();

        use Action::*;
        assert_eq!(queue.just_pressed(Left), false);
        assert_eq!(queue.just_pressed(Right), false);
        assert_eq!(queue.just_pressed(Up), false);
        assert_eq!(queue.just_pressed(Down), false);
    }

    #[test]
    fn test_all_pressed() {
        let mut queue = ActionQueue::new();

        use Action::*;
        queue.update(vec![Left, Right, Up, Down]);

        assert_eq!(queue.just_pressed(Left), true);
        assert_eq!(queue.just_pressed(Right), true);
        assert_eq!(queue.just_pressed(Up), true);
        assert_eq!(queue.just_pressed(Down), true);
    }

    #[test]
    fn update_queue() {
        let mut queue = ActionQueue::new();

        use Action::*;

        queue.update(vec![Left]);

        assert_eq!(queue.is_pressed(Left), true);
        assert_eq!(queue.is_pressed(Right), false);

        queue.update::<_, Action>(vec![]);

        assert_eq!(queue.is_pressed(Left), false);
    }

    #[test]
    fn held_is_not_just_pressed() {
        let mut queue = ActionQueue::new();

        use Action::*;

        queue.update(vec![Left]);

        assert_eq!(queue.just_pressed(Left), true);
        assert_eq!(queue.just_pressed(Right), false);

        queue.update(vec![Left]);

        assert_eq!(queue.just_pressed(Left), false);
    }

    #[test]
    fn test_just_released() {
        let mut queue = ActionQueue::new();

        use Action::*;

        queue.update(vec![Left, Right]);

        assert_eq!(queue.just_released(Left), false);

        queue.update(vec![Left, Right]);
        assert_eq!(queue.just_released(Left), false);

        queue.update(vec![Right]);
        assert_eq!(queue.just_released(Left), true);

        queue.update(vec![Right]);
        assert_eq!(queue.just_released(Left), false);
    }
}
