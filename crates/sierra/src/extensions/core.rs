use super::ap_tracking::RevokeApTrackingLibFunc;
use super::drop::DropLibFunc;
use super::duplicate::DupLibFunc;
use super::modules::felt::{FeltLibFunc, FeltType};
use super::modules::function_call::FunctionCallLibFunc;
use super::modules::gas::{GasBuiltinType, GasLibFunc};
use super::modules::integer::{IntegerLibFunc, IntegerType};
use super::modules::mem::MemLibFunc;
use super::modules::non_zero::{NonZeroType, UnwrapNonZeroLibFunc};
use super::modules::reference::{RefLibFunc, RefType};
use super::modules::unconditional_jump::UnconditionalJumpLibFunc;
use super::uninitialized::UninitializedType;
use crate::{define_libfunc_hierarchy, define_type_hierarchy};

define_type_hierarchy! {
    pub enum CoreType {
        Felt(FeltType),
        GasBuiltin(GasBuiltinType),
        Integer(IntegerType),
        NonZero(NonZeroType),
        Ref(RefType),
        Uninitialized(UninitializedType),
    }, CoreTypeConcrete
}

define_libfunc_hierarchy! {
    pub enum CoreLibFunc {
        ApTracking(RevokeApTrackingLibFunc),
        Drop(DropLibFunc),
        Dup(DupLibFunc),
        Felt(FeltLibFunc),
        FunctionCall(FunctionCallLibFunc),
        Gas(GasLibFunc),
        Integer(IntegerLibFunc),
        Mem(MemLibFunc),
        Ref(RefLibFunc),
        UnwrapNonZero(UnwrapNonZeroLibFunc),
        UnconditionalJump(UnconditionalJumpLibFunc),
    }, CoreConcreteLibFunc
}
