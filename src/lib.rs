/// Ein Wrapper für u8, der komfortable Bitoperationen per Punktoperator erlaubt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitByte(pub u8);

impl BitByte {
    // --- Konstruktor ---
    pub fn new(val: u8) -> Self {
        BitByte(val)
    }

    // --- Zu entwickelnde Methoden (Produktivcode) ---

    // Gibt den inneren u8-Wert zurück
    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn bitwise_and(&self, _other: BitByte) -> Self {
        todo!("Implementiere AND")
    }

    pub fn bitwise_or(&self, _other: BitByte) -> Self {
        todo!("Implementiere OR")
    }

    pub fn bitwise_xor(&self, _other: BitByte) -> Self {
        todo!("Implementiere XOR")
    }

    pub fn bitwise_not(&self) -> Self {
        todo!("Implementiere NOT")
    }

    pub fn shift_left(&self, _positions: u32) -> Self {
        todo!("Implementiere Shift Left")
    }

    pub fn shift_right(&self, _positions: u32) -> Self {
        todo!("Implementiere Shift Right")
    }

    // Nutzen `mut self` für Method Chaining (geben verändertes Self zurück)
    pub fn set_bit(mut self, _index: u8) -> Self {
        todo!("Setze Bit")
    }

    pub fn clear_bit(mut self, _index: u8) -> Self {
        todo!("Lösche Bit")
    }

    pub fn toggle_bit(mut self, _index: u8) -> Self {
        todo!("Invertiere Bit")
    }

    // Gibt einen bool zurück, bricht die Kette auf
    pub fn check_bit(&self, _index: u8) -> bool {
        todo!("Prüfe Bit")
    }
}

// --- TDD Testumgebung mit Punktnotation (Vollständige Version) ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        // 0b1100 & 0b1010 = 0b1000
        assert_eq!(
            BitByte::new(0b0000_1100).bitwise_and(BitByte::new(0b0000_1010)),
            BitByte::new(0b0000_1000)
        );
        assert_eq!(
            BitByte::new(0xFF).bitwise_and(BitByte::new(0x00)),
            BitByte::new(0x00)
        );
    }

    #[test]
    fn test_or() {
        // 0b1100 | 0b1010 = 0b1110
        assert_eq!(
            BitByte::new(0b0000_1100).bitwise_or(BitByte::new(0b0000_1010)),
            BitByte::new(0b0000_1110)
        );
        assert_eq!(
            BitByte::new(0x00).bitwise_or(BitByte::new(0x55)),
            BitByte::new(0x55)
        );
    }

    #[test]
    fn test_xor() {
        // 0b1100 ^ 0b1010 = 0b0110
        assert_eq!(
            BitByte::new(0b0000_1100).bitwise_xor(BitByte::new(0b0000_1010)),
            BitByte::new(0b0000_0110)
        );
        assert_eq!(
            BitByte::new(0xFF).bitwise_xor(BitByte::new(0xFF)),
            BitByte::new(0x00)
        );
    }

    #[test]
    fn test_not() {
        // !0b0000_1111 = 0b1111_0000
        assert_eq!(
            BitByte::new(0b0000_1111).bitwise_not(),
            BitByte::new(0b1111_0000)
        );
        assert_eq!(BitByte::new(0x00).bitwise_not(), BitByte::new(0xFF));
    }

    #[test]
    fn test_shift_left() {
        assert_eq!(
            BitByte::new(0b0000_0001).shift_left(3),
            BitByte::new(0b0000_1000)
        );
        // Überlauf-Verhalten bei u8 (Grenzfall)
        assert_eq!(
            BitByte::new(0b1000_0000).shift_left(1),
            BitByte::new(0b0000_0000)
        );
    }

    #[test]
    fn test_shift_right() {
        assert_eq!(
            BitByte::new(0b0000_1000).shift_right(3),
            BitByte::new(0b0000_0001)
        );
        assert_eq!(
            BitByte::new(0b0000_0001).shift_right(1),
            BitByte::new(0b0000_0000)
        );
    }

    #[test]
    fn test_set_bit() {
        // Setze Bit an Index 2 (Wert 4)
        assert_eq!(
            BitByte::new(0b0000_0000).set_bit(2),
            BitByte::new(0b0000_0100)
        );
        // Bleibt 1, wenn es schon 1 war
        assert_eq!(
            BitByte::new(0b0000_0100).set_bit(2),
            BitByte::new(0b0000_0100)
        );
    }

    #[test]
    fn test_clear_bit() {
        // Lösche Bit an Index 3 (Wert 8)
        assert_eq!(
            BitByte::new(0b0000_1111).clear_bit(3),
            BitByte::new(0b0000_0111)
        );
        // Bleibt 0, wenn es schon 0 war
        assert_eq!(
            BitByte::new(0b0000_0111).clear_bit(3),
            BitByte::new(0b0000_0111)
        );
    }

    #[test]
    fn test_toggle_bit() {
        // Aus 0 wird 1
        assert_eq!(
            BitByte::new(0b0000_0000).toggle_bit(4),
            BitByte::new(0b0001_0000)
        );
        // Aus 1 wird 0
        assert_eq!(
            BitByte::new(0b0001_0000).toggle_bit(4),
            BitByte::new(0b0000_0000)
        );
    }

    #[test]
    fn test_check_bit() {
        assert!(BitByte::new(0b0000_1000).check_bit(3));
        assert!(!BitByte::new(0b0000_1000).check_bit(2));
    }

    #[test]
    fn test_method_chaining() {
        // Komplexer Ablauf über mehrere Mutationen hinweg
        let result = BitByte::new(0)
            .set_bit(0) // -> 0b0000_0001
            .set_bit(3) // -> 0b0000_1001
            .toggle_bit(4) // -> 0b0001_1001
            .clear_bit(0); // -> 0b0001_1000

        assert_eq!(result, BitByte::new(0b0001_1000));
        assert!(result.check_bit(3));
        assert!(result.check_bit(4));
        assert!(!result.check_bit(0));
    }
}
