// ============================================================================
//  logstats-core — EL MOTOR
// ============================================================================
//  Este crate es una LIBRERÍA pura: recibe texto y devuelve datos (Reporte).
//  No sabe qué es un archivo, ni un hilo, ni una terminal. Esa es la razón por
//  la que después lo vas a poder reusar tal cual desde WASM y desde una web.
//
//  La regla: si estás por escribir `std::fs`, `println!` o `thread` acá,
//  pará — eso va en el crate `cli`, no en `core`.
// ============================================================================

pub mod model;
pub mod parser;
pub mod stats;

// Re-exportamos los tipos principales para que el que use la librería escriba
// `logstats_core::Reporte` en vez de `logstats_core::model::Reporte`.
pub use model::{LogEntry, Nivel, Reporte};

/// Analiza el contenido completo de un log y devuelve un Reporte.
///
/// - `texto`: el contenido del archivo entero.
/// - `filtro`: si es `Some(patrón)`, solo cuentan las líneas que lo contienen.
///
/// [US1.3] versión mínima: contar `total_lineas` y `coincidencias`.
/// [US3.x] después: parsear cada línea y llenar `por_nivel` / `por_mensaje`.
pub fn analizar(texto: &str, filtro: Option<&str>) -> Reporte {
    todo!("US1.3: recorrer texto.lines(), contar totales y coincidencias con el filtro")
}
