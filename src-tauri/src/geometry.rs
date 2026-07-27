#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ViewportRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl ViewportRect {
    pub fn clamped_to(self, bounds: ViewportRect) -> ViewportRect {
        let width = self.width.max(0.0).min(bounds.width);
        let height = self.height.max(0.0).min(bounds.height);
        let max_x = (bounds.x + bounds.width - width).max(bounds.x);
        let max_y = (bounds.y + bounds.height - height).max(bounds.y);
        let x = self.x.max(bounds.x).min(max_x);
        let y = self.y.max(bounds.y).min(max_y);
        ViewportRect {
            x,
            y,
            width,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn window() -> ViewportRect {
        ViewportRect {
            x: 0.0,
            y: 0.0,
            width: 1000.0,
            height: 800.0,
        }
    }

    #[test]
    fn fits_inside_bounds_unchanged() {
        let rect = ViewportRect {
            x: 100.0,
            y: 50.0,
            width: 400.0,
            height: 300.0,
        };
        assert_eq!(rect.clamped_to(window()), rect);
    }

    #[test]
    fn clamps_width_and_height_that_exceed_bounds() {
        let rect = ViewportRect {
            x: 0.0,
            y: 0.0,
            width: 5000.0,
            height: 5000.0,
        };
        let clamped = rect.clamped_to(window());
        assert_eq!(clamped.width, 1000.0);
        assert_eq!(clamped.height, 800.0);
    }

    #[test]
    fn clamps_position_that_would_overflow_right_edge() {
        let rect = ViewportRect {
            x: 900.0,
            y: 0.0,
            width: 400.0,
            height: 100.0,
        };
        let clamped = rect.clamped_to(window());
        assert_eq!(clamped.x, 600.0);
        assert_eq!(clamped.width, 400.0);
    }

    #[test]
    fn clamps_negative_position_to_bounds_origin() {
        let rect = ViewportRect {
            x: -50.0,
            y: -20.0,
            width: 200.0,
            height: 100.0,
        };
        let clamped = rect.clamped_to(window());
        assert_eq!(clamped.x, 0.0);
        assert_eq!(clamped.y, 0.0);
    }

    #[test]
    fn clamps_negative_width_and_height_to_zero() {
        let rect = ViewportRect {
            x: 10.0,
            y: 10.0,
            width: -5.0,
            height: -5.0,
        };
        let clamped = rect.clamped_to(window());
        assert_eq!(clamped.width, 0.0);
        assert_eq!(clamped.height, 0.0);
    }
}
