pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
}
impl RotationDirection {
    // Gets reverse
    pub fn flip(direction: RotationDirection) -> RotationDirection {
        match direction {
            RotationDirection::Clockwise => RotationDirection::CounterClockwise,
            RotationDirection::CounterClockwise => RotationDirection::Clockwise,
        }
    }
}