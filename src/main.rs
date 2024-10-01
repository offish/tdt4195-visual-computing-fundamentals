#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

extern crate nalgebra_glm as glm;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::{mem, os::raw::c_void, ptr};

// mod obj;
mod mesh;
mod scene_graph;
mod shader;
mod toolbox;
mod util;

use glutin::event::{
    DeviceEvent,
    ElementState::{Pressed, Released},
    Event, KeyboardInput,
    VirtualKeyCode::{self, *},
    WindowEvent,
};
use glutin::event_loop::ControlFlow;
use scene_graph::SceneNode;
use toolbox::simple_heading_animation;

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 600;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //

// Get the size of an arbitrary array of numbers measured in bytes
// Example usage:  byte_size_of_array(my_array)
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
// Example usage:  pointer_to_array(my_array)
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
// Example usage:  size_of::<u64>()
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T, represented as a relative pointer
// Example usage:  offset::<u64>(4)
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()

unsafe fn create_vao(
    vertices: &Vec<f32>,
    indices: &Vec<u32>,
    colors: &Vec<f32>,
    normals: &Vec<f32>,
) -> u32 {
    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    let mut color_vbo: u32 = 0;
    let mut ibo: u32 = 0;
    let mut normal_vbo: u32 = 0;

    // vertex array object (VAO)
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);

    // vertex buffer object (VBO)
    gl::GenBuffers(1, &mut vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(vertices),
        pointer_to_array(vertices),
        gl::STATIC_DRAW,
    );

    // vertex attribute pointer (position)
    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * size_of::<f32>(),
        ptr::null(),
    );
    gl::EnableVertexAttribArray(0);

    // color buffer object
    gl::GenBuffers(1, &mut color_vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, color_vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(colors),
        pointer_to_array(colors),
        gl::STATIC_DRAW,
    );

    // normal buffer object
    gl::GenBuffers(1, &mut normal_vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, normal_vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(normals),
        pointer_to_array(normals),
        gl::STATIC_DRAW,
    );

    // vertex attribute pointer (color)
    gl::VertexAttribPointer(
        1,
        4,
        gl::FLOAT,
        gl::FALSE,
        4 * size_of::<f32>(),
        ptr::null(),
    );
    gl::EnableVertexAttribArray(1);

    // vertex attribute pointer (normal)
    gl::VertexAttribPointer(
        2,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * size_of::<f32>(),
        ptr::null(),
    );
    gl::EnableVertexAttribArray(2);

    // index buffer object (IBO)
    gl::GenBuffers(1, &mut ibo);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        byte_size_of_array(indices),
        pointer_to_array(indices),
        gl::STATIC_DRAW,
    );

    // return the actual vao id
    vao
}

unsafe fn draw_scene(
    node: &scene_graph::SceneNode,
    view_projection_matrix: &glm::Mat4,
    transformation_so_far: &glm::Mat4,
) {
    // let  my_transformation: glm::Mat4 = glm::identity();
    let rotation = glm::rotation(node.rotation.x, &glm::vec3(1.0, 0.0, 0.0))
        * glm::rotation(node.rotation.y, &glm::vec3(0.0, 1.0, 0.0))
        * glm::rotation(node.rotation.z, &glm::vec3(0.0, 0.0, 1.0));
    let translation = glm::translation(&node.position);
    // let scale = glm::scaling(&node.scale);
    let reference_point = glm::translation(&node.reference_point);

    let my_transformation = translation * rotation * reference_point;

    // Check if node is drawable, if so: set uniforms, bind VAO and draw VAO
    if node.index_count > 0 {
        // Perform any logic needed before drawing the node
        let my_matrix: glm::Mat4 =
            view_projection_matrix * transformation_so_far * my_transformation;
        // view_projection_matrix * translation * rotation * scale * reference_point;
        gl::UniformMatrix4fv(3, 1, gl::FALSE, my_matrix.as_ptr());

        gl::BindVertexArray(node.vao_id);
        gl::DrawElements(
            gl::TRIANGLES,
            node.index_count,
            gl::UNSIGNED_INT,
            ptr::null(),
        );
    }

    // Recurse
    for &child in &node.children {
        draw_scene(&*child, view_projection_matrix, &my_transformation);
    }
}

fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(
            INITIAL_SCREEN_W,
            INITIAL_SCREEN_H,
        ));
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Set up shared tuple for tracking changes to the window size
    let arc_window_size = Arc::new(Mutex::new((INITIAL_SCREEN_W, INITIAL_SCREEN_H, false)));
    // Make a reference of this tuple to send to the render thread
    let window_size = Arc::clone(&arc_window_size);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers.
        // This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        let mut window_aspect_ratio = INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32;

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE); // only draw counter-clockwise triangles
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!(
                "{}\t: {}",
                util::get_gl_string(gl::VENDOR),
                util::get_gl_string(gl::RENDERER)
            );
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!(
                "GLSL\t: {}",
                util::get_gl_string(gl::SHADING_LANGUAGE_VERSION)
            );
        }

        // actually creating the VAO
        let lunar = mesh::Terrain::load("./resources/lunarsurface.obj");
        let helicopter = mesh::Helicopter::load("./resources/helicopter.obj");

        let lunar_vao: u32 = unsafe {
            create_vao(
                &lunar.vertices,
                &lunar.indices,
                &lunar.colors,
                &lunar.normals,
            )
        };

        let helicopter_door_vao: u32 = unsafe {
            create_vao(
                &helicopter.door.vertices,
                &helicopter.door.indices,
                &helicopter.door.colors,
                &helicopter.door.normals,
            )
        };
        let mut helicopter_door =
            SceneNode::from_vao(helicopter_door_vao, helicopter.door.index_count);

        let helicopter_body_vao: u32 = unsafe {
            create_vao(
                &helicopter.body.vertices,
                &helicopter.body.indices,
                &helicopter.body.colors,
                &helicopter.body.normals,
            )
        };
        let mut helicopter_body =
            SceneNode::from_vao(helicopter_body_vao, helicopter.body.index_count);

        let helicopter_main_rotor_vao: u32 = unsafe {
            create_vao(
                &helicopter.main_rotor.vertices,
                &helicopter.main_rotor.indices,
                &helicopter.main_rotor.colors,
                &helicopter.main_rotor.normals,
            )
        };
        let mut helicopter_main_rotor =
            SceneNode::from_vao(helicopter_main_rotor_vao, helicopter.main_rotor.index_count);

        let helicopter_tail_rotor_vao: u32 = unsafe {
            create_vao(
                &helicopter.tail_rotor.vertices,
                &helicopter.tail_rotor.indices,
                &helicopter.tail_rotor.colors,
                &helicopter.tail_rotor.normals,
            )
        };
        let mut helicopter_tail_rotor =
            SceneNode::from_vao(helicopter_tail_rotor_vao, helicopter.tail_rotor.index_count);

        let mut master_scene = SceneNode::new();
        let mut lunar_node = SceneNode::from_vao(lunar_vao, lunar.index_count);
        let mut helicopter_node = SceneNode::new();

        lunar_node.reference_point = glm::vec3(0.0, 0.0, 0.0);
        master_scene.add_child(&lunar_node);

        helicopter_body.reference_point = glm::vec3(0.35, 2.3, 10.4);
        helicopter_door.reference_point = glm::vec3(0.35, 2.3, 10.4);
        helicopter_main_rotor.reference_point = glm::vec3(0.0, 0.0, 0.0);
        // helicopter_main_rotor.reference_point = glm::vec3(0.35, 2.3, 10.4);
        helicopter_tail_rotor.reference_point = glm::vec3(0.35, 2.3, 10.4);
        helicopter_main_rotor.position = glm::vec3(0.35, 2.3, 10.4);

        helicopter_node.add_child(&helicopter_body);
        helicopter_node.add_child(&helicopter_door);
        helicopter_node.add_child(&helicopter_main_rotor);
        helicopter_node.add_child(&helicopter_tail_rotor);

        master_scene.add_child(&helicopter_node);

        // master_scene.print();
        // lunar_node.print();
        // helicopter_node.print();

        // == // Set up your shaders here
        // Basic usage of shader helper:
        // The example code below creates a 'shader' object.
        // It which contains the field `.program_id` and the method `.activate()`.
        // The `.` in the path is relative to `Cargo.toml`.
        // This snippet is not enough to do the exercise, and will need to be modified (outside
        // of just using the correct path), but it only needs to be called once

        let simple_shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.vert")
                .attach_file("./shaders/simple.frag")
                .link()
                .activate()
        };

        // offset of camera (height)
        let y_offset: f32 = 0.5;
        // let falling_speed: f32 = 0.0;

        // xyz position of camera
        let mut _x_position: f32 = 0.0;
        let mut _y_position: f32 = y_offset;
        let mut _z_position: f32 = 0.0;

        // rotation of camera (in radians around x and y axis)
        let mut _x_rotation: f32 = 0.0;
        let mut _y_rotation: f32 = 0.0;

        // movement and rotation speed
        let movement_unit: f32 = 1.0;
        let rotation_unit: f32 = 2.0;

        // x and y axis for rotation
        let x_axis: glm::Vec3 = glm::vec3(1.0, 0.0, 0.0);
        let y_axis: glm::Vec3 = glm::vec3(0.0, 1.0, 0.0);

        // The main rendering loop
        let first_frame_time = std::time::Instant::now();
        let mut previous_frame_time = first_frame_time;
        loop {
            // Compute time passed since the previous frame and since the start of the program
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(previous_frame_time).as_secs_f32();
            previous_frame_time = now;

            // Handle resize events
            if let Ok(mut new_size) = window_size.lock() {
                if new_size.2 {
                    context.resize(glutin::dpi::PhysicalSize::new(new_size.0, new_size.1));
                    window_aspect_ratio = new_size.0 as f32 / new_size.1 as f32;
                    (*new_size).2 = false;
                    println!("Window was resized to {}x{}", new_size.0, new_size.1);
                    unsafe {
                        gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32);
                    }
                }
            }

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        // The `VirtualKeyCode` enum is defined here:
                        //    https://docs.rs/winit/0.25.0/winit/event/enum.VirtualKeyCode.html
                        VirtualKeyCode::W => {
                            _z_position += 20.0 * movement_unit * delta_time;
                        }
                        VirtualKeyCode::S => {
                            _z_position -= 20.0 * movement_unit * delta_time;
                        }
                        VirtualKeyCode::D => {
                            _x_position += 20.0 * movement_unit * delta_time;
                        }
                        VirtualKeyCode::A => {
                            _x_position -= 20.0 * movement_unit * delta_time;
                        }
                        VirtualKeyCode::Space => {
                            _y_position += movement_unit;
                        }
                        VirtualKeyCode::LShift => {
                            _y_position -= movement_unit;
                        }
                        VirtualKeyCode::LControl => {
                            _y_position -= movement_unit;
                        }
                        VirtualKeyCode::Up => {
                            _x_rotation += rotation_unit * delta_time;
                        }
                        VirtualKeyCode::Down => {
                            _x_rotation -= rotation_unit * delta_time;
                        }
                        VirtualKeyCode::Left => {
                            _y_rotation += rotation_unit * delta_time;
                        }
                        VirtualKeyCode::Right => {
                            _y_rotation -= rotation_unit * delta_time;
                        }

                        // default handler:
                        _ => {}
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {
                // == // Optionally access the accumulated mouse movement between
                // == // frames here with `delta.0` and `delta.1`

                *delta = (0.0, 0.0); // reset when done
            }

            // == // Please compute camera transforms here (exercise 2 & 3)

            unsafe {
                // Clear the color and depth buffers
                gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // pass time to shader
                // gl::Uniform1f(2, elapsed.sin());

                // pass transformations to shader
                // let identity: glm::Mat4x4 = glm::identity();
                // gl::UniformMatrix4fv(2, 1, gl::FALSE, identity.as_ptr());

                // set perspective of camera
                let perspective: glm::Mat4x4 =
                    glm::perspective(window_aspect_ratio, 55.0, 1.0, 1_000.0);

                // set position of camera
                let position: glm::Mat4x4 =
                    glm::translation(&glm::vec3(_x_position, _y_position, _z_position));

                // set rotation of camera
                let rotation: glm::Mat4x4 =
                    glm::rotation(_x_rotation, &x_axis) * glm::rotation(_y_rotation, &y_axis);

                let transform_matrix: glm::Mat4x4 = perspective * rotation * position;

                // animations
                let heading = simple_heading_animation(elapsed);

                helicopter_node.position = glm::vec3(-heading.x, -10.0, -heading.z);
                helicopter_node.rotation = glm::vec3(heading.pitch, heading.yaw, heading.roll);

                helicopter_tail_rotor.rotation = glm::vec3(1.0, 0.0, 0.0) * 5_000.0 * elapsed;
                helicopter_main_rotor.rotation = glm::vec3(0.0, 1.0, 0.0) * 5_000.0 * elapsed;

                draw_scene(&master_scene, &transform_matrix, &glm::zero());
            }
            // Display the new color buffer on the display
            context.swap_buffers().unwrap(); // we use "double buffering" to avoid artifacts
        }
    });

    // == //
    // == // From here on down there are only internals.
    // == //

    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events are initially handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                println!(
                    "New window size received: {}x{}",
                    physical_size.width, physical_size.height
                );
                if let Ok(mut new_size) = arc_window_size.lock() {
                    *new_size = (physical_size.width, physical_size.height, true);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: key_state,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        }
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle Escape and Q keys separately
                match keycode {
                    Escape => {
                        *control_flow = ControlFlow::Exit;
                    }
                    Q => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => {}
        }
    });
}
