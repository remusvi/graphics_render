use raylib::prelude::*;

const WORLD_SIZE: usize = 8; // Size of our voxel world (8x8x8 cubes)

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("raylib [models] example - basic voxel")
        .build();

    rl.disable_cursor();


    let mut camera = Camera3D::perspective(
        Vector3::new(-2.0, 2.0, -2.0), // Camera position
        Vector3::new(0.0, 0.0, 0.0),   // Camera looking at point
        Vector3::new(0.0, 1.0, 0.0),   // Camera up vector
        45.0,                          // Camera field-of-view Y
    );

    // Create a cube model
    let cube_mesh = unsafe { Mesh::gen_mesh_cube(&thread, 1.0, 1.0, 1.0).make_weak() };
    let mut cube_model = rl.load_model_from_mesh(&thread, cube_mesh).unwrap();


    cube_model.materials_mut()[0].maps_mut()[MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize]
        
        .color = Color::BEIGE;

    // Initialize voxel world - fill with voxels
    let mut voxels = [[[false; WORLD_SIZE]; WORLD_SIZE]; WORLD_SIZE];

    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            for z in 0..WORLD_SIZE {
                voxels[x][y][z] = true;
            }
        }
    }

    rl.set_target_fps(60);


    while !rl.window_should_close() {
        // Simple manual first-person camera movement fallback (WASD)
        let camera_speed = 0.05;
        if rl.is_key_down(KeyboardKey::KEY_W) {
            camera.position.z += camera_speed;
            camera.target.z += camera_speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            camera.position.z -= camera_speed;
            camera.target.z -= camera_speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            camera.position.x -= camera_speed;
            camera.target.x -= camera_speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            camera.position.x += camera_speed;
            camera.target.x += camera_speed;
        }

        // Handle voxel removal with mouse click
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let screen_center = Vector2::new(
                rl.get_screen_width() as f32 / 2.0,
                rl.get_screen_height() as f32 / 2.0,
            );
            let ray = rl.get_screen_to_world_ray(screen_center, &camera);

            let mut closest_distance = 99999.0_f32;
            let mut closest_voxel_position = Vector3::new(-1.0, -1.0, -1.0);
            let mut voxel_found = false;

            for x in 0..WORLD_SIZE {
                for y in 0..WORLD_SIZE {
                    for z in 0..WORLD_SIZE {
                        if !voxels[x][y][z] {
                            continue;
                        }

                        let position = Vector3::new(x as f32, y as f32, z as f32);
                        let bbox = BoundingBox::new(
                            Vector3::new(position.x - 0.5, position.y - 0.5, position.z - 0.5),
                            Vector3::new(position.x + 0.5, position.y + 0.5, position.z + 0.5),
                        );

                        let collision = bbox.get_ray_collision_box(ray);
                        if collision.hit && (collision.distance < closest_distance) {
                            closest_distance = collision.distance;
                            closest_voxel_position = Vector3::new(x as f32, y as f32, z as f32);
                            voxel_found = true;
                        }
                    }
                }
            }

            if voxel_found {ls[closest_voxel_position.x as usize][closest_voxel_position.y as usize]
                    [closest_voxel_position.z as usize] = false;
            }
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        {
            let mut c = d.begin_mode3D(&camera);

            c.draw_grid(10, 1.0);

            for x in 0..WORLD_SIZE {
                for y in 0..WORLD_SIZE {
                    for z in 0..WORLD_SIZE {
                        if !voxels[x][y][z] {
                            continue;
                        }

                        let position = Vector3::new(x as f32, y as f32, z as f32);
                        c.draw_model(&cube_model, position, 1.0, Color::BEIGE);
                        c.draw_cube_wires(position, 1.0, 1.0, 1.0, Color::BLACK);
                    }
                }
            }
        }

        let sw = d.get_screen_width();
        let sh = d.get_screen_height();
        d.draw_circle(sw / 2, sh / 2, 4.0, Color::RED);

        d.draw_text(
            "Left-click a voxel to remove it!",
            10,
            10,
            20,
            Color::DARKGRAY,
        );
        d.draw_text(
            "WASD to move, mouse to look around",
            10,
            35,
            10,
            Color::GRAY,
        );
    }
}
