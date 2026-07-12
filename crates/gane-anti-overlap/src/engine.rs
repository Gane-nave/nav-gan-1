/// Anti-overlap system: prevent visual element collisions, clear boundaries.
#[derive(Debug, Clone)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}
impl Rect {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }
    pub fn right(&self) -> f64 {
        self.x + self.w
    }
    pub fn bottom(&self) -> f64 {
        self.y + self.h
    }
    pub fn overlaps(&self, other: &Rect) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }
    pub fn overlap_area(&self, other: &Rect) -> f64 {
        let ox = (self.right().min(other.right()) - self.x.max(other.x)).max(0.0);
        let oy = (self.bottom().min(other.bottom()) - self.y.max(other.y)).max(0.0);
        ox * oy
    }
    pub fn area(&self) -> f64 {
        self.w * self.h
    }
}
#[derive(Debug, Clone)]
pub struct LayoutElement {
    pub name: String,
    pub bounds: Rect,
    pub priority: u32,
    pub moveable: bool,
}
#[derive(Debug, Clone)]
pub struct AntiOverlapSystem {
    pub elements: Vec<LayoutElement>,
    pub margin: f64,
}
impl AntiOverlapSystem {
    pub fn new(margin: f64) -> Self {
        Self {
            elements: Vec::new(),
            margin,
        }
    }
    pub fn add_element(&mut self, e: LayoutElement) {
        self.elements.push(e);
    }
    pub fn find_overlaps(&self) -> Vec<(usize, usize)> {
        let mut overlaps = Vec::new();
        for i in 0..self.elements.len() {
            for j in (i + 1)..self.elements.len() {
                let a = &self.elements[i].bounds;
                let b = &self.elements[j].bounds;
                let expanded_a = Rect::new(
                    a.x - self.margin,
                    a.y - self.margin,
                    a.w + 2.0 * self.margin,
                    a.h + 2.0 * self.margin,
                );
                if expanded_a.overlaps(b) {
                    overlaps.push((i, j));
                }
            }
        }
        overlaps
    }
    pub fn has_overlaps(&self) -> bool {
        !self.find_overlaps().is_empty()
    }
    pub fn overlap_count(&self) -> usize {
        self.find_overlaps().len()
    }
    pub fn resolve_overlaps(&mut self) {
        // Iterate until no overlaps remain or we hit a safety guard
        let max_iterations = 100;
        for _ in 0..max_iterations {
            let overlaps = self.find_overlaps();
            if overlaps.is_empty() {
                break;
            }
            for (i, j) in overlaps {
                let (lo, hi) = if self.elements[i].priority <= self.elements[j].priority {
                    (i, j)
                } else {
                    (j, i)
                };
                if self.elements[hi].moveable {
                    self.elements[hi].bounds.y = self.elements[lo].bounds.bottom() + self.margin;
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_overlap() {
        let a = Rect::new(0.0, 0.0, 10.0, 10.0);
        let b = Rect::new(5.0, 5.0, 10.0, 10.0);
        assert!(a.overlaps(&b));
        assert!(a.overlap_area(&b) > 0.0);
    }
    #[test]
    fn test_no_overlap() {
        let a = Rect::new(0.0, 0.0, 10.0, 10.0);
        let b = Rect::new(20.0, 20.0, 10.0, 10.0);
        assert!(!a.overlaps(&b));
        assert_eq!(a.overlap_area(&b), 0.0);
    }
    #[test]
    fn test_system() {
        let mut s = AntiOverlapSystem::new(2.0);
        s.add_element(LayoutElement {
            name: "a".into(),
            bounds: Rect::new(0.0, 0.0, 50.0, 20.0),
            priority: 1,
            moveable: false,
        });
        s.add_element(LayoutElement {
            name: "b".into(),
            bounds: Rect::new(10.0, 5.0, 50.0, 20.0),
            priority: 2,
            moveable: true,
        });
        assert!(s.has_overlaps());
        s.resolve_overlaps();
    }
    #[test]
    fn test_empty() {
        let s = AntiOverlapSystem::new(5.0);
        assert!(!s.has_overlaps());
    }
}
