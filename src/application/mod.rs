use glium::Display;
use glutin::{Context, WindowId};
use std::collections::HashMap;

mod node;
mod window;

struct Application {
    windows: HashMap<WindowId, Display>,
}
/*
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

fn build_window(wc: WindowAttributes) {
    use glium::{glutin, Surface};
    debug!("{:#?}", wc);
    let mut events_loop = glium::glutin::EventsLoop::new();

    let mut window = glium::glutin::WindowBuilder::new();
    window.window = wc;

    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        display.target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.finish().unwrap();

        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, window_id } => match event {
                glutin::WindowEvent::CloseRequested => {
                    closed = true;
                    debug!("window_id: {:?}", window_id);
                }
                _ => (),
            },
            _ => (),
        });
    }
}
*/
