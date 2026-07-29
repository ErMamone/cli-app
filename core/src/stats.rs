use crate::model::Reporte;

pub fn combinar(reportes: Vec<Reporte>) -> Reporte {
    let mut total = Reporte::default();

    for r in reportes {
        total.total_lineas += r.total_lineas;
        total.coincidencias += r.coincidencias;
        total.coincidencias_lineas.extend(r.coincidencias_lineas);

    }

    total
}

/// Devuelve los `n` mensajes más frecuentes, de mayor a menor.
///
/// [US3.3] PISTA: pasá `reporte.por_mensaje` a un Vec, ordenalo por la cantidad
/// (`sort_by_key` + `.rev()`), y quedate con los primeros `n` (`.take(n)`).
pub fn top_mensajes(reporte: &Reporte, n: usize) -> Vec<(String, usize)> {
    todo!("US3.3: ordenar por frecuencia y tomar los primeros n")
}
