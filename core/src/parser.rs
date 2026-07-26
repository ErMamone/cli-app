// ============================================================================
//  PARSER — de una línea de texto cruda a un LogEntry estructurado.
//  (Se usa recién en la Iteración 3. En v1 podés dejarlo con todo!())
// ============================================================================

use crate::model::{LogEntry, Nivel};

/// Intenta parsear una línea en un LogEntry.
/// Devuelve `None` si la línea está vacía o no tiene formato reconocible.
///
/// [US3.1] Ejemplo de línea a parsear:
///   "2026-07-26 08:02:45 ERROR failed to process request id=42"
pub fn parsear_linea(linea: &str) -> Option<LogEntry> {
    todo!("US3.1: si la línea sirve, devolvé Some con un LogEntry (nivel + mensaje)")
}

/// Detecta el nivel de severidad mirando el texto de la línea.
///
/// [US3.2] PISTA: `if linea.contains("ERROR") { Nivel::Error } else if ...`
pub fn detectar_nivel(linea: &str) -> Nivel {
    todo!("US3.2: devolver Nivel::Error / Warn / Info / Otro según el contenido")
}
