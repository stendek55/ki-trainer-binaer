use ki_trainer_binaer::*;
fn main() {
    println!("Hello, world!");
    // lädt den ersten datensatz aus deiner csv-datei
    let foobar = lade_test_datensatz(1);

    println!("--------------------------------------------------");
    println!("visualisierung des geladenen bildes (16x16 raster):");
    println!("--------------------------------------------------");

    // wir gehen durch alle 32 BitBytes im eingangs-array
    for (index, bit_byte) in foobar.input.iter().enumerate() {
        // wir greifen auf die innere zahl (.0) zu und drucken sie als 8 bits aus
        print!("{:08b}", bit_byte.0);

        // da das bild 16 pixel breit ist, machen wir nach jedem zweiten byte (16 bits) einen zeilenumbruch
        if (index + 1) % 2 == 0 {
            println!();
        }
    }

    println!("--------------------------------------------------");
}
