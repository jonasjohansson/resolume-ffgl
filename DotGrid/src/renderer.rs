use core_graphics::base::kCGImageAlphaPremultipliedLast;
use core_graphics::color_space::CGColorSpace;
use core_graphics::context::CGContext;
use core_graphics::geometry::{CGPoint, CGRect, CGSize};

pub struct DotGridRenderer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

pub struct RenderParams {
    pub columns: u32,
    pub rows: u32,
    pub dot_size: f32,      // 0..1 fraction of cell's shorter side
    pub align_x: u32,       // 0=left, 1=center, 2=right
    pub align_y: u32,       // 0=top,  1=center, 2=bottom
    pub fill: [f32; 4],
    pub background: [f32; 4],
}

impl DotGridRenderer {
    pub fn render(width: usize, height: usize, p: &RenderParams) -> Self {
        let w = width;
        let h = height;
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

        ctx.clear_rect(CGRect::new(
            &CGPoint::new(0.0, 0.0),
            &CGSize::new(w as f64, h as f64),
        ));

        if p.background[3] > 0.0 {
            ctx.set_rgb_fill_color(
                p.background[0] as f64,
                p.background[1] as f64,
                p.background[2] as f64,
                p.background[3] as f64,
            );
            ctx.fill_rect(CGRect::new(
                &CGPoint::new(0.0, 0.0),
                &CGSize::new(w as f64, h as f64),
            ));
        }

        let cols = p.columns.max(1) as f64;
        let rows = p.rows.max(1) as f64;
        let cell_w = w as f64 / cols;
        let cell_h = h as f64 / rows;
        let cell_min = cell_w.min(cell_h);
        let diameter = cell_min * p.dot_size.clamp(0.0, 1.0) as f64;

        ctx.set_should_antialias(true);
        ctx.set_allows_antialiasing(true);
        ctx.set_rgb_fill_color(
            p.fill[0] as f64,
            p.fill[1] as f64,
            p.fill[2] as f64,
            p.fill[3] as f64,
        );

        for row in 0..p.rows.max(1) {
            for col in 0..p.columns.max(1) {
                let cell_x = col as f64 * cell_w;
                // CG origin is bottom-left; rows 0 = top in UI terms means
                // flipping the row index when computing y.
                let cell_y_top_ui = row as f64 * cell_h;
                let cell_y = h as f64 - cell_y_top_ui - cell_h;

                let x = match p.align_x {
                    0 => cell_x,                        // left
                    2 => cell_x + cell_w - diameter,    // right
                    _ => cell_x + (cell_w - diameter) / 2.0, // center
                };
                let y = match p.align_y {
                    // UI "top" = highest on screen = highest CG y value
                    0 => cell_y + cell_h - diameter,
                    2 => cell_y,
                    _ => cell_y + (cell_h - diameter) / 2.0,
                };

                let rect = CGRect::new(
                    &CGPoint::new(x, y),
                    &CGSize::new(diameter, diameter),
                );
                ctx.fill_ellipse_in_rect(rect);
            }
        }

        let data = ctx.data();
        let pixels = data.to_vec();

        DotGridRenderer {
            width: w,
            height: h,
            pixels,
        }
    }
}
