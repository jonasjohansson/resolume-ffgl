use std::ffi::CString;
use std::sync::LazyLock;

use ffgl_core::parameters::{ParameterTypes, SimpleParamInfo};

// ---------------------------------------------------------------------------
// Parameter indices
// ---------------------------------------------------------------------------

pub const PARAM_TEXT: usize = 0;
pub const PARAM_TEXT_FILE: usize = 1;
pub const PARAM_TEXT_TRANSFORM: usize = 2;
pub const PARAM_BEAT_CYCLE: usize = 3;
pub const PARAM_CYCLE_DURATION: usize = 4;
pub const PARAM_CYCLE_MODE: usize = 5;
pub const PARAM_CYCLE_RESET: usize = 6;
pub const PARAM_FONT: usize = 7;
pub const PARAM_FONT_SIZE: usize = 8;
pub const PARAM_TRACKING: usize = 9;
pub const PARAM_LEADING: usize = 10;
pub const PARAM_ALIGNMENT: usize = 11;
pub const PARAM_V_ALIGN: usize = 12;
pub const PARAM_POSITION_X: usize = 13;
pub const PARAM_POSITION_Y: usize = 14;
pub const PARAM_FILL_R: usize = 15;
pub const PARAM_FILL_G: usize = 16;
pub const PARAM_FILL_B: usize = 17;
pub const PARAM_FILL_A: usize = 18;
pub const PARAM_STROKE: usize = 19;
pub const PARAM_STROKE_POSITION: usize = 20;
pub const PARAM_STROKE_WIDTH: usize = 21;
pub const PARAM_STROKE_R: usize = 22;
pub const PARAM_STROKE_G: usize = 23;
pub const PARAM_STROKE_B: usize = 24;
pub const PARAM_STROKE_A: usize = 25;
pub const PARAM_DROP_SHADOW: usize = 26;
pub const PARAM_SHADOW_X: usize = 27;
pub const PARAM_SHADOW_Y: usize = 28;
pub const PARAM_SHADOW_R: usize = 29;
pub const PARAM_SHADOW_G: usize = 30;
pub const PARAM_SHADOW_B: usize = 31;
pub const PARAM_SHADOW_A: usize = 32;
pub const NUM_PARAMS: usize = 33;

// Cycle mode option values
pub const CYCLE_MODE_LOOP: u32 = 0;
pub const CYCLE_MODE_HOLD: u32 = 1;
pub const CYCLE_MODE_BLACK: u32 = 2;

// Cycle duration option indices (as rounded f32 values)
pub const CYCLE_QUARTER_BEAT: u32 = 0;
pub const CYCLE_HALF_BEAT: u32 = 1;
pub const CYCLE_1_BEAT: u32 = 2;
pub const CYCLE_2_BEATS: u32 = 3;
pub const CYCLE_1_BAR: u32 = 4;
pub const CYCLE_2_BARS: u32 = 5;
pub const CYCLE_4_BARS: u32 = 6;
pub const CYCLE_8_BARS: u32 = 7;

/// Convert cycle duration option index to interval in bars (assuming 4 beats/bar).
pub fn cycle_duration_bars(option: u32) -> f32 {
    match option {
        CYCLE_QUARTER_BEAT => 0.0625,
        CYCLE_HALF_BEAT => 0.125,
        CYCLE_1_BEAT => 0.25,
        CYCLE_2_BEATS => 0.5,
        CYCLE_1_BAR => 1.0,
        CYCLE_2_BARS => 2.0,
        CYCLE_4_BARS => 4.0,
        CYCLE_8_BARS => 8.0,
        _ => 1.0,
    }
}

// ---------------------------------------------------------------------------
// Font enumeration via CoreText
// ---------------------------------------------------------------------------

