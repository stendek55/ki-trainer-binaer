use crate::BitNeuralNetwork;
use std::fs::File;
use std::io::{Read, Write};

/// Speichert das trainierte Netzwerk als JSON-Datei auf die Festplatte
pub fn save_champion(network: &BitNeuralNetwork, path: &str) -> std::io::Result<()> {
    let json_text = serde_json::to_string_pretty(network).map_err(std::io::Error::other)?;

    let mut datei = File::create(path)?;
    datei.write_all(json_text.as_bytes())?;

    println!("Champion erfolgreich gespeichert!");
    Ok(())
}

/// Lädt ein zuvor gespeichertes Netzwerk von der Festplatte
pub fn load_champion(path: &str) -> std::io::Result<BitNeuralNetwork> {
    let mut datei = File::open(path)?;
    let mut json_text = String::new();
    datei.read_to_string(&mut json_text)?;

    let netzwerk: BitNeuralNetwork =
        serde_json::from_str(&json_text).map_err(std::io::Error::other)?;

    println!("Champion erfolgreich geladen!");
    Ok(netzwerk)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_speichern_und_laden_des_champions_erfolgreich() {
        // erstelle ein echtes test netzwerk aus deiner library
        let original_netzwerk = BitNeuralNetwork::new_random();
        let datei_pfad = "test_champion_loesch_mich.json"; // fester dateiname zum testen

        // speichere das netzwerk ab
        let speicher_ergebnis = save_champion(&original_netzwerk, datei_pfad);
        assert!(
            speicher_ergebnis.is_ok(),
            "Das Speichern des Champions ist fehlgeschlagen"
        );

        // lade das netzwerk wieder ein
        let geladenes_ergebnis = load_champion(datei_pfad);
        assert!(
            geladenes_ergebnis.is_ok(),
            "Das Laden des Champions ist fehlgeschlagen"
        );

        let geladenes_netzwerk = geladenes_ergebnis.unwrap();

        // ueberpruefe die gleichheit
        assert_eq!(
            original_netzwerk, geladenes_netzwerk,
            "Das geladene Netzwerk unterscheidet sich vom Original!"
        );

        // loesche die datei manuell damit der ordner sauber bleibt
        let _ = std::fs::remove_file(datei_pfad);
    }
}
