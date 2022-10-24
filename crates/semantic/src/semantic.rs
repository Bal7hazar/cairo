use defs::db::DefsGroup;
use defs::ids::LocalVarId;
// Reexport objects
pub use defs::ids::{ParamId, VarId};
use diagnostics_proc_macros::DebugWithDb;
use syntax::node::ast;

pub use super::expr::objects::*;
use crate::db::SemanticGroup;
pub use crate::expr::pattern::{
    Pattern, PatternEnum, PatternLiteral, PatternOtherwise, PatternStruct, PatternTuple,
    PatternVariable,
};
pub use crate::items::enm::{ConcreteVariant, Variant};
pub use crate::items::free_function::FreeFunctionDefinition;
pub use crate::items::functions::{ConcreteFunction, FunctionId, FunctionLongId, Signature};
pub use crate::items::strct::Member;
pub use crate::types::{
    ConcreteEnumId, ConcreteExternTypeId, ConcreteStructId, ConcreteTypeId, TypeId, TypeLongId,
};

/// Semantic model of a variable.
#[derive(Clone, Debug, Hash, PartialEq, Eq, DebugWithDb)]
#[debug_db(dyn SemanticGroup + 'static)]
pub struct LocalVariable {
    pub id: LocalVarId,
    pub ty: TypeId,
    pub modifiers: Modifiers,
}
impl LocalVariable {
    pub fn stable_ptr(&self, db: &dyn DefsGroup) -> ast::TerminalIdentifierPtr {
        self.id.stable_ptr(db)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, DebugWithDb)]
#[debug_db(dyn SemanticGroup + 'static)]
pub struct Parameter {
    pub id: ParamId,
    pub ty: TypeId,
    pub modifiers: Modifiers,
}

#[derive(Clone, Default, Debug, Hash, PartialEq, Eq)]
pub struct Modifiers {
    pub is_mut: bool,
    pub is_ref: bool,
}
impl Modifiers {
    pub fn new_implicit() -> Modifiers {
        Modifiers { is_mut: true, is_ref: true }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, DebugWithDb)]
#[debug_db(dyn SemanticGroup + 'static)]
pub enum Variable {
    Local(LocalVariable),
    Param(Parameter),
    ImplicitParam(Parameter),
}
impl Variable {
    pub fn id(&self) -> VarId {
        match self {
            Variable::Local(local) => VarId::Local(local.id),
            Variable::Param(param) => VarId::Param(param.id),
            Variable::ImplicitParam(implicit_param) => VarId::Param(implicit_param.id),
        }
    }
    pub fn ty(&self) -> TypeId {
        match self {
            Variable::Local(local) => local.ty,
            Variable::Param(param) => param.ty,
            Variable::ImplicitParam(implicit_param) => implicit_param.ty,
        }
    }
    pub fn modifiers(&self) -> &Modifiers {
        match self {
            Variable::Local(local) => &local.modifiers,
            Variable::Param(param) => &param.modifiers,
            Variable::ImplicitParam(implicit_param) => &implicit_param.modifiers,
        }
    }
}

/// Generic argument.
/// A value assigned to a generic parameter.
/// May be a type, impl, constant, etc..
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, DebugWithDb)]
#[debug_db(dyn SemanticGroup + 'static)]
pub enum GenericArgumentId {
    Type(TypeId),
    // TODO(spapini): impls and constants as generic values.
}
