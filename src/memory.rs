// this section would not be possible without the help of mGBA's source
// and https://www.akkit.org/info/gbatek.htm.
// thank you!
//
// sizes
const BIOS_SIZE: usize = 0x0004000;
const WRAM_SIZE: usize = 0x0040000;
const WIRAM_SIZE: usize = 0x0008000;
const IO_REGISTERS_SIZE: usize = 0x0000400;
const PALETTE_RAM_SIZE: usize = 0x0000400;
const VRAM_SIZE: usize = 0x0018000;
const OAM_SIZE: usize = 0x0000400;
const CART0_SIZE: usize = 0x2000000;
const CART1_SIZE: usize = 0x2000000;
const CART2_SIZE: usize = 0x2000000;
const CART_SRAM_SIZE: usize = 0x0008000;
const CART_FLASH512_SIZE: usize = 0x0010000;
const CART_FLASH1M_SIZE: usize = 0x0020000;
const CART_EEPROM_SIZE: usize = 0x0002000;
const CART_EEPROM512_SIZE: usize = 0x0000200;

// base addresses
const BIOS_ADDR: usize = 0x0000000;
const WORKING_RAM_ADDR: usize = 0x2000000;
const WORKING_IRAM_ADDR: usize = 0x3000000;
const IO_REGISTERS_ADDR: usize = 0x4000000;
const PALETTE_RAM_ADDR: usize = 0x5000000;
const VRAM_ADDR: usize = 0x6000000;
const OAM_ADDR: usize = 0x7000000;
const CART0_ADDR: usize = 0x8000000;
const CART0_EX_ADDR: usize = 0x9000000;
const CART1_ADDR: usize = 0xA000000;
const CART1_EX_ADDR: usize = 0xB000000;
const CART2_ADDR: usize = 0xC000000;
const CART2_EX_ADDR: usize = 0xD000000;
const CART_SRAM_ADDR: usize = 0xE000000;
const CART_SRAM_MIRROR_ADDR: usize = 0xF000000;

struct MMU {
    wram: Box<[u8; WRAM_SIZE]>,
}

impl MMU {
    fn new() -> Self {
        Self {
            wram: [0; WRAM_SIZE],
        }
    }
}