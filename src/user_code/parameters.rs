use std::sync::Mutex;
use vst::util::AtomicFloat;
use dsp_lab::utils::math::pre_post_gains;
use dsp_lab::utils::conversion::gain_to_db;

use crate::parameter_manager::{Parameter, ParameterManager};

// VST_LAB calls this to determine which parameters are displayed in the plugin
// window (no GUI) and which parameters the plugin editor can read and write to
// (with GUI)
pub fn create_parameters() -> ParameterManager {
    
    let parameters = vec![
        Parameter {
            display_name: Mutex::new("drive".to_string()),
            raw: AtomicFloat::new(0.5),
            value_format: Box::new(|x| 
                format!("{:.2} dB", gain_to_db(pre_post_gains(x as f64 * 10.0 - 5.0).0))
            ),
            ..Default::default()
        },
        Parameter {
            display_name: Mutex::new("bias".to_string()),
            raw: AtomicFloat::new(0.5),
            ..Default::default()
        },
        Parameter {
            display_name: Mutex::new("mode".to_string()),
            value_format: Box::new(|x| match x {
                _ if x < (1.0/6.0) => "leaky ReLU".to_string(),
                _ if x < (2.0/6.0) => "max tanh".to_string(),
                _ if x < (3.0/6.0) => "soft plus".to_string(),
                _ if x < (4.0/6.0) => "swish".to_string(),
                _ if x < (5.0/6.0) => "mish".to_string(),
                _ if x <= 1.0 => "soft tanh".to_string(),
                _ => "ERR!".to_string(),
            }),
            ..Default::default()
        },
        Parameter {
            display_name: Mutex::new("rectify".to_string()),
            ..Default::default()
        },
        Parameter {
            display_name: Mutex::new("dry/wet".to_string()),
            raw: AtomicFloat::new(1.0),
            ..Default::default()
        },
    ];
    ParameterManager::from_vec(parameters)
}