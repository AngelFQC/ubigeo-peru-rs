mod reniec;
mod inei;

#[derive(Debug)]
pub struct Ubigeo {
    pub departamento: &'static str,
    pub provincia: &'static str,
    pub distrito: &'static str,
    pub nombre: &'static str
}

pub static RENIEC: &[Ubigeo] = reniec::RENIEC;
pub static INEI: &[Ubigeo] = inei::INEI;
