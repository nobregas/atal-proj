use serde_json::{Value, json};

pub fn erro(mensagem: &str) -> Value {
    json!({"status": "entrada_invalida", "erro": mensagem})
}

pub fn resultado(custo: Option<u32>) -> Value {
    match custo {
        Some(custo) => json!({"status": "ok", "custo": custo}),
        None => json!({"status": "sem_caminho", "custo": null}),
    }
}