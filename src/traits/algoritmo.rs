use serde_json::Value;

use crate::entrada::Entrada;

pub trait Algoritmo {
    fn resolver(&self, entrada: &Entrada) -> (Value, usize);
}
