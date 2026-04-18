use std::ffi::CString;
use std::sync::LazyLock;

use ffgl_core::parameters::{ParameterTypes, SimpleParamInfo};

pub const PARAM_COLUMNS: usize = 0;
pub const PARAM_ROWS: usize = 1;
pub const PARAM_DOT_SIZE: usize = 2;
pub const PARAM_ALIGN_X: usize = 3;
pub const PARAM_ALIGN_Y: usize = 4;
pub const PARAM_FILL_R: usize = 5;
pub const PARAM_FILL_G: usize = 6;
pub const PARAM_FILL_B: usize = 7;
pub const PARAM_FILL_A: usize = 8;
pub const PARAM_BG_R: usize = 9;
pub const PARAM_BG_G: usize = 10;
pub const PARAM_BG_B: usize = 11;
pub const PARAM_BG_A: usize = 12;
pub const NUM_PARAMS: usize = 13;

pub const ALIGN_LEFT: u32 = 0;
pub const ALIGN_CENTER: u32 = 1;
pub const ALIGN_RIGHT: u32 = 2;

pub const VALIGN_TOP: u32 = 0;
pub const VALIGN_CENTER: u32 = 1;
pub const VALIGN_BOTTOM: u32 = 2;

pub static PARAM_INFOS: LazyLock<Vec<SimpleParamInfo>> = LazyLock::new(|| {
    vec![
        // 0 – Columns
        SimpleParamInfo {
            name: CString::new("Columns").unwrap(),
            param_type: ParameterTypes::Integer,
            default: Some(8.0),
            min: Some(1.0),
            max: Some(128.0),
            ..Default::default()
        },
        // 1 – Rows
        SimpleParamInfo {
            name: CString::new("Rows").unwrap(),
            param_type: ParameterTypes::Integer,
            default: Some(8.0),
            min: Some(1.0),
            max: Some(128.0),
            ..Default::default()
        },
        // 2 – Dot Size (fraction of cell, 0..1)
        SimpleParamInfo {
            name: CString::new("Dot Size").unwrap(),
            param_type: ParameterTypes::Standard,
            default: Some(0.5),
            ..Default::default()
        },
        // 3 – Align X
        SimpleParamInfo {
            name: CString::new("Align X").unwrap(),
            param_type: ParameterTypes::Option,
            default: Some(ALIGN_CENTER as f32),
            elements: Some(vec![
                (CString::new("Left").unwrap(), ALIGN_LEFT as f32),
                (CString::new("Center").unwrap(), ALIGN_CENTER as f32),
                (CString::new("Right").unwrap(), ALIGN_RIGHT as f32),
            ]),
            ..Default::default()
        },
        // 4 – Align Y
        SimpleParamInfo {
            name: CString::new("Align Y").unwrap(),
            param_type: ParameterTypes::Option,
            default: Some(VALIGN_CENTER as f32),
            elements: Some(vec![
                (CString::new("Top").unwrap(), VALIGN_TOP as f32),
                (CString::new("Center").unwrap(), VALIGN_CENTER as f32),
                (CString::new("Bottom").unwrap(), VALIGN_BOTTOM as f32),
            ]),
            ..Default::default()
        },
        // 5 – Fill R
        SimpleParamInfo {
            name: CString::new("Fill R").unwrap(),
            param_type: ParameterTypes::Red,
            default: Some(1.0),
            group: Some("Fill".to_string()),
            ..Default::default()
        },
        // 6 – Fill G
        SimpleParamInfo {
            name: CString::new("Fill G").unwrap(),
            param_type: ParameterTypes::Green,
            default: Some(1.0),
            group: Some("Fill".to_string()),
            ..Default::default()
        },
        // 7 – Fill B
        SimpleParamInfo {
            name: CString::new("Fill B").unwrap(),
            param_type: ParameterTypes::Blue,
            default: Some(1.0),
            group: Some("Fill".to_string()),
            ..Default::default()
        },
        // 8 – Fill A
        SimpleParamInfo {
            name: CString::new("Fill A").unwrap(),
            param_type: ParameterTypes::Alpha,
            default: Some(1.0),
            group: Some("Fill".to_string()),
            ..Default::default()
        },
        // 9 – Background R
        SimpleParamInfo {
            name: CString::new("Bg R").unwrap(),
            param_type: ParameterTypes::Red,
            default: Some(0.0),
            group: Some("Background".to_string()),
            ..Default::default()
        },
        // 10 – Background G
        SimpleParamInfo {
            name: CString::new("Bg G").unwrap(),
            param_type: ParameterTypes::Green,
            default: Some(0.0),
            group: Some("Background".to_string()),
            ..Default::default()
        },
        // 11 – Background B
        SimpleParamInfo {
            name: CString::new("Bg B").unwrap(),
            param_type: ParameterTypes::Blue,
            default: Some(0.0),
            group: Some("Background".to_string()),
            ..Default::default()
        },
        // 12 – Background A
        SimpleParamInfo {
            name: CString::new("Bg A").unwrap(),
            param_type: ParameterTypes::Alpha,
            default: Some(0.0),
            group: Some("Background".to_string()),
            ..Default::default()
        },
    ]
});
