use crate::error::AppError;
use logstats_core::Reporte;
use std::path::PathBuf;
use std::thread;

pub fn analizar_archivo(ruta: &str, filtro: Option<&str>) -> Result<Reporte, AppError> {
    let texto = std::fs::read_to_string(ruta).map_err(|e| AppError::Lectura(e.to_string()))?;

    Ok(logstats_core::analizar(&texto, filtro))
}

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
