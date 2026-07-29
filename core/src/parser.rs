use std::str::SplitWhitespace;
use crate::model::{LogEntry, Nivel};
use chrono::NaiveDateTime;

pub fn parsear_linea(linea: &str) -> Option<LogEntry> {
    //todo!("US3.1: si la línea sirve, devolvé Some con un LogEntry (nivel + mensaje)")

    let mut linea_split = linea.split_whitespace();
    let date_validated = date_time_validation_format(linea_split.clone())?;

    //Date
    if linea_split.next().is_none() {
        return None;
    }

    //Time
    if linea_split.next().is_none() {
        return None;
    }

    let level = detectar_nivel(linea.clone());

    //Level
    if linea_split.next().is_none() {
        return None;
    }

    let information = linea_split.map(|line| line.to_string() + " ").collect::<String>().to_string();

    Some(LogEntry{ nivel: level, mensaje: information, timestamp: date_validated})
}

fn date_time_validation_format(linea_split: SplitWhitespace) -> Option<String> {
    let format_date_time = "%Y-%m-%d %H:%M:%S";

    let date = linea_split.clone().collect::<Vec<&str>>().get(0)?.to_string();
    let time = linea_split.clone().collect::<Vec<&str>>().get(1)?.to_string();

    let date_validated: String;

    if NaiveDateTime::parse_from_str(&format!("{} {}", date, time), format_date_time).is_ok() {
        date_validated = format!("{} {}", date, time);
    } else {
        panic!()
    }
    Some(date_validated)
}

/// Detecta el nivel de severidad mirando el texto de la línea.
///
/// [US3.2] PISTA: `if linea.contains("ERROR") { Nivel::Error } else if ...`
pub fn detectar_nivel(linea: &str) -> Nivel {
    let mut level_string = linea.split_whitespace().filter(
        |line| line.contains("ERROR") || line.contains("INFO") || line.contains("WARN") || line.contains("OTRO")
    );

    match level_string.next().unwrap_or("") {
        "ERROR" => Nivel::Error,
        "INFO" => Nivel::Info,
        "WARN" => Nivel::Warn,
        "OTRO" => Nivel::Otro,
        _ => panic!()
    }
}
