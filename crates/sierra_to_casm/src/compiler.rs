use std::fmt::Display;

use casm::instructions::{Instruction, InstructionBody, RetInstruction};
use sierra::extensions::core::{CoreConcreteLibFunc, CoreLibFunc, CoreType};
use sierra::extensions::ConcreteLibFunc;
use sierra::program::{BranchTarget, Invocation, Program, Statement, StatementIdx};
use sierra::program_registry::{ProgramRegistry, ProgramRegistryError};
use thiserror::Error;

use crate::annotations::{AnnotationError, ProgramAnnotations, StatementAnnotations};
use crate::invocations::{check_references_on_stack, compile_invocation, InvocationError};
use crate::references::{check_types_match, ReferencesError};
use crate::relocations::{relocate_instructions, RelocationEntry};
use crate::type_sizes::get_type_size_map;

#[cfg(test)]
#[path = "compiler_test.rs"]
mod test;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum CompilationError {
    #[error("Failed building type information")]
    FailedBuildingTypeInformation,
    #[error("Error from program registry")]
    ProgramRegistryError(ProgramRegistryError),
    #[error(transparent)]
    AnnotationError(#[from] AnnotationError),
    #[error(transparent)]
    InvocationError(#[from] InvocationError),
    #[error(transparent)]
    ReferencesError(#[from] ReferencesError),
    #[error("Invocation mismatched to libfunc")]
    LibFuncInvocationMismatch,
}

#[derive(Error, Debug, Eq, PartialEq)]
pub struct CairoProgram {
    instructions: Vec<Instruction>,
}
impl Display for CairoProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for instruction in &self.instructions {
            writeln!(f, "{};", instruction)?
        }
        Ok(())
    }
}

/// Ensure the basic structure of the invocation is the same as the library function.
fn check_basic_structure(
    invocation: &Invocation,
    libfunc: &CoreConcreteLibFunc,
) -> Result<(), CompilationError> {
    if invocation.args.len() != libfunc.input_types().len()
        || !itertools::equal(
            invocation.branches.iter().map(|branch| branch.results.len()),
            libfunc.output_types().iter().map(|types| types.len()),
        )
        || match libfunc.fallthrough() {
            Some(expected_fallthrough) => {
                invocation.branches[expected_fallthrough].target != BranchTarget::Fallthrough
            }
            None => false,
        }
    {
        Err(CompilationError::LibFuncInvocationMismatch)
    } else {
        Ok(())
    }
}

pub fn compile(program: &Program) -> Result<CairoProgram, CompilationError> {
    let mut instructions = Vec::new();
    let mut relocations: Vec<RelocationEntry> = Vec::new();

    // Maps statement_idx to program_offset.
    let mut statement_offsets = Vec::with_capacity(program.statements.len());

    let registry = ProgramRegistry::<CoreType, CoreLibFunc>::new(program)
        .map_err(CompilationError::ProgramRegistryError)?;
    let type_sizes = get_type_size_map(program, &registry)
        .ok_or(CompilationError::FailedBuildingTypeInformation)?;
    let mut program_annotations =
        ProgramAnnotations::create(program.statements.len(), &program.funcs)?;

    let mut program_offset: usize = 0;

    for (statement_id, statement) in program.statements.iter().enumerate() {
        let statement_idx = StatementIdx(statement_id);

        statement_offsets.push(program_offset);

        match statement {
            Statement::Return(ref_ids) => {
                let (annotations, return_refs) = program_annotations
                    .get_annotations_after_take_args(statement_idx, ref_ids.iter())?;

                if !annotations.refs.is_empty() {
                    return Err(ReferencesError::DanglingReferences(statement_idx).into());
                }
                program_annotations.validate_return_type(&return_refs, annotations.return_types)?;
                check_references_on_stack(&type_sizes, &return_refs)?;

                let ret_instruction = RetInstruction {};
                program_offset += ret_instruction.op_size();
                instructions.push(Instruction {
                    body: InstructionBody::Ret(ret_instruction),
                    inc_ap: false,
                });
            }
            Statement::Invocation(invocation) => {
                let (annotations, invoke_refs) = program_annotations
                    .get_annotations_after_take_args(statement_idx, invocation.args.iter())?;

                let libfunc = registry
                    .get_libfunc(&invocation.libfunc_id)
                    .map_err(CompilationError::ProgramRegistryError)?;
                check_basic_structure(invocation, libfunc)?;

                check_types_match(&invoke_refs, libfunc.input_types())?;
                let compiled_invocation = compile_invocation(
                    invocation,
                    libfunc,
                    &invoke_refs,
                    &type_sizes,
                    annotations.environment,
                )?;

                for instruction in &compiled_invocation.instructions {
                    program_offset += instruction.body.op_size();
                }

                for entry in compiled_invocation.relocations {
                    relocations.push(RelocationEntry {
                        instruction_idx: instructions.len() + entry.instruction_idx,
                        relocation: entry.relocation,
                    });
                }
                instructions.extend(compiled_invocation.instructions);

                program_annotations.propagate_annotations(
                    statement_idx,
                    StatementAnnotations {
                        environment: compiled_invocation.environment,
                        ..annotations
                    },
                    &invocation.branches,
                    compiled_invocation.results.into_iter(),
                )?;
            }
        }
    }

    relocate_instructions(&relocations, statement_offsets, &mut instructions);

    Ok(CairoProgram { instructions })
}
