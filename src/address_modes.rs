use crate::cpu;

/// Address mode: Implicit
/// No extra data is needed
/// Need to target accumulator for instructions like PHA
pub fn imp(cpu: &mut cpu::olc6502) -> u8 {
    cpu.fetched_data = cpu.accumulator;
    0
}

/// Address mode: Immediate
/// The instruction expected the next byte to be used as data
/// Increment the program counter to access that
pub fn imm(cpu: &mut cpu::olc6502) -> u8 {
    cpu.addr_abs = cpu.program_counter;
    cpu.program_counter += 1;
    0
}

/// Address mode: Zero Page
/// Allows you to absolutely access the first 256 bytes of a location
pub fn zp0(cpu: &mut cpu::olc6502) -> u8 {
    let address = cpu.read(cpu.program_counter, false);
    cpu.program_counter += 1;
    cpu.addr_abs = (address & 0x00FF) as u16;
    0
}

/// Address mode: Zero Page with X Offset
/// Same as zero page but with the X address added
pub fn zpx(cpu: &mut cpu::olc6502) -> u8 {
    let address = cpu.read(cpu.program_counter, false).wrapping_add(cpu.x_register);
    cpu.program_counter += 1;
    cpu.addr_abs = (address & 0x00FF) as u16;
    0
}

/// Address mode: Zero Page with Y Offset
/// Same as zero page but with Y addressadded
pub fn zpy(cpu: &mut cpu::olc6502) -> u8 {
    let address = cpu.read(cpu.program_counter, false).wrapping_add(cpu.y_register);
    cpu.program_counter += 1;
    cpu.addr_abs = (address & 0x00FF) as u16;
    0
}

/// Address mode: Relative
pub fn rel(cpu: &mut cpu::olc6502) -> u8 {
    cpu.addr_rel = cpu.read(cpu.program_counter, false) as u16;
    cpu.program_counter += 1;
    if cpu.addr_rel & 0x80 != 0 {
        cpu.addr_rel = cpu.addr_rel | 0xFF00;
    }

    0
}

/// Address mode: Asbolute
/// Read the low and high from the next two instructions
pub fn abs(cpu: &mut cpu::olc6502) -> u8 {
    cpu.addr_abs = get_absolute_address(cpu);
    0
}

/// Address mode: Absolute with X offset
/// Read the low and high from the next two instructions.
/// Then adds the X register to the result
pub fn abx(cpu: &mut cpu::olc6502) -> u8 {
    let address = get_absolute_address(cpu);
    cpu.addr_abs = address.wrapping_add(cpu.x_register as u16);

    if (cpu.addr_abs & 0xFF00) != (address & 0xFF00) {
        return 1;
    } 

    0
}

/// Address mode: Absolute with X offset
/// Read the low and high from the next two instructions.
/// Then adds the Y register to the result
pub fn aby(cpu: &mut cpu::olc6502) -> u8 {
    let address = get_absolute_address(cpu);
    cpu.addr_abs = address.wrapping_add(cpu.y_register as u16);

    if (cpu.addr_abs & 0xFF00) != (address & 0xFF00) {
        return 1;
    }

    0
}

/// Address mode: Indirect
/// There is a bug in the 6502 implementation where if the lower byte is equal to 0xFF it then needs to cross
/// a page boundary. It does this incorrectly and instead wraps around the same page so we need to replicate that.
pub fn ind(cpu: &mut cpu::olc6502) -> u8 {
    let pointer_address = get_absolute_address(cpu);

    if (pointer_address | 0x00FF) == 0x00FF {
        cpu.addr_abs = 
            (cpu.read(pointer_address, false) as u16 & 0xFF00) << 8 |
            cpu.read(pointer_address, false) as u16;
    } else {
        cpu.addr_abs =
            ((cpu.read(pointer_address + 1, false) as u16) << 8) |
            cpu.read(pointer_address, false) as u16
    }

    0
}

/// Address mode: Indirext X
/// The supplied address is offset by X to index a location in page 0x00
/// The actual address is then read from this location
pub fn izx(cpu: &mut cpu::olc6502) -> u8 {
    let address = cpu.read(cpu.program_counter, false);
    cpu.program_counter += 1;

    let low_address = (address.wrapping_add(cpu.x_register)) & 0x00FF;
    let high_address = (address.wrapping_add(cpu.x_register + 1)) & 0x00FF;
    let low = cpu.read(low_address as u16, false) as u16;
    let high = cpu.read(high_address as u16, false) as u16;

    cpu.addr_abs = (high << 8) | low;

    0
}

/// Address mode: Indirect Y
/// The supplied address is a location in page 0x00
/// The address is then read from this and then offset by Y
/// If a page boundary occurs, an additional clock cycle is required
pub fn izy(cpu: &mut cpu::olc6502) -> u8 {
    let pointer_address = cpu.read(cpu.program_counter, false) as u16;
    cpu.program_counter += 1;

    let low = cpu.read(pointer_address & 0x00FF, false) as u16;
    let high = (cpu.read((pointer_address + 1) & 0x00FF, false) as u16) << 8;
    cpu.addr_abs = high | low;
    if cpu.addr_abs & 0xFF00 != high {
        return 1;
    }

    0
}

fn get_absolute_address(cpu: &mut cpu::olc6502) -> u16 {
    let low = cpu.read(cpu.program_counter, false) as u16;
    cpu.program_counter += 1;
    let high = cpu.read(cpu.program_counter, false) as u16;
    cpu.program_counter += 1;

    let address = (high << 8) | low;
    return address;
}