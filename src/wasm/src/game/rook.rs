pub mod rook {
    use crate::game::{Piece, Point, Renderer, Rect, Running, FONT_L, FONT_LEFT, ROOK_WIDTH, ROOK_HEIGHT, State, StateMachine, Context};

/* <-- CONSTANT VALUE */
    const ROOK: [&str; 3] = ["      (,,(,",
                               "^^^('(〇)(((`/",
                               "___))--.))`"];
    const ROOK_BODY: [&str; 3] = ["      (,,(,  ",
                               "   (,'(((((((`/",
                               "  ,')),--.))`"];

    #[derive(Clone)]
    pub struct Rook {
        pub state_machine: StateMachine,
    }
    impl Piece for Rook {
        fn new(position: Point, velocity: Point) -> Self {
            Rook {
                state_machine: StateMachine::Running(State {
                    context: Context {
                        position: position,
                        velocity: velocity,
                    },
                    _state: Running {},
                })
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
                    width: ROOK_WIDTH,
                    height: ROOK_HEIGHT,
                    character: ROOK,
                    font_size: FONT_L,
                    font_align: FONT_LEFT,
                }
            );
            renderer.draw_text(
                &Rect {
                    x: self.state_machine.context().position.x + 80,
                    y: self.state_machine.context().position.y + 50,
                    width: ROOK_WIDTH,
                    height: ROOK_HEIGHT,
                    character: ROOK_BODY,
                    font_size: FONT_L,
                    font_align: FONT_LEFT,
                }
            );
        }
    }
}