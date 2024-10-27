pub mod bishop {
    use crate::game::{Piece, Point, Renderer, Rect, FONT_L, FONT_CENTER, BISHOP_WIDTH, BISHOP_HEIGHT,State, StateMachine};

    pub const BISHOP_RUNNING: [&str; 3] = ["╲♦╱", " ╱ ╲ ", "  ⎺  "];

    pub struct Bishop {
        pub state_machine: StateMachine,
    }
    impl Piece for Bishop {
        fn new(position: Point, velocity: Point) -> Self {
            Bishop {
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
                    x: self.state_machine.context().position.x + BISHOP_WIDTH / 2,
                    y: self.state_machine.context().position.y + BISHOP_HEIGHT / 2,
                    width: BISHOP_WIDTH,
                    height: BISHOP_HEIGHT,
                    character: BISHOP_RUNNING,
                    font_size: FONT_L,
                    font_align: FONT_CENTER,
                },
            );
        }
    }
}