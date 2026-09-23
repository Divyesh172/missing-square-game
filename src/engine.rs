use crate::geometry::{evaluate_cassini, Point};
use crate::grid::{magnetic_snap_position, validate_puzzle};
use crate::piece::{Piece, PieceId};
use crate::svg::{SvgRenderOptions, SvgRenderer};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PieceStateDto {
    pub id: String,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub color: String,
    pub area: f64,
    pub slope: Option<f64>,
}

#[wasm_bindgen]
pub struct PuzzleEngine {
    pieces: Vec<Piece>,
    renderer: SvgRenderer,
    options: SvgRenderOptions,
}

#[wasm_bindgen]
impl PuzzleEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();

        let pieces = Piece::create_all();
        let renderer = SvgRenderer::new();
        let options = SvgRenderOptions {
            laser_visible: true,
            lozenge_visible: false,
            magnify_bend: false,
            selected_piece_id: None,
        };

        let mut engine = Self {
            pieces,
            renderer,
            options,
        };
        engine.set_configuration("A");
        engine
    }

    /// Sets all pieces to a canonical configuration ("A", "B", or "reset").
    pub fn set_configuration(&mut self, config: &str) -> bool {
        match config.trim().to_uppercase().as_str() {
            "A" | "CONFIG_A" | "CONFIGA" | "SOLID" => {
                for p in &mut self.pieces {
                    p.position = p.canonical_pos_a();
                }
                true
            }
            "B" | "CONFIG_B" | "CONFIGB" | "PARADOX" | "HOLE" => {
                for p in &mut self.pieces {
                    p.position = p.canonical_pos_b();
                }
                true
            }
            "RESET" => {
                self.reset();
                true
            }
            _ => false,
        }
    }

    /// Sets configuration by ConfigurationType enum.
    pub fn set_configuration_type(&mut self, config: crate::ConfigurationType) -> bool {
        match config {
            crate::ConfigurationType::Original => self.set_configuration("A"),
            crate::ConfigurationType::Rearranged => self.set_configuration("B"),
            crate::ConfigurationType::Reset => {
                self.reset();
                true
            }
        }
    }

    /// Resets the puzzle to Configuration A.
    pub fn reset(&mut self) {
        for p in &mut self.pieces {
            p.position = p.canonical_pos_a();
        }
        self.options.selected_piece_id = None;
    }

    /// Selects a piece by its identifier ("red", "green", "orange", "yellow", or enum name).
    pub fn select_piece(&mut self, id: &str) -> bool {
        if let Some(pid) = PieceId::from_str_loose(id) {
            self.options.selected_piece_id = Some(pid);
            true
        } else {
            false
        }
    }

    /// Clears any selected piece.
    pub fn deselect_piece(&mut self) {
        self.options.selected_piece_id = None;
    }

    /// Returns the currently selected piece identifier, if any.
    pub fn get_selected_piece(&self) -> Option<String> {
        self.options.selected_piece_id.map(|id| format!("{:?}", id))
    }

    /// Updates the position of a piece during interactive dragging.
    pub fn drag_move(&mut self, id: &str, target_x: f64, target_y: f64) -> bool {
        if let Some(pid) = PieceId::from_str_loose(id) {
            if let Some(piece) = self.pieces.iter_mut().find(|p| p.id == pid) {
                piece.position = Point::new(target_x, target_y);
                self.options.selected_piece_id = Some(pid);
                return true;
            }
        }
        false
    }

    /// Moves a piece by a relative offset (dx, dy).
    pub fn drag_offset(&mut self, id: &str, dx: f64, dy: f64) -> bool {
        if let Some(pid) = PieceId::from_str_loose(id) {
            if let Some(piece) = self.pieces.iter_mut().find(|p| p.id == pid) {
                piece.position = piece.position.translate(dx, dy);
                self.options.selected_piece_id = Some(pid);
                return true;
            }
        }
        false
    }

    /// Sets piece position directly.
    pub fn set_piece_position(&mut self, id: &str, x: f64, y: f64) -> bool {
        self.drag_move(id, x, y)
    }

    /// Magnetically snaps the specified piece to the nearest valid grid coordinate or canonical slot.
    pub fn snap_selected_piece(&mut self, id: &str) -> bool {
        if let Some(pid) = PieceId::from_str_loose(id) {
            if let Some(piece) = self.pieces.iter_mut().find(|p| p.id == pid) {
                piece.position = magnetic_snap_position(piece, piece.position);
                return true;
            }
        }
        false
    }

    /// Magnetically snaps all pieces on the board.
    pub fn snap_all_pieces(&mut self) {
        for p in &mut self.pieces {
            p.position = magnetic_snap_position(p, p.position);
        }
    }

    /// Toggles visibility of the laser straightedge hypotenuse.
    pub fn toggle_laser(&mut self) -> bool {
        self.options.laser_visible = !self.options.laser_visible;
        self.options.laser_visible
    }

    pub fn set_laser_visible(&mut self, visible: bool) {
        self.options.laser_visible = visible;
    }

    pub fn is_laser_visible(&self) -> bool {
        self.options.laser_visible
    }

    /// Toggles visibility of the mystery lozenge strip.
    pub fn toggle_lozenge(&mut self) -> bool {
        self.options.lozenge_visible = !self.options.lozenge_visible;
        self.options.lozenge_visible
    }

    pub fn set_lozenge_visible(&mut self, visible: bool) {
        self.options.lozenge_visible = visible;
    }

    pub fn is_lozenge_visible(&self) -> bool {
        self.options.lozenge_visible
    }

    /// Toggles dynamic bend magnification loupe.
    pub fn toggle_magnification(&mut self) -> bool {
        self.options.magnify_bend = !self.options.magnify_bend;
        self.options.magnify_bend
    }

    pub fn set_magnify_bend(&mut self, magnify: bool) {
        self.options.magnify_bend = magnify;
    }

    pub fn is_magnify_bend(&self) -> bool {
        self.options.magnify_bend
    }

    /// Renders the complete vector SVG board based on current state and options.
    pub fn render_svg(&self) -> String {
        self.renderer.render(&self.pieces, &self.options)
    }

    /// Renders SVG board with ad-hoc option flags.
    pub fn render_svg_with_options(&self, laser: bool, lozenge: bool, magnify: bool) -> String {
        let options = SvgRenderOptions {
            laser_visible: laser,
            lozenge_visible: lozenge,
            magnify_bend: magnify,
            selected_piece_id: self.options.selected_piece_id,
        };
        self.renderer.render(&self.pieces, &options)
    }

    /// Runs geometric and mathematical validation on the current piece positions.
    /// Returns a JSON string containing the full `ValidationReport`.
    pub fn run_validation(&self) -> String {
        let report = validate_puzzle(&self.pieces);
        serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".to_string())
    }

    /// Returns a friendly human-readable summary of the current puzzle state.
    pub fn get_status_message(&self) -> String {
        let report = validate_puzzle(&self.pieces);
        report.message
    }

    /// Returns a JSON array of all piece states including position, area, slope, and color.
    pub fn get_piece_positions_json(&self) -> String {
        let dtos: Vec<PieceStateDto> = self
            .pieces
            .iter()
            .map(|p| PieceStateDto {
                id: format!("{:?}", p.id),
                name: p.name.clone(),
                x: (p.position.x * 100.0).round() / 100.0,
                y: (p.position.y * 100.0).round() / 100.0,
                color: p.color.clone(),
                area: p.theoretical_area,
                slope: p.slope,
            })
            .collect();
        serde_json::to_string_pretty(&dtos).unwrap_or_else(|_| "[]".to_string())
    }

    /// Returns an in-depth mathematical explanation of the Curry Missing Square illusion.
    pub fn get_mathematical_explanation(&self) -> String {
        r#"{
  "title": "Curry Paradox & Cassini's Identity",
  "total_pieces_area": 32.0,
  "enclosing_triangle_theoretical_area": 32.5,
  "red_triangle": {
    "dimensions": "8x3",
    "area": 12.0,
    "slope": "3/8 = 0.375"
  },
  "green_triangle": {
    "dimensions": "5x2",
    "area": 5.0,
    "slope": "2/5 = 0.400"
  },
  "terracotta_polyomino": {
    "area": 8.0
  },
  "honey_amber_polyomino": {
    "area": 7.0
  },
  "slope_difference": 0.025,
  "deflection_angle_deg": 1.24536,
  "cassini_identity": "F_5 * F_7 - F_6^2 = 5 * 13 - 8^2 = 65 - 64 = 1 = (-1)^6",
  "explanation": "Because 3/8 (0.375) is strictly less than 2/5 (0.400), the combined hypotenuse is bent. In Configuration A, the bend dips inward (concave) by 1/13 unit, subtracting 0.5 units of area (32.5 - 0.5 = 32). In Configuration B, the bend bulges outward (convex) by 1/13 unit, adding 0.5 units of area (32.5 + 0.5 = 33). The 1 unit difference between 33 and 32 accounts precisely for the 1x1 missing void at (10, 0)!"
}"#.to_string()
    }

    /// Evaluates Cassini's Identity for n=6 and returns JSON.
    pub fn get_cassini_info(&self) -> String {
        let res = evaluate_cassini(6);
        serde_json::to_string_pretty(&res).unwrap_or_else(|_| "{}".to_string())
    }
}

