#[macro_use]
extern crate glium;
extern crate glutin;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate xml;
#[macro_use]
extern crate futures;
extern crate image;
extern crate tokio;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use futures::future::lazy;
use glium::backend::Facade;
use glium::glutin::WindowAttributes;
use glutin::dpi::LogicalSize;
use image::{GenericImageView, ImageFormat, RgbaImage};
use std::any::Any;
use std::env;
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use tokio::prelude::*;

use glium::texture::buffer_texture::{BufferTexture, BufferTextureType};
use glium::texture::RawImage2d;
use glium::Surface;

use std::collections::HashMap;
use xml::reader::XmlEvent;
use xml::EventReader;

mod application;

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

fn read_config(path: &str) -> Vec<WindowAttributes> {
    debug!("reading {}", path);

    let file = File::open(path).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;

    let mut windows = Vec::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                debug!("{}+{}", indent(depth), name);
                match name.local_name.as_str() {
                    "window" => {
                        let mut windowconfig = WindowAttributes::default();
                        for attrib in attributes {
                            debug!("{} ==> {}", attrib.name, attrib.value);
                            match attrib.name.local_name.as_str() {
                                "width" => {
                                    if let Some(mut size) = windowconfig.dimensions {
                                        windowconfig.dimensions =
                                            Some(glium::glutin::dpi::LogicalSize {
                                                width: FromStr::from_str(attrib.value.as_str())
                                                    .unwrap(),
                                                height: size.height,
                                            });
                                    } else {
                                        windowconfig.dimensions =
                                            Some(glium::glutin::dpi::LogicalSize {
                                                width: FromStr::from_str(attrib.value.as_str())
                                                    .unwrap(),
                                                height: 0.0,
                                            });
                                    }
                                    debug!("{:#?}", windowconfig.dimensions);
                                }
                                "height" => {
                                    if let Some(mut size) = windowconfig.dimensions {
                                        windowconfig.dimensions =
                                            Some(glium::glutin::dpi::LogicalSize {
                                                width: size.width,
                                                height: FromStr::from_str(attrib.value.as_str())
                                                    .unwrap(),
                                            });
                                    } else {
                                        windowconfig.dimensions =
                                            Some(glium::glutin::dpi::LogicalSize {
                                                width: 0.0,
                                                height: FromStr::from_str(attrib.value.as_str())
                                                    .unwrap(),
                                            });
                                    }
                                    debug!("{:#?}", windowconfig.dimensions);
                                }
                                "min_width" => {}
                                "min_height" => {}
                                "max_width" => {}
                                "max_height" => {}

                                "resizeable" => {
                                    windowconfig.resizable =
                                        FromStr::from_str(attrib.value.as_str()).unwrap();
                                }
                                "fullscreen" => {}
                                "title" => windowconfig.title = attrib.value,
                                "maximized" => {
                                    windowconfig.maximized =
                                        FromStr::from_str(attrib.value.as_str()).unwrap();
                                }
                                "visible" => {
                                    windowconfig.visible =
                                        FromStr::from_str(attrib.value.as_str()).unwrap();
                                }
                                "transparent" => {
                                    windowconfig.transparent =
                                        FromStr::from_str(attrib.value.as_str()).unwrap();
                                }
                                "decorations" => {
                                    windowconfig.decorations =
                                        FromStr::from_str(attrib.value.as_str()).unwrap();
                                }
                                "always_on_top" => {
                                    windowconfig.always_on_top =
                                        FromStr::from_str(attrib.value.as_str()).unwrap();
                                }
                                "window_icon" => {}
                                _ => {}
                            }
                        }
                        windows.push(windowconfig);
                    }
                    _ => {}
                }

                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                debug!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            Ok(XmlEvent::Characters(string)) => {
                debug!("{}|{}|", indent(depth), string.trim());
            }
            _ => {}
        }
    }
    return windows;
}

fn build_windows(windowconfig: Vec<WindowAttributes>) {
    use glium::{glutin, Surface};

    let mut events_loop = glium::glutin::EventsLoop::new();
    let mut windows = HashMap::new();

    for wc in windowconfig {
        let mut window = glium::glutin::WindowBuilder::new();
        window.window = wc;
        let context = glium::glutin::ContextBuilder::new();
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        let window_id = display.gl_window().id();
        windows.insert(window_id, display);
    }

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_coords);

    let vertex1 = Vertex {
        position: [-1.0, 1.0],
        tex_coords: [0.0, 0.0],
    };
    let vertex2 = Vertex {
        position: [-1.0, -1.0],
        tex_coords: [0.0, 1.0],
    };
    let vertex3 = Vertex {
        position: [1.0, 1.0],
        tex_coords: [1.0, 0.0],
    };
    let vertex4 = Vertex {
        position: [1.0, -1.0],
        tex_coords: [1.0, 1.0],
    };
    let shape = vec![vertex1, vertex2, vertex3, vertex2, vertex3, vertex4];

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
        uniform mat4 matrix;
        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec2 v_tex_coords;
        out vec4 color;
        uniform sampler2D tex;
        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let mut closed = false;
    while !closed {
        for (id, display) in windows.iter_mut() {
            let vertex_buffer = glium::VertexBuffer::new(&*display, &shape).unwrap();
            let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
            let mut buffer = [0xaa as u8; (800 * 600) * 4];
            let program = glium::Program::from_source(
                &*display,
                vertex_shader_src,
                fragment_shader_src,
                None,
            )
            .unwrap();
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 1.0, 1.0);

            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&buffer, (800, 600));

            let texture = glium::texture::Texture2d::new(&*display, image).unwrap();

            let uniforms = uniform! {
                matrix: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [ 0.0 , 0.0, 0.0, 1.0f32],
                ],
                tex: &texture,
            };

            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
            target.finish().unwrap();
        }
        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, window_id } => match event {
                glutin::WindowEvent::CloseRequested => {
                    windows.remove(&window_id);
                    debug!("window_id: {:?}", window_id);
                }
                glutin::WindowEvent::CursorMoved { position, .. } => {
                    if windows.get(&window_id).is_some() {
                        // TODO: change the buffer...
                    }
                }
                _ => (),
            },
            _ => (),
        });
        closed = windows.is_empty();
    }
}

fn main() {
    env::set_var("RUST_LOG", "debug");

    pretty_env_logger::init_custom_env("RUST_LOG");
    info!("Hallo Welt!");

    let windows = read_config("./idee.xml");

    build_windows(windows);
}
