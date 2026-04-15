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
pub const PARAM_FONT: usize = 4;
pub const PARAM_FONT_SIZE: usize = 5;
pub const PARAM_TRACKING: usize = 6;
pub const PARAM_LEADING: usize = 7;
pub const PARAM_ALIGNMENT: usize = 8;
pub const PARAM_V_ALIGN: usize = 9;
pub const PARAM_POSITION_X: usize = 10;
pub const PARAM_POSITION_Y: usize = 11;
pub const PARAM_FILL_R: usize = 12;
pub const PARAM_FILL_G: usize = 13;
pub const PARAM_FILL_B: usize = 14;
pub const PARAM_FILL_A: usize = 15;
pub const PARAM_STROKE: usize = 16;
pub const PARAM_STROKE_POSITION: usize = 17;
pub const PARAM_STROKE_WIDTH: usize = 18;
pub const PARAM_STROKE_R: usize = 19;
pub const PARAM_STROKE_G: usize = 20;
pub const PARAM_STROKE_B: usize = 21;
pub const PARAM_STROKE_A: usize = 22;
pub const PARAM_DROP_SHADOW: usize = 23;
pub const PARAM_SHADOW_X: usize = 24;
pub const PARAM_SHADOW_Y: usize = 25;
pub const PARAM_SHADOW_R: usize = 26;
pub const PARAM_SHADOW_G: usize = 27;
pub const PARAM_SHADOW_B: usize = 28;
pub const PARAM_SHADOW_A: usize = 29;
pub const NUM_PARAMS: usize = 30;

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
        // 4 – Font
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
