// ============================================================================
//  ERROR — el tipo de error propio de la CLI (US1.4).
//  En vez de usar unwrap() y explotar, las funciones devuelven Result<_, AppError>
//  y acá definimos qué puede salir mal y cómo se muestra.
// ============================================================================

use std::fmt;

/// Errores que puede producir la aplicación.
#[derive(Debug)]
pub enum AppError {
    ArchivoNoEncontrado(String),
    Lectura(String),
}

// Implementamos Display para que el mensaje al usuario sea legible.
// (esto es lo que se imprime con `{e}` en main.rs)
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

// Con esto AppError se comporta como un "error de verdad" de Rust.
impl std::error::Error for AppError {}
