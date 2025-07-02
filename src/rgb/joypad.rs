// Game Boy Joypad Implementation
// Handles the joypad register (0xFF00) and button state management

#[derive(Debug, Clone, Copy)]
pub struct JoypadButtons {
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl JoypadButtons {
    pub fn new() -> Self {
        Self {
            a: false,
            b: false,
            start: false,
            select: false,
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

pub struct Joypad {
    buttons: JoypadButtons,
    direction_selected: bool,
    button_selected: bool,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            buttons: JoypadButtons::new(),
            direction_selected: false,
            button_selected: false,
        }
    }

    pub fn update_buttons(&mut self, buttons: JoypadButtons) -> bool {
        // Check if any new button was pressed (edge detection)
        let old_buttons = self.buttons;
        self.buttons = buttons;
        
        // Return true if any button transitioned from not pressed to pressed
        (!old_buttons.a && buttons.a) ||
        (!old_buttons.b && buttons.b) ||
        (!old_buttons.start && buttons.start) ||
        (!old_buttons.select && buttons.select) ||
        (!old_buttons.up && buttons.up) ||
        (!old_buttons.down && buttons.down) ||
        (!old_buttons.left && buttons.left) ||
        (!old_buttons.right && buttons.right)
    }

    // Read joypad register (0xFF00)
    pub fn read_register(&self) -> u8 {
        let mut result = 0xCF; // Upper 2 bits always set, unused bits set

        // Bit 5: Button keys (0=select)
        if !self.button_selected {
            result |= 0x20;
        }

        // Bit 4: Direction keys (0=select)  
        if !self.direction_selected {
            result |= 0x10;
        }

        // Lower 4 bits: button states (0=pressed, 1=not pressed)
        if self.direction_selected {
            // Direction keys selected
            if self.buttons.down { result &= !0x08; }
            if self.buttons.up { result &= !0x04; }
            if self.buttons.left { result &= !0x02; }
            if self.buttons.right { result &= !0x01; }
        }
        
        if self.button_selected {
            // Action buttons selected
            if self.buttons.start { result &= !0x08; }
            if self.buttons.select { result &= !0x04; }
            if self.buttons.b { result &= !0x02; }
            if self.buttons.a { result &= !0x01; }
        }

        result
    }

    // Write joypad register (0xFF00)
    pub fn write_register(&mut self, value: u8) {
        // Only bits 5 and 4 are writable (select which button group to read)
        self.button_selected = (value & 0x20) == 0;
        self.direction_selected = (value & 0x10) == 0;
    }

    // Check if any button is currently pressed (for interrupt generation)
    pub fn any_button_pressed(&self) -> bool {
        self.buttons.a || self.buttons.b || self.buttons.start || self.buttons.select ||
        self.buttons.up || self.buttons.down || self.buttons.left || self.buttons.right
    }
}