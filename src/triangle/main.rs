/// Copy of `triangle` example on gfx.rs

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

// This trait is responsible for creating and managing graphics resources.
use gfx::traits::FactoryExt;
// This trait is responsible for subitting `CommandBuffer`s to the GPU.
// gfx::CommandBuffer
// An interface of the abstract command buffer.
// Command buffer is a buffer that keeps draw calls. It's on GPU memory.
use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
// Format of depth buffer.
// Depth buffer is a memory area that contains depth informations.
pub type DepthFormat = gfx::format::DepthStencil;

// Defines vertex, constant and pipeline formats in one block.
gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const TRIANGLE: [Vertex; 3] = [
    // Device cordinate system: 2x2x2, [-1, 1]
    Vertex { pos: [-0.5, -0.5], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [ 0.5, -0.5], color: [0.0, 1.0, 0.0] },
    Vertex { pos: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

fn main() {
    // Retreive events from the window.
    let events_loop = glutin::EventsLoop::new();

    // Build and initialize a window.
    let builder = glutin::WindowBuilder::new()
        .with_title("Triangle example".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();
    // From glutin_window_glutin
    // window: glutin::Window -> OpenGL context & the Window
    // device: gfx_device_gl::Device -> OpenGL device with GLSL shaders
    // factory: gfx_device_gl::Factory -> GL resource factory
    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);
    // Wrapper of CommandBuffer.
    // Since it is responsible for sending commands to the CommandBuffer,
    // we will probably use this.
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    // Create PSO.
    let pso = factory.create_pipeline_simple(
        // Shaders
        include_bytes!("shader/triangle_150.glslv"),
        include_bytes!("shader/triangle_150.glslf"),
        // Defined in `gfx_defines!()`.
        pipe::new()
    ).unwrap();;
}
