// ============================================================================
//  STATS — agregaciones sobre reportes.
// ============================================================================

use crate::model::Reporte;

/// Combina varios reportes (de distintos archivos) en uno solo.
/// Se usa en la Iteración 2, cuando procesás muchos archivos en paralelo y
/// tenés que juntar los resultados parciales.
///
/// [US2.3] PISTA: partí de `Reporte::default()` y sumá total_lineas,
/// coincidencias y fusioná los HashMap con el idiom `entry().or_insert()`.
pub fn combinar(reportes: Vec<Reporte>) -> Reporte {
    todo!("US2.3: fusionar todos los reportes en uno solo")
}

/// Devuelve los `n` mensajes más frecuentes, de mayor a menor.
///
/// [US3.3] PISTA: pasá `reporte.por_mensaje` a un Vec, ordenalo por la cantidad
/// (`sort_by_key` + `.rev()`), y quedate con los primeros `n` (`.take(n)`).
pub fn top_mensajes(reporte: &Reporte, n: usize) -> Vec<(String, usize)> {
    todo!("US3.3: ordenar por frecuencia y tomar los primeros n")
}
