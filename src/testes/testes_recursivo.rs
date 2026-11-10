use crate::recursivo::Recursivo;
use crate::traits::algoritmo::Algoritmo;
use super::util::validacoes_util::{validar_custo, validar_sem_caminho};

#[test]
fn teste_matriz_1x1() {
    let json = r#"{"custos": [[7]]}"#;

    validar_custo(json, &Recursivo, 7);
}
#[test]
fn teste_matriz_1x3() {
    let json = r#"{"custos": [[1], [2], [3]]}"#;

    validar_custo(json, &Recursivo, 6);
}

#[test]
fn teste_matriz_2x3() {
    let json = r#"{"custos": [[1, 1, 1], [1, 1, 1]]}"#;

    validar_custo(json, &Recursivo, 4);
}

#[test]
fn teste_matriz_exemplo() {
    let json = r#"{"custos": [[2, 1, 9, 9, 9],[3, null, 1, 1, 9],[1, 1, 1, null, 2],[9, 9, 1, 1, 1]]}"#;

    validar_custo(json, &Recursivo, 11);
}

#[test]
fn teste_matriz_3x3() {
    let json = r#"{"custos": [[1, 1, null],[2, null, 1],[1, 1, 1]]}"#;

    validar_custo(json, &Recursivo, 6);
}

#[test]
fn teste_sem_caminho() {
    let json = r#"{"custos": [[null]]}"#;

    validar_sem_caminho(json, &Recursivo);
}

#[test]
fn teste_matriz_linha_unica() {
    let json = r#"{"custos": [[2, 4, 1, 3]]}"#;

    validar_custo(json, &Recursivo, 10);
}

#[test]
fn teste_matriz_menor_custo_total() {
    let json = r#"{"custos": [[1, 1, 50], [5, 50, 50], [1, 1, 1]]}"#;

    validar_custo(json, &Recursivo, 9);
}

#[test]
fn teste_matriz_barreira_sem_caminho() {
    let json = r#"{"custos": [[1, 2, 3], [null, null, null], [4, 5, 6]]}"#;

    validar_sem_caminho(json, &Recursivo);
}

#[test]
fn medir_chamadas_recursivo() {
    for (linhas, colunas, esperado) in [
        (2, 3, 9),
        (3, 3, 19),
        (6, 6, 923),
        (10, 10, 184755),
        (1, 10, 10),
    ] {
        let entrada = crate::entrada::Entrada {
            custos: vec![vec![Some(1); colunas]; linhas],
        };
        let (resultado, chamadas) = Recursivo.resolver(&entrada);

        assert_eq!(chamadas, esperado, "Grade {linhas}x{colunas}");
        assert_eq!(resultado, serde_json::json!({"status": "ok", "custo": linhas + colunas - 1}));
        
        println!("Recursivo | Grade {linhas}x{colunas}: {chamadas} chamadas de movimentar; {resultado}");
    }

    let entrada = serde_json::from_str(
        r#"{"custos": [[2, 1, 9, 9, 9], [3, null, 1, 1, 9], [1, 1, 1, null, 2], [9, 9, 1, 1, 1]]}"#,
    ).unwrap();
    let (resultado, chamadas) = Recursivo.resolver(&entrada);

    assert_eq!(chamadas, 43);
    assert_eq!(resultado, serde_json::json!({"status": "ok", "custo": 11}));
    println!("Recursivo | Grade do exemplo (4x5): {chamadas} chamadas de movimentar; {resultado}");
}
