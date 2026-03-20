//! Polygon operations — point-in-polygon, area calculation, convex hull.

/// A 2D point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    /// Create a new point.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Distance to another point.
    pub fn distance_to(&self, other: &Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// A polygon defined by a list of vertices (in order).
#[derive(Debug, Clone)]
pub struct Polygon {
    /// Vertices of the polygon.
    pub vertices: Vec<Point2D>,
}

impl Polygon {
    /// Create a new polygon from vertices.
    pub fn new(vertices: Vec<Point2D>) -> Self {
        Self { vertices }
    }

    /// Number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Check if a point is inside the polygon using ray casting algorithm.
    pub fn contains(&self, point: &Point2D) -> bool {
        let n = self.vertices.len();
        if n < 3 {
            return false;
        }

        let mut inside = false;
        let mut j = n - 1;
        for i in 0..n {
            let vi = &self.vertices[i];
            let vj = &self.vertices[j];

            if ((vi.y > point.y) != (vj.y > point.y))
                && (point.x < (vj.x - vi.x) * (point.y - vi.y) / (vj.y - vi.y) + vi.x)
            {
                inside = !inside;
            }
            j = i;
        }
        inside
    }

    /// Signed area using the shoelace formula.
    /// Positive if vertices are counter-clockwise, negative if clockwise.
    pub fn signed_area(&self) -> f64 {
        let n = self.vertices.len();
        if n < 3 {
            return 0.0;
        }
        let mut area = 0.0;
        for i in 0..n {
            let j = (i + 1) % n;
            area += self.vertices[i].x * self.vertices[j].y;
            area -= self.vertices[j].x * self.vertices[i].y;
        }
        area / 2.0
    }

    /// Absolute area.
    pub fn area(&self) -> f64 {
        self.signed_area().abs()
    }

    /// Perimeter (sum of edge lengths).
    pub fn perimeter(&self) -> f64 {
        let n = self.vertices.len();
        if n < 2 {
            return 0.0;
        }
        let mut perim = 0.0;
        for i in 0..n {
            let j = (i + 1) % n;
            perim += self.vertices[i].distance_to(&self.vertices[j]);
        }
        perim
    }

    /// Centroid of the polygon.
    pub fn centroid(&self) -> Point2D {
        let n = self.vertices.len();
        if n == 0 {
            return Point2D::new(0.0, 0.0);
        }
        let area = self.signed_area();
        if area.abs() < 1e-12 {
            // Degenerate polygon — return average
            let sx: f64 = self.vertices.iter().map(|v| v.x).sum();
            let sy: f64 = self.vertices.iter().map(|v| v.y).sum();
            return Point2D::new(sx / n as f64, sy / n as f64);
        }

        let mut cx = 0.0;
        let mut cy = 0.0;
        for i in 0..n {
            let j = (i + 1) % n;
            let cross =
                self.vertices[i].x * self.vertices[j].y - self.vertices[j].x * self.vertices[i].y;
            cx += (self.vertices[i].x + self.vertices[j].x) * cross;
            cy += (self.vertices[i].y + self.vertices[j].y) * cross;
        }
        let factor = 1.0 / (6.0 * area);
        Point2D::new(cx * factor, cy * factor)
    }

    /// Check if vertices are in counter-clockwise order.
    pub fn is_ccw(&self) -> bool {
        self.signed_area() > 0.0
    }

    /// Bounding box of the polygon (min_x, min_y, max_x, max_y).
    pub fn bounding_box(&self) -> (f64, f64, f64, f64) {
        if self.vertices.is_empty() {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        for v in &self.vertices {
            min_x = min_x.min(v.x);
            min_y = min_y.min(v.y);
            max_x = max_x.max(v.x);
            max_y = max_y.max(v.y);
        }
        (min_x, min_y, max_x, max_y)
    }
}

/// Compute convex hull of a set of points using Graham scan.
pub fn convex_hull(points: &[Point2D]) -> Polygon {
    if points.len() < 3 {
        return Polygon::new(points.to_vec());
    }

    // Find bottom-most point (lowest y, then leftmost x)
    let mut pts: Vec<Point2D> = points.to_vec();
    let pivot_idx = pts
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            a.y.partial_cmp(&b.y)
                .unwrap()
                .then(a.x.partial_cmp(&b.x).unwrap())
        })
        .map(|(i, _)| i)
        .unwrap();
    pts.swap(0, pivot_idx);
    let pivot = pts[0];

    // Sort by polar angle with pivot
    pts[1..].sort_by(|a, b| {
        let angle_a = (a.y - pivot.y).atan2(a.x - pivot.x);
        let angle_b = (b.y - pivot.y).atan2(b.x - pivot.x);
        angle_a.partial_cmp(&angle_b).unwrap().then(
            a.distance_to(&pivot)
                .partial_cmp(&b.distance_to(&pivot))
                .unwrap(),
        )
    });

    let mut hull: Vec<Point2D> = Vec::new();
    for &p in &pts {
        while hull.len() >= 2 {
            let a = hull[hull.len() - 2];
            let b = hull[hull.len() - 1];
            let cross = (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x);
            if cross <= 0.0 {
                hull.pop();
            } else {
                break;
            }
        }
        hull.push(p);
    }

    Polygon::new(hull)
}

