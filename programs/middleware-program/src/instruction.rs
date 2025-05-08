use solana_program::{msg, program_error::ProgramError};

#[derive(Debug, PartialEq)]
pub enum ProgramInstruction {
    MiddlewareInstruction { instruction_data: Vec<u8> },
}

impl ProgramInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.is_empty() {
            msg!("Instruction data is empty");
            return Err(ProgramError::InvalidInstructionData);
        }
        let (variant_bytes, rest) = input.split_at(8);
        let mut variant = [0u8; 8];
        variant.copy_from_slice(variant_bytes);
        match variant {
            [0, 0, 0, 0, 0, 0, 0, 0] => Ok(ProgramInstruction::MiddlewareInstruction {
                instruction_data: rest.to_vec(),
            }),

            _ => {
                msg!("Unknown instruction variant");
                Err(ProgramError::InvalidInstructionData)
            }
        }
    }
}
