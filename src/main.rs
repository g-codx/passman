#[cfg(feature = "cmd")]
mod cmd;
mod core;
#[cfg(feature = "gui")]
mod gui;

fn main() {
    //cargo run --features cmd --release
    #[cfg(feature = "cmd")]
    cmd::run();

    //cargo run --features gui --release
    #[cfg(feature = "gui")]
    gui::run();
}
