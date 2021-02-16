use sdl2::video::Window;
use sdl2::messagebox::{
    MessageBoxFlag,
    MessageBoxButtonFlag,
    ButtonData,
    show_message_box,
    ClickedButton,
};

static CONFIRM_BUTTONS: [ButtonData; 2] = [
    ButtonData {
        flags: MessageBoxButtonFlag::empty(),
        button_id: 1,
        text: "OK",
    },
    ButtonData {
        flags: MessageBoxButtonFlag::empty(),
        button_id: 0,
        text: "Cancel",
    },
];

static ALERT_BUTTONS: [ButtonData; 1] = [
    ButtonData {
        flags: MessageBoxButtonFlag::empty(),
        button_id: 1,
        text: "OK",
    },
];

pub fn alert(window: &Window, text: &str) -> bool {
    let msg = show_message_box(
        MessageBoxFlag::empty(),
        &ALERT_BUTTONS,
        "Alert",
        text,
        window,
        None,
    );
    matches!(msg, Ok(_))
}

pub fn confirm(window: &Window, text: &str) -> bool {
    let msg = show_message_box(
        MessageBoxFlag::empty(),
        &CONFIRM_BUTTONS,
        "Confirm",
        text,
        window,
        None,
    );
    matches!(msg, Ok(ClickedButton::CustomButton(ButtonData { button_id: 1, .. })))
}
