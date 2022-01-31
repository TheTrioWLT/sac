
//! Low level assembly generation.
//! There is a separate mod for each supported backend architutrute.
//! Currently the following are supported:
//! - aarch64
//! - ARMv7 (proposed)
//! - x86_64 (proposed)
//! 
//! Register allocation occurs in this step

mod aarch64;

#[derive(Copy, Clone, Debug)]
pub enum Backend {
    Aarch64,
    Armv7,
    X86_64,
}

pub fn do_codegen<USize>(instructions: Vec<super::high::Instruction<USize>>, backend: Backend) /* -> WHAT */ {
    match backend {
        Backend::Aarch64 => unimplemented!(),
        //Backend::Aarch64 => aarch64::do_codegen(instructions), // AHHHH generics
        Backend::Armv7 => unimplemented!(),
        Backend::X86_64 => unimplemented!(),
    }
}