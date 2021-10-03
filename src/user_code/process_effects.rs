use crate::parameter_manager::ParameterManager;
use crate::user_code::algorithms::{lrelu, maxTanh, softPlus, swish, mish, softTanh};
use dsp_lab::utils::math::{pre_post_gains, x_fade};
use dsp_lab::core::lin_filter::{BiquadHighPass};
use dsp_lab::traits::Process;
use std::sync::Arc;
use std::collections::VecDeque;

pub struct EffectProcessor {
    hpl1: BiquadHighPass,
    hpr1: BiquadHighPass,
    hpl2: BiquadHighPass,
    hpr2: BiquadHighPass,
}

const RESOLUTION: usize = 512;

impl EffectProcessor {
    pub fn new() -> Self { 
        let mut ret = Self {
            hpl1: BiquadHighPass::new(),
            hpr1: BiquadHighPass::new(),
            hpl2: BiquadHighPass::new(),
            hpr2: BiquadHighPass::new(),
        };

        ret.hpl1.cutoff = 10.0;
        ret.hpl2.cutoff = 10.0;
        ret.hpr1.cutoff = 10.0;
        ret.hpr2.cutoff = 10.0;
        ret
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.hpl1.set_sr(sr);
        self.hpl2.set_sr(sr);
        self.hpr1.set_sr(sr);
        self.hpr2.set_sr(sr);
    }

    pub fn process_effects(&mut self, param_mngr: Arc<ParameterManager>, l: f64, r: f64) -> (f64, f64) {
        let drive = param_mngr.params[0].filtered.get() as f64;
        let bias = param_mngr.params[1].filtered.get() as f64 - 0.5;
        let mode = param_mngr.params[2].filtered.get() as f64;
        let rect = param_mngr.params[3].filtered.get() as f64;
        let wet = param_mngr.params[4].filtered.get() as f64;
        let (pre, post) = pre_post_gains(drive*10.0 - 5.0);

        let l_dry = l;
        let r_dry = r;
        let l_in = self.hpl1.step(l) * pre + bias*bias*bias * 4.0;
        let r_in = self.hpr1.step(r) * pre + bias*bias*bias * 4.0;

        let (mut l_out, mut r_out) = match mode {
            _ if mode < (1.0/6.0) => (lrelu(l_in, rect), lrelu(r_in, rect)),
            _ if mode < (2.0/6.0) => (maxTanh(l_in, rect), maxTanh(r_in, rect)),
            _ if mode < (3.0/6.0) => (softPlus(l_in, rect), softPlus(r_in, rect)),
            _ if mode < (4.0/6.0) => (swish(l_in, rect), swish(r_in, rect)),
            _ if mode < (5.0/6.0) => (mish(l_in, rect), mish(r_in, rect)),
            _ if mode <= 1.0 => (softTanh(l_in, rect), softTanh(r_in, rect)),
            _ => (l_in, r_in)
        };

        l_out = self.hpl2.step((l_out * 0.5).tanh() * 2.0) * post;
        r_out = self.hpr2.step((r_out * 0.5).tanh() * 2.0) * post;

        (x_fade(l_dry, wet, l_out), x_fade(r_dry, wet, r_out))
    }
}
