pub mod knight {
    //use crate::engine::Point;
    use crate::game::{Point, Renderer, Rect, FLOOR_HEIGHT, FONT_L,FONT_CENTER};

/* <-- CONSTANT VALUE */
    const KNIGHT_RUNNING: [&str; 3]  = ["  〇  ", " \u{20D2}╱∎⛉ ", "  ╱ ╲  "];
    const KNIGHT_KNOCKED: [&str; 3] = ["", "", "〇\u{2571}\u{220E}\u{26C9}"];
    const KNIGHT_ATTACKING: [&str; 3] = [" 〇  ", "\u{26C9}\u{220E} \u{2500}", " \u{2571} \u{2572}  "];
    const KNIGHT_CLEAR: [&str; 3]  = ["＼〇／", " 　\u{220E}　 ", "　\u{2571} \u{2572}　"];
    const BRANK: [&str; 3] = ["", "", ""];
    const GRAVITY: i16 = 2;
    const JUMPING_HEIGHT:i16 = 110;
    const KNIGHT_WIDTH: i16 = 30;
    const KNIGHT_HEIGHT: i16 = 60;
    const ATTACK_WIDTH: i16 =  110;
    const ATTACK_HEIGHT: i16 = 60;
    const ATTACK_TIME: i16 = 8;
/* CONSTANT VALUE --> */

    pub struct Knight {
        pub state_machine: KnightStateMachine,
    }
    impl Knight {
        pub fn new(position: Point, velocity: Point) -> Self {
            Knight {
                state_machine: KnightStateMachine::Idle(KnightState::new(position, velocity)),
            }
        }
        fn state_machine(&self) -> KnightStateMachine {
            self.state_machine
        }
        fn set_state_machine(&mut self, _state_machine: KnightStateMachine) {
            self.state_machine = _state_machine.update();
        }
        pub fn velocity(&self) -> Point{
            self.state_machine().context().velocity
        }
        pub fn run(&mut self, velocity:Point) {
            let _from_state_machine = self.state_machine();
            let _to_state_machine = _from_state_machine.transition(Event::Run(velocity));
            self.set_state_machine(_to_state_machine);
        }
        pub fn jump(&mut self) {
            self.state_machine = self.state_machine.transition(Event::Jump);
        }
        pub fn knocked(&mut self) {
            self.state_machine = self.state_machine.transition(Event::Knocked);
        }
        pub fn attack(&mut self ) {
            self.state_machine = self.state_machine.transition(Event::Attack);
        }
        pub fn clear(&mut self) {
            self.state_machine = self.state_machine.transition(Event::Clear);
        }
        pub fn bounding_box(&self) -> Rect {
            Rect {
                x: self.state_machine.context().position.x - KNIGHT_WIDTH / 2,
                y: self.state_machine.context().position.y - KNIGHT_HEIGHT / 2,
                width: KNIGHT_WIDTH,
                height: KNIGHT_HEIGHT,
                character: BRANK,
                font_size: FONT_L,
                font_align: FONT_CENTER,
            }
        }
        pub fn attacking_box(&self) -> Rect {
            Rect {
                x: self.state_machine.context().position.x,
                y: self.state_machine.context().position.y,
                width: ATTACK_WIDTH,
                height: ATTACK_HEIGHT,
                character: BRANK,
                font_size: FONT_L,
                font_align: FONT_CENTER,
            }
        }
        pub fn update(&mut self) {
            self.state_machine = self.state_machine.update();
        }
        pub fn draw(&self, renderer: &Renderer) {
            match self.state_machine {
                KnightStateMachine::Knocked(_state) => {
                    renderer.draw_text(
                        &Rect {
                            x: self.state_machine.context().position.x,
                            y: self.state_machine.context().position.y,
                            width: KNIGHT_WIDTH,
                            height: KNIGHT_HEIGHT,
                            character: KNIGHT_KNOCKED,
                            font_size: FONT_L,
                            font_align: FONT_CENTER,
                        },
                    );
                }
                KnightStateMachine::Attacking(_state) => {
                    renderer.draw_text(
                        &Rect {
                            x: self.state_machine.context().position.x,
                            y: self.state_machine.context().position.y,
                            width: KNIGHT_WIDTH,
                            height: KNIGHT_HEIGHT,
                            character: KNIGHT_ATTACKING,
                            font_size: FONT_L,
                            font_align: FONT_CENTER,
                        },
                    );
                }
                KnightStateMachine::Clear(_state) => {
                    renderer.draw_text(
                        &Rect {
                            x: self.state_machine.context().position.x,
                            y: self.state_machine.context().position.y,
                            width: KNIGHT_WIDTH,
                            height: KNIGHT_HEIGHT,
                            character: KNIGHT_CLEAR,
                            font_size: FONT_L,
                            font_align: FONT_CENTER,
                        },
                    );
                }
                _ => {
                    renderer.draw_text(
                        &Rect {
                            x: self.state_machine.context().position.x,
                            y: self.state_machine.context().position.y,
                            width: KNIGHT_WIDTH,
                            height: KNIGHT_HEIGHT,
                            character: KNIGHT_RUNNING,
                            font_size: FONT_L,
                            font_align: FONT_CENTER,
                        },
                    );
                }
            }
        }
    }

