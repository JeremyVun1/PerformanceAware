use std::fmt;

use super::{SF_BIT, ZF_BIT};

pub struct Transition {
    pub op: &'static str,
    pub src: String,
    pub dst: String,
    pub before: u16,
    pub after: u16,
    pub flags: u16,
}

impl Transition {
    pub fn default() -> Transition {
        Transition {
            op: "",
            src: "".to_string(),
            dst: "".to_string(),
            before: 0,
            after: 0,
            flags: 0,
        }
    }
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
        let flags = get_flags(&self.flags);
        let flag_line = if flags.len() > 0 {
            format!("flags:->{}", flags.join("->"))
        } else {
            String::from("")
        };

        write!(f, "{} {}, {} ; {}:{:#x}->{:#x} ; {}",
            self.op, self.dst, self.src,
            get_full_register_name(&self.dst), self.before, self.after,
            flag_line
        )
    }
}

pub fn get_flags(flag_bits: &u16) -> Vec<String> {
    let mut flags: Vec<String> = Vec::new();
    
    if flag_bits & SF_BIT > 0 {
        flags.push("SF".to_string());
    }
    if flag_bits & ZF_BIT > 0 {
        flags.push("ZF".to_string());
    }

    return flags;
}

fn get_full_register_name(reg: &str) -> String {
    let c1 = reg.chars().nth(1).unwrap();
    if c1 == 'l' || c1 == 'h' {
        let c0 = reg.chars().nth(0).unwrap();
        return format!("{c0}x");
    }
    else {
        return reg.to_string();
    }
}