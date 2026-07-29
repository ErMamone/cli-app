use std::fmt;

#[derive(Debug)]
pub enum AppError {
    ArchivoNoEncontrado(String),
    Lectura(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ArchivoNoEncontrado(ruta) => {
                write!(f, "no se encontró el archivo o ruta: {ruta}")
            }
            AppError::Lectura(msg) => write!(f, "no se pudo leer: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}
