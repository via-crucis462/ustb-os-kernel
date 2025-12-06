//! ELF 加载器
//! 负责解析 ELF 文件并初始化用户栈

pub mod flags;
pub mod init_info;
pub mod init_stack;

#[allow(unused)]
use flags::*;
use init_info::InitInfo;
use init_stack::InitStack;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use log::{debug, info};

use crate::config::PAGE_SIZE;
use crate::mm::translated_byte_buffer;

use xmas_elf::{
    header,
    program::Type,
    ElfFile,
};



pub struct ElfLoader<'a> {
    pub elf: ElfFile<'a>,
}

impl<'a> ElfLoader<'a> {
    pub fn new(elf_data: &'a [u8]) -> Result<Self, &'static str> {
        let elf = ElfFile::new(elf_data).map_err(|_| "Invalid ELF")?;
        
        // 检查类型
        if elf.header.pt1.class() != header::Class::SixtyFour {
            return Err("32-bit ELF is not supported on the riscv64");
        }
        
        match elf.header.pt2.machine().as_machine() {
            header::Machine::RISC_V => {}
            _ => return Err("invalid ELF arch"),
        };
        
        Ok(Self { elf })
    }

    /// Load segments into memory
    pub fn load_segments(&self, memory_token: usize) -> Result<(), &'static str> {
        for ph in self.elf.program_iter() {
            if ph.get_type().map_err(|_| "Invalid Segment Type")? != Type::Load {
                continue;
            }
            let vaddr = ph.virtual_addr() as usize;
            let mem_size = ph.mem_size() as usize;
            let file_size = ph.file_size() as usize;
            let offset = ph.offset() as usize;
            
            // Copy data from ELF to memory
            let data = &self.elf.input[offset..offset + file_size];
            let buffers = translated_byte_buffer(memory_token, vaddr as *const u8, mem_size);
            
            let mut current_offset = 0;
            for buffer in buffers {
                let buffer_len = buffer.len();
                let end_offset = current_offset + buffer_len;
                
                // Determine what part of data goes here
                if current_offset < file_size {
                    let copy_len = core::cmp::min(file_size - current_offset, buffer_len);
                    buffer[..copy_len].copy_from_slice(&data[current_offset..current_offset + copy_len]);
                }
                
                // Zero out BSS if needed
                if end_offset > file_size {
                    let start_zero = if current_offset < file_size {
                        file_size - current_offset
                    } else {
                        0
                    };
                    buffer[start_zero..].fill(0);
                }
                
                current_offset += buffer_len;
            }
        }
        Ok(())
    }

    /// 初始化用户栈，并返回用户栈栈顶
    ///
    /// 这里会把 argc 存在用户栈顶， argv 存在栈上第二个元素，**且用 usize(64位) 存储**，即相当于：
    ///
    /// argc = *sp;
    ///
    /// argv = *(sp+8);
    pub fn init_stack(
        &self,
        memory_token: usize,
        stack_top: usize,
        args: Vec<String>,
    ) -> usize {
        let elf_base_vaddr = if let Some(header) = self
            .elf
            .program_iter()
            .find(|ph| ph.get_type() == Ok(Type::Load) && ph.offset() == 0)
        {
            let phdr = header.virtual_addr() as usize;
            if phdr != 0 {
                phdr
            } else {
                0
            }
        } else {
            0
        };

        let info = InitInfo {
            args: {
                let mut new_args = Vec::new();
                for i in args.iter() {
                    let arg = i.to_string();
                    new_args.push(arg);
                }
                new_args
            },
            envs: {
                Vec::new()
            },
            auxv: {
                use alloc::collections::btree_map::BTreeMap;
                let mut map = BTreeMap::new();
                
                map.insert(
                    AT_PHDR,
                    elf_base_vaddr + self.elf.header.pt2.ph_offset() as usize,
                );
                
                map.insert(AT_PHENT, self.elf.header.pt2.ph_entry_size() as usize);
                map.insert(AT_PHNUM, self.elf.header.pt2.ph_count() as usize);
                // AT_RANDOM 比较特殊，要求指向栈上的 16Byte 的随机子串。因此这里的 0 只是占位，在之后序列化时会特殊处理
                map.insert(AT_RANDOM, 0);
                map.insert(AT_PAGESZ, PAGE_SIZE);
                
                map
            },
        };

        info!("info {:#?}", info);
        let init_stack = info.serialize(stack_top);
        debug!("init user proc: stack len {}", init_stack.len());
        let stack_top = stack_top - init_stack.len();
        // stack_pma.write(USER_STACK_SIZE - init_stack.len(), &init_stack)?; could be written as followed:
        let stack = translated_byte_buffer(memory_token, stack_top as *const u8, init_stack.len());
        // 接下来要把 init_stack 复制到 stack 上
        let mut pos = 0;
        for page in stack {
            let len = page.len();
            page.copy_from_slice(&init_stack[pos..pos + len]);
            pos += len;
        }
        assert!(pos == init_stack.len());
        stack_top
    }
}