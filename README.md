## 1. Preparar o ambiente

1. Instale o Rust com o `rustup` pela [página oficial](https://rust-lang.org/tools/install/),

```powershell
rustc --version
cargo --version
```

Compile o projeto:

```powershell
cargo build
```

## 2. Executar os testes

execute:

```powershell
cargo test
```

Para ver o print das Medicoes:

```powershell
cargo test -- --nocapture --test-threads=1
```

Para executar somente os testes de um algoritmo:

```powershell
cargo test testes_recursivo -- --nocapture --test-threads=1
cargo test testes_guloso -- --nocapture --test-threads=1
```

## 4. Mostrar as medições dos dois algoritmos

```powershell
cargo test medir_ -- --nocapture --test-threads=1
```