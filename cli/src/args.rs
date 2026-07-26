// ============================================================================
//  ARGS — definición de los argumentos de línea de comandos con `clap`.
//  El #[derive(Parser)] genera TODO el parsing y el --help automáticamente.
// ============================================================================

use clap::Parser;

/// Analizador de logs por línea de comandos.
#[derive(Parser, Debug)]
#[command(name = "logstats", version, about = "Analiza archivos de log")]
pub struct Args {
    /// Ruta al archivo de log a analizar (en v2, también un directorio).
    pub ruta: String,

    /// Muestra/cuenta solo las líneas que contengan este texto.
    #[arg(short, long)]
    pub filtro: Option<String>,

    // v3 (US3.4): salida en JSON.
    // #[arg(long)]
    // pub json: bool,
}
