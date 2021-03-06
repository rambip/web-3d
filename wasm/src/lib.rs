use wasm_bindgen::prelude::*;

use  web_sys::console;

macro_rules! log {
    ($($msg: tt)*) => {
        unsafe {
            console::log_1(&format!($($msg)*).into())
        }
    }
}
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlUniformLocation;

mod webgl;
use webgl::Engine;

mod camera;
use camera::Camera;

mod geometry;

#[wasm_bindgen]
pub struct Universe {
    engine: Engine,
    camera: Camera,
    n_update: u32,
    last_update: u32,
}
 
#[wasm_bindgen]
impl Universe {
    #[wasm_bindgen(constructor)]
    pub fn new(gl: GL, trans_location: WebGlUniformLocation, time_location: WebGlUniformLocation, t: u32) -> Self {
        let camera = Camera {x:2.0, y:-2.0, z:0.0, angle:1.80};
        let engine = Engine {gl, trans_location, time_location, n_indices: 0i32};
        Self {engine, camera, n_update: 0, last_update: t}
    }

    pub fn update(&mut self, t: u32, left: bool, right: bool, down: bool, up: bool, space: bool, shift: bool) {

        let dt = (t - self.last_update) as f32 / 1000.0;



        if left  {self.camera.rotate( 2.0 * dt);};
        if right {self.camera.rotate( 2.0 * -dt);};
        if down  {self.camera.forward(2.0 * -dt);};
        if up    {self.camera.forward(2.0 * dt);};
        if space {self.camera.up(2.0 * dt);};
        if shift {self.camera.up(-2.0*dt);};

        if self.n_update % 3000 == 0 {
            // update landscape
            let mut points   = Vec::with_capacity(10000000);
            let mut indices = Vec::with_capacity(10000000);
            //geometry::test_sphere(&mut points, &mut indices);
            geometry::test_octree_shape(&mut points, &mut indices);
            // geometry::rand_surface(&mut points, &mut indices);
            geometry::shade(&mut points, &mut indices);

            self.engine.update_triangles(points, indices);
        }

        self.n_update += 1;
        self.last_update = t;

        // debug log
        if self.n_update % 10 == 0 {
            log!("{}", self.camera.get_info());
        };

    }

    pub fn render(&mut self, t: u32){
        let time = (t as f32) * 0.001;

        self.engine.render(self.camera.get_transform(
                                self.engine.width(), 
                                self.engine.height()),
                            time
            );
    }
}