    #[derive(Copy, Clone)]
    pub enum KnightStateMachine{
        Idle(KnightState<Idle>),
        Running(KnightState<Running>),
        Jumping(KnightState<Jumping>),
        Falling(KnightState<Falling>),
        Attacking(KnightState<Attacking>),
        Knocked(KnightState<Knocked>),
        Clear(KnightState<Clear>),
    }

    pub enum Event {
        Run(Point),
        Update,
        Jump,
        Knocked,
        Attack,
        Clear,
    }

    impl KnightStateMachine {
        fn transition(self, event: Event) -> Self {
            match (self, event) {
                (KnightStateMachine::Idle(state), Event::Update) => state.update().into(),
                (KnightStateMachine::Idle(state), Event::Run(velocity)) => state.run(velocity).into(),
                (KnightStateMachine::Idle(state), Event::Jump) => state.jump().into(),
                (KnightStateMachine::Idle(state), Event::Knocked) => state.knocked().into(),
                (KnightStateMachine::Idle(state), Event::Attack) => state.attack().into(),
                (KnightStateMachine::Running(state), Event::Run(velocity)) => state.run(velocity).into(),
                (KnightStateMachine::Running(state), Event::Update) => state.update().into(),
                (KnightStateMachine::Running(state), Event::Jump) => state.jump().into(),
                (KnightStateMachine::Running(state), Event::Knocked) => state.knocked().into(),
                (KnightStateMachine::Running(state), Event::Attack) => state.attack().into(),
                (KnightStateMachine::Running(state), Event::Clear) => state.clear().into(),
                (KnightStateMachine::Jumping(state), Event::Update) => state.update().into(),
                (KnightStateMachine::Jumping(state), Event::Knocked) => state.update().into(),
                (KnightStateMachine::Jumping(state), Event::Attack) => state.attack().into(),
                (KnightStateMachine::Falling(state), Event::Update) => state.update().into(),
                (KnightStateMachine::Falling(state), Event::Knocked) => state.update().into(),
                (KnightStateMachine::Falling(state), Event::Attack) => state.attack().into(),
                (KnightStateMachine::Attacking(state), Event::Update) => state.update().into(),
            _ => self,
            }
        }
        pub fn context(&self) -> &KnightContext {
            match self {
                KnightStateMachine::Idle(state) => state.context(),
                KnightStateMachine::Running(state) => state.context(),
                KnightStateMachine::Jumping(state) => state.context(),
                KnightStateMachine::Falling(state) => state.context(),
                KnightStateMachine::Knocked(state) => state.context(),
                KnightStateMachine::Attacking(state) => state.context(),
                KnightStateMachine::Clear(state) => state.context(),
            }
        }
        fn update(self) -> Self {
            self.transition(Event::Update)
        }
    }
    impl From<KnightState<Idle>> for KnightStateMachine{
        fn from(state: KnightState<Idle>) -> Self {
            KnightStateMachine::Idle(state)
        }
    }
    impl From<KnightState<Running>> for KnightStateMachine{
        fn from(state: KnightState<Running>) -> Self {
            KnightStateMachine::Running(state)
        }
    }
    impl From<KnightState<Jumping>> for KnightStateMachine{
        fn from(state: KnightState<Jumping>) -> Self {
            KnightStateMachine::Jumping(state)
        }
    }
    impl From<KnightState<Falling>> for KnightStateMachine{
        fn from(state: KnightState<Falling>) -> Self {
            KnightStateMachine::Falling(state)
        }
    }
    impl From<JumpingEndState> for KnightStateMachine {
        fn from(state: JumpingEndState) -> Self {
            match state {
                JumpingEndState::Jumping(jumping) => jumping.into(),
                JumpingEndState::Ending(ending) => ending.into(),
                JumpingEndState::Knocked(knocked) => knocked.into(),
                JumpingEndState::Attacking(attack) => attack.into(),
            }
        }
    }
    impl From<FallingEndState> for KnightStateMachine {
        fn from(state: FallingEndState) -> Self {
            match state {
                FallingEndState::Falling(falling) => falling.into(),
                FallingEndState::Landing(landing) => landing.into(),
                FallingEndState::Knocked(knocked) => knocked.into(),
                FallingEndState::Attacking(attack) => attack.into(),
            }
        }
    }
    impl From<KnightState<Knocked>> for KnightStateMachine{
        fn from(state: KnightState<Knocked>) -> Self {
            KnightStateMachine::Knocked(state)
        }
    }
    impl From<KnightState<Attacking>> for KnightStateMachine{
        fn from(state: KnightState<Attacking>) -> Self {
            KnightStateMachine::Attacking(state)
        }
    }
    impl From<AttackingEndState> for KnightStateMachine {
        fn from(state: AttackingEndState) -> Self {
            match state {
                AttackingEndState::Attacking(attacking) => attacking.into(),
                AttackingEndState::AttackingBack(attackingback) => attackingback.into(),
            }
        }
    }
    impl From<KnightState<Clear>> for KnightStateMachine{
        fn from(state: KnightState<Clear>) -> Self {
            KnightStateMachine::Clear(state)
        }
    }
    #[derive(Copy, Clone)]
    pub struct KnightState<S> {
        context: KnightContext,
        _state: S,
    }
    impl<S> KnightState<S> {
        pub fn context(&self) -> &KnightContext {
            &self.context
        }
        fn update_context(&mut self){
            self.context = self.context.update();
        }
    }

