use std::fs;
use ts_rs::ExportError;

pub struct TsExporter {
    pub export_fn: fn() -> Result<String, ExportError>,
}

inventory::collect!(TsExporter);

pub fn export_all_types(output_file: &str) -> Result<(), ExportError> {
    let types: Vec<String> = inventory::iter::<TsExporter>
        .into_iter()
        .filter_map(|exporter| (exporter.export_fn)().ok())
        .collect();

    fs::write(output_file, types.join("\n"))?;
    Ok(())
} 