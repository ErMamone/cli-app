// ============================================================================
//  logstats CLI — punto de entrada.
//  Este archivo ORQUESTA: lee args, llama al runner, imprime el resultado.
//  La lógica de verdad vive en `logstats-core`; acá solo pegamos las piezas.
// ============================================================================

mod args;
mod error;
mod runner;

use args::Args;
use clap::Parser;
use std::process::ExitCode;

// main devuelve ExitCode para poder cortar con código != 0 ante un error (US1.4).
fn main() -> ExitCode {
    let args = Args::parse();

    // v1: un solo archivo. En v2 acá decidirás archivo vs. directorio.
    let resultado = runner::analizar_archivo(&args.ruta, args.filtro.as_deref());

    match resultado {
        Ok(reporte) => {
            // [US1.3] TODO: reemplazá este print por una salida linda
            // (total de líneas, coincidencias, y en v3 el desglose por nivel).
            println!("{reporte:?}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Error: {e}"); // usa tu Display de AppError (US1.4)
            ExitCode::FAILURE
        }
    }
}
