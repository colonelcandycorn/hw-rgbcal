use crate::*;

struct UiState {
    levels: [u32; 3],
    frame_rate: u64,
}

impl UiState {
    /// Displays the current state of the UI, including the levels of the RGB LEDs and the frame rate.
    fn show(&self) {
        let names = ["red", "green", "blue"];
        rprintln!();
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("frame rate: {}", self.frame_rate);
    }
}

impl Default for UiState {
    /// Creates a default `UiState` with all levels set to the maximum level and a default frame rate of 100.
    /// Maximum level is defined by the constant `LEVELS`, which is set to 10.
    fn default() -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1],
            frame_rate: 100,
        }
    }
}

/// Used to determine what value we are setting with the knob.
/// - APressed means the knob is controlling the blue LED,
/// - BPressed means the knob is controlling the green LED,
/// - ABPressed means the knob is controlling the red LED,
/// - and None means the knob is controlling the frame rate.
enum ButtonState {
    APressed,
    BPressed,
    ABPressed,
    None,
}

pub struct Ui {
    knob: Knob,
    _button_a: Button,
    _button_b: Button,
    state: UiState,
}

impl Ui {

    /// Determines whether button A, button B, or both buttons are pressed.
    /// # Returns
    /// A `ButtonState` enum indicating the state of the buttons:
    /// - `APressed` if only button A is pressed,
    /// - `BPressed` if only button B is pressed,
    /// - `ABPressed` if both buttons are pressed,
    /// - `None` if neither button is pressed.
    ///    
    fn button_state(&self) -> ButtonState {
        if self._button_a.is_low() && self._button_b.is_low() {
            ButtonState::ABPressed
        } else if self._button_a.is_low() {
            ButtonState::APressed
        } else if self._button_b.is_low() {
            ButtonState::BPressed
        } else {
            ButtonState::None
        }
    }
    /// Sets the level of the LED or frame rate based on the current button state.
    /// If button A is pressed, it sets the blue LED level.
    /// If button B is pressed, it sets the green LED level.
    /// If both buttons are pressed, it sets the red LED level.
    /// If no buttons are pressed, it sets the frame rate.
    /// 
    /// # Arguments
    /// * `level` - The level to set, which is a u32 value.
    /// 
    /// # Returns
    /// An `Option<ButtonState>`, which indicates the state of the button after setting the level.
    /// If nothing changed, it returns `None`.
    /// 
    /// I went back and forth on this return type because it feels kind of dumb to return the button state, but
    /// I also didn't want to have the program reread the button state after setting the level. I wanted whatever
    /// calls this function to know what button state was active when the level was set.
    fn set_level(&mut self, level: u32) -> Option<ButtonState> {

        match self.button_state() {
            ButtonState::APressed => {
                if self.state.levels[2] == level {
                    return None; // No change if the level is the same
                }
                self.state.levels[2] = level;
                Some(ButtonState::APressed)
            }
            ButtonState::BPressed => {
                if self.state.levels[1] == level {
                    return None; // No change if the level is the same
                }
                self.state.levels[1] = level;
                Some(ButtonState::BPressed)
            }
            ButtonState::ABPressed => {
                if self.state.levels[0] == level {
                    return None; // No change if the level is the same
                }
                self.state.levels[0] = level;
                Some(ButtonState::ABPressed)
            }
            ButtonState::None => {
                let new_level = (level + 1) as u64 * 10;
                if self.state.frame_rate == new_level {
                    return None; // No change if the frame rate is the same
                }
                self.state.frame_rate = new_level;
                Some(ButtonState::None)
            }
        }
    }

    /// Creates a new instance of the `Ui` struct.
    /// 
    /// # Arguments
    /// * `knob` - An instance of the `Knob` struct used to measure the knob's position.
    /// * `_button_a` - An instance of the `Button` struct representing button A.
    /// * `_button_b` - An instance of the `Button` struct representing button B.
    /// # Returns
    /// A new instance of the `Ui` struct.
    pub fn new(knob: Knob, _button_a: Button, _button_b: Button) -> Self {
        Self {
            knob,
            _button_a,
            _button_b,
            state: UiState::default(),
        }
    }

    /// Runs the UI loop, measuring the knob and updating the state based on button presses.
    /// 
    /// The changes I made from the skeleton code mostly have to do with reading the button state, calling
    /// a function that sets the level of either the LED or frame rate and sets the state of the UI, and then
    /// updates one of the global Mutexes so the rgb object can read the new values.
    /// 
    /// The structure of this code seems weird to me reading it out after the fact in terms of design, but I know it
    /// works so I decided not to stress about it too much.
    pub async fn run(&mut self) -> ! {
        // self.state.levels[2] = self.knob.measure().await;
        let level = self.knob.measure().await;
        let button_state= self.set_level(level);
        match button_state {
            Some(ButtonState::None) => {
                set_frame_rate(|frame_rate| {
                    *frame_rate = self.state.frame_rate;
                })
                .await;
                self.state.show();
            },
            Some(_) => {
                set_rgb_levels(
                    |rgb | {
                        *rgb = self.state.levels;
                    }
                )
                .await;
                self.state.show();
            }
            None => {
                // No change, do nothing
            }
        }
        loop {
            let level = self.knob.measure().await;
            let button_state= self.set_level(level);
            match button_state {
                Some(ButtonState::None) => {
                    set_frame_rate(|frame_rate| {
                        *frame_rate = self.state.frame_rate;
                    })
                    .await;
                    self.state.show();
                },
                Some(_) => {
                    set_rgb_levels(
                        |rgb | {
                            *rgb = self.state.levels;
                        }
                    )
                    .await;
                    self.state.show();
                }
                None => {
                    // No change, do nothing
                }
            }
            Timer::after_millis(50).await;
        }
    }
}
