use crate::geometry::mystery_lozenge_points;
use crate::grid::{validate_puzzle, ConfigurationState, GRID_HEIGHT, GRID_WIDTH};
use crate::piece::{Piece, PieceId};

pub struct SvgRenderOptions {
    pub laser_visible: bool,
    pub lozenge_visible: bool,
    pub magnify_bend: bool,
    pub selected_piece_id: Option<PieceId>,
}

impl Default for SvgRenderOptions {
    fn default() -> Self {
        Self {
            laser_visible: true,
            lozenge_visible: false,
            magnify_bend: false,
            selected_piece_id: None,
        }
    }
}

pub struct SvgRenderer {
    pub scale: f64,
    pub margin_left: f64,
    pub margin_top: f64,
    pub width: f64,
    pub height: f64,
}

impl Default for SvgRenderer {
    fn default() -> Self {
        let scale = 48.0;
        let margin_left = 65.0;
        let margin_top = 50.0;
        let width = margin_left * 2.0 + (GRID_WIDTH as f64) * scale;
        let height = margin_top + (GRID_HEIGHT as f64) * scale + 90.0;
        Self {
            scale,
            margin_left,
            margin_top,
            width: width.max(800.0),
            height: height.max(420.0),
        }
    }
}

impl SvgRenderer {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn to_svg_x(&self, x: f64) -> f64 {
        self.margin_left + x * self.scale
    }

    #[inline]
    pub fn to_svg_y(&self, y: f64) -> f64 {
        self.margin_top + (GRID_HEIGHT as f64 - y) * self.scale
    }

    pub fn render(&self, pieces: &[Piece], options: &SvgRenderOptions) -> String {
        let mut svg = String::with_capacity(16384);
        let report = validate_puzzle(pieces);

        svg.push_str(&format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {:.1} {:.1}" width="100%" height="100%" style="background:#FAF8F5;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif;user-select:none;">"##,
            self.width, self.height
        ));
        self.append_defs(&mut svg);

        // Background canvas card
        svg.push_str(&format!(
            r##"<rect x="10" y="10" width="{:.1}" height="{:.1}" rx="16" fill="#FFFDFB" stroke="#EAE4DC" stroke-width="1.5" filter="url(#shadow-card)"/>"##,
            self.width - 20.0,
            self.height - 20.0
        ));

        self.render_grid(&mut svg);

        if options.lozenge_visible {
            self.render_lozenge(&mut svg);
        }

        if report.is_config_b {
            self.render_missing_square_highlight(&mut svg);
        }

        for piece in pieces {
            let is_selected = options.selected_piece_id == Some(piece.id);
            self.render_piece(&mut svg, piece, is_selected);
        }

        if options.laser_visible {
            self.render_laser_straightedge(&mut svg);
        }

        if options.magnify_bend {
            self.render_magnification_loupe(&mut svg, pieces);
        }

        self.render_status_bar(&mut svg, &report);

