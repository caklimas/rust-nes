use crate::bus;

pub enum AddressMode {
    Imp = 0,
    Imm = 1,
    Zp0 = 2,
    Zpx = 3,
    Zpy = 4,
    Rel = 5,
    Abs = 6,
    Abx = 7,
    Aby = 8,
    Ind = 9,
    Izx = 10,
    Izy = 11
}

/// Address mode: Implicit
/// No extra data is needed
/// Need to target accumulator for instructions like PHA
pub fn imp(bus: &mut bus::Bus) -> u8 {
    bus.cpu.fetched_data = bus.cpu.accumulator;
    0
}

/// Address mode: Immediate
/// The instruction expected the next byte to be used as data
/// Increment the program counter to access that
pub fn imm(bus: &mut bus::Bus) -> u8 {
    bus.cpu.addr_abs = bus.cpu.program_counter;
    bus.cpu.program_counter += 1;
    0
}

/// Address mode: Zero Page
/// Allows you to absolutely access the first 256 bytes of a location
pub fn zp0(bus: &mut bus::Bus) -> u8 {
    let address = bus.cpu_read(bus.cpu.program_counter, false);
    bus.cpu.program_counter += 1;
    bus.cpu.addr_abs = (address & 0x00FF) as u16;
    0
}

/// Address mode: Zero Page with X Offset
/// Same as zero page but with the X address added
pub fn zpx(bus: &mut bus::Bus) -> u8 {
    let address = bus.cpu_read(bus.cpu.program_counter, false).wrapping_add(bus.cpu.x_register);
    bus.cpu.program_counter += 1;
    bus.cpu.addr_abs = (address & 0x00FF) as u16;
    0
}

/// Address mode: Zero Page with Y Offset
/// Same as zero page but with Y addressadded
pub fn zpy(bus: &mut bus::Bus) -> u8 {
    let address = bus.cpu_read(bus.cpu.program_counter, false).wrapping_add(bus.cpu.y_register);
    bus.cpu.program_counter += 1;
    bus.cpu.addr_abs = (address & 0x00FF) as u16;
    0
}

/// Address mode: Relative
/// Branching instructions can't jump any further than 127 memory locations
pub fn rel(bus: &mut bus::Bus) -> u8 {
    bus.cpu.addr_rel = bus.cpu_read(bus.cpu.program_counter, false) as u16;
    bus.cpu.program_counter += 1;
    if bus.cpu.addr_rel & 0x80 != 0 {
        bus.cpu.addr_rel = bus.cpu.addr_rel | 0xFF00;
    }

    0
}

/// Address mode: Asbolute
/// Read the low and high from the next two instructions
pub fn abs(bus: &mut bus::Bus) -> u8 {
    bus.cpu.addr_abs = get_absolute_address(bus);
    0
}

/// Address mode: Absolute with X offset
/// Read the low and high from the next two instructions.
/// Then adds the X register to the result
pub fn abx(bus: &mut bus::Bus) -> u8 {
    let address = get_absolute_address(bus);
    bus.cpu.addr_abs = address.wrapping_add(bus.cpu.x_register as u16);

    if (bus.cpu.addr_abs & 0xFF00) != (address & 0xFF00) {
        return 1;
    } 

    0
}

/// Address mode: Absolute with X offset
/// Read the low and high from the next two instructions.
/// Then adds the Y register to the result
pub fn aby(bus: &mut bus::Bus) -> u8 {
    let address = get_absolute_address(bus);
    bus.cpu.addr_abs = address.wrapping_add(bus.cpu.y_register as u16);

    if (bus.cpu.addr_abs & 0xFF00) != (address & 0xFF00) {
        return 1;
    }

    0
}

/// Address mode: Indirect
/// There is a bug in the 6502 implementation where if the lower byte is equal to 0xFF it then needs to cross
/// a page boundary. It does this incorrectly and instead wraps around the same page so we need to replicate that.
pub fn ind(bus: &mut bus::Bus) -> u8 {
    let pointer_address = get_absolute_address(bus);

    if (pointer_address | 0x00FF) == 0x00FF {
        bus.cpu.addr_abs = 
            (bus.cpu_read(pointer_address, false) as u16 & 0xFF00) << 8 |
            bus.cpu_read(pointer_address, false) as u16;
    } else {
        bus.cpu.addr_abs =
            ((bus.cpu_read(pointer_address + 1, false) as u16) << 8) |
            bus.cpu_read(pointer_address, false) as u16
    }

    0
}

/// Address mode: Indirext X
/// The supplied address is offset by X to index a location in page 0x00
/// The actual address is then read from this location
pub fn izx(bus: &mut bus::Bus) -> u8 {
    let address = bus.cpu_read(bus.cpu.program_counter, false);
    bus.cpu.program_counter += 1;

    let low_address = (address.wrapping_add(bus.cpu.x_register)) & 0x00FF;
    let high_address = (address.wrapping_add(bus.cpu.x_register + 1)) & 0x00FF;
    let low = bus.cpu_read(low_address as u16, false) as u16;
    let high = bus.cpu_read(high_address as u16, false) as u16;

    bus.cpu.addr_abs = (high << 8) | low;

    0
}

/// Address mode: Indirect Y
/// The supplied address is a location in page 0x00
/// The address is then read from this and then offset by Y
/// If a page boundary occurs, an additional clock cycle is required
pub fn izy(bus: &mut bus::Bus) -> u8 {
    let pointer_address = bus.cpu_read(bus.cpu.program_counter, false) as u16;
    bus.cpu.program_counter += 1;

    let low = bus.cpu_read(pointer_address & 0x00FF, false) as u16;
    let high = (bus.cpu_read((pointer_address + 1) & 0x00FF, false) as u16) << 8;
    bus.cpu.addr_abs = (high | low).wrapping_add(bus.cpu.y_register.into());
    if bus.cpu.addr_abs & 0xFF00 != high {
        return 1;
    }

    0
}

fn get_absolute_address(bus: &mut bus::Bus) -> u16 {
    let low = bus.cpu_read(bus.cpu.program_counter, false) as u16;
    bus.cpu.program_counter += 1;
    let high = bus.cpu_read(bus.cpu.program_counter, false) as u16;
    bus.cpu.program_counter += 1;

    let address = (high << 8) | low;
    return address;
}