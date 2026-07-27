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

use std::fmt::Debug;
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
    let total_lineas = texto.lines().count();

    // Junto las líneas que contienen el filtro. Si no hay filtro, va vacío.
    let coincidencias_lineas: Vec<String> = match filtro {
        Some(patron) => texto
            .lines()
            .filter(|linea| linea.contains(patron))
            .map(|linea| linea.to_string())
            .collect(),
        None => Vec::new(),
    };

    Reporte {
        total_lineas,
        coincidencias: coincidencias_lineas.len(), // el conteo sale del Vec
        coincidencias_lineas,
        ..Default::default() // rellena por_nivel y por_mensaje con sus defaults
    }
}
