#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

impl Point2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn translate(&self, dx: f64, dy: f64) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub start: Point2,
    pub end: Point2,
}

impl Line {
    pub fn new(start: Point2, end: Point2) -> Self {
        Self { start, end }
    }

    pub fn translate(&self, dx: f64, dy: f64) -> Self {
        Self {
            start: self.start.translate(dx, dy),
            end: self.end.translate(dx, dy),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub center: Point2,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Point2, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn translate(&self, dx: f64, dy: f64) -> Self {
        Self {
            center: self.center.translate(dx, dy),
            radius: self.radius,
        }
    }
}
