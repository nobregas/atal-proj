use crate::entrada::{Entrada, validar};

pub(super) fn transformar_entrada(json: &str) -> Entrada {
    let entrada: Entrada = serde_json::from_str(json).expect("Falha ao transformar a entrada");

    validar(&entrada).expect("Entrada invalida");
    
    entrada
}
