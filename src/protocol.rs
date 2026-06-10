//! Protocol definitions for remctrl.
use serde::{Deserialize, Serialize};

/// Represents a mouse button.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button
    Middle,
}

/// Represents control events from the remote client.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ControlEvent {
    /// Relative mouse movement
    MouseMove { 
        /// Delta X
        dx: f32, 
        /// Delta Y
        dy: f32 
    },
    /// Mouse button press or release
    MouseButton { 
        /// Which button
        button: MouseButton, 
        /// True if pressed down, false if released
        pressed: bool 
    },
    /// Scrolling
    Scroll { 
        /// Horizontal scroll amount
        dx: f32, 
        /// Vertical scroll amount
        dy: f32 
    },
    /// Key event
    KeyEvent { 
        /// Windows Virtual Key code
        keycode: u32, 
        /// True if pressed down, false if released
        pressed: bool 
    },
    /// Ping event
    Ping,
    /// Pong event
    Pong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_mousemove() {
        let event = ControlEvent::MouseMove { dx: 1.0, dy: 2.0 };
        let json = serde_json::to_string(&event).unwrap();
        assert_eq!(json, r#"{"type":"MouseMove","dx":1.0,"dy":2.0}"#);
    }

    #[test]
    fn test_deserialize_mousemove() {
        let json = r#"{"type":"MouseMove","dx":1.0,"dy":2.0}"#;
        let event: ControlEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event, ControlEvent::MouseMove { dx: 1.0, dy: 2.0 });
    }

    #[test]
    fn test_serialize_ping() {
        let event = ControlEvent::Ping;
        let json = serde_json::to_string(&event).unwrap();
        assert_eq!(json, r#"{"type":"Ping"}"#);
    }
}
