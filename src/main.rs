extern crate nalgebra as na;
extern crate minifb;
extern crate nalgebra_glm;

mod cube;

use cube::{Cube, Ray};
use minifb::{Key, Window, WindowOptions};
use na::{Point3, Vector3};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Cubos 3D - Raytracing", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Crear varios cubos
    let cubes = vec![
        Cube {
            min: Point3::new(-1.0, -1.0, -1.0),
            max: Point3::new(1.0, 1.0, 1.0),
        },
        Cube {
            min: Point3::new(1.5, -1.0, -1.0),
            max: Point3::new(2.5, 1.0, 1.0),
        },
        Cube {
            min: Point3::new(-1.5, -1.0, 1.5),
            max: Point3::new(-0.5, 1.0, 2.5),
        },
    ];

    let mut camera_pos = Point3::new(0.0, 0.0, -5.0); // Posición inicial de la cámara

    // Bucle principal
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Control de la cámara
        if window.is_key_down(Key::W) {
            camera_pos.z += 0.1; // Mover hacia adelante
        }
        if window.is_key_down(Key::S) {
            camera_pos.z -= 0.1; // Mover hacia atrás
        }
        if window.is_key_down(Key::A) {
            camera_pos.x -= 0.1; // Mover a la izquierda
        }
        if window.is_key_down(Key::D) {
            camera_pos.x += 0.1; // Mover a la derecha
        }

        // Bucle por cada píxel de la pantalla
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                // Normaliza las coordenadas de la pantalla (de -1 a 1)
                let u = (x as f32 / WIDTH as f32) * 2.0 - 1.0;
                let v = (y as f32 / HEIGHT as f32) * 2.0 - 1.0;

                // Rayos que salen de la cámara en la dirección de cada píxel
                let ray_dir = Vector3::new(u, v, 1.0).normalize();
                let ray = Ray {
                    origin: camera_pos,
                    direction: ray_dir,
                };

                // Variable para almacenar la distancia más corta a un cubo
                let mut closest_t = f32::MAX;
                let mut pixel_color = rgb_to_u32(0, 0, 0); // Color predeterminado (negro)

                // Checar intersección de cada cubo
                for cube in &cubes {
                    if let Some(t) = cube.intersect(&ray) {
                        // Si hay intersección, checa si es el más cercano
                        if t < closest_t {
                            closest_t = t;
                            pixel_color = rgb_to_u32(255, (255.0 * (1.0 - t / 10.0)) as u8, 0); // Color del cubo
                        }
                    }
                }

                // Asigna el color del píxel
                buffer[y * WIDTH + x] = pixel_color;
            }
        }

        // Renderiza el buffer
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

// Función auxiliar para convertir de RGB a formato de 32 bits
fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}