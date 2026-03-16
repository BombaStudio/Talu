use raylib::consts::KeyboardKey;

pub const KEYS: [(&str, KeyboardKey); 94] = [
    // Directional keys
    ("UP", KeyboardKey::KEY_UP),
    ("DOWN", KeyboardKey::KEY_DOWN),
    ("LEFT", KeyboardKey::KEY_LEFT),
    ("RIGHT", KeyboardKey::KEY_RIGHT),
    
    // Letters
    ("A", KeyboardKey::KEY_A), ("B", KeyboardKey::KEY_B), ("C", KeyboardKey::KEY_C),
    ("D", KeyboardKey::KEY_D), ("E", KeyboardKey::KEY_E), ("F", KeyboardKey::KEY_F),
    ("G", KeyboardKey::KEY_G), ("H", KeyboardKey::KEY_H), ("I", KeyboardKey::KEY_I),
    ("J", KeyboardKey::KEY_J), ("K", KeyboardKey::KEY_K), ("L", KeyboardKey::KEY_L),
    ("M", KeyboardKey::KEY_M), ("N", KeyboardKey::KEY_N ), ("O", KeyboardKey::KEY_O),
    ("P", KeyboardKey::KEY_P), ("Q", KeyboardKey::KEY_Q), ("R", KeyboardKey::KEY_R),
    ("S", KeyboardKey::KEY_S), ("T", KeyboardKey::KEY_T), ("U", KeyboardKey::KEY_U),
    ("V", KeyboardKey::KEY_V), ("W", KeyboardKey::KEY_W), ("X", KeyboardKey::KEY_X),
    ("Y", KeyboardKey::KEY_Y), ("Z", KeyboardKey::KEY_Z),

    // Numbers
    ("ZERO", KeyboardKey::KEY_ZERO), ("ONE", KeyboardKey::KEY_ONE),
    ("TWO", KeyboardKey::KEY_TWO), ("THREE", KeyboardKey::KEY_THREE),
    ("FOUR", KeyboardKey::KEY_FOUR), ("FIVE", KeyboardKey::KEY_FIVE),
    ("SIX", KeyboardKey::KEY_SIX), ("SEVEN", KeyboardKey::KEY_SEVEN),
    ("EIGHT", KeyboardKey::KEY_EIGHT), ("NINE", KeyboardKey::KEY_NINE),

    // Function keys
    ("F1", KeyboardKey::KEY_F1), ("F2", KeyboardKey::KEY_F2), ("F3", KeyboardKey::KEY_F3),
    ("F4", KeyboardKey::KEY_F4), ("F5", KeyboardKey::KEY_F5), ("F6", KeyboardKey::KEY_F6),
    ("F7", KeyboardKey::KEY_F7), ("F8", KeyboardKey::KEY_F8), ("F9", KeyboardKey::KEY_F9),
    ("F10", KeyboardKey::KEY_F10), ("F11", KeyboardKey::KEY_F11), ("F12", KeyboardKey::KEY_F12),

    // Special keys
    ("SPACE", KeyboardKey::KEY_SPACE),
    ("ESCAPE", KeyboardKey::KEY_ESCAPE),
    ("ENTER", KeyboardKey::KEY_ENTER),
    ("TAB", KeyboardKey::KEY_TAB),
    ("BACKSPACE", KeyboardKey::KEY_BACKSPACE),
    ("INSERT", KeyboardKey::KEY_INSERT),
    ("DELETE", KeyboardKey::KEY_DELETE),
    ("PAGE_UP", KeyboardKey::KEY_PAGE_UP),
    ("PAGE_DOWN", KeyboardKey::KEY_PAGE_DOWN),
    ("HOME", KeyboardKey::KEY_HOME),
    ("END", KeyboardKey::KEY_END),
    ("CAPS_LOCK", KeyboardKey::KEY_CAPS_LOCK),
    ("SCROLL_LOCK", KeyboardKey::KEY_SCROLL_LOCK),
    ("NUM_LOCK", KeyboardKey::KEY_NUM_LOCK),
    ("PRINT_SCREEN", KeyboardKey::KEY_PRINT_SCREEN),
    ("PAUSE", KeyboardKey::KEY_PAUSE),

    // Modifiers
    ("LEFT_SHIFT", KeyboardKey::KEY_LEFT_SHIFT),
    ("LEFT_CONTROL", KeyboardKey::KEY_LEFT_CONTROL),
    ("LEFT_ALT", KeyboardKey::KEY_LEFT_ALT),
    ("LEFT_SUPER", KeyboardKey::KEY_LEFT_SUPER),
    ("RIGHT_SHIFT", KeyboardKey::KEY_RIGHT_SHIFT),
    ("RIGHT_CONTROL", KeyboardKey::KEY_RIGHT_CONTROL),
    ("RIGHT_ALT", KeyboardKey::KEY_RIGHT_ALT),
    ("RIGHT_SUPER", KeyboardKey::KEY_RIGHT_SUPER),
    ("KB_MENU", KeyboardKey::KEY_KB_MENU),

    // Keypad
    ("KP_0", KeyboardKey::KEY_KP_0), ("KP_1", KeyboardKey::KEY_KP_1),
    ("KP_2", KeyboardKey::KEY_KP_2), ("KP_3", KeyboardKey::KEY_KP_3),
    ("KP_4", KeyboardKey::KEY_KP_4), ("KP_5", KeyboardKey::KEY_KP_5),
    ("KP_6", KeyboardKey::KEY_KP_6), ("KP_7", KeyboardKey::KEY_KP_7),
    ("KP_8", KeyboardKey::KEY_KP_8), ("KP_9", KeyboardKey::KEY_KP_9),
    ("KP_DECIMAL", KeyboardKey::KEY_KP_DECIMAL),
    ("KP_DIVIDE", KeyboardKey::KEY_KP_DIVIDE),
    ("KP_MULTIPLY", KeyboardKey::KEY_KP_MULTIPLY),
    ("KP_SUBTRACT", KeyboardKey::KEY_KP_SUBTRACT),
    ("KP_ADD", KeyboardKey::KEY_KP_ADD),
    ("KP_ENTER", KeyboardKey::KEY_KP_ENTER),
    ("KP_EQUAL", KeyboardKey::KEY_KP_EQUAL),
];