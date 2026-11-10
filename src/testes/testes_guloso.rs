use crate::guloso::Guloso;
use crate::traits::algoritmo::Algoritmo;
use super::util::validacoes_util::{validar_custo, validar_sem_caminho};

#[test]
fn teste_matriz_exemplo() {
    let json = r#"{"custos": [[2, 1, 9, 9, 9],[3, null, 1, 1, 9],[1, 1, 1, null, 2],[9, 9, 1, 1, 1]]}"#;

    validar_custo(json, &Guloso, 26);
}

#[test]
fn teste_matriz_4x4() {
    let json = r#"{"custos": [[2, null, 3, 1], [1, null, 4, 5], [3, 1, 2, 2], [2, 5, 1, 3]]}"#;

    validar_custo(json, &Guloso, 13);
}

#[test]
fn teste_sem_caminho() {
    for json in [
        r#"{"custos": [[null, 1], [1, 2]]}"#,
        r#"{"custos": [[1, 1], [1, null]]}"#,
        r#"{"custos": [[1, null], [null, 2]]}"#,
    ] {
        validar_sem_caminho(json, &Guloso);
    }
}

#[test]
fn medir_iteracoes_guloso() {
    for (linhas, colunas, esperado) in [
        (2, 3, 3),
        (3, 3, 4),
        (6, 6, 10),
        (10, 10, 18),
        (1, 10, 9),
        (1, 1, 0),
    ] {
        let entrada = crate::entrada::Entrada {
            custos: vec![vec![Some(1); colunas]; linhas],
        };
        let (resultado, iteracoes) = Guloso.resolver(&entrada);
        assert_eq!(iteracoes, esperado, "Grade {linhas}x{colunas}");
        assert_eq!(resultado, serde_json::json!({"status": "ok", "custo": linhas + colunas - 1}));
        println!("Guloso | Grade {linhas}x{colunas}: 1 chamada de resolver; {iteracoes} iteracoes; {resultado}");
    }

    let entrada = serde_json::from_str(
        r#"{"custos": [[2, 1, 9, 9, 9], [3, null, 1, 1, 9], [1, 1, 1, null, 2], [9, 9, 1, 1, 1]]}"#,
    ).unwrap();
    let (resultado, iteracoes) = Guloso.resolver(&entrada);

    assert_eq!(iteracoes, 7);
    assert_eq!(resultado, serde_json::json!({"status": "ok", "custo": 26}));
    println!("Guloso | Grade do exemplo (4x5): 1 chamada de resolver; {iteracoes} iteracoes; {resultado}");

    for (json, esperado) in [
        (r#"{"custos": [[null]]}"#, 0),
        (r#"{"custos": [[1, null], [null, 2]]}"#, 1),
    ] {
        let entrada = serde_json::from_str(json).unwrap();
        let (resultado, iteracoes) = Guloso.resolver(&entrada);

        assert_eq!(iteracoes, esperado);
        assert_eq!(resultado, serde_json::json!({"status": "sem_caminho", "custo": null}));
    }
}
