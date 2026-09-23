use crate::geometry::Point;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Identifiers for the four classic Curry dissection pieces.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PieceId {
    RedTriangle = 0,
    GreenTriangle = 1,
    OrangePolyomino = 2,
    YellowPolyomino = 3,
}

impl PieceId {
    pub fn from_str_loose(s: &str) -> Option<Self> {
        match s.to_lowercase().replace(['-', '_', ' '], "").as_str() {
            "red" | "redtriangle" | "0" => Some(Self::RedTriangle),
            "green" | "meadowgreen" | "greentriangle" | "1" => Some(Self::GreenTriangle),
            "orange" | "terracotta" | "warmterracotta" | "orangepolyomino" | "2" => {
                Some(Self::OrangePolyomino)
            }
            "yellow" | "amber" | "honeyamber" | "yellowpolyomino" | "3" => {
                Some(Self::YellowPolyomino)
            }
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::RedTriangle => "Warm Coral Red Triangle (8×3)",
            Self::GreenTriangle => "Meadow Green Triangle (5×2)",
            Self::OrangePolyomino => "Warm Terracotta Polyomino (8 units)",
            Self::YellowPolyomino => "Honey Amber Polyomino (7 units)",
        }
    }

    pub fn base_color(&self) -> &'static str {
        match self {
            Self::RedTriangle => "#E05344",        // warm coral red
            Self::GreenTriangle => "#2A9D8F",      // meadow green
            Self::YellowPolyomino => "#E9C46A",    // honey amber
            Self::OrangePolyomino => "#F4A261",    // warm terracotta
        }
    }

    pub fn stroke_color(&self) -> &'static str {
        match self {
            Self::RedTriangle => "#C43D2F",
            Self::GreenTriangle => "#1F7A6F",
            Self::YellowPolyomino => "#D4AA47",
            Self::OrangePolyomino => "#E0873E",
        }
    }
}

/// Represents one puzzle piece with both continuous floating coordinates
/// (for smooth dragging) and integer grid coordinates (for snapped validation).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Piece {
    pub id: PieceId,
    pub name: String,
    pub color: String,
    pub stroke_color: String,
    pub canonical_vertices: Vec<Point>,
    pub canonical_cells: Vec<(i32, i32)>,
    pub theoretical_area: f64,
    pub slope: Option<f64>,
    pub position: Point,
}

impl Piece {
    pub fn new_red_triangle() -> Self {
        Self {
            id: PieceId::RedTriangle,
            name: "Warm Coral Red Triangle".to_string(),
            color: PieceId::RedTriangle.base_color().to_string(),
            stroke_color: PieceId::RedTriangle.stroke_color().to_string(),
            canonical_vertices: vec![
                Point::new(0.0, 0.0),
                Point::new(8.0, 0.0),
                Point::new(8.0, 3.0),
            ],
            canonical_cells: vec![],
            theoretical_area: 12.0,
            slope: Some(3.0 / 8.0), // 0.375
            position: Point::new(0.0, 0.0),
        }
    }

    pub fn new_green_triangle() -> Self {
        Self {
            id: PieceId::GreenTriangle,
            name: "Meadow Green Triangle".to_string(),
            color: PieceId::GreenTriangle.base_color().to_string(),
            stroke_color: PieceId::GreenTriangle.stroke_color().to_string(),
            canonical_vertices: vec![
                Point::new(0.0, 0.0),
                Point::new(5.0, 0.0),
                Point::new(5.0, 2.0),
            ],
            canonical_cells: vec![],
            theoretical_area: 5.0,
            slope: Some(2.0 / 5.0), // 0.400
            position: Point::new(8.0, 3.0),
        }
    }

    pub fn new_orange_polyomino() -> Self {
        Self {
            id: PieceId::OrangePolyomino,
            name: "Warm Terracotta Polyomino".to_string(),
            color: PieceId::OrangePolyomino.base_color().to_string(),
            stroke_color: PieceId::OrangePolyomino.stroke_color().to_string(),
            canonical_vertices: vec![
                Point::new(0.0, 0.0),
                Point::new(5.0, 0.0),
                Point::new(5.0, 1.0),
                Point::new(3.0, 1.0),
                Point::new(3.0, 2.0),
                Point::new(0.0, 2.0),
            ],
            canonical_cells: vec![
                (0, 0), (1, 0), (2, 0), (3, 0), (4, 0),
                (0, 1), (1, 1), (2, 1),
            ],
            theoretical_area: 8.0,
            slope: None,
            position: Point::new(8.0, 0.0),
        }
    }

