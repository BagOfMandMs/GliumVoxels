#![allow(unused_parens)] 

extern crate winit;
#[macro_use] extern crate glium;

use std::{mem, str::Matches, time::Duration, time::Instant};
use glium::Surface;

type ChunkRow = u32;
const CHUNKSIZE: usize = mem::size_of::<ChunkRow>() * 8;
type TileType = u8;
const TILETYPES: TileType = 2;

#[derive(PartialEq)]
enum Axis {
    X = 1,
    Y = 2,
    Z = 3
}

#[derive(PartialEq)]
enum Direction {
    UP = 2,
    DOWN = -2,
    NORTH = 3,
    EAST = 1,
    SOUTH = -3,
    WEST = -1
}

fn rel_direction(axis: &Axis, up: bool) -> Direction{
    match(axis) {
        Axis::X=>{if(up){Direction::EAST}else{Direction::WEST}},
        Axis::Y=>{if(up){Direction::UP}else{Direction::DOWN}},
        Axis::Z=>{if(up){Direction::NORTH}else{Direction::SOUTH}}
    }
}


#[derive(Copy, Clone)]
struct Vec3 {
    position: [f32; 3]
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
    tid: i32
}
implement_vertex!(Vertex, position, uv, tid);


fn main() {

    let x = 5;
    let mut x = x;
    x += 3;

    // let chunk: [[[TileType; CHUNKSIZE]; CHUNKSIZE]; CHUNKSIZE] = [
    //     [[1, 1, 1, 0, 0, 0, 0, 0], [1, 1, 1, 0, 0, 0, 0, 0], [1, 1, 1, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]],
    //     [[1, 1, 1, 1, 1, 1, 1, 0], [1, 1, 1, 0, 0, 0, 0, 0], [1, 1, 1, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 1, 0, 0, 0, 0, 1, 0], [0, 1, 0, 0, 0, 0, 0, 0]],
    //     [[1, 1, 1, 0, 0, 0, 0, 0], [1, 1, 1, 0, 0, 0, 0, 0], [1, 1, 1, 0, 0, 0, 0, 0], [0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 1, 1, 0, 0, 0, 0], [0, 0, 0, 0, 0, 1, 1, 0], [0, 1, 1, 0, 1, 1, 1, 0], [0, 0, 1, 0, 0, 0, 0, 0]],
    //     [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 1, 0, 1, 0, 0, 0], [0, 0, 1, 1, 0, 0, 0, 0], [0, 0, 0, 0, 0, 1, 1, 0], [0, 0, 1, 1, 1, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]],
    //     [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 1, 1, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]],
    //     [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 1, 1, 0, 0, 0], [0, 1, 0, 0, 1, 0, 0, 0]],
    //     [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 1, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 1, 1, 1, 0, 0]],
    //     [[0, 0, 0, 0, 0, 0, 0, 0], [0, 1, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0]]
    // ];

    let mut chunk2: [[[TileType; CHUNKSIZE]; CHUNKSIZE]; CHUNKSIZE] = [[[0; CHUNKSIZE]; CHUNKSIZE]; CHUNKSIZE];
    for x in 0..CHUNKSIZE {
        for y in 0..CHUNKSIZE {
            for z in 0..CHUNKSIZE {
                chunk2[x][y][z] = ((x + y + z) % 2) as TileType;
            }
        }
    }

    let now = Instant::now();
    let res = gen_chunk_mesh(chunk2);
    println!("{}", now.elapsed().as_micros());

    // for vert in &res{
    //     let x = vert.position[0];
    //     let y = vert.position[1];
    //     let z = vert.position[2];
    //     println!("({x}, {y}, {z})  ");
    // }

    let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("Failed to build event loop");

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    // let vertex1 = Vertex { position: [-0.5, -0.5, 1.0], uv: [0.0, 0.0], tid: 0 };
    // let vertex2 = Vertex { position: [ 0.5,  0.5, 1.0], uv: [1.0, 1.0], tid: 0 };
    // let vertex3 = Vertex { position: [ 0.5, -0.5, 1.0], uv: [1.0, 0.0], tid: 0 };
    // let vertex4 = Vertex { position: [ -0.5,  0.5, 1.0], uv: [0.0, 1.0], tid: 0 };
    // let _shape = vec![vertex1, vertex2, vertex3, vertex4];
    //let shape = gen_chunk_mesh();

    //let indices: [u32; 6] = [0, 1, 2, 0, 3, 1];

    let vertex_buffer = glium::VertexBuffer::new(&display, &res).unwrap();
    //let index_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 330
        in vec3 position;
        in vec2 uv;
        in int tid;

        out vec3 frag_color;

        uniform mat4 matrix;
        uniform mat4 matrix2;

        void main() {
            frag_color = vec3(1.0, uv);
            vec3 corrected_pos = vec3(position.x, position.y - 10, position.z);
            vec4 translation = vec4(0.0, 0.0, 50.0, 0.0);
            gl_Position = (matrix * matrix2 * vec4(corrected_pos / 16.0, 1.0)) - translation;
            //gl_Position = vec4(position / 2.0, 1.0);
        }
    "#;
    
    let fragment_shader_src = r#"
        #version 330
        in vec3 frag_color;

        out vec4 color;

        void main() {
            color = vec4(frag_color.xyz, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    let mut t: f32 = 0.0;

    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                winit::event::WindowEvent::RedrawRequested => {

                    let a: f32 = 0.2;
                    let uniforms = uniform! {
                        matrix: [
                            [ t.cos(), 0.0, t.sin(), 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [-t.sin(), 0.0, t.cos(), 0.0],
                            [0.0, 0.0, 0.0, 1.0f32],
                        ],
                        matrix2: [
                            [ a.cos(), a.sin(), 0.0, 0.0],
                            [-a.sin(), a.cos(), 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [0.0, 0.0, 0.0, 1.0f32],
                        ]
                    };

                    t+= 0.01;

                    let mut target = display.draw();
                    target.clear_color_and_depth((0.5, 0.5, 0.5, 1.0), 1.0);

                    
                    target.draw(&vertex_buffer, &index_buffer, &program, &uniforms,
                        &params).unwrap();
                    target.finish().unwrap();
                },
                _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };
    });
}

fn gen_chunk_mesh(grid: [[[TileType; CHUNKSIZE]; CHUNKSIZE]; CHUNKSIZE]) -> Vec<Vertex> {
    let mut vec = Vec::new();
    let mut x_slices_f: [[TileType; CHUNKSIZE * CHUNKSIZE]; CHUNKSIZE] = [[0; CHUNKSIZE * CHUNKSIZE]; CHUNKSIZE];
    let mut x_slices_r: [[TileType; CHUNKSIZE * CHUNKSIZE]; CHUNKSIZE] = [[0; CHUNKSIZE * CHUNKSIZE]; CHUNKSIZE];
    let mut y_slices_f: [[TileType; CHUNKSIZE * CHUNKSIZE]; CHUNKSIZE] = [[0; CHUNKSIZE * CHUNKSIZE]; CHUNKSIZE];
    let mut y_slices_r: [[TileType; CHUNKSIZE * CHUNKSIZE]; CHUNKSIZE] = [[0; CHUNKSIZE * CHUNKSIZE]; CHUNKSIZE];
    let mut z_slices_f: [[TileType; CHUNKSIZE * CHUNKSIZE]; CHUNKSIZE] = [[0; CHUNKSIZE * CHUNKSIZE]; CHUNKSIZE];
    let mut z_slices_r: [[TileType; CHUNKSIZE * CHUNKSIZE]; CHUNKSIZE] = [[0; CHUNKSIZE * CHUNKSIZE]; CHUNKSIZE];
    for x in 0..CHUNKSIZE {
        for y in 0..CHUNKSIZE {
            for z in 0..CHUNKSIZE {
                if(x == 0 || grid[x][y][z] != grid[x - 1][y][z]){
                    x_slices_f[x][y * CHUNKSIZE + z] = grid[x][y][z];
                }
                if(y == 0 || grid[x][y][z] != grid[x][y - 1][z]){
                    y_slices_f[y][z * CHUNKSIZE + x] = grid[x][y][z];
                }
                if(z == 0 || grid[x][y][z] != grid[x][y][z - 1]){
                    z_slices_f[z][x * CHUNKSIZE + y] = grid[x][y][z];
                }
                if(x == CHUNKSIZE - 1 || grid[x][y][z] != grid[x + 1][y][z]){
                    x_slices_r[x][y * CHUNKSIZE + z] = grid[x][y][z];
                }
                if(y == CHUNKSIZE - 1 || grid[x][y][z] != grid[x][y + 1][z]){
                    y_slices_r[y][z * CHUNKSIZE + x] = grid[x][y][z];
                }
                if(z == CHUNKSIZE - 1 || grid[x][y][z] != grid[x][y][z + 1]){
                    z_slices_r[z][x * CHUNKSIZE + y] = grid[x][y][z];
                }
            }
        }    
    }
    gen_chunk_tile_mesh(x_slices_r, &mut vec, Direction::EAST);
    gen_chunk_tile_mesh(y_slices_r, &mut vec, Direction::UP);
    gen_chunk_tile_mesh(z_slices_r, &mut vec, Direction::NORTH);
    gen_chunk_tile_mesh(x_slices_f, &mut vec, Direction::WEST);
    gen_chunk_tile_mesh(y_slices_f, &mut vec, Direction::DOWN);
    gen_chunk_tile_mesh(z_slices_f, &mut vec, Direction::SOUTH);
    //stupid_mesher(x_slices, &mut vec, Axis::X);
    //stupid_mesher(y_slices, &mut vec, Axis::Y);
    //stupid_mesher(z_slices, &mut vec, Axis::Z);
    return vec;
}

fn _stupid_mesher(grid: [[TileType; CHUNKSIZE * CHUNKSIZE]; CHUNKSIZE], target: &mut Vec<Vertex>, axis: Axis){
    for slice_num in 0..CHUNKSIZE{
        for i in 0..CHUNKSIZE{
            for j in 0..CHUNKSIZE{
                match(axis){
                    Axis::X => {
                        if(grid[slice_num][i * CHUNKSIZE + j] == 1){
                            let x = slice_num as f32;
                            let y = i as f32;
                            let z = j as f32;
                            let tile = 1;
                            target.push(Vertex { position: [x, y, z], uv: [0.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x, y + 1.0, z + 1.0], uv: [1.0, 1.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x, y + 1.0, z], uv: [1.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x, y, z], uv: [0.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x, y, z + 1.0], uv: [0.0, 1.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x, y + 1.0, z + 1.0], uv: [1.0, 1.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y, z], uv: [0.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y + 1.0, z + 1.0], uv: [1.0, 1.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y + 1.0, z], uv: [1.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y, z], uv: [0.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y, z + 1.0], uv: [0.0, 1.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y + 1.0, z + 1.0], uv: [1.0, 1.0], tid: (tile as i32) });
                        }
                    }
                    Axis::Y => {
                        if(grid[slice_num][i * CHUNKSIZE + j] == 1){
                            let x = j as f32;
                            let y = slice_num as f32;
                            let z = i as f32;
                            let tile = 1;
                            target.push(Vertex { position: [x, y, z], uv: [0.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y, z + 1.0], uv: [1.0, 1.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y, z], uv: [1.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x, y, z], uv: [0.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x, y, z + 1.0], uv: [0.0, 1.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y, z + 1.0], uv: [1.0, 1.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x, y + 1.0, z], uv: [0.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y + 1.0, z + 1.0], uv: [1.0, 1.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y + 1.0, z], uv: [1.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x, y + 1.0, z], uv: [0.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x, y + 1.0, z + 1.0], uv: [0.0, 1.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y + 1.0, z + 1.0], uv: [1.0, 1.0], tid: (tile as i32) });
                        }
                    }
                    Axis::Z => {
                        if(grid[slice_num][i * CHUNKSIZE + j] == 1){
                            let x = i as f32;
                            let y = j as f32;
                            let z = slice_num as f32;
                            let tile = 1;
                            target.push(Vertex { position: [x, y, z], uv: [0.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y + 1.0, z], uv: [1.0, 1.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y, z], uv: [1.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x, y, z], uv: [0.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x, y + 1.0, z], uv: [0.0, 1.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y + 1.0, z], uv: [1.0, 1.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x, y, z + 1.0], uv: [0.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y + 1.0, z + 1.0], uv: [1.0, 1.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y, z + 1.0], uv: [1.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x, y, z + 1.0], uv: [0.0, 0.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x, y + 1.0, z + 1.0], uv: [0.0, 1.0], tid: (tile as i32) });
                            target.push(Vertex { position: [x + 1.0, y + 1.0, z + 1.0], uv: [1.0, 1.0], tid: (tile as i32) });
                        }
                    }
                }
            }
        }
    }
}


fn gen_chunk_tile_mesh(grid: [[TileType; CHUNKSIZE * CHUNKSIZE]; CHUNKSIZE], target: &mut Vec<Vertex>, dir: Direction) {
    for tile_type in 1..TILETYPES {
        for slice_num in 0..CHUNKSIZE {
            let mut bitgrid: [ChunkRow; CHUNKSIZE] = [0; CHUNKSIZE];
            for i in 0..CHUNKSIZE {
                for j in 0..CHUNKSIZE{
                    bitgrid[i] |= (if(grid[slice_num][j * CHUNKSIZE + i] == tile_type){1}else{0}) << j;
                }
            }
            gen_tile_faces(bitgrid, target, &dir, tile_type, &(slice_num as ChunkRow));
        }
    }
}

fn gen_tile_faces(mut bitgrid: [ChunkRow; CHUNKSIZE], target: &mut Vec<Vertex>, dir: &Direction, tile: TileType, slice_number: &ChunkRow){
    
    for row in 0..CHUNKSIZE {
        let mut y: ChunkRow = 0;
        loop {
            y = ((bitgrid[row]).trailing_zeros() as ChunkRow);
            if((y as usize) == CHUNKSIZE){
                break;
            }
            let yheight: ChunkRow = ((bitgrid[row] >> y).trailing_ones() as ChunkRow);
            let mask = (((1 as u128) << yheight) - 1) << y;
            let mask = mask as ChunkRow;
            let mut x: ChunkRow = ((row + 1) as ChunkRow);
            bitgrid[row] ^= mask;
            while((x as usize) < CHUNKSIZE && (bitgrid[(x as usize)] & mask) == mask){
                bitgrid[(x as usize)] ^= mask;
                x += 1;
            }
            x -= 1;
            let ytop = y + yheight - 1;
            // Build face quad here for
            // (row, y) -> (x, ytop)
            gen_face(target, &(row as ChunkRow), &y, &x, &ytop, tile, &dir, slice_number);
            let t = bitgrid[row];
            //println!("{t} : ({row}, {y})->({x}, {ytop}) : {mask}");
        }
    }
}

fn gen_face(target: &mut Vec<Vertex>, x1: &ChunkRow, y1: &ChunkRow, x2: &ChunkRow, y2: &ChunkRow, tile: TileType, dir: &Direction, slice_number: &ChunkRow){
    let mut pos1: [f32; 3] = [0.0; 3];
    let mut pos2: [f32; 3] = [0.0; 3];
    
    // X - EAST/WEST    ->  YZ space: (x, y) -> {S X Y}
    // Y - UP/DOWN      ->  ZX space: (x, y) -> {Y S X}
    // Z - NORTH/SOUTH  ->  XY space: (x, y) -> {X Y S}

    match(dir){
        Direction::EAST | Direction::WEST => {
            let uvx = ((x2 + 1) - x1) as f32;
            let uvy = ((y2 + 1) - y1) as f32;

            let mut xcoord = (*slice_number as f32);
            if(*dir == Direction::EAST){
                xcoord += 1.0;
            }

            let xc1 = (*x1 as f32);
            let xc2 = (*x2 as f32)+ 1.0;
            let yc1 = (*y1 as f32);
            let yc2 = (*y2 as f32)+ 1.0;

            //println!("{x2} - {x1} = {uvx}");

            target.push(Vertex { position: [xcoord, yc1, xc1], uv: [0.0, 0.0], tid: (tile as i32) });
            target.push(Vertex { position: [xcoord, yc2, xc2], uv: [uvx, uvy], tid: (tile as i32) });
            target.push(Vertex { position: [xcoord, yc1, xc2], uv: [uvx, 0.0], tid: (tile as i32) });
            
            target.push(Vertex { position: [xcoord, yc1, xc1], uv: [0.0, 0.0], tid: (tile as i32) });
            target.push(Vertex { position: [xcoord, yc2, xc1], uv: [0.0, uvy], tid: (tile as i32) });
            target.push(Vertex { position: [xcoord, yc2, xc2], uv: [uvx, uvy], tid: (tile as i32) });
        }
        Direction::UP | Direction::DOWN => {
            let uvx = ((x2 + 1) - x1) as f32;
            let uvy = ((y2 + 1) - y1) as f32;

            let mut ycoord = (*slice_number as f32);
            if(*dir == Direction::UP){
                ycoord += 1.0;
            }

            let xc1 = (*x1 as f32);
            let xc2 = (*x2 as f32) + 1.0;
            let yc1 = (*y1 as f32);
            let yc2 = (*y2 as f32) + 1.0;

            target.push(Vertex { position: [xc1, ycoord, yc1], uv: [0.0, 0.0], tid: (tile as i32) });
            target.push(Vertex { position: [xc2, ycoord, yc2], uv: [uvx, uvy], tid: (tile as i32) });
            target.push(Vertex { position: [xc2, ycoord, yc1], uv: [uvx, 0.0], tid: (tile as i32) });
            
            target.push(Vertex { position: [xc1, ycoord, yc1], uv: [0.0, 0.0], tid: (tile as i32) });
            target.push(Vertex { position: [xc1, ycoord, yc2], uv: [0.0, uvy], tid: (tile as i32) });
            target.push(Vertex { position: [xc2, ycoord, yc2], uv: [uvx, uvy], tid: (tile as i32) });
        }
        Direction::NORTH | Direction::SOUTH => {
            let uvx = ((x2 + 1) - x1) as f32;
            let uvy = ((y2 + 1) - y1) as f32;

            let mut zcoord = (*slice_number as f32);
            if(*dir == Direction::NORTH){
                zcoord += 1.0;
            }

            let xc1 = (*x1 as f32);
            let xc2 = (*x2 as f32) + 1.0;
            let yc1 = (*y1 as f32);
            let yc2 = (*y2 as f32) + 1.0;

            target.push(Vertex { position: [yc1, xc1, zcoord], uv: [0.0, 0.0], tid: (tile as i32) });
            target.push(Vertex { position: [yc2, xc2, zcoord], uv: [uvx, uvy], tid: (tile as i32) });
            target.push(Vertex { position: [yc1, xc2, zcoord], uv: [uvx, 0.0], tid: (tile as i32) });
            
            target.push(Vertex { position: [yc1, xc1, zcoord], uv: [0.0, 0.0], tid: (tile as i32) });
            target.push(Vertex { position: [yc2, xc1, zcoord], uv: [0.0, uvy], tid: (tile as i32) });
            target.push(Vertex { position: [yc2, xc2, zcoord], uv: [uvx, uvy], tid: (tile as i32) });
        }
    }
}

