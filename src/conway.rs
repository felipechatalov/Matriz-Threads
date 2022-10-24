extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use crate::conway::piston::window::AdvancedWindow;
use core::any::type_name;
use std::time::Instant;

use rand::*;
use std::time;
use std::thread;


const NUMBER_OF_THREADS: usize = 4;


pub struct Screen{
    gl: GlGraphics,
    width: u32,
    height: u32,
    cell_size: u32,
}
impl Screen{
    fn render(&mut self, arg: &RenderArgs, board: &Vec<Vec<bool>>) {
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
        });

        for (j, row) in board.iter().enumerate() {
            for (i, cell) in row.iter().enumerate() {
                if *cell == true{
                    self.gl.draw(arg.viewport(), |c, gl| {
                        let mut color: [f32; 4];
                        if *cell { 
                            // println!("cell is true");
                            color = [1.0, 1.0, 1.0, 1.0]; 
                        }else{
                            color = [0.0, 1.0, 0.0, 1.0];
                        }
                        graphics::rectangle(color,
                                            [i as f64 * self.cell_size as f64,
                                            j as f64 * self.cell_size as f64,
                                            self.cell_size as f64,
                                            self.cell_size as f64],
                                            c.transform,
                                            gl);
                    });
                }
            }
        }
    }
    fn update(&mut self, arg: &UpdateArgs) {
        // self.board.update_sequential();
    }
    
}

// # [derive(Clone, PartialEq)]


fn board_new(width: u32, height: u32) -> Vec<Vec<bool>>{
    let mut cells = Vec::new();
    for i in 0..height{
        let mut row = Vec::new();
        for j in 0..width{
            row.push(false);
        }
        cells.push(row);
    }
    cells
}

fn board_get_cell(board: &Vec<Vec<bool>>, x: usize, y: usize) -> bool{
    board[x][y]
}

fn board_set_cell(board: &mut Vec<Vec<bool>>, x: usize, y: usize, value: bool){
    board[x][y] = value;
}


fn board_get_neighbours(board: &Vec<Vec<bool>>, x: usize, y: usize, width: usize, height: usize) -> i32{
    // println!("x: {}, y: {}", x, y);
    let mut neighbours = 0;
    let mut i:i32;
    let mut j:i32;
    for i  in -1..2 as i32{
        for j  in -1..2 as i32{
            // println!("{} {}", x as i32 + i, y as i32 + j);
            // println!("bl: {}", board.len());
            if (i == 0 && j == 0) || x as i32 + i < 0 || y as i32 + j < 0 || x as i32 + i >= board.len() as i32 -1 || y as i32 + j >= width as i32{
                // println!("continue");
                continue;
            }
            // println!("{} {} {} {}", x, i, y, j);
            // println!("{}", x + i as usize);
            // println!("{}", y + j as usize);
            if board_get_cell(&board, (x as i32 + i) as usize, (y as i32 + j) as usize){
                neighbours += 1;
            }
        }
    }
    neighbours
}


fn update_sequential(board: Vec<Vec<bool>>, width: usize, height: usize) -> Vec<Vec<bool>>{
    let mut new_cells = Vec::new();
    for i in 0..height{
        let mut row = Vec::new();
        for j in 0..width{
            let neighbours = board_get_neighbours(&board, i, j, width, height);
            let cell = board_get_cell(&board, i, j);
            let new_cell = match neighbours{
                3 => true,
                2 => cell,
                _ => false,
            };
            row.push(new_cell);
        }
        new_cells.push(row);
    }
    new_cells
}
    

fn randomize_board(board: &mut Vec<Vec<bool>>, height: u32, width: u32, chance: f32){
    let mut rng = rand::thread_rng();
    for i in 0..height{
        for j in 0..width{
            let random: i32 = rng.gen_range(0, 10);
            // println!("{} {} {}", random, chance, random as f32 / 10.0);
            if (random as f32 /10f32) < chance {
                board_set_cell(board, i as usize, j as usize, true);
            }
        }
    }
}


fn copy_board(board: &Vec<Vec<bool>>, start: usize, end: usize, width: usize, height: usize) -> Vec<Vec<bool>>{
    let mut new_board = Vec::new();
    
    // println!("start: {}, end: {}, width: {}, height: {}", start, end, width, height);
    // println!("board len: {}", board.len());
    let mut sc = 0;
    if start != 0{
        sc = 1;
    }
    let mut ec = 0;
    if end != height{
        ec = 1;
    }
    // println!("sc: {}, ec: {}", sc, ec);
    // for i in (start as i32+sc) as usize..end+ec{
    for i in start-sc..end+ec{
        let mut row = Vec::new();
        for j in 0..width{
            // new_board[i][j] = board[i][j];
            // println!("i{} j{}", i, j);
            row.push(board[i][j]);
        }
        new_board.push(row);
    }
    new_board
}



