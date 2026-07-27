// ============================================================================
//  RUNNER — la capa de I/O y orquestación.
//  ACÁ (y solo acá) se leen archivos y, en v2, se usan hilos. El runner lee la
//  entrada, se la pasa a `logstats_core::analizar`, y devuelve el Reporte.
// ============================================================================

use crate::error::AppError;
use logstats_core::Reporte;

/// Lee UN archivo y lo analiza. (US1.1 + US1.3 + US1.4)
///
/// PISTA:
///   - Leé el archivo con `std::fs::read_to_string(ruta)`.
///   - Eso devuelve un `Result`; convertí su error a `AppError` (con `map_err`
///     o un `match`) para no filtrar el error crudo de std.
///   - Con el texto, llamá a `logstats_core::analizar(&texto, filtro)`.
pub fn analizar_archivo(ruta: &str, filtro: Option<&str>) -> Result<Reporte, AppError> {
    let texto = std::fs::read_to_string(ruta)
        .map_err(|e| AppError::Lectura(e.to_string()))?;

    Ok(logstats_core::analizar(&texto, filtro))
}

/// v2: lee TODOS los .log de un directorio EN PARALELO y combina los reportes.
///
/// PISTA (para cuando llegues a la Iteración 2):
///   - Listá el directorio con `std::fs::read_dir`.
///   - Quedate con los que terminan en ".log".
///   - Procesalos en paralelo (rayon `.par_iter()` o `thread::spawn` + canales).
///   - Juntá los resultados con `logstats_core::stats::combinar`.
pub fn analizar_directorio(_dir: &str, _filtro: Option<&str>) -> Result<Reporte, AppError> {
    todo!("US2.1/2.2/2.3: listar .log, procesar en paralelo y combinar")
}
