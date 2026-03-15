use anyhow::Result;
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("gzdoom-rs starting up");

    // Phase 1 milestone: load an IWAD and print its lump count.
    let args: Vec<String> = std::env::args().collect();
    if let Some(iwad_path) = args.get(1) {
        info!("Loading IWAD: {}", iwad_path);
        let data = std::fs::read(iwad_path)?;
        let wad = gz_filesystem::wad::WadReader::from_bytes(&data, 0)?;
        info!(
            "Loaded {} ({} lumps)",
            if wad.is_iwad { "IWAD" } else { "PWAD" },
            wad.lumps.len()
        );
        for lump in wad.lumps.iter().take(20) {
            info!("  {:8}  {} bytes", lump.name, lump.size);
        }
    } else {
        eprintln!("Usage: gzdoom <path/to/doom.wad>");
    }

    Ok(())
}
