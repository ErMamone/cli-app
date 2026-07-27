// ============================================================================
//  RUNNER — la capa de I/O y orquestación.
//  ACÁ (y solo acá) se leen archivos y, en v2, se usan hilos. El runner lee la
//  entrada, se la pasa a `logstats_core::analizar`, y devuelve el Reporte.
// ============================================================================

use crate::error::AppError;
use logstats_core::Reporte;
use std::path::PathBuf;
use std::thread;

/// Lee UN archivo y lo analiza. (US1.1 + US1.3 + US1.4)
///
/// PISTA:
///   - Leé el archivo con `std::fs::read_to_string(ruta)`.
///   - Eso devuelve un `Result`; convertí su error a `AppError` (con `map_err`
///     o un `match`) para no filtrar el error crudo de std.
///   - Con el texto, llamá a `logstats_core::analizar(&texto, filtro)`.
pub fn analizar_archivo(ruta: &str, filtro: Option<&str>) -> Result<Reporte, AppError> {
    let texto = std::fs::read_to_string(ruta).map_err(|e| AppError::Lectura(e.to_string()))?;

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
    let rutas: Vec<PathBuf> = std::fs::read_dir(_dir)
        .map_err(|e| AppError::Lectura(e.to_string()))?
        .filter_map(|entrada| entrada.ok()) 
        .map(|entrada| entrada.path()) 
        .filter(|ruta| ruta.extension().map(|ext| ext == "log").unwrap_or(false))
        .collect();

    let mut handles = Vec::new();

    for ruta in rutas {
        let filtro_propio: Option<String> = _filtro.map(|s| s.to_string());

        let handle = thread::spawn(move || {
            let texto = std::fs::read_to_string(&ruta).unwrap_or_default();
            logstats_core::analizar(&texto, filtro_propio.as_deref())
        });
        handles.push(handle);
    }

    let reportes: Vec<Reporte> = handles
        .into_iter()
        .map(|h| h.join().expect("thread panic"))
        .collect();

    Ok(logstats_core::stats::combinar(reportes))
}
