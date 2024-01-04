use std::fmt;

pub struct Transition {
    pub op: String,
    pub src: String,
    pub dst: String,
    pub before: u16,
    pub after: u16,
}

impl fmt::Display for Transition {

    /*
    --- test\listing_0043_immediate_movs execution ---
    mov ax, 1 ; ax:0x0->0x1 
    mov bx, 2 ; bx:0x0->0x2 
    mov cx, 3 ; cx:0x0->0x3 
    mov dx, 4 ; dx:0x0->0x4 
    mov sp, 5 ; sp:0x0->0x5 
    mov bp, 6 ; bp:0x0->0x6 
    mov si, 7 ; si:0x0->0x7 
    mov di, 8 ; di:0x0->0x8 
    */
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}, {} ; {}:{:#x}->{:#x}",
            self.op, self.dst, self.src, self.dst, self.before, self.after 
        )
    }
}