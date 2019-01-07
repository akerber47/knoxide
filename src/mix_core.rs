use crate::mix_types::*;
use crate::mix_util;

// Helper function to pull out address and apply index registers.
// Panic if the address field overflows when adding the index.
fn effective_address(instr: MixWord, r: &MixRegisters) -> MixAddr {
    let base_addr = mix_util::get_bytes(instr, 0, 2) as u16;
    let i = mix_util::get_bytes(instr, 3, 3) as usize;
    if i == 0 {
        return base_addr;
    }
    else {
        new_addr = mix_util::from_mix_addr(base_addr) +
            mix_util::from_mix_addr(r.i[i-1])
        // Check address overflow
        if (new_addr > MIX_ADDR_MAX || new_addr < MIX_ADDR_MIN) {
            panic!(format!("Address overflow! {}", new_addr));
        }
        return mix_util::to_mix_addr(new_addr);
    }
}

// Look up the value at an address in memory.
// Include fields according to the field specification.
// Panic if the address doesn't correspond to a real memory address.
fn memory_get(m: MixAddr, f: MixByte, st: &MixState) -> MixWord {
    m_val = mix_util::from_mix_addr(m);
    if (m_val > MIX_MEMORY_ADDR_MAX || m_val < MIX_MEMORY_ADDR_MIN) {
        panic!(format!("Out of bounds memory access! {}", m_val));
    }
    return mix_util::get_field_word(st.m[m_val], f);
}

fn do_mix_instruction(instr: MixWord, st: &mut MixState) -> void {
    let f = mix_util::get_bytes(instr, 4, 4) as u8;
    let c = mix_util::get_bytes(instr, 5, 5) as u8;
    let m = effective_address(instr, st.r);
    match c {
        // NOP
        0 => {},
        // Arithmetic operators
        1...4 => do_arithmetic(c, f, m, st),
        // Special operators
        5 => do_special(st),
        // Shift operators
        6 => do_shift(f, st),
        // MOVE
        7 => do_move(m, f, st),
        // Load operators
        8...23 => do_load(c, m, st),
        // Store operators
        24...33 => do_store(c, m, st),
        // I/O operators
        34...38 => do_io(c, f, st),
        // Jump operators
        39...47 => do_jump(c, f, st),
        // Address transfer operators
        48...55 => do_address_transfer(c, m, f, st),
        // Comparison operators
        56...63 => do_compare(c, m, f, st),
        _ => panic!(format!("Bad operator {}", c)),
    }
}

fn do_arithmetic(c: MixByte, f: MixByte, m: MixAddr, st: &mut MixState)
    -> void {
    v = memory_get(m);
    match c {
        // ADD
        1 => {
            let new_val = mix_util::from_mix_word(v) +
                mix_util::from_mix_word(st.r.a);
            if (new_val < MIX_WORD_MIN || new_val > MIX_WORD_MAX) {
                st.overflow = true;
            }
            st.r.a = mix_util::to_mix_word(new_val);
        },
        // SUB
        2 => {
            let new_val = mix_util::from_mix_word(v) -
                mix_util::from_mix_word(st.r.a);
            if (new_val < MIX_WORD_MIN || new_val > MIX_WORD_MAX) {
                st.overflow = true;
            }
            st.r.a = mix_util::to_mix_word(new_val);
        }
        // MUL
        3 => {
            let new_val = (mix_util::from_mix_word(v) as i64) *
                (mix_util::from_mix_word(st.r.a) as i64);
            (st.r.a, st.r.x) = mix_util::to_mix_doubleword(new_val);
        }
        // DIV
        4 => {
            if (mix_util::get_bytes(v, 1, 5) == 0) ||
                (mix_util::get_bytes(st.r.a, 1, 5) >=
                    mix_util::get_bytes(v, 1, 5) {
                st.oveflow = true;
                // A, X registers undefined.
            } else {
                let new_quot = mix_util::from_mix_dword(st.r.a, st.r.x) /
                    (mix_util::from_mix_word(v) as i64);
                let new_rem = mix_util::from_mix_dword(st.r.a, st.r.x).abs() %
                    (mix_util::from_mix_word(v).abs() as i64);
                let old_a_sign = (1 << 30) & st.r.a;
                st.r.a = mix_util::to_mix_word(new_quot);
                st.r.x = old_a_sign | to_mix_word(new_rem);
            }
        }
    }
}

fn do_special(f: MixByte, st: &mut MixState) -> void {
    match f {
        // NUM
        0 => do_num(st),
        // CHAR
        1 => do_char(st),
        // HLT
        2 => do_hlt(st),
        _ => panic!(format!("Bad field specification {}", f)),
    }
}

fn do_shift(f: MixByte, st: &mut MixState) -> void {
    match f {
        // SLA
        0 => do_sla(st),
        // SRA
        1 => do_sra(st),
        // SLAX
        2 => do_slax(st),
        // SRAX
        3 => do_srax(st),
        // SLC
        4 => do_slc(st),
        // SRC
        5 => do_src(st),
        _ => panic!(format!("Bad field specification {}", f)),
    }
}

fn do_io(c: MixByte, f: MixByte, st: &mut MixState) -> void {
    match c {
        // JBUS
        34 => do_jbus(f, st),
        // IOC
        35 => do_ioc(f, st),
        // IN
        36 => do_in(f, st),
        // OUT
        37 => do_out(f, st),
        // JRED
        38 => do_jred(f, st),
    }
}
