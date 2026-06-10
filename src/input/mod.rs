//! Input injection module.
use crate::protocol::{ControlEvent, MouseButton as ProtocolMouseButton};
use anyhow::Result;

/// Injects a control event into the OS.
///
/// Notes:
/// - Keycodes are Windows Virtual Key codes.
/// - MouseMove is relative (delta), not absolute.
/// - Ping and Pong are silently ignored.
pub fn inject(event: &ControlEvent) -> Result<()> {
    use enigo::{Enigo, Settings, Mouse, Keyboard, Coordinate, Button, Direction, Key, Axis};

    let mut enigo = Enigo::new(&Settings::default())?;

    match event {
        ControlEvent::MouseMove { dx, dy } => {
            enigo.move_mouse(*dx as i32, *dy as i32, Coordinate::Rel)?;
        }
        ControlEvent::MouseButton { button, pressed } => {
            let enigo_btn = match button {
                ProtocolMouseButton::Left => Button::Left,
                ProtocolMouseButton::Right => Button::Right,
                ProtocolMouseButton::Middle => Button::Middle,
            };
            let direction = if *pressed { Direction::Press } else { Direction::Release };
            enigo.button(enigo_btn, direction)?;
        }
        ControlEvent::Scroll { dx, dy } => {
            if *dy != 0.0 {
                enigo.scroll(*dy as i32, Axis::Vertical)?;
            }
            if *dx != 0.0 {
                enigo.scroll(*dx as i32, Axis::Horizontal)?;
            }
        }
        ControlEvent::KeyEvent { keycode, pressed } => {
            let direction = if *pressed { Direction::Press } else { Direction::Release };
            enigo.key(Key::Other(*keycode), direction)?;
        }
        ControlEvent::Ping | ControlEvent::Pong => {
            // Silently ignore
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::ControlEvent;

    #[test]
    fn test_inject_ping() {
        let result = inject(&ControlEvent::Ping);
        assert!(result.is_ok());
    }

    #[test]
    fn test_inject_zero_mousemove() {
        let result = inject(&ControlEvent::MouseMove { dx: 0.0, dy: 0.0 });
        assert!(result.is_ok());
    }
}
