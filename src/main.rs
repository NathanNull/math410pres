use std::collections::VecDeque;

use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::*;

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
    let mut offset = use_signal_sync(|| 0isize);
    let mut written = use_signal_sync(|| VecDeque::from_iter(['1']));
    let state = 'A';
    let mut x = use_motion(0.0)?;
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
                    val: written.read().get((*offset.read() + i) as usize).copied().unwrap_or('0'),
                    selected: i == 0,
                }
            }
        },
        button {
            onclick: move |_| {
                let tween = AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(250),
                    easing: Quad::ease_in_out,
                });
                let _ = x.animate_sequence(AnimationSequence::new()
                    .then(-40., AnimationConfig::new(tween))
                    .then(0., AnimationConfig::tween_ms(0).with_on_complete(move || {
                        *offset.write() += 1;
                        info!("{:?}", written.read());
                    })));
            },
            margin_top: "45px",
            justify_self: "center",
            "Click me!"
        },
        button {
            onclick: move |_| {
                let tween = AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(250),
                    easing: Quad::ease_in_out,
                });
                let _ = x.animate_sequence(AnimationSequence::new()
                    .then(40., AnimationConfig::new(tween))
                    .then(0., AnimationConfig::tween_ms(0).with_on_complete(move || {
                        *offset.write() -= 1;
                        info!("{:?}", written.read());
                    })));
            },
            justify_self: "center",
            "Me too!"
        },
        button {
            onclick: move |_| {
                while *offset.read() < 0 {
                    *offset.write() += 1;
                    written.write().push_front('0');
                }
                while *offset.read() >= written.read().len() as isize {
                    written.write().push_back('0');
                }
                written.write()[*offset.read() as usize] = '1';
                info!("{:?}", written.read());
            },
            justify_self: "center",
            "Me three!"
        },
        div {
            border: "1px solid white",
            height: "40px",
            font_size: "25pt",
            align_content: "center",
            text_align: "center",
            margin_top: "20px",
            "State: {state}"
        },
    }
}

#[component]
fn TapeCell(val: char, selected: bool) -> Element {
    rsx! {
        div {
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
}
