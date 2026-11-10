use serde_json::Value;

use crate::entrada::Entrada;
use crate::traits::algoritmo::Algoritmo;
use crate::json_util::{resultado};

pub struct Guloso;

fn pode_direita(limite: usize, coluna: usize) -> bool {
    coluna + 1 < limite
}

fn pode_baixo(limite: usize, linha: usize) -> bool {
    linha + 1 < limite
}

impl Guloso {
    pub fn resolver_com_contagem(&self, entrada: &Entrada) -> (Value, usize) {
        let custos = &entrada.custos;

        let linhas = custos.len();
        let colunas = custos[0].len();

        if custos[0][0].is_none() || custos[linhas - 1][colunas - 1].is_none() {
            // sem caminho
            return (resultado(None), 0);
        }

        let mut linha = 0;
        let mut coluna = 0;

        let mut custo_total = custos[0][0].unwrap();
        let mut iteracoes = 0;

        while linha != linhas -1 || coluna != colunas -1 {
            iteracoes += 1;
            let direita = if pode_direita(colunas, coluna) {
                custos[linha][coluna + 1]
            } else {
                None
            };

            let baixo = if pode_baixo(linhas, linha) {
                custos[linha + 1][coluna]
            } else {
                None
            };

            let (nova_linha, nova_coluna, custo) = match (direita, baixo) {
                
                // direita e baixo disponivel direita menor
                (Some(direita), Some(baixo)) if direita <= baixo => {
                    (linha, coluna + 1, direita)
                }
                
                // direita e baixo disponivel baixo menor
                (Some(_), Some(baixo)) => {
                    (linha + 1, coluna, baixo)
                }

                // somente direita disponivel
                (Some(direita), None) => {
                    (linha, coluna + 1, direita)
                }

                 // somente baixo disponivel
                (None, Some(baixo)) => {
                    (linha + 1, coluna, baixo)
                }

                // preso
                (None, None) => {
                    return (resultado(None), iteracoes);
                }
            };

            linha = nova_linha;
            coluna = nova_coluna;

            custo_total += custo;

        }
        
        (resultado(Some(custo_total)), iteracoes)
    }
}

impl Algoritmo for Guloso {
    fn resolver(&self, entrada: &Entrada) -> (Value, usize) {
        self.resolver_com_contagem(entrada)
    }
}