        svg.push_str("</svg>");
        svg
    }

    fn append_defs(&self, svg: &mut String) {
        svg.push_str(r##"<defs>
<filter id="shadow-card" x="-5%" y="-5%" width="110%" height="110%">
  <feDropShadow dx="0" dy="4" stdDeviation="8" flood-color="#3C342C" flood-opacity="0.06"/>
</filter>
<filter id="glow-laser" x="-20%" y="-20%" width="140%" height="140%">
  <feGaussianBlur stdDeviation="3" result="blur"/>
  <feMerge>
    <feMergeNode in="blur"/>
    <feMergeNode in="SourceGraphic"/>
  </feMerge>
</filter>
<filter id="piece-shadow" x="-10%" y="-10%" width="120%" height="120%">
  <feDropShadow dx="0" dy="2" stdDeviation="3" flood-color="#2D241E" flood-opacity="0.18"/>
</filter>
<pattern id="diagonal-stripe" width="8" height="8" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
  <line x1="0" y1="0" x2="0" y2="8" stroke="#9333EA" stroke-width="2.5" opacity="0.3"/>
</pattern>
<pattern id="missing-stripe" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
  <line x1="0" y1="0" x2="0" y2="6" stroke="#EF4444" stroke-width="2" opacity="0.4"/>
</pattern>
</defs>"##);
    }

    fn render_grid(&self, svg: &mut String) {
        let x0 = self.to_svg_x(0.0);
        let x13 = self.to_svg_x(GRID_WIDTH as f64);
        let y0 = self.to_svg_y(0.0);
        let y5 = self.to_svg_y(GRID_HEIGHT as f64);

        svg.push_str(&format!(
            r##"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" fill="#F4EFEA" rx="4"/>"##,
            x0, y5, x13 - x0, y0 - y5
        ));

        for y in 0..=GRID_HEIGHT {
            let sy = self.to_svg_y(y as f64);
            let is_axis = y == 0 || y == GRID_HEIGHT;
            let stroke = if is_axis { "#8A7D70" } else { "#DDD5CA" };
            let width = if is_axis { "1.8" } else { "1.0" };

            svg.push_str(&format!(
                r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="{}" stroke-width="{}"/>"##,
                x0, sy, x13, sy, stroke, width
            ));

            svg.push_str(&format!(
                r##"<text x="{:.1}" y="{:.1}" font-size="11" font-weight="600" fill="#786D61" text-anchor="end" dominant-baseline="middle">{}</text>"##,
                x0 - 10.0, sy, y
            ));
        }

        for x in 0..=GRID_WIDTH {
            let sx = self.to_svg_x(x as f64);
            let is_axis = x == 0 || x == GRID_WIDTH;
            let stroke = if is_axis { "#8A7D70" } else { "#DDD5CA" };
            let width = if is_axis { "1.8" } else { "1.0" };

            svg.push_str(&format!(
                r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="{}" stroke-width="{}"/>"##,
                sx, y0, sx, y5, stroke, width
            ));

            svg.push_str(&format!(
                r##"<text x="{:.1}" y="{:.1}" font-size="11" font-weight="600" fill="#786D61" text-anchor="middle">{}</text>"##,
                sx, y0 + 18.0, x
            ));
        }
    }

    fn render_piece(&self, svg: &mut String, piece: &Piece, is_selected: bool) {
        let wv = piece.world_vertices();
        if wv.len() < 3 {
            return;
        }

        let mut path_data = String::new();
        for (i, p) in wv.iter().enumerate() {
            let sx = self.to_svg_x(p.x);
            let sy = self.to_svg_y(p.y);
            if i == 0 {
                path_data.push_str(&format!("M {:.2} {:.2} ", sx, sy));
            } else {
                path_data.push_str(&format!("L {:.2} {:.2} ", sx, sy));
            }
        }
        path_data.push('Z');

        let stroke_width = if is_selected { "3.5" } else { "2.2" };
        let stroke_color = if is_selected { "#1D4ED8" } else { &piece.stroke_color };
        let filter = if is_selected { r##"filter="url(#shadow-card)""## } else { r##"filter="url(#piece-shadow)""## };

        svg.push_str(&format!(
            r##"<path d="{}" fill="{}" stroke="{}" stroke-width="{}" stroke-linejoin="round" stroke-linecap="round" {} cursor="grab"/>"##,
            path_data, piece.color, stroke_color, stroke_width, filter
        ));

        let center_x = wv.iter().map(|p| p.x).sum::<f64>() / (wv.len() as f64);
        let center_y = wv.iter().map(|p| p.y).sum::<f64>() / (wv.len() as f64);
        let scx = self.to_svg_x(center_x);
        let scy = self.to_svg_y(center_y);

        let label = match piece.id {
            PieceId::RedTriangle => "8×3 (12)",
            PieceId::GreenTriangle => "5×2 (5)",
            PieceId::OrangePolyomino => "Area 8",
            PieceId::YellowPolyomino => "Area 7",
        };

        svg.push_str(&format!(
            r##"<text x="{:.1}" y="{:.1}" font-size="12" font-weight="700" fill="#FFFFFF" text-anchor="middle" dominant-baseline="central" pointer-events="none" style="text-shadow:0 1px 2px rgba(0,0,0,0.5);">{}</text>"##,
            scx, scy, label
        ));
    }

    fn render_laser_straightedge(&self, svg: &mut String) {
        let x0 = self.to_svg_x(0.0);
        let y0 = self.to_svg_y(0.0);
        let x13 = self.to_svg_x(13.0);
        let y5 = self.to_svg_y(5.0);

        svg.push_str(&format!(
            r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#FF2A6D" stroke-width="5" stroke-linecap="round" opacity="0.35" filter="url(#glow-laser)" pointer-events="none"/>"##,
            x0, y0, x13, y5
        ));

        svg.push_str(&format!(
            r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#FF0055" stroke-width="2.2" stroke-dasharray="7,4" stroke-linecap="round" pointer-events="none"/>"##,
            x0, y0, x13, y5
        ));

        let mid_x = (x0 + x13) * 0.5;
        let mid_y = (y0 + y5) * 0.5;
        svg.push_str(&format!(
            r##"<g transform="translate({:.1}, {:.1}) rotate(-21)" pointer-events="none">
<rect x="-110" y="-18" width="220" height="18" rx="4" fill="#FFE4E6" opacity="0.9" stroke="#FDA4AF" stroke-width="1"/>
<text x="0" y="-6" font-size="10" font-weight="700" fill="#BE123C" text-anchor="middle">True Hypotenuse: Slope 5/13 ≈ 0.3846</text>
</g>"##,
            mid_x, mid_y
        ));
    }

    fn render_lozenge(&self, svg: &mut String) {
        let pts = mystery_lozenge_points();
        let mut d = String::new();
        for (i, p) in pts.iter().enumerate() {
            let sx = self.to_svg_x(p.x);
            let sy = self.to_svg_y(p.y);
            if i == 0 {
                d.push_str(&format!("M {:.2} {:.2} ", sx, sy));
            } else {
                d.push_str(&format!("L {:.2} {:.2} ", sx, sy));
            }
        }
        d.push('Z');

        svg.push_str(&format!(
            r##"<path d="{}" fill="url(#diagonal-stripe)" stroke="#9333EA" stroke-width="2.2" stroke-dasharray="4,3" pointer-events="none"/>"##,
            d
        ));

        let cx = self.to_svg_x(6.5);
        let cy = self.to_svg_y(2.5);
        svg.push_str(&format!(
            r##"<g transform="translate({:.1}, {:.1})" pointer-events="none">
<rect x="-105" y="-12" width="210" height="24" rx="12" fill="#581C87" opacity="0.92"/>
<text x="0" y="4" font-size="10.5" font-weight="700" fill="#F3E8FF" text-anchor="middle">Mystery Lozenge Strip (Area = 1.0)</text>
</g>"##,
            cx, cy
        ));
    }

    fn render_missing_square_highlight(&self, svg: &mut String) {
        let sx = self.to_svg_x(10.0);
        let sy = self.to_svg_y(1.0);
        let s = self.scale;

        svg.push_str(&format!(
            r##"<g pointer-events="none">
<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" fill="url(#missing-stripe)" stroke="#EF4444" stroke-width="2.5" stroke-dasharray="5,3" rx="4"/>
<circle cx="{:.1}" cy="{:.1}" r="12" fill="#EF4444"/>
<text x="{:.1}" y="{:.1}" font-size="14" font-weight="800" fill="#FFFFFF" text-anchor="middle" dominant-baseline="central">?</text>
<text x="{:.1}" y="{:.1}" font-size="10" font-weight="700" fill="#B91C1C" text-anchor="middle">Missing 1×1 Void</text>
</g>"##,
            sx, sy, s, s,
            sx + s * 0.5, sy + s * 0.5,
            sx + s * 0.5, sy + s * 0.5,
            sx + s * 0.5, sy + s + 13.0
        ));
    }

    fn render_magnification_loupe(&self, svg: &mut String, pieces: &[Piece]) {
        let report = validate_puzzle(pieces);
        let (bend_x, bend_y, slope_1, slope_2, deflection) = if report.is_config_b {
            (5.0, 2.0, "2/5 = 0.400", "3/8 = 0.375", "+1/13 ≈ +0.077 (Bulge)")
        } else {
            (8.0, 3.0, "3/8 = 0.375", "2/5 = 0.400", "-1/13 ≈ -0.077 (Dip)")
        };

        let loupe_cx = self.width - 120.0;
        let loupe_cy = 110.0;
        let loupe_r = 68.0;

        let src_sx = self.to_svg_x(bend_x);
        let src_sy = self.to_svg_y(bend_y);

        svg.push_str(&format!(
            r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#4F46E5" stroke-width="1.5" stroke-dasharray="3,3" opacity="0.6"/>
<circle cx="{:.1}" cy="{:.1}" r="5" fill="#4F46E5" stroke="#FFFFFF" stroke-width="1.5"/>"##,
            src_sx, src_sy, loupe_cx, loupe_cy, src_sx, src_sy
        ));

        svg.push_str(&format!(
            r##"<g pointer-events="none">
<circle cx="{:.1}" cy="{:.1}" r="{:.1}" fill="#1E1B4B" stroke="#6366F1" stroke-width="3" filter="url(#shadow-card)"/>
<text x="{:.1}" y="{:.1}" font-size="10.5" font-weight="700" fill="#A5B4FC" text-anchor="middle">Dynamic Bend Magnifier (4×)</text>
<text x="{:.1}" y="{:.1}" font-size="9.5" font-weight="600" fill="#E0E7FF" text-anchor="middle">Point ({:.0}, {:.0})</text>
<text x="{:.1}" y="{:.1}" font-size="9" fill="#FCD34D" text-anchor="middle">Δθ = 1.245° (Bending)</text>
<text x="{:.1}" y="{:.1}" font-size="8.5" fill="#F87171" text-anchor="middle">Δy = {}</text>
<text x="{:.1}" y="{:.1}" font-size="8" fill="#CBD5E1" text-anchor="middle">{} vs {}</text>
</g>"##,
            loupe_cx, loupe_cy, loupe_r,
            loupe_cx, loupe_cy - 40.0,
            loupe_cx, loupe_cy - 24.0, bend_x, bend_y,
            loupe_cx, loupe_cy - 6.0,
            loupe_cx, loupe_cy + 12.0, deflection,
            loupe_cx, loupe_cy + 28.0, slope_1, slope_2
        ));
    }

    fn render_status_bar(&self, svg: &mut String, report: &crate::grid::ValidationReport) {
        let bar_y = self.height - 42.0;
        let (badge_color, badge_text) = match report.state {
            ConfigurationState::ConfigA => ("#059669", "Configuration A (Solid)"),
            ConfigurationState::ConfigB => ("#DC2626", "Configuration B (Paradox Hole!)"),
            ConfigurationState::CustomValid => ("#2563EB", "Custom Arrangement"),
            ConfigurationState::Collision(_) => ("#D97706", "Collision Warning"),
            ConfigurationState::OutOfBounds(_) => ("#B91C1C", "Out of Bounds"),
        };

        svg.push_str(&format!(
            r##"<g transform="translate(25, {:.1})" pointer-events="none">
<rect x="0" y="0" width="190" height="26" rx="6" fill="{}" />
<text x="95" y="17" font-size="11" font-weight="700" fill="#FFFFFF" text-anchor="middle">{}</text>
<text x="205" y="17" font-size="11.5" font-weight="500" fill="#4B443B">{}</text>
</g>"##,
            bar_y, badge_color, badge_text, report.message
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_generation_validity() {
        let pieces = Piece::create_all();
        let renderer = SvgRenderer::new();
        let options = SvgRenderOptions {
            laser_visible: true,
            lozenge_visible: true,
            magnify_bend: true,
            selected_piece_id: Some(PieceId::RedTriangle),
        };
        let svg = renderer.render(&pieces, &options);

        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
        assert!(svg.contains("viewBox="));
        assert!(svg.contains("True Hypotenuse"));
        assert!(svg.contains("Mystery Lozenge Strip"));
        assert!(svg.contains("Dynamic Bend Magnifier"));
    }
}