    #[derive(Copy, Clone)]
    pub struct Idle;
    impl KnightState<Idle> {
        pub fn new(position: Point, velocity: Point) -> Self {
             KnightState {
                context: KnightContext {
                    frame: 0,
                    position: position,
                    velocity: velocity,
                },
                _state: Idle {},
             }
        }
        pub fn update(mut self) -> KnightState<Idle> {
            self.update_context();
            self
        }
        pub fn run(self, velocity:Point) -> KnightState<Running> {
            KnightState {
                context: self.context.run(velocity),
                _state: Running{},
            }
        }
        pub fn jump(self) -> KnightState<Jumping> {
            KnightState {
                context: self.context.jump(),
                _state: Jumping {},
            }
        }
        pub fn knocked(self) -> KnightState<Knocked> {
            KnightState {
                context: self.context.knocked(),
                _state: Knocked {},
            }
        }
        pub fn attack(self) -> KnightState<Attacking> {
            KnightState {
                context: self.context.attack(),
                _state: Attacking {},
            }
        }
        /*
        pub fn clear(self) -> KnightState<Clear> {
            KnightState {
                context: self.context.clear(),
                _state: Clear {},
            }
        }
        */
    }
    #[derive(Copy, Clone)]
    pub struct Running;
    impl KnightState<Running> {
        pub fn update(mut self)  -> KnightState<Running> {
            if self.context.position.y < FLOOR_HEIGHT {
                self.context.position.y += GRAVITY;
            } 
            self.update_context();
            self
        }
        pub fn run(self, velocity:Point) -> KnightState<Running> {
            KnightState {
                context: self.context.run(velocity),
                _state: Running{},
            }
        }
        pub fn jump(self) -> KnightState<Jumping> {
            KnightState {
                context: self.context.jump(),
                _state: Jumping{},
            }
        }
        pub fn knocked(self) -> KnightState<Knocked> {
            KnightState {
                context: self.context.knocked(),
                _state: Knocked {},
            }
        }
        pub fn attack(self) -> KnightState<Attacking> {
            KnightState {
                context: self.context.attack(),
                _state: Attacking {},
            }
        }
        pub fn clear(self) -> KnightState<Clear> {
            KnightState {
                context: self.context.clear(),
                _state: Clear {},
            }
        }
    }

