#[macro_use]
extern crate glium;
extern crate tinyfiledialogs as tfd;

use crate::tool::{build_axis, build_vb, deg2rad, load_data, Vertexd, rebuild_vb};
use glam;
use glium::index::PrimitiveType;
use glium::{Program, VertexBuffer};
#[allow(unused_imports)]
use glium::{glutin, Surface};
use std::f32::consts::PI;
use std::ops::Mul;
use std::{thread, process};
use text_io::*;
use tfd::MessageBoxIcon;

mod filetool;
mod gui;
mod shaders;
mod tool;

fn update(
    tmp: [i64; 9],
    posx: &mut f32,
    posy: &mut f32,
    posz: &mut f32,
    rotx: &mut f32,
    roty: &mut f32,
    rotz: &mut f32,
    buf: &mut f32,
    ind: &mut f32,
    fov: &mut f32,
) {
    *rotx = tmp[0] as f32;
    *roty = tmp[1] as f32;
    *rotz = tmp[2] as f32;

    *posx = tmp[3] as f32;
    *posy = tmp[4] as f32;
    *posz = tmp[5] as f32;

    *buf = tmp[6] as f32;
    *ind = tmp[7] as f32;

    *fov = tmp[8] as f32;
}

fn endprgm_fns() -> String{
    tfd::message_box_ok(
        "Warning",
        "No file selected.",
        MessageBoxIcon::Warning,
    );
    process::exit(0x001);
    "".to_string()
}

fn main() {

    let fd: String = match tfd::open_file_dialog("Please choose a file...", "", None) {
                            Some(path) => path,
                            None => endprgm_fns(),
    };

    //init ringbuffer
    let rb: ringbuf::RingBuffer<[i64; 9]> = ringbuf::RingBuffer::new(32);
    let (mut prod, mut cons) = rb.split();

    //init values
    let mut rotx: f32 = 0.0;
    let mut roty: f32 = 0.0;
    let mut rotz: f32 = 0.0;

    let mut posx: f32 = 0.0;
    let mut posy: f32 = 0.0;
    let mut posz: f32 = 0.0;

    let mut buf: f32 = 0.0;
    let mut ind: f32 = 0.0;

    let mut fov: f32 = 0.0;

    let mut window_size = glutin::dpi::Size::new(glutin::dpi::PhysicalSize::new(800, 600));
    let event_loop = glutin::event_loop::EventLoop::new();
    let mut wb = glutin::window::WindowBuilder::new()
        .with_title("datarender")
        .with_inner_size(window_size);
    let cb = glutin::ContextBuilder::new();
    let mut display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let WIDTH: f32 = window_size.to_logical(1f64).width;
    let HEIGHT: f32 = window_size.to_logical(1f64).height;
    println!("size: {},{}", WIDTH, HEIGHT);

    //wb.window.title.as_mut_str().


    //LOADING DATA
    let mut data: Vec<Vertexd> = [].to_vec();
    load_data(
        &mut data,
        fd,
    );

    thread::spawn(|| {
        gui::startgui(prod);
    });

    //BUILDING BUFFERS FOR AXIS
    let vbd_axis = build_axis(display);
    let vertex_buffer_axis = vbd_axis.1;
    display = vbd_axis.0;
    let index_buffer_axis = glium::index::NoIndices(PrimitiveType::LinesList);

    //BUILDING BUFFERS FOR DATA
    let blank : &[Vertexd; 6] = &[
        Vertexd {
            position: [0.0, 0.0, 0.0],
            color: [1.0, 0.0, 0.0],
        },
        Vertexd {
            position: [100.0, 0.0, 0.0],
            color: [1.0, 0.0, 0.0],
        },
        Vertexd {
            position: [0.0, 0.0, 0.0],
            color: [0.0, 1.0, 0.0],
        },
        Vertexd {
            position: [0.0, 100.0, 0.0],
            color: [0.0, 1.0, 0.0],
        },
        Vertexd {
            position: [0.0, 0.0, 0.0],
            color: [0.0, 0.0, 1.0],
        },
        Vertexd {
            position: [0.0, 0.0, 100.0],
            color: [0.0, 0.0, 1.0],
        },
    ];

    let mut vb = {
        glium::VertexBuffer::dynamic(
            &display,
            blank,
        )
            .unwrap()
    };
    let index_buffer = glium::index::NoIndices(PrimitiveType::LineStrip);

    // compiling shaders and linking them together
    let displayandshader = shaders::makeshader(display);
    display = displayandshader.1;
    let program = displayandshader.0;

    //INITIALIZING MATRICIES
    //let model = glam::Mat4::from_translation(glam::Vec3::new(0f32,0f32,0f32));
    let mut model = glam::Mat4::identity();
    //lookat
    let mut view = glam::Mat4::look_at_rh(
        glam::Vec3::new(0f32, 0f32, -2f32),
        glam::Vec3::new(0f32, 0f32, 1f32),
        glam::Vec3::new(0f32, 1f32, 0f32),
    );
    //persp
    let mut projection = glam::Mat4::perspective_rh_gl(1.5708, WIDTH / HEIGHT, 1.0, 100.0);

    let mut PVM = projection * view * model;
    //println!("{:?}", MVP);

    //==================================================================================

    //==================================================================================
    // the main loop
    event_loop.run(move |event, _, control_flow| {
        let tmp = match cons.pop() {
            Some(a) => update(a, &mut posx, &mut posy, &mut posz, &mut rotx, &mut roty, &mut rotz, &mut buf, &mut ind, &mut fov),
            None => (),
        };

        //println!("{},{},{}",posx,posy,posz);
        model = glam::Mat4::from_rotation_x(deg2rad(rotx / 10.0))
            * glam::Mat4::from_rotation_y(deg2rad(roty / 10.0))
            * glam::Mat4::from_rotation_z(deg2rad(rotz / 10.0));
        view = glam::Mat4::look_at_rh(
            glam::Vec3::new(posx / 10000.0, posy / 10000.0, posz / 10000.0),
            glam::Vec3::new(0f32, 0f32, 1f32),
            glam::Vec3::new(0f32, 1f32, 0f32),
        );
        projection = glam::Mat4::perspective_rh_gl(deg2rad(fov / 2.0), WIDTH / HEIGHT, 0.1, 10000000000000.0);
        PVM = (model * view * projection).inverse();

        // building the uniforms
        let uniforms = uniform! {
        matrix: PVM.to_cols_array_2d()
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        for i in (0..=(buf as i64)*100).step_by(6) {

            let tmp = rebuild_vb(i as usize +ind as usize, &data, &mut vb);

            target.draw(
                    &vb,
                    &index_buffer,
                    &program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
        }


        //draw axis ?and other stuff?
        target
            .draw(
                &vertex_buffer_axis,
                &index_buffer_axis,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        //draw data in chunks
        target.finish().unwrap();

        //===============================================================
        *control_flow = match event {
            //==============================================================
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => {
                    //draw();
                    glutin::event_loop::ControlFlow::Poll
                }
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            //================================================================
            _ => glutin::event_loop::ControlFlow::Poll,
        };
        //===============================================================
    });

    //end of main
}
