macro_rules! impl_opcodes {
    ($($n:ident:$l:literal)*) => {
        $(pub const $n: u8 = $l;)*
    };
}

impl_opcodes!(
    END: 0x00
    LDI: 0x01
    ADD: 0x02
    MUL: 0x03
    SUB: 0x04
    DIV: 0x05
    LDR: 0x06
    MOD: 0x07
    EQ: 0x08
    NE: 0x09
    LS: 0x0A
    GR: 0x0B
    LE: 0x0C
    GE: 0x0D
);
