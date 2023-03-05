use std::io;

pub struct Emu {
    buf: [u8; 2],
}

impl Emu {
    pub fn new() -> Self {
        Self { buf: [0; 2] }
    }

    pub fn disassemble<R, W>(&mut self, rdr: &mut R, wrt: &mut W) -> io::Result<()>
    where
        R: io::Read,
        W: io::Write,
    {
        const OPCODE_MOV: u8 = 0b100010;

        const REG_TABLE: &[&str] = &[
            "al", "ax", "cl", "cx", "dl", "dx", "bl", "bx", "ah", "sp", "ch", "bp", "dh", "si",
            "bh", "di",
        ];

        wrt.write("bits 16\n".as_bytes())?;

        loop {
            let n = rdr.read(&mut self.buf[..2])?;
            if n == 0 {
                break;
            }
            let b0 = self.buf[0];
            let opcode = (b0 & 0b11111100) >> 2;
            let d = b0 & 0b10;
            let w = b0 & 0b01;
            match opcode {
                OPCODE_MOV => {
                    wrt.write("mov ".as_bytes())?;
                    let b1 = self.buf[1];
                    let m = (b1 & 0b11000000) >> 6;
                    match m {
                        0b11 => {
                            let reg1 = ((b1 & 0b00111000) >> 2) | w;
                            let reg2 = (b1 & 0b00000111) << 1 | w;

                            let (src, dst) = if d == 0 { (reg1, reg2) } else { (reg2, reg1) };

                            wrt.write(REG_TABLE[dst as usize].as_bytes())?;
                            wrt.write(", ".as_bytes())?;
                            wrt.write(REG_TABLE[src as usize].as_bytes())?;
                        }
                        _ => unimplemented!("mod = {m:b}"),
                    }
                }
                _ => unimplemented!("opcode = {opcode:b}"),
            }

            wrt.write("\n".as_bytes())?;
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut rdr = io::stdin();
    let mut wrt = io::stdout();
    let mut emu = Emu::new();

    emu.disassemble(&mut rdr, &mut wrt)
}
