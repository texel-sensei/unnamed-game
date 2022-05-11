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

#[allow(dead_code)]
impl ActionQueue {
    pub fn new() -> Self {
        Self {
            current_frame: 0,
            last_frame: 0,
        }
    }

    /// Add the input for a new frame to the `ActionQueue`.
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
        assert!(!queue.just_pressed(Left));
        assert!(!queue.just_pressed(Right));
        assert!(!queue.just_pressed(Up));
        assert!(!queue.just_pressed(Down));
    }

    #[test]
    fn test_all_pressed() {
        let mut queue = ActionQueue::new();

        use Action::*;
        queue.update(vec![Left, Right, Up, Down]);

        assert!(queue.just_pressed(Left));
        assert!(queue.just_pressed(Right));
        assert!(queue.just_pressed(Up));
        assert!(queue.just_pressed(Down));
    }

    #[test]
    fn update_queue() {
        let mut queue = ActionQueue::new();

        use Action::*;

        queue.update(vec![Left]);

        assert!(queue.is_pressed(Left));
        assert!(!queue.is_pressed(Right));

        queue.update::<_, Action>(vec![]);

        assert!(!queue.is_pressed(Left));
    }

    #[test]
    fn held_is_not_just_pressed() {
        let mut queue = ActionQueue::new();

        use Action::*;

        queue.update(vec![Left]);

        assert!(queue.just_pressed(Left));
        assert!(!queue.just_pressed(Right));

        queue.update(vec![Left]);

        assert!(!queue.just_pressed(Left));
    }

    #[test]
    fn test_just_released() {
        let mut queue = ActionQueue::new();

        use Action::*;

        queue.update(vec![Left, Right]);

        assert!(!queue.just_released(Left));

        queue.update(vec![Left, Right]);
        assert!(!queue.just_released(Left));

        queue.update(vec![Right]);
        assert!(queue.just_released(Left));

        queue.update(vec![Right]);
        assert!(!queue.just_released(Left));
    }
}
