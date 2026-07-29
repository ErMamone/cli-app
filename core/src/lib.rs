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

use std::collections::HashMap;
use std::fmt::Debug;
// Re-exportamos los tipos principales para que el que use la librería escriba
// `logstats_core::Reporte` en vez de `logstats_core::model::Reporte`.
pub use model::{LogEntry, Nivel, Reporte};
use crate::parser::{detectar_nivel, parsear_linea};

/// Analiza el contenido completo de un log y devuelve un Reporte.
///
/// - `texto`: el contenido del archivo entero.
/// - `filtro`: si es `Some(patrón)`, solo cuentan las líneas que lo contienen.
///
/// [US1.3] versión mínima: contar `total_lineas` y `coincidencias`.
/// [US3.x] después: parsear cada línea y llenar `por_nivel` / `por_mensaje`.
pub fn analizar(texto: &str, filtro: Option<&str>) -> Reporte {
    let total_lineas = texto.lines().count();

    let coincidencias_lineas: Vec<String> = match filtro {
        Some(patron) => texto
            .lines()
            .filter(|linea| linea.contains(patron))
            .map(|linea| linea.to_string())
            .collect(),
        None => Vec::new(),
    };

    let mut por_nivel: HashMap<Nivel, usize> = HashMap::new();
    for linea in texto.lines() {
        if let Some(entry) = parsear_linea(linea) {
            *por_nivel.entry(entry.nivel).or_insert(0) += 1;
        }
    }

    Reporte {
        total_lineas,
        coincidencias: coincidencias_lineas.len(),
        coincidencias_lineas,
        por_nivel,
        por_mensaje: parsear_linea()
    }
}
