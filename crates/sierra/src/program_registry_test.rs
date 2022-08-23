use indoc::indoc;

use crate::program::TypeDeclaration;
use crate::program_registry::{ProgramRegistry, ProgramRegistryError};
use crate::ProgramParser;

#[test]
fn basic_insertion() {
    assert_eq!(
        ProgramRegistry::new(
            &ProgramParser::new()
                .parse(indoc! {"
        type int = int;
        type GasBuiltin = GasBuiltin;
        type NonZeroInt = NonZeroInt;
        libfunc move_int = move<int>;
        libfunc move_gb = move<GasBuiltin>;
        Func1@1(a: int, gb: GasBuiltin) -> (GasBuiltin);
        Func2@6() -> ();
    "})
                .unwrap()
        )
        .map(|_| ()),
        Ok(())
    );
}

#[test]
fn function_id_double_declaration() {
    assert_eq!(
        ProgramRegistry::new(
            &ProgramParser::new()
                .parse(indoc! {"
        used_id@1(a: int, gb: GasBuiltin) -> (GasBuiltin);
        used_id@6() -> ();
    "})
                .unwrap()
        )
        .map(|_| ()),
        Err(ProgramRegistryError::FunctionIdAlreadyExists("used_id".into()))
    );
}

#[test]
fn type_id_double_declaration() {
    assert_eq!(
        ProgramRegistry::new(
            &ProgramParser::new()
                .parse(indoc! {"
        type used_id = int;
        type used_id = GasBuiltin;
    "})
                .unwrap()
        )
        .map(|_| ()),
        Err(ProgramRegistryError::TypeConcreteIdAlreadyExists("used_id".into()))
    );
}

#[test]
fn concrete_type_double_declaration() {
    assert_eq!(
        ProgramRegistry::new(
            &ProgramParser::new()
                .parse(indoc! {"
        type int1 = int;
        type int2 = int;
    "})
                .unwrap()
        )
        .map(|_| ()),
        Err(ProgramRegistryError::TypeAlreadyDeclared(TypeDeclaration {
            id: "int2".into(),
            generic_id: "int".into(),
            args: vec![]
        }))
    );
}

#[test]
fn libfunc_id_double_declaration() {
    assert_eq!(
        ProgramRegistry::new(
            &ProgramParser::new()
                .parse(indoc! {"
        libfunc used_id = move<int>;
        libfunc used_id = move<GasBuiltin>;
    "})
                .unwrap()
        )
        .map(|_| ()),
        Err(ProgramRegistryError::LibFuncConcreteIdAlreadyExists("used_id".into()))
    );
}
