use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Entrada {
    pub custos: Vec<Vec<Option<u32>>>,
}

/*
matriz deve ser retangular ou quadrada
custo != 0
tipo invalido = erro
coluna vazia = erro
matriz vazia = erro
custo != none ou num = erro (serde trata)
*/
pub fn validar(entrada: &Entrada) -> Result<(), String> {
    let matriz = &entrada.custos;

    if matriz.is_empty() {
        return Err("A matriz nao pode vazia".into());
    }

    let colunas = matriz[0].len();

    if colunas == 0 {
        return Err("A matriz nao pode ter colunas vazias".into());
    }

    for linha in matriz {
        if linha.len() != colunas {
            return Err("A matriz nao pode ter linhas de tamanhos diferentes".into());
        }

        for custo in linha {
            match custo {
                Some(custo) if *custo == 0 => {
                    return Err("A matriz nao pode ter custos iguais a zero".into());
                },
                Some(_) => {}
                None => {}
            }
        }
    }

    Ok(())
}
