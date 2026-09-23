use crate::geometry::Point;
use crate::piece::{Piece, PieceId};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const GRID_WIDTH: i32 = 13;
pub const GRID_HEIGHT: i32 = 5;
pub const MAGNETIC_SNAP_THRESHOLD: f64 = 0.45;
pub const CONFIG_MAGNET_THRESHOLD: f64 = 0.95;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfigurationState {
    ConfigA,           // Solid apparent triangle (area 32 beneath concave hypotenuse)
    ConfigB,           // Missing square paradox (area 32 + 1 missing square at (10,0))
    CustomValid,       // Pieces on grid without collisions, but neither A nor B
    Collision(String), // Pieces collide
    OutOfBounds(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub state: ConfigurationState,
    pub is_valid: bool,
    pub is_config_a: bool,
    pub is_config_b: bool,
    pub missing_squares: Vec<(i32, i32)>,
    pub message: String,
    pub total_piece_area: f64,
}

/// Checks whether all vertices of a piece lie within the 13x5 grid.
pub fn is_within_bounds(piece: &Piece) -> bool {
    let (min_x, max_x, min_y, max_y) = piece.bounding_box();
    min_x >= -0.01 && max_x <= (GRID_WIDTH as f64 + 0.01) && min_y >= -0.01 && max_y <= (GRID_HEIGHT as f64 + 0.01)
}

/// Checks if a cell center (cx + 0.5, cy + 0.5) is strictly inside a right triangle
/// anchored at (tx, ty) with base width w and height h.
fn is_cell_inside_right_triangle(cx: i32, cy: i32, tx: f64, ty: f64, w: f64, h: f64) -> bool {
    let px = cx as f64 + 0.5;
    let py = cy as f64 + 0.5;

    if px <= tx || px >= tx + w || py <= ty || py >= ty + h {
        return false;
    }

    // Hypotenuse line: y - ty = (h / w) * (x - tx)
    let y_hypotenuse = ty + (h / w) * (px - tx);
    py < y_hypotenuse
}

/// Checks if two right triangles with disjoint bounds overlap.
fn triangles_overlap(r_pos: Point, g_pos: Point) -> bool {
    // Red triangle: [r_pos.x, r_pos.x + 8] x [r_pos.y, r_pos.y + 3]
    // Green triangle: [g_pos.x, g_pos.x + 5] x [g_pos.y, g_pos.y + 2]
    let r_min_x = r_pos.x;
    let r_max_x = r_pos.x + 8.0;
    let r_min_y = r_pos.y;
    let r_max_y = r_pos.y + 3.0;

    let g_min_x = g_pos.x;
    let g_max_x = g_pos.x + 5.0;
    let g_min_y = g_pos.y;
    let g_max_y = g_pos.y + 2.0;

    // Check AABB overlap with slight tolerance
    let overlap_x = r_min_x < g_max_x - 0.01 && r_max_x > g_min_x + 0.01;
    let overlap_y = r_min_y < g_max_y - 0.01 && r_max_y > g_min_y + 0.01;

    if !overlap_x || !overlap_y {
        return false;
    }

    // If both bounding boxes overlap significantly, sample points
    let sample_points = [
        Point::new(g_pos.x + 1.0, g_pos.y + 0.5),
        Point::new(g_pos.x + 2.5, g_pos.y + 0.8),
        Point::new(g_pos.x + 4.0, g_pos.y + 1.2),
    ];

    for sp in sample_points {
        if sp.x > r_min_x && sp.x < r_max_x && sp.y > r_min_y && sp.y < (r_min_y + (3.0 / 8.0) * (sp.x - r_min_x)) {
            return true;
        }
    }

    false
}

/// Validates the entire arrangement of pieces on the 13x5 grid.
pub fn validate_puzzle(pieces: &[Piece]) -> ValidationReport {
    // 1. Check out of bounds
    for p in pieces {
        if !is_within_bounds(p) {
            let msg = format!("{} is outside the 13x5 grid bounds.", p.name);
            return ValidationReport {
                state: ConfigurationState::OutOfBounds(msg.clone()),
                is_valid: false,
                is_config_a: false,
                is_config_b: false,
                missing_squares: vec![],
                message: msg,
                total_piece_area: 32.0,
            };
        }
    }

    // 2. Check collisions between pieces
    let mut occupied_poly_cells = HashSet::new();
    let mut orange_cells = Vec::new();
    let mut yellow_cells = Vec::new();
    let mut red_pos = None;
    let mut green_pos = None;

    for p in pieces {
        match p.id {
            PieceId::OrangePolyomino => {
                let cells = p.world_cells();
                for c in &cells {
                    if !occupied_poly_cells.insert(*c) {
                        let msg = format!("Collision: {} overlaps with another piece at grid cell ({}, {}).", p.name, c.0, c.1);
                        return ValidationReport {
                            state: ConfigurationState::Collision(msg.clone()),
                            is_valid: false,
                            is_config_a: false,
                            is_config_b: false,
                            missing_squares: vec![],
                            message: msg,
                            total_piece_area: 32.0,
                        };
                    }
                }
                orange_cells = cells;
            }
            PieceId::YellowPolyomino => {
                let cells = p.world_cells();
                for c in &cells {
                    if !occupied_poly_cells.insert(*c) {
                        let msg = format!("Collision: {} overlaps with another piece at grid cell ({}, {}).", p.name, c.0, c.1);
                        return ValidationReport {
                            state: ConfigurationState::Collision(msg.clone()),
                            is_valid: false,
                            is_config_a: false,
                            is_config_b: false,
                            missing_squares: vec![],
                            message: msg,
                            total_piece_area: 32.0,
                        };
                    }
                }
                yellow_cells = cells;
            }
            PieceId::RedTriangle => {
                red_pos = Some(p.position);
            }
            PieceId::GreenTriangle => {
                green_pos = Some(p.position);
            }
        }
    }

    // Check collision between triangles and polyomino cells
    if let Some(r_pos) = red_pos {
        for &(cx, cy) in orange_cells.iter().chain(yellow_cells.iter()) {
            if is_cell_inside_right_triangle(cx, cy, r_pos.x, r_pos.y, 8.0, 3.0) {
                let msg = format!("Collision: Polyomino cell ({}, {}) overlaps with the Red Triangle.", cx, cy);
                return ValidationReport {
                    state: ConfigurationState::Collision(msg.clone()),
                    is_valid: false,
                    is_config_a: false,
                    is_config_b: false,
                    missing_squares: vec![],
                    message: msg,
                    total_piece_area: 32.0,
                };
            }
        }
    }

    if let Some(g_pos) = green_pos {
        for &(cx, cy) in orange_cells.iter().chain(yellow_cells.iter()) {
            if is_cell_inside_right_triangle(cx, cy, g_pos.x, g_pos.y, 5.0, 2.0) {
                let msg = format!("Collision: Polyomino cell ({}, {}) overlaps with the Green Triangle.", cx, cy);
                return ValidationReport {
                    state: ConfigurationState::Collision(msg.clone()),
                    is_valid: false,
                    is_config_a: false,
                    is_config_b: false,
                    missing_squares: vec![],
                    message: msg,
                    total_piece_area: 32.0,
                };
            }
        }
    }

    if let (Some(r_pos), Some(g_pos)) = (red_pos, green_pos) {
        if triangles_overlap(r_pos, g_pos) {
            let msg = "Collision: The Red Triangle and Green Triangle overlap.".to_string();
            return ValidationReport {
                state: ConfigurationState::Collision(msg.clone()),
                is_valid: false,
                is_config_a: false,
                is_config_b: false,
                missing_squares: vec![],
                message: msg,
                total_piece_area: 32.0,
            };
        }
    }

    // 3. Test for Configuration A or B
    let is_a = pieces.iter().all(|p| {
        let expected = p.canonical_pos_a();
        (p.position.x - expected.x).abs() < 0.05 && (p.position.y - expected.y).abs() < 0.05
    });

    if is_a {
        return ValidationReport {
            state: ConfigurationState::ConfigA,
            is_valid: true,
            is_config_a: true,
            is_config_b: false,
            missing_squares: vec![],
            message: "Configuration A: Solid Apparent Triangle. Pieces total 32 units. The hypotenuse dips inward (concave) by 0.077 units, matching the 0.5 unit area defect (32.5 - 32.0 = 0.5).".to_string(),
            total_piece_area: 32.0,
        };
    }

    let is_b = pieces.iter().all(|p| {
        let expected = p.canonical_pos_b();
        (p.position.x - expected.x).abs() < 0.05 && (p.position.y - expected.y).abs() < 0.05
    });

    if is_b {
        return ValidationReport {
            state: ConfigurationState::ConfigB,
            is_valid: true,
            is_config_a: false,
            is_config_b: true,
            missing_squares: vec![(10, 0)],
            message: "Configuration B: Hole Paradox! Pieces total 32 units. The hypotenuse bulges outward (convex) by 0.077 units, creating an envelope of area 33 with a 1x1 empty missing square at grid cell (10, 0)!".to_string(),
            total_piece_area: 32.0,
        };
    }

    ValidationReport {
        state: ConfigurationState::CustomValid,
        is_valid: true,
        is_config_a: false,
        is_config_b: false,
        missing_squares: vec![],
        message: "Pieces placed on the 13x5 grid without collisions. Drag pieces to align with Configuration A or B.".to_string(),
        total_piece_area: 32.0,
    }
}

/// Applies magnetic snapping to a piece position.
/// First checks if within magnetic pull of Config A or Config B canonical positions.
/// Otherwise snaps to the nearest integer grid coordinate if within threshold.
pub fn magnetic_snap_position(piece: &Piece, candidate: Point) -> Point {
    // 1. Check Config A magnetic pull
    let pos_a = piece.canonical_pos_a();
    if candidate.distance(&pos_a) <= CONFIG_MAGNET_THRESHOLD {
        return pos_a;
    }

    // 2. Check Config B magnetic pull
    let pos_b = piece.canonical_pos_b();
    if candidate.distance(&pos_b) <= CONFIG_MAGNET_THRESHOLD {
        return pos_b;
    }

    // 3. Grid snap
    let rx = candidate.x.round();
    let ry = candidate.y.round();

    let snapped_x = if (candidate.x - rx).abs() <= MAGNETIC_SNAP_THRESHOLD {
        rx
    } else {
        candidate.x
    };

    let snapped_y = if (candidate.y - ry).abs() <= MAGNETIC_SNAP_THRESHOLD {
        ry
    } else {
        candidate.y
    };

    Point::new(snapped_x, snapped_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_a_validation() {
        let mut pieces = Piece::create_all();
        for p in &mut pieces {
            p.position = p.canonical_pos_a();
        }
        let report = validate_puzzle(&pieces);
        assert!(report.is_valid);
        assert!(report.is_config_a);
        assert!(!report.is_config_b);
        assert_eq!(report.missing_squares.len(), 0);
    }

    #[test]
    fn test_config_b_validation_with_missing_square() {
        let mut pieces = Piece::create_all();
        for p in &mut pieces {
            p.position = p.canonical_pos_b();
        }
        let report = validate_puzzle(&pieces);
        assert!(report.is_valid);
        assert!(!report.is_config_a);
        assert!(report.is_config_b);
        assert_eq!(report.missing_squares, vec![(10, 0)]);
    }

    #[test]
    fn test_magnetic_snap_near_config_b() {
        let piece = Piece::new_orange_polyomino();
        let near_pos = Point::new(5.15, 0.08); // Target is (5,0)
        let snapped = magnetic_snap_position(&piece, near_pos);
        assert_eq!(snapped, Point::new(5.0, 0.0));
    }
}