    #[derive(Copy, Clone)]
    pub struct Jumping;
    pub enum JumpingEndState {
        Jumping(KnightState<Jumping>),
        Ending(KnightState<Falling>),
        Knocked(KnightState<Knocked>),
        Attacking(KnightState<Attacking>),
    }
    impl KnightState<Jumping> {
        pub fn update(mut self) -> JumpingEndState {
            self.context.position.y -= GRAVITY;
            if self.context.position.y < JUMPING_HEIGHT {
                return JumpingEndState::Ending(self.end());
            };
            JumpingEndState::Jumping(self)
        }
        fn end(self) -> KnightState<Falling> {
            KnightState {
                context: *self.context(),
                _state: Falling {},
            }
        }
        pub fn attack(self) -> KnightState<Attacking> {
            KnightState {
                context: self.context.attack(),
                _state: Attacking {},
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct Falling;
    pub enum FallingEndState {
        Falling(KnightState<Falling>),
        Landing(KnightState<Running>),
        Knocked(KnightState<Knocked>),
        Attacking(KnightState<Attacking>),
    }
    impl KnightState<Falling> {
        pub fn update(mut self) -> FallingEndState {
            if self.context.position.y < FLOOR_HEIGHT {
                self.context.position.y += GRAVITY;
                return FallingEndState::Landing(self.land());
            } 
            FallingEndState::Falling(self)
        }
        fn land(self) -> KnightState<Running> {
            KnightState {
                context: *self.context(),
                _state: Running {},
            }
        }
        pub fn attack(self) -> KnightState<Attacking> {
            KnightState {
                context: self.context.attack(),
                _state: Attacking {},
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct Knocked;
    impl KnightState<Knocked> {
    }

    #[derive(Copy, Clone)]
    pub struct Clear;
    impl KnightState<Clear> {
    }

    #[derive(Copy, Clone)]
    pub struct Attacking;
    pub enum AttackingEndState {
        Attacking(KnightState<Attacking>),
        AttackingBack(KnightState<Running>),
    }
    impl KnightState<Attacking> {

        pub fn update(mut self) -> AttackingEndState {
            self.context.frame += 1;
            if self.context.frame > ATTACK_TIME {
                self.context.frame = 0; 
                return AttackingEndState::AttackingBack(self.end());
            }
            AttackingEndState::Attacking(self)
        }
        pub fn end(self) -> KnightState<Running> {
            KnightState {
                context: *self.context(),
                 _state: Running {},
            }
        }
    }

    #[derive(Copy, Clone)]
    pub struct KnightContext {
        pub frame: i16,
        pub position: Point,
        pub velocity: Point
    }
    impl KnightContext {
        pub fn update(self) -> Self {
            self
        }
        fn jump(self) -> Self {
            self
        }
        fn run(mut self, velocity: Point) -> Self {
            self.velocity = velocity;
            self
        }
        fn knocked(self) -> Self {
            self
        }
        fn attack(self) -> Self {
            self
        }
        fn clear(self) -> Self {
            self
        }
    }
}