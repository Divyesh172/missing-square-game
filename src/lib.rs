pub mod engine;
pub mod geometry;
pub mod grid;
pub mod piece;
pub mod svg;

pub use engine::PuzzleEngine;
pub use geometry::{cross_determinant, evaluate_cassini, fibonacci, shoelace_area, Point};
pub use grid::{validate_puzzle, ConfigurationState, ValidationReport};
pub use piece::{Piece, PieceId};
pub use svg::{SvgRenderOptions, SvgRenderer};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigurationType {
    Original = 0,
    Rearranged = 1,
    Reset = 2,
}

#[wasm_bindgen(start)]
pub fn main_js() {
    console_error_panic_hook::set_once();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mathematical_invariants() {
        // 1. Enclosing triangle area
        let theoretical_triangle_area = 0.5 * 13.0 * 5.0;
        assert_eq!(theoretical_triangle_area, 32.5);

        // 2. Piece areas
        let red = Piece::new_red_triangle();
        let green = Piece::new_green_triangle();
        let orange = Piece::new_orange_polyomino();
        let yellow = Piece::new_yellow_polyomino();

        let area_red = shoelace_area(&red.canonical_vertices);
        let area_green = shoelace_area(&green.canonical_vertices);
        let area_orange = shoelace_area(&orange.canonical_vertices);
        let area_yellow = shoelace_area(&yellow.canonical_vertices);

        assert_eq!(area_red, 12.0);
        assert_eq!(area_green, 5.0);
        assert_eq!(area_orange, 8.0);
        assert_eq!(area_yellow, 7.0);

        let total_pieces_area = area_red + area_green + area_orange + area_yellow;
        assert_eq!(total_pieces_area, 32.0);

        // 3. Cassini's Identity
        let cassini = evaluate_cassini(6);
        assert_eq!(cassini.difference, 1);
        assert!(cassini.identity_holds);

        // 4. Mystery Lozenge Parallelogram
        let lozenge_pts = geometry::mystery_lozenge_points();
        let lozenge_area = shoelace_area(&lozenge_pts);
        assert!((lozenge_area - 1.0).abs() < 1e-10);

        // 5. Slopes
        let slope_red: f64 = 3.0 / 8.0;
        let slope_green: f64 = 2.0 / 5.0;
        assert_ne!(slope_red, slope_green);
        assert!((slope_green - slope_red - 0.025f64).abs() < 1e-10);
    }

    #[test]
    fn test_config_a_and_b_engine_flow() {
        let mut engine = PuzzleEngine::new();

        // Starts in Config A
        let rep_a: ValidationReport = serde_json::from_str(&engine.run_validation()).unwrap();
        assert!(rep_a.is_config_a);
        assert!(!rep_a.is_config_b);
        assert!(rep_a.missing_squares.is_empty());

        // Switch to Config B
        assert!(engine.set_configuration("B"));
        let rep_b: ValidationReport = serde_json::from_str(&engine.run_validation()).unwrap();
        assert!(!rep_b.is_config_a);
        assert!(rep_b.is_config_b);
        assert_eq!(rep_b.missing_squares, vec![(10, 0)]);

        // Check SVG output
        let svg = engine.render_svg();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("Missing 1×1 Void"));
    }
}
