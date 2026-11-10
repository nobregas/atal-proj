use crate::traits::algoritmo::Algoritmo;
use serde_json::json;
use super::json_util::transformar_entrada;

pub(crate) fn validar_custo(json: &str, algoritmo: &impl Algoritmo, esperado: u32) {
    let entrada = transformar_entrada(json);

    assert_eq!(
        algoritmo.resolver(&entrada).0,
        json!({"status": "ok", "custo": esperado}),
        "Custo incorreto para entrada: {json}"
    );
}

pub(crate) fn validar_sem_caminho(json: &str, algoritmo: &impl Algoritmo) {
    let entrada = transformar_entrada(json);

    assert_eq!(
        algoritmo.resolver(&entrada).0,
        json!({"status": "sem_caminho", "custo": null}),
        "Deveria retornar sem caminho: {json}"
    );
}

