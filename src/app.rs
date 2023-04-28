use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, HtmlCanvasElement};
use crate::input::InputManager;
use crate::log;
use crate::render_passes::{CloudRenderPass, DemoRenderPass, FractalRenderPass, RasterRenderPass};
use crate::vec_lib::vec3::Vec3f;


pub struct TestApp{
    ctx: WebGl2RenderingContext,
    canvas: HtmlCanvasElement,
    window: web_sys::Window,
    raster_pass: RasterRenderPass,
    demo_pass: DemoRenderPass,
    fractal_pass: FractalRenderPass,
    cloud_pass: CloudRenderPass,
    input_manager: Rc<InputManager>,
}

impl TestApp {
    pub fn new(ctx: WebGl2RenderingContext, canvas: HtmlCanvasElement, window: web_sys::Window)
        -> Result<Self, String>{

        ctx.get_extension("EXT_color_buffer_float")
            .or(Err("EXT_color_buffer_float extension not supported"))?;
        ctx.get_extension("OES_texture_float_linear")
            .or(Err("OES_texture_float_linear extension not supported"))?;

        let input_manager_res = InputManager::new(&canvas, &window)?;
        let input_manager = Rc::new(input_manager_res);

        let raster_pass = RasterRenderPass::new(ctx.clone(), input_manager.clone())?;
        let demo_pass = DemoRenderPass::new(ctx.clone(), input_manager.clone())?;
        let fractal_pass: FractalRenderPass = FractalRenderPass::new(ctx.clone(), input_manager.clone(),
            raster_pass.color_texture(), raster_pass.depth_buffer()
        )?;
        let cloud_pass = CloudRenderPass::new(ctx.clone(), input_manager.clone(),
        raster_pass.color_texture())?;

        Ok(TestApp{
            ctx,
            canvas,
            window,
            raster_pass,
            demo_pass,
            fractal_pass,
            cloud_pass,
            input_manager,
        })
    }

    // fn mat_to_str(mat : &Mat4f) -> String{
    //     let out = String::new();
    //     let vals = mat.vals();
    //     format!("{:#?}", vals)
    // }

    pub fn draw(&self){
        self.ctx.clear_color(0.0, 0.37254903, 0.37254903, 1.0);
        self.ctx.enable(WebGl2RenderingContext::DEPTH_TEST);


        match self.input_manager.mode() {
            1 =>{
                self.demo_pass.draw();
            },
            _ =>{
                self.raster_pass.draw();
                self.fractal_pass.draw();
                self.ctx.disable(WebGl2RenderingContext::DEPTH_TEST);
                self.cloud_pass.draw();
            }
        };
    }
}

