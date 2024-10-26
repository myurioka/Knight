pub mod parwn {
    use crate::game::{Piece, Point, Renderer, Rect, FONT_L, FONT_CENTER, PARWN_WIDTH, PARWN_HEIGHT, State, StateMachine};

    pub const PARWN_RUNNING: [&str; 3] = ["", "←〇  ", " ╱ ╲ "];
    pub struct Parwn {
        pub state_machine: StateMachine,
    }
    impl Piece for Parwn {
        fn new(position: Point, velocity: Point) -> Self {
            Parwn {
                state_machine: StateMachine::Running(State::new(position, velocity)),
            }
        }
        fn get_state_machine(&self) -> StateMachine {
            self.state_machine
        }
        fn set_state_machine(&mut self, _state_machine: StateMachine) {
            self.state_machine = _state_machine.update();
        }
        fn draw(&self, renderer: &Renderer) {
            renderer.draw_text(
                &Rect {
                    x: self.state_machine.context().position.x,
                    y: self.state_machine.context().position.y,
                    width: PARWN_WIDTH,
                    height: PARWN_HEIGHT,
                    character: PARWN_RUNNING,
                    font_size: FONT_L,
                    font_align: FONT_CENTER,
                }
            );
        }
    }
}