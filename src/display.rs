pub struct PatternTable {

}

#[derive(Default)]
pub struct Tile {
    pub planes: [[u8; 8]; 2]
}

impl Tile {
    pub fn get_colors(&mut self) -> [[u8; 8]; 8] {
        let mut colors: [[u8; 8]; 8] = Default::default();
        let bytes0 = self.planes[0];
        let bytes1 = self.planes[1];

        for i in 0..colors.len() {
            let byte0 = bytes0[i];
            let byte1 = bytes1[i];

            for j in 0..8 {
                let bit1 = ((byte1 >> (7 - j)) & 0x01) << 1;
                let bit0 = (byte0 >> (7 - j)) & 0x01;

                colors[i][j] = bit1 + bit0;         
            }
        }

        colors
    }
}