fn update_threads(board: Vec<Vec<bool>>, width: usize, height: usize, threads: usize) -> Vec<Vec<bool>>{
    let mut result = Vec::new();
    let mut handles = Vec::new();


    for i in 0..threads{
        let mut start = i as usize * height / threads;
        let mut end = (i as usize + 1) * height / threads;
        // caso seja a ultima thread ela recebe todo o resto, em casos de divisao desigual
        if i == threads - 1{
            end = height as usize;
        };
        // faz uma copia do tabuleiro que sera trabalho, +1 para cima ou para baixo dependendo da iteracao
        let board = copy_board(&board, start, end, width, height);
        let handle = thread::spawn( move || {
            let mut new_cells = Vec::new();
            let mut add_to_start = 1;
            let mut sub_to_end = 0;
            // na 1 iteracao, comecamos pelo 1 indice e terminamos 1 anterior ao ultimo
            if start == 0{
                add_to_start = 0;
                sub_to_end = 1;
            }
            for i in 0+add_to_start..end-start-sub_to_end+1{
                let mut row = Vec::new();
                for j in 0..width{
                    let neighbours = board_get_neighbours(&board, i as usize, j as usize, width, height);
                    let cell = board[i as usize][j as usize];
                    let new_cell = match neighbours{
                        3 => true,
                        2 => cell,
                        _ => false,
                    };
                    row.push(new_cell);
                }
                new_cells.push(row);
            }
            new_cells
        });
        handles.push(handle);
    }
    // chama todas as threads para trabalhor e recebe o resultado de um pedaco do tabuleiro cada
    for handle in handles{
        let mut res = handle.join().unwrap();
        result.append(&mut res);
    }
    // retorna o tabuleiro montado
    result
}


pub fn instantiate_game(SCREEN_WIDTH: usize, SCREEN_HEIGHT: usize, use_threads: usize, CELL_SIZE: usize){
    // declara opengl para graficos, assim como a janela e o objeto screen para tela
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new(
        "Conway's Game of Life",
        [SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut screen = Screen{
        gl: GlGraphics::new(opengl),
        width: SCREEN_WIDTH as u32,
        height: SCREEN_HEIGHT as u32,
        cell_size: CELL_SIZE as u32,
    };
    let mut events = Events::new(EventSettings::new());
    // fixa frames e updates por segundo
    events.set_max_fps(16);
    events.set_ups(16);
    //tamanho do tabuleiro, em celulas
    let h = (SCREEN_HEIGHT/CELL_SIZE) as u32;
    let w = (SCREEN_WIDTH/CELL_SIZE) as u32;
    // cria o tabuleiro com w x h celulas
    let mut board = board_new(w, h);
    // randomiza o tabuleiro com 40% de chance de cada celula estar viva
    randomize_board(&mut board, w, h, 0.4);
    
    //variaveis para fazer a media do tempo de execucao
    let mut media_u = Vec::new();
    let mut media_r = Vec::new();
    let mut stop_r = false;
    let mut stop_u = false;
    let iter = 30;

    // loop principal do jogo, que captura cada evento
    while let Some(e) = events.next(&mut window) {
        

        if let Some(r) = e.render_args() {
            let st = Instant::now();

            screen.render(&r, &board);

            let et = Instant::now();

            let render_time = et.duration_since(st);



            if !stop_r && media_r.len() < iter{
                // adiciona o tempo de execucao e rendererizacao nos vetores de calcular media
                media_r.push(render_time.as_millis());
                // mostra na console o tempo em ms, alem de mudar o titulo para mostrar os ms tambem
                // window.set_title(format!("Conway's Game of Life - update: {}ms   render: {}ms", (update_time).as_millis(), render_time.as_millis()));
                println!("Time taken to render: {}ms", render_time.as_millis());
                if media_r.len() == iter{
                    stop_r = true;
                }
            }
            if stop_r{
                // faz o calculo da media e mostra no console assim como no titulo
                let mut sum = 0;
                for i in 0..iter{
                    sum += media_r[i];
                }
                // window.set_title(format!("Conway's Game of Life - update: {}ms   render: {}ms", (update_time).as_millis(), render_time.as_millis()));
                println!("Average time taken for {} iterations to render: {}ms", iter, sum/iter as u128);
                stop_r = false;
            }


        }
        else if let Some(u) = e.update_args() {
            let st = Instant::now();
            
            // decisao principal, caso queira usar o sequencial ou o paralelo
            // *--------------------------------------------*
            if use_threads == 0{
                board = update_sequential(board, (SCREEN_WIDTH/CELL_SIZE) as usize, (SCREEN_HEIGHT/CELL_SIZE) as usize);
            }
            else{
                board = update_threads(board, (SCREEN_WIDTH/CELL_SIZE) as usize, (SCREEN_HEIGHT/CELL_SIZE) as usize, NUMBER_OF_THREADS);
            }
            // *--------------------------------------------*
            
            let et = Instant::now();
            let update_time = et.duration_since(st);

            if !stop_u && media_u.len() < iter{
                // adiciona o tempo de execucao e rendererizacao nos vetores de calcular media
                media_u.push(update_time.as_millis());
                // mostra na console o tempo em ms, alem de mudar o titulo para mostrar os ms tambem
                window.set_title(format!("Conway's Game of Life - update: {}ms", (update_time).as_millis()));
                println!("Time taken to update: {}ms", update_time.as_millis());
                if media_u.len() == iter{
                    stop_u = true;
                }
            }
            if stop_u{
                // faz o calculo da media e mostra no console assim como no titulo
                let mut sum = 0;
                for i in 0..iter{
                    sum += media_u[i];
                }
                window.set_title(format!("Conway's Game of Life - update: {}ms", (update_time).as_millis()));
                println!("Average time taken for {} iterations to update: {}ms", iter, sum/iter as u128);
                stop_u = false;
            }
            
        }
    }
}

fn main(){
    // tamanho da tela e tamanho das celulas
    // caso queira adicionar o jogo para outro codigo bastar importar usando
    // 
    // mod conway;
    // use conway::*;
    // 
    // e escrever a linha abaixo

    // instantiate_game(800, 600, 1, 1);
}
