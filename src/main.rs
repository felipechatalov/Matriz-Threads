use std::thread;
use rand::Rng;
use std::time;

mod conway;
use conway::*;



fn create_matrix_from_random(maxi: i32, maxj: i32, threshold: i32) -> Vec<Vec<i32>> {
    let mut rng = rand::thread_rng();

    let mut matrix = Vec::new();
    for _ in 0..maxi {
        let mut row = Vec::new();
        for _ in 0..maxj {
            // row.push(rand::random::<i32>() % threshold);
            let n: i32 = rng.gen_range(0, threshold);
            row.push(n);
        }
        matrix.push(row);
    }
    matrix
}

fn print_matrix(matrix: &Vec<Vec<i32>>, th: usize) {
    println!("Matrix [{}, {}]:", matrix.len(), matrix[0].len());
    for row in matrix {
        for col in row {
            print!("{: >th$}", col);
        }
        println!("");
    }
}

fn sum_matrix_sequential(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for i in 0..m1.len() {
        let mut row = Vec::new();
        for j in 0..m1[0].len() {
            row.push(m1[i][j] + m2[i][j]);
        }
        result.push(row);
    }
    result
}
fn sum_matrix_threads(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>, thr: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    
    let rows = m1.len();
    let iter = rows / thr;
    let mut handles = vec![];

    for i in 0..thr{
        let m1 = m1.clone();
        let m2 = m2.clone();
        let handle = thread::spawn(move || {
            let mut result = Vec::new();
            let start = i * iter;
            let mut end = start + iter;

            // in case of uneven division
            if i == thr - 1 {
                end = rows;
            }

            for i in start..end {
                let mut row = Vec::new();
                for j in 0..m1[0].len() {
                    row.push(m1[i][j] + m2[i][j]);
                }
                result.push(row);
            }
            result
        });
        handles.push(handle);
    }
    for handle in handles {
        let mut res = handle.join().unwrap();
        result.append(&mut res);
    }
    result
}

fn sub_matrix_sequential(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for i in 0..m1.len() {
        let mut row = Vec::new();
        for j in 0..m1[0].len() {
            row.push(m1[i][j] - m2[i][j]);
        }
        result.push(row);
    }
    result
}
fn sub_matrix_threads(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>, thr: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    
    let rows = m1.len();
    let iter = rows / thr;
    let mut handles = vec![];

    for i in 0..thr{
        let m1 = m1.clone();
        let m2 = m2.clone();
        let handle = thread::spawn(move || {
            let mut result = Vec::new();
            let start = i * iter;
            let mut end = start + iter;

            // in case of uneven division
            if i == thr - 1 {
                end = rows;
            }

            for i in start..end {
                let mut row = Vec::new();
                for j in 0..m1[0].len() {
                    row.push(m1[i][j] - m2[i][j]);
                }
                result.push(row);
            }
            result
        });
        handles.push(handle);
    }
    for handle in handles {
        let mut res = handle.join().unwrap();
        result.append(&mut res);
    }
    result
}

fn mul_matrix_sequential(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for i in 0..m1.len() {
        let mut row = Vec::new();
        for j in 0..m2[0].len() {
            let mut sum = 0;
            for k in 0..m1[0].len() {
                sum += m1[i][k] * m2[k][j];
            }
            row.push(sum);
        }
        result.push(row);
    }
    result
}

fn mul_matrix_threads(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>, thr: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    
    let rows = m1.len();
    let iter = rows / thr;
    let mut handles = vec![];

    for i in 0..thr{
        let m1 = m1.clone();
        let m2 = m2.clone();
        let handle = thread::spawn(move || {
            let mut result = Vec::new();
            let start = i * iter;
            let mut end = start + iter;

            // in case of uneven division
            if i == thr - 1 {
                end = rows;
            }

            for i in start..end {
                let mut row = Vec::new();
                for j in 0..m2[0].len() {
                    let mut sum = 0;
                    for k in 0..m1[0].len() {
                        sum += m1[i][k] * m2[k][j];
                    }
                    row.push(sum);
                }
                result.push(row);
            }
            result
        });
        handles.push(handle);
    }
    for handle in handles {
        let mut res = handle.join().unwrap();
        result.append(&mut res);
    }
    result
}

fn main() {

    // escolhe o tamanho das matrizes, linhas, colunas e valor maximo para cada elemento
    let matrix1 = create_matrix_from_random(100, 100, 10);
    let matrix2 = create_matrix_from_random(100, 100, 10);
    // print_matrix(&matrix1, 3);
    // print_matrix(&matrix2, 3);

    
    let mut start = time::Instant::now();
    let result_s = sum_matrix_sequential(&matrix1, &matrix2);
    let mut end = time::Instant::now();
    println!("Sum - Sequential: {} ms", end.duration_since(start).as_millis());

    start = time::Instant::now();
    // o terceiro parametro é o numero de threads
    let result_t = sum_matrix_threads(&matrix1, &matrix2, 10);
    end = time::Instant::now();
    println!("Sum - Threads: {} ms", end.duration_since(start).as_millis());

    assert_eq!(result_s, result_t);



    let mut start = time::Instant::now();
    let result_s = sub_matrix_sequential(&matrix1, &matrix2);
    let mut end = time::Instant::now();
    println!("Sub - Sequential: {} ms", end.duration_since(start).as_millis());

    start = time::Instant::now();
    let result_t = sub_matrix_threads(&matrix1, &matrix2, 10);
    end = time::Instant::now();
    println!("Sub - Threads: {} ms", end.duration_since(start).as_millis());

    assert_eq!(result_s, result_t);



    let mut start = time::Instant::now();
    let result_s = mul_matrix_sequential(&matrix1, &matrix2);
    let mut end = time::Instant::now();
    println!("Mul - Sequential: {} ms", end.duration_since(start).as_millis());

    start = time::Instant::now();
    let result_t = mul_matrix_threads(&matrix1, &matrix2, 10);
    end = time::Instant::now();
    println!("Mul - Threads: {} ms", end.duration_since(start).as_millis());

    assert_eq!(result_s, result_t);


    // recomendo rodar o jogo em codigo separado porem é possivel rodar aqui tbm
    
    conway::instantiate_game(800, 800, 0, 4);
    conway::instantiate_game(800, 800, 1, 4);

}