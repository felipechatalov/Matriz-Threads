use std::thread;
use rand::Rng;
use std::time;

mod conway;
use conway::*;


// cria uma matrix com numeros aleatorios, de tamanho maxi de altura e maxj de largura
// valor pode variar de 0 ate threshold
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


// printa a matrix na tela
fn print_matrix(matrix: &Vec<Vec<i32>>, th: usize) {
    println!("Matrix [{}, {}]:", matrix.len(), matrix[0].len());
    for row in matrix {
        for col in row {
            print!("{: >th$}", col);
        }
        println!("");
    }
}

// faz a soma entre 2 matrizes de item i, j da m1 e item i, j da m2
// recebe m1 e m2 como emprestimo, pois nao precisa modificar as matrizes originais
fn sum_matrix_sequential(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // matriz que sera retornada no final
    let mut result = Vec::new();
    // percorre as linhas da matriz
    for i in 0..m1.len() {
        // cria uma nova linha
        let mut row = Vec::new();
        for j in 0..m1[0].len() {
            // soma os valores das matrizes
            row.push(m1[i][j] + m2[i][j]);
        }
        // adiciona a linha na matriz resultante
        result.push(row);
    }
    result
}



// faz a soma entre 2 matrizes de item i, j da m1 e item i, j da m2 usando threads
fn sum_matrix_threads(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>, thr: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    // rows determina quantas linhas temos no total
    let rows = m1.len();
    // rows_per_thread determina quantas linhas cada thread vai processar
    let rows_per_thread = rows / thr;
    // threads armazena as threads que serao criadas
    let mut handles = vec![];

    for i in 0..thr{
        // clona as 2 matrizes para serem utilizadas na thread
        let m1 = m1.clone();
        let m2 = m2.clone();
        // cria uma nova thread
        let handle = thread::spawn(move || {
            let mut result = Vec::new();
            // determina onde cada thread vai comecar e terminar
            let start = i * rows_per_thread;
            let mut end = start + rows_per_thread;

            // no caso de divisao nao exata, a ultima thread vai processar o resto das linhas
            if i == thr - 1 {
                end = rows;
            }
            // faz todo o processo, igual na funcao sequencial
            for i in start..end {
                let mut row = Vec::new();
                for j in 0..m1[0].len() {
                    row.push(m1[i][j] + m2[i][j]);
                }
                result.push(row);
            }
            // cada thread retorna o resultado dela
            result
        });
        // adiciona a thread na lista de threads
        handles.push(handle);
    }
    // percorre as threads e espera elas terminarem
    for handle in handles {
        let mut res = handle.join().unwrap();
        // adiciona o resultado da thread na matriz resultante
        result.append(&mut res);
    }
    // retorna o resultado final
    result
}

// faz a subtraco entre 2 matrizes de item i, j da m1 e item i, j da m2 
fn sub_matrix_sequential(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // tem o mesmo funcionamento que a funcao de soma, apenas com subtracao
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

// faz a subtraco entre 2 matrizes de item i, j da m1 e item i, j da m2 usando threads
fn sub_matrix_threads(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>, thr: usize) -> Vec<Vec<i32>> {
    // mesmo funcionamento que a funcao de soma, apenas com subtracao
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

// faz a multiplicacao entre 2 matrizes de soma de linhas por colunas de m1 e m2 no item i, j
fn mul_matrix_sequential(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for i in 0..m1.len() {
        let mut row = Vec::new();
        for j in 0..m2[0].len() {
            // percorre todas as linhas e colunas, somando o valor da multiplicacao de linhas por coluna
            let mut sum = 0;
            for k in 0..m1[0].len() {
                // percore todas as colunas de m1 e todas as linhas de m2
                sum += m1[i][k] * m2[k][j];
            }
            row.push(sum);
        }
        result.push(row);
    }
    result
}

// faz a multiplicacao entre 2 matrizes de soma de linhas por colunas de m1 e m2 no item i, j usando threads
fn mul_matrix_threads(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>, thr: usize) -> Vec<Vec<i32>> {
    // tem o mesmo funcionamento que a funcao de multiplicacao, apenas com threads
    let mut result = Vec::new();
    
    let rows = m1.len();
    let iter = rows / thr;
    let mut handles = vec![];

    for i in 0..thr{
        let m1 = m1.clone();
        let m2 = m2.clone();
        let handle = thread::spawn(move || {
            let mut result = Vec::new();
            // determina onde cada thread vai comecar e terminar
            let start = i * iter;
            let mut end = start + iter;

            // em caso de divisao nao exata, a ultima thread vai processar o resto das linhas
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

    // pega o tempo inicial
    let mut start = time::Instant::now();
    let result_s = sum_matrix_sequential(&matrix1, &matrix2);
    let mut end = time::Instant::now();
    // pega o tempo final e calcula a diferenca
    println!("Sum - Sequential: {} ms", end.duration_since(start).as_millis());

    start = time::Instant::now();
    // o terceiro parametro é o numero de threads
    let result_t = sum_matrix_threads(&matrix1, &matrix2, 10);
    end = time::Instant::now();
    println!("Sum - Threads: {} ms", end.duration_since(start).as_millis());

    // garante que o resultado das 2 funcoes sao o mesmo
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


    // 1 instancia é para o modo sequencial, determinado pelo 3 parametro, 0 para sequencial e 1 para threads
    conway::instantiate_game(800, 800, 0, 4);
    conway::instantiate_game(800, 800, 1, 4);

}