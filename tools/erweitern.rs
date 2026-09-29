use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

const GRID_SIZE: usize = 16;
const TOTAL_PIXELS: usize = GRID_SIZE * GRID_SIZE; // 256

struct DataRow {
    label: u8,
    grid_data: Vec<u8>,
}

fn main() {
    println!("#######-----DATA AUGMENTATION GESTARTET-----#######");

    // 1. Quelldaten einlesen
    let dataset = load_csv("zahlen_hand.csv");
    if dataset.is_empty() {
        println!("Fehler: 'zahlen_hand.csv' konnte nicht geöffnet werden oder ist leer.");
        return;
    }
    println!("-> {} Ursprungsmuster geladen.", dataset.len());

    // 2. Neue Ausgabedatei vorbereiten
    let write_file =
        File::create("zahlen_erweitert.csv").expect("Kann Ausgabedatei nicht erstellen");
    let mut writer = BufWriter::new(write_file);

    let mut count_original = 0;
    let mut count_augmented = 0;

    // 3. Transformation und Generierung
    for row in &dataset {
        // Schritt A: Das unveränderte Original in die Datei schreiben
        write_row(&mut writer, row);
        count_original += 1;

        // Schritt B: Die Erweiterung (Dilatation) berechnen
        let augmented_row = dilate_left_and_up(row);
        write_row(&mut writer, &augmented_row);
        count_augmented += 1;
    }

    // Speicher-Buffer physisch auf die Festplatte schreiben
    writer.flush().unwrap();

    println!("============================================================");
    println!("  Erweiterung erfolgreich abgeschlossen!");
    println!("  - Originale exportiert:         {}", count_original);
    println!("  - Erweiterte Muster exportiert: {}", count_augmented);
    println!("  ----------------------------------------------------------");
    println!(
        "  Gesamte Zeilen in neuer CSV:    {}",
        count_original + count_augmented
    );
    println!("============================================================");
}

/// Transformiert das Muster, indem für jedes aktive Pixel (1)
/// zusätzlich das linke und das obere Nachbarpixel auf 1 gesetzt werden.
fn dilate_left_and_up(source: &DataRow) -> DataRow {
    // Wir klonen das originale Gitter als Basis, damit bestehende Pixel erhalten bleiben
    let mut new_grid = source.grid_data.clone();

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let current_idx = y * GRID_SIZE + x;

            // Prüfen, ob das aktuelle Pixel im Original-Datensatz aktiv (1) war
            if source.grid_data[current_idx] == 1 {
                // 1. Pixel LINKS dazusetzen (falls wir nicht am linken Rand x == 0 sind)
                if x > 0 {
                    let left_idx = y * GRID_SIZE + (x - 1);
                    new_grid[left_idx] = 1;
                }

                // 2. Pixel DARÜBER dazusetzen (falls wir nicht am oberen Rand y == 0 sind)
                if y > 0 {
                    let up_idx = (y - 1) * GRID_SIZE + x;
                    new_grid[up_idx] = 1;
                }
            }
        }
    }

    DataRow {
        label: source.label,
        grid_data: new_grid,
    }
}

/// Schreibt eine 'DataRow'-Struktur im schnellen, sequentiellen CSV-Format in den Stream
fn write_row(writer: &mut BufWriter<File>, row: &DataRow) {
    write!(writer, "{}", row.label).unwrap();
    for pixel in &row.grid_data {
        write!(writer, ",{}", pixel).unwrap();
    }
    writeln!(writer).unwrap();
}

/// Optimierter CSV-Reader aus den vorherigen Schritten
fn load_csv(path: &str) -> Vec<DataRow> {
    let file = if let Ok(f) = File::open(path) {
        f
    } else {
        return Vec::new();
    };
    let reader = BufReader::new(file);
    let mut dataset = Vec::new();

    for line_text in reader.lines().map_while(Result::ok) {
        let trimmed = line_text.trim();
        if trimmed.is_empty() {
            continue;
        }

        let parts: Vec<_> = trimmed.split(',').collect();

        if parts.len() == TOTAL_PIXELS + 1 {
            let label = parts[0].parse().unwrap_or(0);
            let grid_data = parts[1..].iter().map(|p| p.parse().unwrap_or(0)).collect();

            dataset.push(DataRow { label, grid_data });
        }
    }
    dataset
}
