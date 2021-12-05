use glam::Vec4;
use imgui_wgpu::RendererConfig;
use rend3::{
    types::{Color, TextureFormat},
    RenderGraph, RenderPassTarget, RenderPassTargets, RenderTargetHandle, Renderer,
};

pub struct ImguiRenderRoutine {
    pub renderer: imgui_wgpu::Renderer,
}

impl ImguiRenderRoutine {
    pub fn new(renderer: &Renderer, imgui: &mut imgui::Context, output_format: TextureFormat) -> Self {
        let base = if output_format.describe().srgb {
            RendererConfig::new_srgb()
        } else {
            RendererConfig::new()
        };

        let renderer = imgui_wgpu::Renderer::new(
            imgui,
            &renderer.device,
            &renderer.queue,
            RendererConfig {
                texture_format: output_format,
                ..base
            },
        );

        Self { renderer }
    }

    pub fn add_to_graph<'node>(
        &'node mut self,
        graph: &mut RenderGraph<'node>,
        draw_data: imgui::DrawData,
        output: RenderTargetHandle,
    ) {
        let mut builder = graph.add_node("imgui");

        let output_handle = builder.add_render_target_output(output);

        let rpass_handle = builder.add_renderpass(RenderPassTargets {
            targets: vec![RenderPassTarget {
                color: output_handle,
                clear: Color::BLACK,
                resolve: None,
            }],
            depth_stencil: None,
        });

        let pt_handle = builder.passthrough_data(self);

        builder.build(move |pt, renderer, encoder_or_pass, temps, _ready, graph_data| {
            let this = pt.get(pt_handle);
            let rpass = encoder_or_pass.get_rpass(rpass_handle);

            this.renderer
                .render(&draw_data, &renderer.queue, &renderer.device, rpass);
        })
    }
}
