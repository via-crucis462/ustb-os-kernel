use riscv::register::sstatus::{self, Sstatus};

#[repr(C)]
#[derive(Debug)]
/// trap context structure containing sstatus, sepc and registers
#[derive(Copy, Clone)]
pub struct TrapContext {
    /// General-Purpose Register x0-31
    pub x: [usize; 32],
    /// Supervisor Status Register
    pub sstatus: Sstatus,
    /// Supervisor Exception Program Counter
    pub sepc: usize,
}

impl TrapContext {
    /// set stack pointer to x_2 reg (sp)
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    /// init app context
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        crate::println!("[kernel] app_init_context: entry={:#x}, sp={:#x}", entry, sp);
        let mut sstatus = sstatus::read();
        
        // set SPP to User (0) and SPIE to Enable (1)
        // SPP is bit 8, SPIE is bit 5
        unsafe {
            let sstatus_ptr = &mut sstatus as *mut Sstatus as *mut usize;
            *sstatus_ptr &= !(1 << 8); // Clear SPP (User)
            *sstatus_ptr |= 1 << 5;    // Set SPIE (Enable Interrupts)
        }
        
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry, // entry point of app
        };
        cx.set_sp(sp); // app's user stack pointer
        cx // return initial Trap Context of app
    }
}
