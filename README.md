Códigos de Ubicación Geográfica del Perú
========================================

# Ubigeo

Este es un port en Rust de [AngelFQC/ubigeo-peru](https://github.com/AngelFQC/ubigeo-peru).

> El Ubigeo es el identificador numérico único que se asigna a cada ámbito político administrativo del país para identificar al departamento, provincia y distrito.
> INEI

# Uso

```rust
use ubigeo_peru;

fn main() {
    let ubigeo = ubigeo_peru::RENIEC
        .iter()
        .find(|u| u.departamento == "13" && u.provincia == "01" && u.distrito == "01")
        .expect("UBIGEO no encontrado");

    println!("{}{}{}", ubigeo.departamento, ubigeo.provincia, ubigeo.distrito);
    println!("{}", ubigeo.nombre);
}

// 130101
// Chiclayo

```
