use std::collections::VecDeque;

use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::*;
use itertools::Itertools as _;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const DKBLUE: &str = "#000066";
const DKGREY: &str = "#222222";

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

fn select<T>(b: bool, if_t: T, if_f: T) -> T {
    if b { if_t } else { if_f }
}

#[component]
fn Machine() -> Element {
    const TM_STRING: &str = "1RB1RZ_1LB0RC_1LC1LA";
    let mut machine = use_signal_sync(|| TuringMachine::from_string(TM_STRING).unwrap());
    let mut x = use_motion(0.0)?;
    let mut trigger_step = use_signal_sync(|| -1i32);
    let mut active = use_signal_sync(|| false);
    let mut trigger_reset = use_signal_sync(|| false);
    let mut machine_code = use_signal(|| TM_STRING.to_string());
    let mut do_write = use_signal(|| true);
    use_effect(move || {
        if *trigger_step.read() != -1 && !*active.read() {
            *active.write() = true;
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
                            machine.write().step_state();
                        }),
                    )
                    .then(target, AnimationConfig::new(tween))
                    .then(0., AnimationConfig::tween_ms(0))
                    .on_complete(move || {
                        machine.write().step_move();
                        if *trigger_reset.read() {
                            machine.write().reset();
                            *trigger_reset.write() = false;
                            *trigger_step.write() = -1;
                        } else if machine.write().step_next() {
                            *trigger_step.write() *= -1;
                        } else {
                            *trigger_step.write() = -1;
                        }
                        *active.write() = false;
                    }),
            );
        };
    });
    use_effect(move || {
        let curr_code = machine.read().get_code();
        if *do_write.read() && curr_code != *machine_code.read() {
            machine_code.set(curr_code);
        }
    });
    rsx! {
        document::Title {"Turing Machine Simulator"},
        div {
            overflow_x: "hidden",
            overflow_y: "clip",
            width: "100%",
            height: "60px",
            position: "relative",
            div {
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
        },
        button {
            onclick: move |_| {
                *trigger_step.write() = 1;
            },
            disabled: machine.read().state.is_none(),
            padding: "10px",
            font_size: "20pt",
            "Step"
        },
        button {
            onclick: move |_| {
                *trigger_step.write() = 2;
            },
            disabled: machine.read().state.is_none(),
            padding: "10px",
            font_size: "20pt",
            margin: 10,
            "Run"
        },
        button {
            onclick: move |_| {
                if *active.read() {
                    *trigger_reset.write() = true;
                } else {
                    machine.write().reset();
                }
            },
            padding: "10px",
            font_size: "20pt",
            "Reset"
        },
        div {
            padding: "10",
            font_size: "25pt",
            align_content: "center",
            justify_self: "center",
            text_align: "center",
            margin_top: "10px",
            "State: {machine.read().state_display()}",
            br {},
            "Steps: {machine.read().steps}",
        },
        div {
            width: "100%",
            justify_content: "center",
            display: "flex",
            flex_direction: "row",
            margin_top: "15px",
            div {
                display: "grid",
                grid_template_columns: "auto auto auto auto auto",
                flex: "0",
                width: "auto",
                font_size: "20pt",
                text_align: "center",
                column_gap: "2px",
                background_color: "white",
                div {
                    background_color: DKGREY,
                    padding: "5px",
                    margin_bottom: "2px",
                    display: "flex",
                    flex_direction: "row",
                    align_items: "center",
                    "State",
                    button {
                        border: "3px solid lightgreen",
                        border_radius: "100%",
                        height: "15pt",
                        width: "15pt",
                        color: "lightgreen",
                        margin_left: "5px",
                        onclick: move |_| {
                            machine.write().add_state();
                            if *active.read() {
                                *trigger_reset.write() = true;
                            } else {
                                machine.write().reset();
                            }
                        },
                        "+"
                    }
                    button {
                        border: "3px solid #ffaaaa",
                        border_radius: "100%",
                        height: "15pt",
                        width: "15pt",
                        padding: 0,
                        display: "inline-flex",
                        align_items: "center",
                        justify_content: "center",
                        color: "#ffaaaa",
                        font_weight: "800",
                        background_color: "inherit",
                        margin_left: "5px",
                        onclick: move |_| {
                            machine.write().remove_state();
                            if *active.read() {
                                *trigger_reset.write() = true;
                            } else {
                                machine.write().reset();
                            }
                        },
                        "-"
                    }
                },
                div {
                    background_color: DKGREY,
                    padding: "5px",
                    margin_bottom: "2px",
                    "Read"
                },
                div {
                    background_color: DKGREY,
                    padding: "5px",
                    margin_bottom: "2px",
                    "Write"
                },
                div {
                    background_color: DKGREY,
                    padding: "5px",
                    margin_bottom: "2px",
                    "Move"
                },
                div {
                    background_color: DKGREY,
                    padding: "5px",
                    margin_bottom: "2px",
                    "Next"
                },
                for (i, ((state, tape), (newstate, write, dir))) in machine.read().instructions.iter().enumerate() {
                    {
                        let mach = &*machine.read();
                        let highlight = if i == mach.curr_rule {"grey"} else {DKGREY};
                        rsx! {div {
                            background_color: highlight,
                            padding: "5px",
                            "{state}"
                        },
                        div {
                            background_color: highlight,
                            padding: "5px",
                            "{tape}"
                        },
                        div {
                            class: "clickable-text",
                            background_color: highlight,
                            padding: "5px",
                            onclick: move |_| {
                                machine.write().change_write(i);
                                if *active.read() {
                                    *trigger_reset.write() = true;
                                } else {
                                    machine.write().reset();
                                }
                            },
                            "{write}"
                        },
                        div {
                            class: "clickable-text",
                            background_color: highlight,
                            padding: "5px",
                            onclick: move |_| {
                                machine.write().change_move(i);
                                if *active.read() {
                                    *trigger_reset.write() = true;
                                } else {
                                    machine.write().reset();
                                }
                            },
                            "{select(*dir, 'L', 'R')}"
                        },
                        div {
                            class: "clickable-text",
                            background_color: highlight,
                            padding: "5px",
                            onclick: move |_| {
                                machine.write().change_next(i);
                                if *active.read() {
                                    *trigger_reset.write() = true;
                                } else {
                                    machine.write().reset();
                                }
                            },
                            "{newstate.map(|c|c.to_string()).unwrap_or_else(||\"HALT\".to_string())}"
                        },
                        }
                    }
                }
            }
        }
        "Click a row's Write/Move/Next entry to edit",
        br {},
        "Click the +/- buttons to add/remove states",
        br {},
        "Or edit the machine's code directly below",
        br {},
        div {
            display: "flex",
            flex_direction: "row",
            width: "50%",
            justify_self: "center",
            input {
                border: "none",
                border_radius: "10px",
                font_size: "15pt",
                font_family: "inherit",
                padding: "10px",
                value: "{machine_code.read()}",
                flex: 1,
                oninput: move |ev| {
                    do_write.set(false);
                    machine_code.set(ev.value());
                }
            },
            button {
                font_size: "15pt",
                padding: "5px",
                onclick: move |_| {
                    if let Some(tm) = TuringMachine::from_string(&machine_code.read()) {
                        machine.set(tm);
                    }
                    do_write.set(true);
                },
                "Set"
            },
            button {
                font_size: "15pt",
                padding: "5px",
                onclick: move |_| {
                    do_write.set(true);
                },
                "Revert"
            }
        }
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

#[derive(Debug)]
struct TuringMachine {
    state: Option<char>,

    tape: VecDeque<char>,
    offset: i32,
    default_char: char,

    instructions: Vec<Instruction>,
    curr_rule: usize,

    steps: usize,
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
            curr_rule: 0,
            steps: 0,
        };
        me.curr_rule = me.next_rule();
        me
    }

    fn from_string(string: &str) -> Option<Self> {
        Some(Self::new(
            string
                .split('_')
                .enumerate()
                .map(|(sn, s)| {
                    s.chars()
                        .chunks(3)
                        .into_iter()
                        .enumerate()
                        .map(|(tn, mut inst)| {
                            let write = inst.next()?;
                            let dir = inst.next()?;
                            let new = inst.next()?;
                            if !"RL".contains(dir) || !"01".contains(write) {
                                return None;
                            }
                            Some((
                                (offset_char('A', sn as u8), offset_char('0', tn as u8)),
                                ((new != 'Z').then_some(new), write, dir == 'L'),
                            ))
                        })
                        .collect::<Option<Vec<_>>>()
                })
                .collect::<Option<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect(),
        ))
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

    fn next_rule(&self) -> usize {
        self.instructions
            .iter()
            .position(|((s, t), _)| self.state == Some(*s) && self.cell_at(0) == *t)
            .unwrap()
    }

    fn rule(&self) -> &Rule {
        &self.instructions[self.curr_rule].1
    }

    fn next_shift_is_left(&self) -> bool {
        self.rule().2
    }

    fn step_write(&mut self) {
        self.write(self.rule().1);
        self.steps += 1;
    }

    fn step_move(&mut self) {
        if self.rule().2 {
            self.shift_left();
        } else {
            self.shift_right();
        }
    }

    fn step_state(&mut self) {
        self.state = self.rule().0;
    }

    fn step_next(&mut self) -> bool {
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

    fn reset(&mut self) {
        *self = Self::new(self.instructions.clone())
    }

    fn add_state(&mut self) {
        if self.instructions.len() >= 26 * 2 {
            return;
        }
        let state = offset_char('A', (self.instructions.len() / 2) as u8);
        self.instructions.extend([
            ((state, '0'), (None, '1', false)),
            ((state, '1'), (None, '1', false)),
        ]);
    }

    fn remove_state(&mut self) {
        if self.instructions.is_empty() {
            return;
        }
        let state = self.instructions.pop().unwrap().0.0;
        self.instructions.pop();
        for (_, (s, _, _)) in self.instructions.iter_mut() {
            if let Some(next) = s
                && *next == state
            {
                *s = None;
            }
        }
    }

    fn change_write(&mut self, row: usize) {
        let write = &mut self.instructions[row].1.1;
        *write = if *write == '1' { '0' } else { '1' };
    }

    fn change_move(&mut self, row: usize) {
        let dir = &mut self.instructions[row].1.2;
        *dir = !*dir;
    }

    fn change_next(&mut self, row: usize) {
        let chars = self.instructions.len() / 2;
        let next = &mut self.instructions[row].1.0;
        if let Some(n) = next {
            if *n as usize - 'A' as usize >= chars - 1 {
                *next = None;
            } else {
                *n = offset_char(*n, 1);
            }
        } else {
            *next = Some('A');
        }
    }

    fn get_code(&self) -> String {
        self.instructions
            .iter()
            .chunks(2)
            .into_iter()
            .map(|cs| {
                cs.map(|(_, (next, write, dir))| {
                    format!(
                        "{}{}{}",
                        write,
                        if *dir { 'L' } else { 'R' },
                        next.unwrap_or('Z')
                    )
                })
                .collect_vec()
                .join("")
            })
            .collect_vec()
            .join("_")
    }
}
