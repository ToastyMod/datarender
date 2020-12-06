//use ultraviolet::*;
//pub fn ultraviolet_get_4x4(inmat: Mat4) -> [[f32; 4];4] {
//    let mut outer :  [[f32; 4];4]  = [[0f32,0f32,0f32,0f32],[0f32,0f32,0f32,0f32],[0f32,0f32,0f32,0f32],[0f32,0f32,0f32,0f32]];
//    let mut y = 0;
//    for v in inmat.cols.iter() {
//        outer[y][0] = v[0];
//        outer[y][1] = v[1];
//        outer[y][2] = v[2]+100f32;
//        outer[y][3] = v[3]+0f32;
//        println!("{:?}", outer[y]);
//        y +=1;
//    }
//    outer
//}

use glium::VertexBuffer;
use std::borrow::Cow;
use std::f32::consts::PI;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Error;
use std::sync::{Mutex, MutexGuard};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

const TWO_GB: u32 = 2147483648;

#[derive(Debug, Copy, Clone)]
pub struct Vertexd {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

implement_vertex!(Vertexd, position, color);

pub fn deg2rad(inp: f32) -> f32 {
    ((inp * PI) / 180f32)
}

//eventually simulate symmetry by rotating 180 around the y-axis

pub fn map(value: f32, rg1: (f32, f32), rg2: (f32, f32)) -> f32 {
    let rg1unit = rg1.1 - rg1.0; // eq 1 unit
    let rg2unit = rg2.1 - rg2.0; // eq 1 unit

    let scaledif = rg1unit / rg2unit;

    let value_inunits = value / rg1unit; //value in rg1 units

    value_inunits * scaledif
}

pub fn clamp(input: usize, min: usize, max: usize) -> usize {
    let mut tmp = 0f32;
    if input < min {
        return min;
    }
    if input >= max {
        return max-1;
    }
    input
}

pub fn load_data(loadme: &mut Vec<Vertexd>, path: String) {
    println!("loading file...");
    let mut f = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("[ERROR] {:?}", e),
    };
    println!("file opened...");
    let mut buffer: [u8; 4] = [0, 0, 0, 0];
    let mut buffer2: [f32; 3] = [0.0, 0.0, 0.0];
    let mut b_ind: usize = 0;
    let mut v3_ind: usize = 0;
    let d = 1000000000000000000000000f32;
    println!("attempting byte read...");
    for b in f.bytes() {
        //print!("[READ]");
        buffer[b_ind] = b.unwrap();
        b_ind += 1;
        if b_ind >= 4 {
            //println!("[FLOATWRITE]");
//            buffer2[v3_ind] = (f32::from_ne_bytes(if cfg!(target_endian = "big") {
//                buffer
//            } else {
//                buffer
//            }));
            buffer2[v3_ind] = (f32::from_le_bytes(buffer));
            b_ind = 0;
            v3_ind += 1;
            if v3_ind >= 3 {
                let sum = buffer2[0] + buffer2[1] + buffer2[2];
                //print!("[VECWRITE] {:?}", [buffer2[0]/d,buffer2[1]/d,buffer2[2]/d]);
                loadme.push(Vertexd {
                    position: [buffer2[0]/d,buffer2[1]/d,buffer2[2]/d],
                    color: [(buffer2[0] / sum), (buffer2[1] / sum), (buffer2[2] / sum)],
                });
                v3_ind = 0;
            }
        }
    }
    println!("DONE! size loaded: {}", loadme.len());
}

pub fn build_vb(
    display: glium::Display,
    oindex: usize,
    base: &Vec<Vertexd>,
) -> (glium::Display, VertexBuffer<Vertexd>) {
    let vb = {
        glium::VertexBuffer::dynamic(
            &display,
            &[
                base[clamp(oindex+0,0, base.len())],
                base[clamp(oindex+1,0, base.len())],
                base[clamp(oindex+2,0, base.len())],
                base[clamp(oindex+3,0, base.len())],
                base[clamp(oindex+4,0, base.len())],
                base[clamp(oindex+5,0, base.len())],
            ],
        )
        .unwrap()
    };
    (display, vb)
}

pub fn rebuild_vb(
    oindex: usize,
    base: &Vec<Vertexd>,
    vb: &mut glium::VertexBuffer<Vertexd>,
) {
    let mut tmp = 0usize;
    match oindex {

        _ => {}
    }

        vb.write(
            &[
                base[clamp(oindex+0,0, base.len())],
                base[clamp(oindex+1,0, base.len())],
                base[clamp(oindex+2,0, base.len())],
                base[clamp(oindex+3,0, base.len())],
                base[clamp(oindex+4,0, base.len())],
                base[clamp(oindex+5,0, base.len())],
            ],
        );
}

pub fn build_tri(display: glium::Display) -> (glium::Display, VertexBuffer<Vertexd>) {
    let vb = {
        glium::VertexBuffer::new(
            &display,
            &[
                Vertexd {
                    position: [-0.5, -0.5, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertexd {
                    position: [0.0, 0.5, 0.0],
                    color: [0.0, 0.0, 1.0],
                },
                Vertexd {
                    position: [0.5, -0.5, 0.0],
                    color: [1.0, 0.0, 0.0],
                },
            ],
        )
        .unwrap()
    };

    (display, vb)
}

pub fn build_axis(display: glium::Display) -> (glium::Display, VertexBuffer<Vertexd>) {
    let vb = {
        glium::VertexBuffer::new(
            &display,
            &[
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
            ],
        )
        .unwrap()
    };

    (display, vb)
}
