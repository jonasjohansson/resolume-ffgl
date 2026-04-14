use std::ffi::CString;
use std::sync::LazyLock;

use ffgl_core::parameters::{ParameterTypes, SimpleParamInfo};

// ---------------------------------------------------------------------------
// Parameter indices
// ---------------------------------------------------------------------------

pub const PARAM_TEXT: usize = 0;
pub const PARAM_BEAT_CYCLE: usize = 1;
pub const PARAM_FONT: usize = 2;
pub const PARAM_FONT_SIZE: usize = 3;
pub const PARAM_FILL_R: usize = 4;
pub const PARAM_FILL_G: usize = 5;
pub const PARAM_FILL_B: usize = 6;
pub const PARAM_FILL_A: usize = 7;
pub const PARAM_OUTLINE_ENABLED: usize = 8;
pub const PARAM_OUTLINE_R: usize = 9;
pub const PARAM_OUTLINE_G: usize = 10;
pub const PARAM_OUTLINE_B: usize = 11;
pub const PARAM_OUTLINE_A: usize = 12;
pub const PARAM_OUTLINE_WIDTH: usize = 13;
pub const PARAM_SHADOW_ENABLED: usize = 14;
pub const PARAM_SHADOW_R: usize = 15;
pub const PARAM_SHADOW_G: usize = 16;
pub const PARAM_SHADOW_B: usize = 17;
pub const PARAM_SHADOW_A: usize = 18;
pub const PARAM_SHADOW_OFFSET: usize = 19;
pub const PARAM_H_ALIGN: usize = 20;
pub const PARAM_V_ALIGN: usize = 21;
pub const PARAM_LINE_SPACING: usize = 22;
pub const PARAM_LETTER_SPACING: usize = 23;
pub const PARAM_POSITION_X: usize = 24;
pub const PARAM_POSITION_Y: usize = 25;
pub const NUM_PARAMS: usize = 26;

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
        // 1 – Beat Cycle
        SimpleParamInfo {
            name: CString::new("Beat Cycle").unwrap(),
            param_type: ParameterTypes::Boolean,
            default: Some(0.0),
            ..Default::default()
        },
        // 2 – Font
        SimpleParamInfo {
            name: CString::new("Font").unwrap(),
            param_type: ParameterTypes::Option,
            default: Some(0.0),
            elements: Some(font_elements),
            ..Default::default()
        },
        // 3 – Font Size
        SimpleParamInfo {
            name: CString::new("Font Size").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.25),
            ..Default::default()
        },
        // 4 – Fill R
        SimpleParamInfo {
            name: CString::new("Fill R").unwrap(),
            param_type: ParameterTypes::Red,
            default: Some(1.0),
            group: Some("Fill Color".to_string()),
            ..Default::default()
        },
        // 5 – Fill G
        SimpleParamInfo {
            name: CString::new("Fill G").unwrap(),
            param_type: ParameterTypes::Green,
            default: Some(1.0),
            group: Some("Fill Color".to_string()),
            ..Default::default()
        },
        // 6 – Fill B
        SimpleParamInfo {
            name: CString::new("Fill B").unwrap(),
            param_type: ParameterTypes::Blue,
            default: Some(1.0),
            group: Some("Fill Color".to_string()),
            ..Default::default()
        },
        // 7 – Fill A
        SimpleParamInfo {
            name: CString::new("Fill A").unwrap(),
            param_type: ParameterTypes::Alpha,
            default: Some(1.0),
            group: Some("Fill Color".to_string()),
            ..Default::default()
        },
        // 8 – Outline Enabled
        SimpleParamInfo {
            name: CString::new("Outline Enabled").unwrap(),
            param_type: ParameterTypes::Boolean,
            default: Some(0.0),
            ..Default::default()
        },
        // 9 – Outline R
        SimpleParamInfo {
            name: CString::new("Outline R").unwrap(),
            param_type: ParameterTypes::Red,
            default: Some(0.0),
            group: Some("Outline Color".to_string()),
            ..Default::default()
        },
        // 10 – Outline G
        SimpleParamInfo {
            name: CString::new("Outline G").unwrap(),
            param_type: ParameterTypes::Green,
            default: Some(0.0),
            group: Some("Outline Color".to_string()),
            ..Default::default()
        },
        // 11 – Outline B
        SimpleParamInfo {
            name: CString::new("Outline B").unwrap(),
            param_type: ParameterTypes::Blue,
            default: Some(0.0),
            group: Some("Outline Color".to_string()),
            ..Default::default()
        },
        // 12 – Outline A
        SimpleParamInfo {
            name: CString::new("Outline A").unwrap(),
            param_type: ParameterTypes::Alpha,
            default: Some(1.0),
            group: Some("Outline Color".to_string()),
            ..Default::default()
        },
        // 13 – Outline Width
        SimpleParamInfo {
            name: CString::new("Outline Width").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.1),
            ..Default::default()
        },
        // 14 – Shadow Enabled
        SimpleParamInfo {
            name: CString::new("Shadow Enabled").unwrap(),
            param_type: ParameterTypes::Boolean,
            default: Some(0.0),
            ..Default::default()
        },
        // 15 – Shadow R
        SimpleParamInfo {
            name: CString::new("Shadow R").unwrap(),
            param_type: ParameterTypes::Red,
            default: Some(0.0),
            group: Some("Shadow Color".to_string()),
            ..Default::default()
        },
        // 16 – Shadow G
        SimpleParamInfo {
            name: CString::new("Shadow G").unwrap(),
            param_type: ParameterTypes::Green,
            default: Some(0.0),
            group: Some("Shadow Color".to_string()),
            ..Default::default()
        },
        // 17 – Shadow B
        SimpleParamInfo {
            name: CString::new("Shadow B").unwrap(),
            param_type: ParameterTypes::Blue,
            default: Some(0.0),
            group: Some("Shadow Color".to_string()),
            ..Default::default()
        },
        // 18 – Shadow A
        SimpleParamInfo {
            name: CString::new("Shadow A").unwrap(),
            param_type: ParameterTypes::Alpha,
            default: Some(0.5),
            group: Some("Shadow Color".to_string()),
            ..Default::default()
        },
        // 19 – Shadow Offset
        SimpleParamInfo {
            name: CString::new("Shadow Offset").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.1),
            ..Default::default()
        },
        // 20 – H Align
        SimpleParamInfo {
            name: CString::new("H Align").unwrap(),
            param_type: ParameterTypes::Option,
            default: Some(1.0),
            elements: Some(vec![
                (CString::new("Left").unwrap(), 0.0),
                (CString::new("Center").unwrap(), 1.0),
                (CString::new("Right").unwrap(), 2.0),
            ]),
            ..Default::default()
        },
        // 21 – V Align
        SimpleParamInfo {
            name: CString::new("V Align").unwrap(),
            param_type: ParameterTypes::Option,
            default: Some(1.0),
            elements: Some(vec![
                (CString::new("Top").unwrap(), 0.0),
                (CString::new("Center").unwrap(), 1.0),
                (CString::new("Bottom").unwrap(), 2.0),
            ]),
            ..Default::default()
        },
        // 22 – Line Spacing
        SimpleParamInfo {
            name: CString::new("Line Spacing").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.5),
            ..Default::default()
        },
        // 23 – Letter Spacing
        SimpleParamInfo {
            name: CString::new("Letter Spacing").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.5),
            ..Default::default()
        },
        // 24 – Position X
        SimpleParamInfo {
            name: CString::new("Position X").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.5),
            ..Default::default()
        },
        // 25 – Position Y
        SimpleParamInfo {
            name: CString::new("Position Y").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.5),
            ..Default::default()
        },
    ]
});
