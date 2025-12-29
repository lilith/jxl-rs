// Copyright (c) the JPEG XL Project Authors. All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::api::JxlCms;

pub enum JxlProgressiveMode {
    /// Renders all pixels in every call to Process.
    Eager,
    /// Renders pixels once passes are completed.
    Pass,
    /// Renders pixels only once the final frame is ready.
    FullFrame,
}

#[non_exhaustive]
pub struct JxlDecoderOptions {
    pub adjust_orientation: bool,
    pub render_spot_colors: bool,
    pub coalescing: bool,
    pub desired_intensity_target: Option<f32>,
    pub skip_preview: bool,
    pub progressive_mode: JxlProgressiveMode,
    pub xyb_output_linear: bool,
    pub enable_output: bool,
    pub cms: Option<Box<dyn JxlCms>>,
    /// Fail decoding images with more than this number of pixels, or with frames with
    /// more than this number of pixels. The limit counts the product of pixels and
    /// channels, so for example an image with 1 extra channel of size 1024x1024 has 4
    /// million pixels.
    pub pixel_limit: Option<usize>,
    /// Use high precision mode for decoding.
    /// When false (default), uses lower precision settings that match libjxl's default.
    /// When true, uses higher precision at the cost of performance.
    ///
    /// This affects multiple decoder decisions including spline rendering precision
    /// and potentially intermediate buffer storage (e.g., using f32 vs f16).
    pub high_precision: bool,
    /// If true, multiply RGB by alpha before writing to output buffer.
    /// This produces premultiplied alpha output, which is useful for compositing.
    /// Default: false (output straight alpha)
    pub premultiply_output: bool,
}

impl Default for JxlDecoderOptions {
    fn default() -> Self {
        Self {
            adjust_orientation: true,
            render_spot_colors: true,
            coalescing: true,
            skip_preview: true,
            desired_intensity_target: None,
            progressive_mode: JxlProgressiveMode::Pass,
            xyb_output_linear: false,
            enable_output: true,
            cms: None,
            pixel_limit: None,
            high_precision: false,
            premultiply_output: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that xyb_output_linear defaults to false to apply sRGB transfer function.
    ///
    /// When decoding XYB images, the sRGB transfer function should be applied by default
    /// to produce correct sRGB output values. Setting xyb_output_linear to true would
    /// skip this conversion, resulting in incorrect (linear) output.
    ///
    /// This was a bug where the default was accidentally set to true.
    #[test]
    fn test_default_xyb_output_linear_is_false() {
        let options = JxlDecoderOptions::default();
        assert!(
            !options.xyb_output_linear,
            "xyb_output_linear should default to false to apply sRGB transfer function"
        );
    }

    #[test]
    fn test_default_options() {
        let options = JxlDecoderOptions::default();
        // Verify critical defaults
        assert!(options.adjust_orientation);
        assert!(options.render_spot_colors);
        assert!(options.coalescing);
        assert!(options.skip_preview);
        assert!(options.enable_output);
        assert!(!options.xyb_output_linear); // Must be false for correct sRGB output
        assert!(!options.high_precision);
        assert!(!options.premultiply_output);
    }
}
