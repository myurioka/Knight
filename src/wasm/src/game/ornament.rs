pub mod ornament {
    use crate::game::{Piece, Point, Renderer, Rect, FONT_L, FLOOR_HEIGHT, FONT_CENTER, STAGE_RIGHT, State, StateMachine};

/* <-- CONSTANT VALUE */
    const OBJ_X: i16 = 250;
    const OBJ_Y: i16 = 120;
    const OBJ_WIDTH: i16 = 30;
    const OBJ_HEIGHT: i16 = 10;
    const OBJ_SPACE: i16 = 150;
    const OBJS_NUMBER: i16 = 6;
    const OBJS: [&str; 3] = ["││", "││", "││",];
    const OBJSBASE: [&str; 3] = ["││", "││", "┌─┐"];
    const FLOOR_X: i16 = -50;
    const FLOOR_Y: i16 = FLOOR_HEIGHT + 75;
    const FLOOR_SPACE: i16 = 95;
    const FLOOR: [&str; 3] = ["───","",""];
    const DECO_X: i16 = 90;
    const DECO_Y: i16 = 80;
    const DECO_SPACE: i16 = 150;
    const DECO: [&str; 3] = ["Ξ","",""];
    const GOAL_X: i16 = STAGE_RIGHT + 250;
    const GOAL_Y: i16 = 290;
    const GOAL: [&str; 3] = ["┌──┐", "├ ◆ ┤","└──┘"];

    pub struct Ornament {
        pub state_machine: StateMachine,
    }
    impl Piece for Ornament {
        fn new(position: Point, velocity: Point) -> Self {
            Ornament {
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
            for i in 0..OBJS_NUMBER {
                renderer.draw_text(
                    &Rect {
                        x: (OBJ_X +  OBJ_SPACE * i) as i16 + self.state_machine.context().position.x,
                        y: (OBJ_Y + 20 + 30*(i%2)) as i16,
                        width: OBJ_WIDTH,
                        height: OBJ_HEIGHT,
                        character: OBJS,
                        font_size: FONT_L,
                        font_align: FONT_CENTER,
                    }
                );
                renderer.draw_text(
                    &Rect {
                        x: (OBJ_X + OBJ_SPACE * i) as i16 + self.state_machine.context().position.x,
                        y: (OBJ_Y + 110 + 30*(i%2)) as i16,
                        width: OBJ_WIDTH,
                        height: OBJ_HEIGHT,
                        character: OBJSBASE,
                        font_size: FONT_L,
                        font_align: FONT_CENTER,
                    }
                );
                renderer.draw_text(
                    &Rect {
                        x: (DECO_X + DECO_SPACE * i) as i16 + self.state_machine.context().position.x,
                        y: (DECO_Y ) as i16,
                        width: OBJ_WIDTH,
                        height: OBJ_HEIGHT,
                        character: DECO,
                        font_size: FONT_L,
                        font_align: FONT_CENTER,
                    }
                );
            }
            // FLOOR LINE OBJECT
            for i in 0..19 {
                renderer.draw_text(
                    &Rect {
                        x: (FLOOR_X) as i16 + FLOOR_SPACE * i + self.state_machine.context().position.x,
                        y: (FLOOR_Y) as i16,
                        width: OBJ_WIDTH,
                        height: OBJ_HEIGHT,
                        character: FLOOR,
                        font_size: FONT_L,
                        font_align: FONT_CENTER,
                    }
                );
            };
            // GOAL
            renderer.draw_text(
                &Rect {
                    x: (GOAL_X) as i16 + self.state_machine.context().position.x,
                    y: (GOAL_Y) as i16,
                    width: OBJ_WIDTH,
                    height: OBJ_HEIGHT,
                    character: GOAL,
                    font_size: FONT_L,
                    font_align: FONT_CENTER,
                }
            );
        }
    }
}