/// Check if two line segments intersect.
pub fn segments_intersect(a1: &Point2D, a2: &Point2D, b1: &Point2D, b2: &Point2D) -> bool {
    let d1 = direction(b1, b2, a1);
    let d2 = direction(b1, b2, a2);
    let d3 = direction(a1, a2, b1);
    let d4 = direction(a1, a2, b2);

    if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0))
        && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0))
    {
        return true;
    }

    if d1.abs() < 1e-12 && on_segment(b1, b2, a1) {
        return true;
    }
    if d2.abs() < 1e-12 && on_segment(b1, b2, a2) {
        return true;
    }
    if d3.abs() < 1e-12 && on_segment(a1, a2, b1) {
        return true;
    }
    if d4.abs() < 1e-12 && on_segment(a1, a2, b2) {
        return true;
    }

    false
}

fn direction(a: &Point2D, b: &Point2D, c: &Point2D) -> f64 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}

fn on_segment(a: &Point2D, b: &Point2D, c: &Point2D) -> bool {
    c.x >= a.x.min(b.x) && c.x <= a.x.max(b.x) && c.y >= a.y.min(b.y) && c.y <= a.y.max(b.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square() -> Polygon {
        Polygon::new(vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(4.0, 0.0),
            Point2D::new(4.0, 4.0),
            Point2D::new(0.0, 4.0),
        ])
    }

    #[test]
    fn test_point_in_polygon() {
        let poly = square();
        assert!(poly.contains(&Point2D::new(2.0, 2.0))); // inside
        assert!(!poly.contains(&Point2D::new(5.0, 5.0))); // outside
        assert!(!poly.contains(&Point2D::new(-1.0, 2.0))); // outside
    }

    #[test]
    fn test_area() {
        let poly = square();
        assert!((poly.area() - 16.0).abs() < 1e-10);
    }

    #[test]
    fn test_perimeter() {
        let poly = square();
        assert!((poly.perimeter() - 16.0).abs() < 1e-10);
    }

    #[test]
    fn test_centroid() {
        let poly = square();
        let c = poly.centroid();
        assert!((c.x - 2.0).abs() < 1e-10);
        assert!((c.y - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_bounding_box() {
        let poly = square();
        let (min_x, min_y, max_x, max_y) = poly.bounding_box();
        assert!((min_x - 0.0).abs() < 1e-10);
        assert!((min_y - 0.0).abs() < 1e-10);
        assert!((max_x - 4.0).abs() < 1e-10);
        assert!((max_y - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_triangle_area() {
        let tri = Polygon::new(vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(4.0, 0.0),
            Point2D::new(0.0, 3.0),
        ]);
        assert!((tri.area() - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_convex_hull() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 1.0), // interior
            Point2D::new(2.0, 0.0),
            Point2D::new(1.0, 2.0),
        ];
        let hull = convex_hull(&points);
        assert_eq!(hull.vertex_count(), 3); // triangle, interior point excluded
    }

    #[test]
    fn test_segments_intersect() {
        let a1 = Point2D::new(0.0, 0.0);
        let a2 = Point2D::new(2.0, 2.0);
        let b1 = Point2D::new(0.0, 2.0);
        let b2 = Point2D::new(2.0, 0.0);
        assert!(segments_intersect(&a1, &a2, &b1, &b2));
    }

    #[test]
    fn test_segments_no_intersect() {
        let a1 = Point2D::new(0.0, 0.0);
        let a2 = Point2D::new(1.0, 0.0);
        let b1 = Point2D::new(0.0, 1.0);
        let b2 = Point2D::new(1.0, 1.0);
        assert!(!segments_intersect(&a1, &a2, &b1, &b2));
    }

    #[test]
    fn test_degenerate_polygon() {
        let poly = Polygon::new(vec![Point2D::new(0.0, 0.0), Point2D::new(1.0, 1.0)]);
        assert!(!poly.contains(&Point2D::new(0.5, 0.5)));
        assert!((poly.area() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_point_distance() {
        let a = Point2D::new(0.0, 0.0);
        let b = Point2D::new(3.0, 4.0);
        assert!((a.distance_to(&b) - 5.0).abs() < 1e-10);
    }
}
