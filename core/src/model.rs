use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Nivel {
    Error,
    Warn,
    Info,
    Otro,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub nivel: Nivel,
    pub mensaje: String,
    pub timestamp: String,
}

#[derive(Debug, Default)]
pub struct Reporte {
    pub total_lineas: usize,
    pub coincidencias: usize,
    pub coincidencias_lineas: Vec<String>,
    pub por_nivel: HashMap<Nivel, usize>,
    pub por_mensaje: HashMap<String, usize>,
}
