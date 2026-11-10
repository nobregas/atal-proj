mod entrada;
mod guloso;
mod json_util;
mod recursivo;
mod traits;

#[cfg(test)]
mod testes;

use entrada::{Entrada, validar};
use guloso::Guloso;
use traits::algoritmo::Algoritmo;
use std::io;

fn main() {
    
    let mut entrada = String::new();

    println!("Digite a matriz: ");

    io::stdin()
        .read_line(&mut entrada)
        .expect("Falha ao ler a entrada");


    let entrada: Entrada = match serde_json::from_str(&entrada) {
        Ok(entrada) => entrada,
        Err(_) => {
            println!();
            return;
        }
    };

    match validar(&entrada) {
        Ok(_) => { println!("Matriz valida"); },
        Err(err) => {
            println!("{}", json_util::erro(&err));
            return;
        },
    }

    let (resultado, iteracoes) = Guloso.resolver(&entrada);
    println!("{resultado}");
    println!("Iteracoes do guloso: {iteracoes}");

    // teste recusivo (usar testes.rs)

    // medir custo maximo - medir matrizes 2x3 3x3 6x6 10x10 1x10 todas de custo 1, e medir numero de movimentos maximos

    // teste guloso (usar teste da grade exemplo res_esperado = 26, depois grade propria de ate 4x4 em que resultado erre)
    // explicacao, o algoritmo guloso comum tem uma falha, se algumas das escolhas que ele fez acaba atrapalhando suas escolhas futuras.
    // tem um detalhe, que djisktra tambem eh um algoritmo guloso mas ele nao tem esse problema por conta da forma em que decide o melhor caminho.
}

