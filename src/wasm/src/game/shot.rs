pub mod shot {
    use crate::game::{Piece, Point, Renderer, Rect, FONT_L, FONT_CENTER, SHOT_WIDTH, SHOT_HEIGHT, State, StateMachine};

    pub const SHOT_RUNNING: [&str; 3] = ["﹏", "﹏", "﹏"];
    pub struct Shot {
        pub state_machine: StateMachine,
    }
    impl Piece for Shot {
        fn new(position: Point, velocity: Point) -> Self {
            Shot {
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
                    width: SHOT_WIDTH,
                    height: SHOT_HEIGHT,
                    character: SHOT_RUNNING,
                    font_size: FONT_L,
                    font_align: FONT_CENTER,
                }
            );
        }
    }
}