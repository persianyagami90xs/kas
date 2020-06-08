// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Drawing API for `kas_wgpu`
//!
//! Extensions to the API of [`kas::draw`], plus some utility types.

mod custom;
mod draw_pipe;
mod draw_text;
mod flat_round;
mod shaded_round;
mod shaded_square;
mod shaders;

use kas::geom::Rect;
use wgpu::{CompareFunction, DepthStencilStateDescriptor, TextureFormat};
use wgpu_glyph::ab_glyph::FontArc;
use wgpu_glyph::GlyphBrush;

pub(crate) use shaders::ShaderManager;

pub use custom::{CustomPipe, CustomPipeBuilder, CustomWindow, DrawCustom};

const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;
pub(crate) const TEX_FORMAT: TextureFormat = TextureFormat::Bgra8UnormSrgb;

const fn new_depth_desc(depth_compare: CompareFunction) -> DepthStencilStateDescriptor {
    DepthStencilStateDescriptor {
        format: DEPTH_FORMAT,
        depth_write_enabled: true,
        depth_compare,
        stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
        stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
        stencil_read_mask: 0,
        stencil_write_mask: 0,
    }
}
const DEPTH_DESC: DepthStencilStateDescriptor = new_depth_desc(CompareFunction::Always);
const GLPYH_DEPTH_DESC: DepthStencilStateDescriptor = new_depth_desc(CompareFunction::GreaterEqual);

/// 3-part colour data
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<kas::draw::Colour> for Rgb {
    fn from(c: kas::draw::Colour) -> Self {
        Rgb {
            r: c.r,
            g: c.g,
            b: c.b,
        }
    }
}

/// Shared pipeline data
pub struct DrawPipe<C> {
    fonts: Vec<FontArc>,
    shaded_square: shaded_square::Pipeline,
    shaded_round: shaded_round::Pipeline,
    flat_round: flat_round::Pipeline,
    custom: C,
}

/// Per-window pipeline data
pub struct DrawWindow<CW: CustomWindow> {
    depth: Option<wgpu::TextureView>,
    clip_regions: Vec<Rect>,
    shaded_square: shaded_square::Window,
    shaded_round: shaded_round::Window,
    flat_round: flat_round::Window,
    custom: CW,
    glyph_brush: GlyphBrush<DepthStencilStateDescriptor>, // TODO: should be in DrawPipe
}
