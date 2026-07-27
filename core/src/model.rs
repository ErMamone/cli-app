// ============================================================================
//  MODELO — los tipos de datos que maneja el motor.
// ============================================================================

use std::collections::HashMap;

/// Nivel de severidad de una línea de log.
/// Deriva Eq + Hash para poder usarse como CLAVE de un HashMap (en `por_nivel`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Nivel {
    Error,
    Warn,
    Info,
    Otro, // cualquier línea que no reconozcamos
}

/// Una línea de log ya parseada en campos.
/// [US3.1] cuando parsees fechas, agregá acá un campo `timestamp`.
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub nivel: Nivel,
    pub mensaje: String,
}

/// El resultado del análisis de uno o varios logs.
/// Deriva Default para poder crear un Reporte vacío con `Reporte::default()`
/// (todos los contadores en 0 y los HashMap vacíos).
#[derive(Debug, Default)]
pub struct Reporte {
    pub total_lineas: usize,
    pub coincidencias: usize,
    pub coincidencias_lineas: Vec<String>,    // [US1.2] las líneas que matchearon
    pub por_nivel: HashMap<Nivel, usize>,     // [US3.2]
    pub por_mensaje: HashMap<String, usize>,  // [US3.3]
}
