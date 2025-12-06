use riscv::register::sstatus::{self, Sstatus};
/// Trap Context
#[repr(C)]
pub struct TrapContext {
    /// general regs[0..31]
    pub x: [usize; 32],
    /// CSR sstatus      
    pub sstatus: Sstatus,
    /// CSR sepc
    pub sepc: usize,
}

impl TrapContext {
    /// set stack pointer to x_2 reg (sp)
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    /// init app context
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        
        // set SPP to User (0) and SPIE to Enable (1)
        // SPP is bit 8, SPIE is bit 5
        unsafe {
            let sstatus_ptr = &mut sstatus as *mut Sstatus as *mut usize;
            *sstatus_ptr &= !(1 << 8); // Clear SPP (User)
            *sstatus_ptr |= 1 << 5;    // Set SPIE (Enable Interrupts)
        }
        
        /*
            It destroy the purity of a function with side effect. 
            It would be more intuitive, but we don't recommend it.
            We just want to create a "To be used in the future" context 
            but end up changing the CPU mode configuration now.

            unsafe {
                sstatus::set_spp(sstatus::SPP::User);
                sstatus::set_spie();
            }
            let sstatus = sstatus::read();
        */
        
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry, // entry point of app
        };
        cx.set_sp(sp); // app's user stack pointer
        cx // return initial Trap Context of app
    }
}