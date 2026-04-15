use core_foundation::attributed_string::CFMutableAttributedString;
use core_foundation::base::{CFRange, CFType, TCFType};
use core_foundation::number::CFNumber;
use core_foundation::string::CFString;
use core_graphics::base::kCGImageAlphaPremultipliedLast;
use core_graphics::color_space::CGColorSpace;
use core_graphics::context::{CGContext, CGTextDrawingMode};
use core_graphics::geometry::{CGPoint, CGRect, CGSize};
use core_text::font;
use core_text::line::CTLine;
use core_text::string_attributes::{
    kCTFontAttributeName, kCTForegroundColorFromContextAttributeName, kCTKernAttributeName,
};

// ---------------------------------------------------------------------------
// Public data types
// ---------------------------------------------------------------------------

pub struct TextRenderer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

pub struct RenderParams {
    pub text: String,
    pub font_family: String,
    pub font_size: f64,
    pub color: [f32; 4],
    pub outline_enabled: bool,
    pub outline_color: [f32; 4],
    pub outline_width: f32,
    pub shadow_enabled: bool,
    pub shadow_color: [f32; 4],
    pub shadow_offset: f32,
    pub h_align: u32,
    pub v_align: u32,
    pub line_spacing: f32,
    pub letter_spacing: f32,
    pub position_x: f32,
    pub position_y: f32,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl TextRenderer {
    /// Render text into an RGBA pixel buffer using CoreText / CoreGraphics.
    pub fn render(width: usize, height: usize, params: &RenderParams) -> Self {
        let w = width;
        let h = height;

        // If text is empty, return a transparent buffer without touching CoreText
        if params.text.is_empty() {
            return TextRenderer {
                width: w,
                height: h,
                pixels: vec![0u8; w * h * 4],
            };
        }

        // --- Create bitmap context (RGBA, premultiplied alpha) ---------------
        let cs = CGColorSpace::create_device_rgb();
        let bytes_per_row = w * 4;
        let mut ctx = CGContext::create_bitmap_context(
            None,
            w,
            h,
            8,
            bytes_per_row,
            &cs,
            kCGImageAlphaPremultipliedLast,
        );

        // Clear to transparent
        ctx.clear_rect(CGRect::new(
            &CGPoint::new(0.0, 0.0),
            &CGSize::new(w as f64, h as f64),
        ));

        // Enable anti-aliasing
        ctx.set_should_antialias(true);
        ctx.set_allows_font_smoothing(true);
        ctx.set_should_smooth_fonts(true);

        // --- Create CTFont ---------------------------------------------------
        let ct_font = font::new_from_name(&params.font_family, params.font_size)
            .unwrap_or_else(|_| font::new_from_name("Helvetica", params.font_size).unwrap());

        // --- Split text into lines and build CTLines -------------------------
        let text_lines: Vec<&str> = params.text.split('\n').collect();
        let ct_lines: Vec<CTLine> = text_lines
            .iter()
            .map(|line_str| build_ct_line(line_str, &ct_font, params.letter_spacing))
            .collect();

        // --- Measure each line -----------------------------------------------
        struct LineMeasure {
            width: f64,
            ascent: f64,
            descent: f64,
            leading: f64,
        }

        let measures: Vec<LineMeasure> = ct_lines
            .iter()
            .map(|line| {
                let bounds = line.get_typographic_bounds();
                LineMeasure {
                    width: bounds.width,
                    ascent: bounds.ascent,
                    descent: bounds.descent,
                    leading: bounds.leading,
                }
            })
            .collect();

        // --- Compute total text block height ---------------------------------
        let line_count = measures.len();
        let spacing_multiplier = params.line_spacing as f64;
        let mut total_height = 0.0_f64;
        for (i, m) in measures.iter().enumerate() {
            total_height += m.ascent + m.descent;
            if i < line_count - 1 {
                total_height += m.leading.max(0.0) * spacing_multiplier
                    + (m.ascent + m.descent) * (spacing_multiplier - 1.0).max(0.0);
            }
        }

        // --- Vertical starting position (CG origin is bottom-left) -----------
        // position_y: 0.0 = top, 0.5 = center, 1.0 = bottom
        let canvas_h = h as f64;
        let canvas_w = w as f64;

        // Base y from v_align (in CG coords where y=0 is bottom)
        let block_top_cg = match params.v_align {
            0 => canvas_h - total_height, // top — block starts near top
            2 => 0.0,                     // bottom — block starts near bottom
            _ => (canvas_h - total_height) / 2.0, // center
        };

        // Apply position_y offset: shift the block. 0.5 = no shift.
        let offset_y = (params.position_y as f64 - 0.5) * canvas_h;
        // Positive offset_y should move text down, which in CG coords means subtracting
        let base_y_cg = block_top_cg - offset_y;

        // Apply position_x offset: 0.5 = no shift
        let offset_x = (params.position_x as f64 - 0.5) * canvas_w;

        // --- Helper: draw all lines at computed positions ---------------------
        let draw_lines =
            |ctx: &CGContext, dx: f64, dy: f64| {
                let origin_y = base_y_cg + dy;
                for (i, (ct_line, m)) in ct_lines.iter().zip(measures.iter()).enumerate() {
                    // Compute accumulated height from top of block to this line's baseline
                    let mut acc = 0.0_f64;
                    for j in 0..=i {
                        if j > 0 {
                            let prev = &measures[j - 1];
                            acc += prev.leading.max(0.0) * spacing_multiplier
                                + (prev.ascent + prev.descent)
                                    * (spacing_multiplier - 1.0).max(0.0);
                        }
                        acc += measures[j].ascent;
                        if j < i {
                            acc += measures[j].descent;
                        }
                    }

                    // CG coords: y=0 is bottom, so baseline_y goes downward from top
                    let baseline_y = origin_y + total_height - acc;

                    // Horizontal position
                    let x = match params.h_align {
                        0 => 0.0,                              // left
                        2 => canvas_w - m.width,               // right
                        _ => (canvas_w - m.width) / 2.0,       // center
                    } + offset_x
                        + dx;

                    ctx.set_text_position(x, baseline_y);
                    ct_line.draw(ctx);

                    let _ = (i, m); // used by enumerate/zip
                }
            };

        // --- Shadow pass -----------------------------------------------------
        if params.shadow_enabled {
            let sc = params.shadow_color;
            let so = params.shadow_offset as f64;

            ctx.save();
            ctx.set_text_drawing_mode(CGTextDrawingMode::CGTextFill);
            ctx.set_rgb_fill_color(sc[0] as f64, sc[1] as f64, sc[2] as f64, sc[3] as f64);
            draw_lines(&ctx, so, -so);
            ctx.restore();
        }

        // --- Outline pass ----------------------------------------------------
        if params.outline_enabled {
            let oc = params.outline_color;
            ctx.save();
            ctx.set_text_drawing_mode(CGTextDrawingMode::CGTextStroke);
            ctx.set_rgb_stroke_color(oc[0] as f64, oc[1] as f64, oc[2] as f64, oc[3] as f64);
            ctx.set_line_width(params.outline_width as f64);
            draw_lines(&ctx, 0.0, 0.0);
            ctx.restore();
        }

        // --- Fill pass -------------------------------------------------------
        {
            let fc = params.color;
            ctx.save();
            ctx.set_text_drawing_mode(CGTextDrawingMode::CGTextFill);
            ctx.set_rgb_fill_color(fc[0] as f64, fc[1] as f64, fc[2] as f64, fc[3] as f64);
            draw_lines(&ctx, 0.0, 0.0);
            ctx.restore();
        }

        // --- Extract pixel data ----------------------------------------------
        let data = ctx.data();
        let pixels = data.to_vec();

        TextRenderer {
            width: w,
            height: h,
            pixels,
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a CTLine from a string with the given font and letter spacing.
fn build_ct_line(text: &str, ct_font: &core_text::font::CTFont, letter_spacing: f32) -> CTLine {
    let cf_string = CFString::new(text);
    let mut attr_string = CFMutableAttributedString::new();
    let range_zero = CFRange::init(0, 0);
    attr_string.replace_str(&cf_string, range_zero);

    let len = attr_string.char_len();
    let full_range = CFRange::init(0, len);

    // Set font attribute
    unsafe {
        attr_string.set_attribute(full_range, kCTFontAttributeName, ct_font);
    }

    // Set kCTForegroundColorFromContextAttributeName = true
    // so that CTLine picks up the fill/stroke color from the CGContext
    unsafe {
        let yes = CFNumber::from(1i32);
        attr_string.set_attribute::<CFType>(
            full_range,
            kCTForegroundColorFromContextAttributeName,
            &CFType::wrap_under_get_rule(yes.as_CFTypeRef()),
        );
    }

    // Set kern (letter spacing) if non-zero
    if letter_spacing.abs() > f32::EPSILON {
        unsafe {
            let kern = CFNumber::from(letter_spacing as f64);
            attr_string.set_attribute::<CFType>(
                full_range,
                kCTKernAttributeName,
                &CFType::wrap_under_get_rule(kern.as_CFTypeRef()),
            );
        }
    }

    // Create CTLine from the attributed string
    let attr_string_ref = attr_string.as_concrete_TypeRef();
    CTLine::new_with_attributed_string(attr_string_ref)
}
