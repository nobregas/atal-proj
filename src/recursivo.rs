use serde_json::Value;

use crate::entrada::Entrada;
use crate::traits::algoritmo::Algoritmo;
use crate::json_util::{resultado};

pub struct Recursivo;

fn pode_direita(limite: usize, coluna: usize) -> bool {
    coluna + 1 < limite
}

fn pode_baixo(limite: usize, linha: usize) -> bool {
    linha + 1 < limite
}

fn movimentar(custos: &[Vec<Option<u32>>], linha: usize, coluna: usize, chamadas: &mut usize) -> Option<u32> {
    *chamadas += 1;
    let linhas = custos.len();
    let colunas = custos[0].len();

    let custo_atual = match custos[linha][coluna] {
        Some(custo) => custo,
        None => return None, // sem caminho
    };

    if linha == linhas - 1 && coluna == colunas - 1 {
        // chegada
        return Some(custo_atual);
    }

    let direita = if pode_direita(colunas, coluna) {
        movimentar(custos, linha, coluna + 1, chamadas)
    } else {
        None
    };

    let baixo = if pode_baixo(linhas, linha) {
        movimentar(custos, linha + 1, coluna, chamadas)
    } else {
        None
    };

    match (direita, baixo) {
        // direita e baixo disponivel direita menor
        (Some(direita), Some(baixo)) if direita <= baixo => {
            Some(custo_atual + direita)
        }

        // direita e baixo disponivel baixo menor
        (Some(_), Some(baixo)) => {
            Some(custo_atual + baixo)
        }

        // somente direita disponivel
        (Some(direita), None) => {
            Some(custo_atual + direita)
        }

        // somente baixo disponivel
        (None, Some(baixo)) => {
            Some(custo_atual + baixo)
        }

        // preso
        (None, None) => {
            None
        }
    }
}

impl Recursivo {
    pub fn resolver_com_contagem(&self, entrada: &Entrada) -> (Value, usize) {
        let custos = &entrada.custos;
        
        let linhas = custos.len();
        let colunas = custos[0].len();

        if custos[0][0].is_none() || custos[linhas - 1][colunas - 1].is_none() {
            // sem caminho
            return (resultado(None), 0);
        }

        let mut chamadas = 0;
        let custo = movimentar(custos, 0, 0, &mut chamadas);
        (resultado(custo), chamadas)
    }
}

impl Algoritmo for Recursivo {
    fn resolver(&self, entrada: &Entrada) -> (Value, usize) {
        self.resolver_com_contagem(entrada)
    }
}