    pub fn new_yellow_polyomino() -> Self {
        Self {
            id: PieceId::YellowPolyomino,
            name: "Honey Amber Polyomino".to_string(),
            color: PieceId::YellowPolyomino.base_color().to_string(),
            stroke_color: PieceId::YellowPolyomino.stroke_color().to_string(),
            canonical_vertices: vec![
                Point::new(3.0, 0.0),
                Point::new(5.0, 0.0),
                Point::new(5.0, 2.0),
                Point::new(0.0, 2.0),
                Point::new(0.0, 1.0),
                Point::new(3.0, 1.0),
            ],
            canonical_cells: vec![
                (3, 0), (4, 0),
                (0, 1), (1, 1), (2, 1), (3, 1), (4, 1),
            ],
            theoretical_area: 7.0,
            slope: None,
            position: Point::new(8.0, 1.0),
        }
    }

    /// Factory function to instantiate all four canonical pieces.
    pub fn create_all() -> Vec<Piece> {
        vec![
            Self::new_red_triangle(),
            Self::new_green_triangle(),
            Self::new_orange_polyomino(),
            Self::new_yellow_polyomino(),
        ]
    }

    /// Computes the transformed world vertices based on the piece's current position.
    pub fn world_vertices(&self) -> Vec<Point> {
        self.canonical_vertices
            .iter()
            .map(|p| p.translate(self.position.x, self.position.y))
            .collect()
    }

    /// Computes the world grid cells occupied by this piece (for integer polyominoes).
    pub fn world_cells(&self) -> Vec<(i32, i32)> {
        let gx = self.position.x.round() as i32;
        let gy = self.position.y.round() as i32;
        self.canonical_cells
            .iter()
            .map(|&(cx, cy)| (cx + gx, cy + gy))
            .collect()
    }

    /// Axis-aligned bounding box: (min_x, max_x, min_y, max_y)
    pub fn bounding_box(&self) -> (f64, f64, f64, f64) {
        let wv = self.world_vertices();
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for p in wv {
            if p.x < min_x { min_x = p.x; }
            if p.x > max_x { max_x = p.x; }
            if p.y < min_y { min_y = p.y; }
            if p.y > max_y { max_y = p.y; }
        }

        (min_x, max_x, min_y, max_y)
    }

    /// Canonical position for this piece under Configuration A (Solid apparent triangle).
    pub fn canonical_pos_a(&self) -> Point {
        match self.id {
            PieceId::RedTriangle => Point::new(0.0, 0.0),
            PieceId::GreenTriangle => Point::new(8.0, 3.0),
            PieceId::OrangePolyomino => Point::new(8.0, 0.0),
            PieceId::YellowPolyomino => Point::new(8.0, 1.0),
        }
    }

    /// Canonical position for this piece under Configuration B (Missing square paradox).
    pub fn canonical_pos_b(&self) -> Point {
        match self.id {
            PieceId::RedTriangle => Point::new(5.0, 2.0),
            PieceId::GreenTriangle => Point::new(0.0, 0.0),
            PieceId::OrangePolyomino => Point::new(5.0, 0.0),
            PieceId::YellowPolyomino => Point::new(8.0, 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::shoelace_area;

    #[test]
    fn test_piece_areas_by_shoelace() {
        let red = Piece::new_red_triangle();
        let green = Piece::new_green_triangle();
        let orange = Piece::new_orange_polyomino();
        let yellow = Piece::new_yellow_polyomino();

        let area_red = shoelace_area(&red.canonical_vertices);
        let area_green = shoelace_area(&green.canonical_vertices);
        let area_orange = shoelace_area(&orange.canonical_vertices);
        let area_yellow = shoelace_area(&yellow.canonical_vertices);

        assert!((area_red - 12.0).abs() < 1e-10, "Red area must be 12");
        assert!((area_green - 5.0).abs() < 1e-10, "Green area must be 5");
        assert!((area_orange - 8.0).abs() < 1e-10, "Orange area must be 8");
        assert!((area_yellow - 7.0).abs() < 1e-10, "Yellow area must be 7");

        let total_area = area_red + area_green + area_orange + area_yellow;
        assert!((total_area - 32.0).abs() < 1e-10, "Total pieces area must be 32.0");
    }

    #[test]
    fn test_cell_counts() {
        let orange = Piece::new_orange_polyomino();
        let yellow = Piece::new_yellow_polyomino();

        assert_eq!(orange.canonical_cells.len(), 8);
        assert_eq!(yellow.canonical_cells.len(), 7);
    }
}
