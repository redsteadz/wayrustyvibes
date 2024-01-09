// use evdev_rs::enums::EventType;
// use evdev_rs::enums::KeyState;
// use evdev_rs::enums::SwitchState;
use evdev::Key;

pub mod key_code {
    /// Option
    const ALT: i32 = 56;
    /// Option_Right
    const ALT_GR: i32 = 3640;
    const BACKSPACE: i32 = 14;
    const CAPS_LOCK: i32 = 58;
    /// Control Right does not exist on Mac
    const CONTROL_LEFT: i32 = 29;
    const DOWN_ARROW: i32 = 57424;
    const ESCAPE: i32 = 1;
    const F1: i32 = 59;
    const F10: i32 = 68;
    const F11: i32 = 87;
    const F12: i32 = 88;
    const F2: i32 = 60;
    const F3: i32 = 61;
    const F4: i32 = 62;
    const F5: i32 = 63;
    const F6: i32 = 64;
    const F7: i32 = 65;
    const F8: i32 = 66;
    const F9: i32 = 67;
    const FUNCTION: i32 = 3666;
    const LEFT_ARROW: i32 = 57419;
    const META_LEFT: i32 = 3675;
    const META_RIGHT: i32 = 3676;
    const RETURN: i32 = 28;
    const RIGHT_ARROW: i32 = 57421;
    const SHIFT_LEFT: i32 = 42;
    const SHIFT_RIGHT: i32 = 54;
    const SPACE: i32 = 57;
    const TAB: i32 = 15;
    const UP_ARROW: i32 = 57416;
    const BACK_QUOTE: i32 = 41;

    const NUM1: i32 = 2;
    const NUM2: i32 = 3;
    const NUM3: i32 = 4;
    const NUM4: i32 = 5;
    const NUM5: i32 = 6;
    const NUM6: i32 = 7;
    const NUM7: i32 = 8;
    const NUM8: i32 = 9;
    const NUM9: i32 = 10;
    const NUM0: i32 = 11;
    const MINUS: i32 = 12;
    const EQUAL: i32 = 13;

    const KEY_Q: i32 = 16;
    const KEY_W: i32 = 17;
    const KEY_E: i32 = 18;
    const KEY_R: i32 = 19;
    const KEY_T: i32 = 20;
    const KEY_Y: i32 = 21;
    const KEY_U: i32 = 22;
    const KEY_I: i32 = 23;
    const KEY_O: i32 = 24;
    const KEY_P: i32 = 25;

    const LEFT_BRACKET: i32 = 26;
    const RIGHT_BRACKET: i32 = 27;

    const KEY_A: i32 = 30;
    const KEY_S: i32 = 31;
    const KEY_D: i32 = 32;
    const KEY_F: i32 = 33;
    const KEY_G: i32 = 34;
    const KEY_H: i32 = 35;
    const KEY_J: i32 = 36;
    const KEY_K: i32 = 37;
    const KEY_L: i32 = 38;
    const SEMI_COLON: i32 = 39;
    const QUOTE: i32 = 40;
    const BACK_SLASH: i32 = 43;

    const KEY_Z: i32 = 44;
    const KEY_X: i32 = 45;
    const KEY_C: i32 = 46;
    const KEY_V: i32 = 47;
    const KEY_B: i32 = 48;
    const KEY_N: i32 = 49;
    const KEY_M: i32 = 50;
    const COMMA: i32 = 51;
    const DOT: i32 = 52;
    const SLASH: i32 = 53;
}
