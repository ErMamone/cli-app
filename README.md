# logstats

Analizador de logs por línea de comandos, escrito en Rust como proyecto de
aprendizaje. Crece por iteraciones, cada una ejercitando una fortaleza distinta
del lenguaje.

## Arquitectura

Workspace de Cargo con separación **motor puro / front-end fino**:

```
cli-app/
├── core/   → librería logstats-core: lógica pura (texto -> Reporte). Sin I/O ni hilos.
└── cli/    → binario logstats: lee archivos, orquesta, imprime. Usa core.
```

Regla de oro: la lógica va en `core`; la I/O y la concurrencia en los bordes (`cli`).
Así el mismo `core` se reusa mañana desde WASM o una web sin tocarlo.

## Roadmap

| Iteración | Fortaleza | Objetivo |
|-----------|-----------|----------|
| **v1** | CLI | leer un archivo, filtrar por texto, contar líneas |
| **v2** | Concurrencia | procesar un directorio de logs en paralelo |
| **v3** | Data processing | parsear líneas y sacar estadísticas (por nivel, top-N, JSON) |
| v4 *(futuro)* | WASM | correr el motor en el navegador |
| v5 *(futuro)* | Web API | exponer el motor por HTTP |

### Historias de usuario — v1
- **US1.1** — pasar la ruta de un log como argumento y que lo lea.
- **US1.2** — filtrar líneas que contengan un texto (`--filtro ERROR`).
- **US1.3** — mostrar totales (líneas totales y coincidentes).
- **US1.4** — errores claros y exit code != 0 (sin panics feos).

## Uso

```bash
cargo run -- samples/server.log --filtro ERROR
```

(El `--` separa los args de cargo de los de tu programa.)

## Estado

🚧 En construcción. Las funciones marcadas con `todo!()` están pendientes de implementar.
