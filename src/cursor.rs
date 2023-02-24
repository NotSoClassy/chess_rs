use piston_window::*;

/* MouseHandler::reset_drag should be called after drag values are used !! */
#[derive(Debug)]
pub struct MouseHandler {
  pub drag_completed: bool,
  pub started_drag: bool,
  pub current: Option<[f64; 2]>,
  pub start: Option<[f64; 2]>,
  pub end: Option<[f64; 2]>,
  pressed: bool
}

impl MouseHandler {
  pub fn new() -> Self {
    return MouseHandler {
      drag_completed: false,
      started_drag: false,
      pressed: false,
      current: None,
      start: None,
      end: None
    }
  }

  pub fn reset_drag(&mut self) {
    self.drag_completed = false;
    self.started_drag = false;
    self.pressed = false;
    self.current = None;
    self.start = None;
    self.end = None;
  }

  pub fn handle_input(&mut self, input: &Input) {
    match input {
      Input::Button(button_args) => {
        if button_args.button != Button::Mouse(MouseButton::Left) { return }

        if self.drag_completed {
          /* they clicked too much? idk retry */
          self.reset_drag();
        }

        if button_args.state == ButtonState::Press {
          self.pressed = true;
        } else if button_args.state == ButtonState::Release && self.started_drag {
          self.end = self.current;
          self.drag_completed = true;
        }
      }

      Input::Move(motion) => {
        match motion {
          Motion::MouseCursor(mouse) => {
            if self.pressed {
              self.started_drag = true;
              self.current = Some(*mouse);
            }

            if self.pressed && self.start.is_none() {
              self.start = Some(*mouse);
            }
          }

          _ => {}
        }
      }

      _ => {}
    }
  }
}