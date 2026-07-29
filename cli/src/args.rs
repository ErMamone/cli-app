use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "logstats", version, about = "Analiza archivos de log")]
pub struct Args {
    pub ruta: String,

    #[arg(short, long)]
    pub filtro: Option<String>,

    #[arg(long)]
    pub json: bool,
}
