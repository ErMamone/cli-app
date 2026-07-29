// ============================================================================
//  logstats CLI — punto de entrada.
//  Este archivo ORQUESTA: lee args, llama al runner, imprime el resultado.
//  La lógica de verdad vive en `logstats-core`; acá solo pegamos las piezas.
// ============================================================================

mod args;
mod error;
mod runner;

use crate::error::AppError;
use args::Args;
use clap::Parser;
use logstats_core::Reporte;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Args::parse();
    let path = Path::new(&args.ruta);

    if path.is_file() {
        is_file_process(&args)
    } else {
        if_dir_process(&args)
    }
}

fn if_dir_process(args: &Args) -> ExitCode {
    let resultado = runner::analizar_directorio(&args.ruta, args.filtro.as_deref());

    resultado_reporte(args, resultado)
}

fn is_file_process(args: &Args) -> ExitCode {
    let resultado = runner::analizar_archivo(&args.ruta, args.filtro.as_deref());

    resultado_reporte(args, resultado)
}

fn resultado_reporte(args: &Args, resultado: Result<Reporte, AppError>) -> ExitCode {
    match resultado {
        Ok(reporte) => {
            for linea in &reporte.coincidencias_lineas {
                println!("{linea}");
            }

            println!("---");
            if args.filtro.is_some() {
                println!(
                    "{} líneas totales, {} coincidencias",
                    reporte.total_lineas, reporte.coincidencias
                );
            } else {
                println!("{} líneas totales", reporte.total_lineas);
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}