fn build_font_elements() -> Vec<(CString, f32)> {
    let collection = core_text::font_collection::create_for_all_families();
    let descriptors = collection.get_descriptors();

    let mut names: Vec<String> = Vec::new();
    if let Some(descs) = descriptors {
        for i in 0..descs.len() as isize {
            if let Some(desc) = descs.get(i) {
                let family = desc.family_name();
                names.push(family);
            }
        }
    }

    names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    names.dedup();

    let count = names.len().max(1);
    names
        .into_iter()
        .enumerate()
        .map(|(i, name)| {
            let val = if count > 1 {
                i as f32 / (count - 1) as f32
            } else {
                0.0
            };
            (CString::new(name).unwrap_or_else(|_| CString::new("Unknown").unwrap()), val)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Static parameter info table
// ---------------------------------------------------------------------------

pub static PARAM_INFOS: LazyLock<Vec<SimpleParamInfo>> = LazyLock::new(|| {
    let font_elements = build_font_elements();

    vec![
        // 0 – Text
        SimpleParamInfo {
            name: CString::new("Text").unwrap(),
            param_type: ParameterTypes::Text,
            default_string: Some(CString::new("Hello World").unwrap()),
            ..Default::default()
        },
        // 1 – Text File
        SimpleParamInfo {
            name: CString::new("Text File").unwrap(),
            param_type: ParameterTypes::File,
            file_extensions: Some(vec![CString::new("txt").unwrap()]),
            ..Default::default()
        },
        // 2 – Text Transform
        SimpleParamInfo {
            name: CString::new("Text Transform").unwrap(),
            param_type: ParameterTypes::Option,
            default: Some(0.0),
            elements: Some(vec![
                (CString::new("None").unwrap(), 0.0),
                (CString::new("Uppercase").unwrap(), 1.0),
                (CString::new("Lowercase").unwrap(), 2.0),
                (CString::new("Title Case").unwrap(), 3.0),
            ]),
            ..Default::default()
        },
        // 3 – Beat Cycle
        SimpleParamInfo {
            name: CString::new("Beat Cycle").unwrap(),
            param_type: ParameterTypes::Boolean,
            default: Some(0.0),
            ..Default::default()
        },
        // 4 – Cycle Duration
        SimpleParamInfo {
            name: CString::new("Cycle Duration").unwrap(),
            param_type: ParameterTypes::Option,
            default: Some(CYCLE_1_BAR as f32),
            elements: Some(vec![
                (CString::new("1/4 Beat").unwrap(), CYCLE_QUARTER_BEAT as f32),
                (CString::new("1/2 Beat").unwrap(), CYCLE_HALF_BEAT as f32),
                (CString::new("1 Beat").unwrap(), CYCLE_1_BEAT as f32),
                (CString::new("2 Beats").unwrap(), CYCLE_2_BEATS as f32),
                (CString::new("1 Bar").unwrap(), CYCLE_1_BAR as f32),
                (CString::new("2 Bars").unwrap(), CYCLE_2_BARS as f32),
                (CString::new("4 Bars").unwrap(), CYCLE_4_BARS as f32),
                (CString::new("8 Bars").unwrap(), CYCLE_8_BARS as f32),
            ]),
            ..Default::default()
        },
        // 5 – Cycle Mode (what happens after last line)
        SimpleParamInfo {
            name: CString::new("Cycle Mode").unwrap(),
            param_type: ParameterTypes::Option,
            default: Some(CYCLE_MODE_LOOP as f32),
            elements: Some(vec![
                (CString::new("Loop").unwrap(), CYCLE_MODE_LOOP as f32),
                (CString::new("Hold Last").unwrap(), CYCLE_MODE_HOLD as f32),
                (CString::new("Black").unwrap(), CYCLE_MODE_BLACK as f32),
            ]),
            ..Default::default()
        },
        // 6 – Cycle Reset (trigger)
        SimpleParamInfo {
            name: CString::new("Restart Cycle").unwrap(),
            param_type: ParameterTypes::Event,
            default: Some(0.0),
            ..Default::default()
        },
        // 7 – Font
        SimpleParamInfo {
            name: CString::new("Font").unwrap(),
            param_type: ParameterTypes::Option,
            default: Some(0.0),
            elements: Some(font_elements),
            ..Default::default()
        },
        // 5 – Font Size (pt)
        SimpleParamInfo {
            name: CString::new("Font Size").unwrap(),
            param_type: ParameterTypes::Integer,
            default: Some(72.0),
            min: Some(1.0),
            max: Some(400.0),
            ..Default::default()
        },
        // 6 – Tracking
        SimpleParamInfo {
            name: CString::new("Tracking").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.5),
            ..Default::default()
        },
        // 7 – Leading
        SimpleParamInfo {
            name: CString::new("Leading").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.5),
            ..Default::default()
        },
        // 8 – Alignment
        SimpleParamInfo {
            name: CString::new("Alignment").unwrap(),
            param_type: ParameterTypes::Option,
            default: Some(1.0),
            elements: Some(vec![
                (CString::new("Left").unwrap(), 0.0),
                (CString::new("Center").unwrap(), 1.0),
                (CString::new("Right").unwrap(), 2.0),
                (CString::new("Justify").unwrap(), 3.0),
            ]),
            ..Default::default()
        },
        // 9 – Vertical Align
        SimpleParamInfo {
            name: CString::new("Vertical Align").unwrap(),
            param_type: ParameterTypes::Option,
            default: Some(1.0),
            elements: Some(vec![
                (CString::new("Top").unwrap(), 0.0),
                (CString::new("Center").unwrap(), 1.0),
                (CString::new("Bottom").unwrap(), 2.0),
            ]),
            ..Default::default()
        },
        // 10 – Position X
        SimpleParamInfo {
            name: CString::new("Position X").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.5),
            ..Default::default()
        },
        // 11 – Position Y
        SimpleParamInfo {
            name: CString::new("Position Y").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.5),
            ..Default::default()
        },
        // 12 – Fill R
        SimpleParamInfo {
            name: CString::new("Fill R").unwrap(),
            param_type: ParameterTypes::Red,
            default: Some(1.0),
            group: Some("Fill".to_string()),
            ..Default::default()
        },
        // 13 – Fill G
        SimpleParamInfo {
            name: CString::new("Fill G").unwrap(),
            param_type: ParameterTypes::Green,
            default: Some(1.0),
            group: Some("Fill".to_string()),
            ..Default::default()
        },
        // 14 – Fill B
        SimpleParamInfo {
            name: CString::new("Fill B").unwrap(),
            param_type: ParameterTypes::Blue,
            default: Some(1.0),
            group: Some("Fill".to_string()),
            ..Default::default()
        },
        // 15 – Fill A
        SimpleParamInfo {
            name: CString::new("Fill A").unwrap(),
            param_type: ParameterTypes::Alpha,
            default: Some(1.0),
            group: Some("Fill".to_string()),
            ..Default::default()
        },
        // 16 – Stroke
        SimpleParamInfo {
            name: CString::new("Stroke").unwrap(),
            param_type: ParameterTypes::Boolean,
            default: Some(0.0),
            ..Default::default()
        },
        // 17 – Stroke Position
        SimpleParamInfo {
            name: CString::new("Stroke Position").unwrap(),
            param_type: ParameterTypes::Option,
            default: Some(0.0),
            elements: Some(vec![
                (CString::new("Center").unwrap(), 0.0),
                (CString::new("Outside").unwrap(), 1.0),
                (CString::new("Inside").unwrap(), 2.0),
            ]),
            ..Default::default()
        },
        // 18 – Stroke Width (px)
        SimpleParamInfo {
            name: CString::new("Stroke Width").unwrap(),
            param_type: ParameterTypes::Integer,
            default: Some(2.0),
            min: Some(1.0),
            max: Some(50.0),
            ..Default::default()
        },
        // 19 – Stroke R
        SimpleParamInfo {
            name: CString::new("Stroke R").unwrap(),
            param_type: ParameterTypes::Red,
            default: Some(0.0),
            group: Some("Stroke Color".to_string()),
            ..Default::default()
        },
        // 20 – Stroke G
        SimpleParamInfo {
            name: CString::new("Stroke G").unwrap(),
            param_type: ParameterTypes::Green,
            default: Some(0.0),
            group: Some("Stroke Color".to_string()),
            ..Default::default()
        },
        // 21 – Stroke B
        SimpleParamInfo {
            name: CString::new("Stroke B").unwrap(),
            param_type: ParameterTypes::Blue,
            default: Some(0.0),
            group: Some("Stroke Color".to_string()),
            ..Default::default()
        },
        // 22 – Stroke A
        SimpleParamInfo {
            name: CString::new("Stroke A").unwrap(),
            param_type: ParameterTypes::Alpha,
            default: Some(1.0),
            group: Some("Stroke Color".to_string()),
            ..Default::default()
        },
        // 23 – Drop Shadow
        SimpleParamInfo {
            name: CString::new("Drop Shadow").unwrap(),
            param_type: ParameterTypes::Boolean,
            default: Some(0.0),
            ..Default::default()
        },
        // 24 – Shadow X
        SimpleParamInfo {
            name: CString::new("Shadow X").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.55),
            ..Default::default()
        },
        // 25 – Shadow Y
        SimpleParamInfo {
            name: CString::new("Shadow Y").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.45),
            ..Default::default()
        },
        // 26 – Shadow R
        SimpleParamInfo {
            name: CString::new("Shadow R").unwrap(),
            param_type: ParameterTypes::Red,
            default: Some(0.0),
            group: Some("Shadow Color".to_string()),
            ..Default::default()
        },
        // 27 – Shadow G
        SimpleParamInfo {
            name: CString::new("Shadow G").unwrap(),
            param_type: ParameterTypes::Green,
            default: Some(0.0),
            group: Some("Shadow Color".to_string()),
            ..Default::default()
        },
        // 28 – Shadow B
        SimpleParamInfo {
            name: CString::new("Shadow B").unwrap(),
            param_type: ParameterTypes::Blue,
            default: Some(0.0),
            group: Some("Shadow Color".to_string()),
            ..Default::default()
        },
        // 29 – Shadow A
        SimpleParamInfo {
            name: CString::new("Shadow A").unwrap(),
            param_type: ParameterTypes::Alpha,
            default: Some(0.5),
            group: Some("Shadow Color".to_string()),
            ..Default::default()
        },
    ]
});
