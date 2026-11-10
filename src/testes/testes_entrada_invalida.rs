use crate::entrada::{Entrada, validar};

fn validar_entrada_invalida(json: &str) {
    let resultado = serde_json::from_str::<Entrada>(json)
        .map_err(|erro| erro.to_string())
        .and_then(|entrada| validar(&entrada));

    assert!(
        resultado.is_err(),
        "Deveria retornar erro de entrada invalida: {json}"
    );
}

#[test]
fn erro_matriz_vazia() {
    let json = r#"{"custos": [[]]}"#;

    validar_entrada_invalida(json);
}

#[test] // tipo do custo invalido
fn erro_tipo_custo_invalido() {
    let json = r#"{"custos": [[true, 1], ["1", 1], [1.5], 1]]}"#;

    validar_entrada_invalida(json);
}

#[test] // custo negativo
fn erro_custo_negativo() {
    let json = r#"{"custos": [[1, -1]]}"#;

    validar_entrada_invalida(json);
}

#[test] // custo 0
fn erro_custo_zero() {
    let json = r#"{"custos": [[1, 0]]}"#;

    validar_entrada_invalida(json);
}

#[test] //chave errada
fn erro_chave_invalida() {
    let json = r#"{"matriz": [[1]]}"#;

    validar_entrada_invalida(json);
}

