use std::collections::VecDeque;

use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::*;
use itertools::Itertools as _;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const DKBLUE: &str = "#000066";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div {
            id: "header",
            width: "100vw",
            height: "5vh",
            background_color: DKBLUE,
            font_size: "25pt",
            font_weight: "600",
            align_content: "center",
            padding: "5px",
            "Turing Machine Simulator"
        },
        div {
            margin_top: "25px",
            Machine {}
        },
    }
}

#[component]
fn Machine() -> Element {
    const TM_STRING: &str = "1RB1RZ_1LB0RC_1LC1LA";
    let mut machine = use_signal_sync(|| TuringMachine::from_string(TM_STRING));
    let mut x = use_motion(0.0)?;
    let mut trigger_step = use_signal_sync(|| -1i32);
    let mut active = use_signal_sync(|| false);
    let mut steps = use_signal(|| 0u32);
    use_effect(move || {
        if *trigger_step.read() != -1 && !*active.read() {
            *active.write() = true;
            *steps.write() += 1;
            let target = match machine.read().next_shift_is_left() {
                true => 40.,
                false => -40.,
            };
            let tween = AnimationMode::Tween(Tween {
                duration: Duration::from_millis(250),
                easing: Quad::ease_in_out,
            });
            let _ = x.animate_sequence(
                AnimationSequence::new()
                    .then(
                        0.,
                        AnimationConfig::tween_ms(0).with_on_complete(move || {
                            machine.write().step_write();
                        }),
                    )
                    .then(target, AnimationConfig::new(tween))
                    .then(
                        0.,
                        AnimationConfig::tween_ms(0).with_on_complete(move || {
                            machine.write().step_move();
                            if machine.write().step_state() {
                                *trigger_step.write() *= -1;
                            } else {
                                *trigger_step.write() = -1;
                            }
                            *active.write() = false;
                        }),
                    ),
            );
        };
    });
    rsx! {
        div {
            width: "100%",
            height: "40px",
            display: "flex",
            flex_direction: "row",
            position: "absolute",
            left: "calc(-40px * ({50} + 3) + 50vw)",
            style: "transform: translateX({x.get_value()}px)",
            for i in -50..=50 {
                TapeCell {
                    val: machine.read().cell_at(i),
                    selected: i == 0,
                }
            }
        },
        button {
            onclick: move |_| {
                *trigger_step.write() = 1;
            },
            disabled: machine.read().state.is_none(),
            margin_top: "45px",
            justify_self: "center",
            "Step"
        },
        button {
            onclick: move |_| {
                *trigger_step.write() = 2;
            },
            disabled: machine.read().state.is_none(),
            margin_top: "45px",
            justify_self: "center",
            "Run"
        },
        button {
            onclick: move |_| {
                *machine.write() = TuringMachine::from_string(TM_STRING);
                *steps.write() = 0;
            },
            margin_top: "45px",
            justify_self: "center",
            "Reset"
        },
        div {
            border: "1px solid white",
            height: "40px",
            font_size: "25pt",
            align_content: "center",
            text_align: "center",
            margin_top: "20px",
            "State: {machine.read().state_display()}\nSteps: {steps.read()}"
        },
    }
}

#[component]
fn TapeCell(val: char, selected: bool) -> Element {
    rsx! {
        div {
            border: "1px solid white",
            height: "100%",
            aspect_ratio: 1,
            font_size: "25pt",
            align_content: "center",
            text_align: "center",
            background_color: if selected {"grey"},
            "{val}"
        },
    }
}

type Instruction = ((char, char), Rule);
type Rule = (Option<char>, char, bool);

struct TuringMachine {
    state: Option<char>,

    tape: VecDeque<char>,
    offset: i32,
    default_char: char,

    instructions: Vec<Instruction>,
    curr_rule: Rule,
}

fn offset_char(c: char, o: u8) -> char {
    (c as u8 + o) as char
}

impl TuringMachine {
    fn new(instructions: Vec<Instruction>) -> Self {
        let mut me = Self {
            state: Some('A'),
            tape: VecDeque::new(),
            offset: 0,
            default_char: '0',
            instructions,
            curr_rule: (None, '1', false),
        };
        me.curr_rule = me.next_rule();
        me
    }

    fn from_string(string: &str) -> Self {
        Self::new(
            string
                .split('_')
                .enumerate()
                .flat_map(|(sn, s)| {
                    s.chars()
                        .chunks(3)
                        .into_iter()
                        .enumerate()
                        .map(|(tn, mut inst)| {
                            let write = inst.next().unwrap();
                            let dir = inst.next().unwrap();
                            let new = inst.next().unwrap();
                            (
                                (offset_char('A', sn as u8), offset_char('0', tn as u8)),
                                ((new != 'Z').then_some(new), write, dir == 'L'),
                            )
                        })
                        .collect_vec()
                })
                .collect(),
        )
    }

    fn shift_left(&mut self) {
        self.offset -= 1;
    }

    fn shift_right(&mut self) {
        self.offset += 1;
    }

    fn write(&mut self, c: char) {
        while self.offset < 0 {
            self.offset += 1;
            self.tape.push_front(self.default_char);
        }
        while self.offset >= self.tape.len() as i32 {
            self.tape.push_back(self.default_char);
        }
        self.tape[self.offset as usize] = c;
    }

    fn next_rule(&self) -> Rule {
        self.instructions
            .iter()
            .find(|((s, t), _)| self.state == Some(*s) && self.cell_at(0) == *t)
            .unwrap()
            .1
    }

    fn next_shift_is_left(&self) -> bool {
        self.curr_rule.2
    }

    fn step_write(&mut self) {
        self.write(self.curr_rule.1);
    }

    fn step_move(&mut self) {
        if self.curr_rule.2 {
            self.shift_left();
        } else {
            self.shift_right();
        }
    }

    fn step_state(&mut self) -> bool {
        self.state = self.curr_rule.0;
        if self.state.is_some() {
            self.curr_rule = self.next_rule();
            true
        } else {
            false
        }
    }

    fn cell_at(&self, pos: i32) -> char {
        self.tape
            .get((self.offset + pos) as usize)
            .copied()
            .unwrap_or(self.default_char)
    }

    fn state_display(&self) -> String {
        self.state
            .map(|s| s.to_string())
            .unwrap_or_else(|| "HALT".to_string())
    }
}