impl Default for PuzzleEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::ValidationReport;

    #[test]
    fn test_engine_init_and_config_switch() {
        let mut engine = PuzzleEngine::new();
        let report_a: ValidationReport = serde_json::from_str(&engine.run_validation()).unwrap();
        assert!(report_a.is_config_a);
        assert!(!report_a.is_config_b);

        assert!(engine.set_configuration("B"));
        let report_b: ValidationReport = serde_json::from_str(&engine.run_validation()).unwrap();
        assert!(!report_b.is_config_a);
        assert!(report_b.is_config_b);
        assert_eq!(report_b.missing_squares, vec![(10, 0)]);

        assert!(engine.set_configuration("A"));
        let report_a2: ValidationReport = serde_json::from_str(&engine.run_validation()).unwrap();
        assert!(report_a2.is_config_a);
    }

    #[test]
    fn test_engine_drag_and_snap() {
        let mut engine = PuzzleEngine::new();
        assert!(engine.set_configuration("B"));

        // Drag Red piece slightly off
        assert!(engine.drag_move("red", 5.2, 2.1));
        let report: ValidationReport = serde_json::from_str(&engine.run_validation()).unwrap();
        // Since it's slightly off, it's not exactly Config B
        assert!(!report.is_config_b);

        // Snap it back
        assert!(engine.snap_selected_piece("red"));
        let report_snapped: ValidationReport = serde_json::from_str(&engine.run_validation()).unwrap();
        assert!(report_snapped.is_config_b);
    }

    #[test]
    fn test_engine_render_svg_options() {
        let mut engine = PuzzleEngine::new();
        engine.set_laser_visible(true);
        engine.set_lozenge_visible(true);
        engine.set_magnify_bend(true);

        let svg = engine.render_svg();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("True Hypotenuse"));
        assert!(svg.contains("Mystery Lozenge Strip"));
        assert!(svg.contains("Dynamic Bend Magnifier"));
    }
}
