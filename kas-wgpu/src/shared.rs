// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Shared state

use log::info;
use std::fmt;
use std::num::NonZeroU32;

use crate::draw::{CustomPipe, CustomPipeBuilder, DrawPipe, DrawWindow, ShaderManager};
use crate::{Error, Options, WindowId};
use kas::event::UpdateHandle;
use kas_theme::Theme;

struct NoAdapterError;

impl fmt::Debug for NoAdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NoAdapterError {{ }}")
    }
}

impl fmt::Display for NoAdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no suitable graphics adapter found")
    }
}

impl std::error::Error for NoAdapterError {}

/// State shared between windows
pub struct SharedState<C: CustomPipe, T> {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub shaders: ShaderManager,
    pub draw: DrawPipe<C>,
    pub theme: T,
    pub pending: Vec<PendingAction>,
    /// Newly created windows need to know the scale_factor *before* they are
    /// created. This is used to estimate ideal window size.
    pub scale_factor: f64,
    window_id: u32,
}

impl<C: CustomPipe, T: Theme<DrawPipe<C>>> SharedState<C, T>
where
    T::Window: kas_theme::Window<DrawWindow<C::Window>>,
{
    /// Construct
    pub fn new<CB: CustomPipeBuilder<Pipe = C>>(
        custom: CB,
        mut theme: T,
        options: Options,
        scale_factor: f64,
    ) -> Result<Self, Error> {
        let adapter_options = options.adapter_options();

        let adapter = match wgpu::Adapter::request(&adapter_options) {
            Some(a) => a,
            None => return Err(Box::new(NoAdapterError)),
        };
        info!("Using graphics adapter: {}", adapter.get_info().name);

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        });

        let shaders = ShaderManager::new(&device)?;
        let mut draw = DrawPipe::new(custom, &device, &shaders);

        theme.init(&mut draw);

        Ok(SharedState {
            device,
            queue,
            shaders,
            draw,
            theme,
            pending: vec![],
            scale_factor,
            window_id: 0,
        })
    }

    pub fn next_window_id(&mut self) -> WindowId {
        self.window_id += 1;
        WindowId::new(NonZeroU32::new(self.window_id).unwrap())
    }

    pub fn render(
        &mut self,
        window: &mut DrawWindow<C::Window>,
        frame_view: &wgpu::TextureView,
        clear_color: wgpu::Color,
    ) {
        let buf = self
            .draw
            .render(window, &mut self.device, frame_view, clear_color);
        self.queue.submit(&[buf]);
    }
}

pub enum PendingAction {
    AddPopup(winit::window::WindowId, WindowId, kas::Popup),
    AddWindow(WindowId, Box<dyn kas::Window>),
    CloseWindow(WindowId),
    ThemeResize,
    RedrawAll,
    Update(UpdateHandle, u64),
}
