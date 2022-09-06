// Autogenerated file. To regenerate, please run `cargo run --bin generate_syntax`.
use super::ids::GreenId;
use super::kind::SyntaxKind;
/// Gets the vector of children ids that are the indexing key for this SyntaxKind.
/// Each SyntaxKind has some children that are defined in the spec to be its indexing key
/// for its stable pointer. See [super::stable_ptr].
pub fn get_key_fields(kind: SyntaxKind, children: Vec<GreenId>) -> Vec<GreenId> {
    match kind {
        SyntaxKind::Terminal => {
            vec![]
        }
        SyntaxKind::TriviumSkippedTerminal => {
            vec![]
        }
        SyntaxKind::Trivia => {
            vec![]
        }
        SyntaxKind::StructArgExpr => {
            vec![]
        }
        SyntaxKind::OptionStructArgExprEmpty => {
            vec![]
        }
        SyntaxKind::StructArgSingle => {
            vec![/* identifier */ children[0]]
        }
        SyntaxKind::StructArgTail => {
            vec![]
        }
        SyntaxKind::StructArgList => {
            vec![]
        }
        SyntaxKind::ArgListBraced => {
            vec![]
        }
        SyntaxKind::ExprList => {
            vec![]
        }
        SyntaxKind::ExprMissing => {
            vec![]
        }
        SyntaxKind::OptionGenericArgsEmpty => {
            vec![]
        }
        SyntaxKind::OptionGenericArgsSome => {
            vec![]
        }
        SyntaxKind::PathSegment => {
            vec![]
        }
        SyntaxKind::ExprPath => {
            vec![]
        }
        SyntaxKind::ExprLiteral => {
            vec![]
        }
        SyntaxKind::ExprParenthesized => {
            vec![]
        }
        SyntaxKind::ExprUnary => {
            vec![]
        }
        SyntaxKind::ExprBinary => {
            vec![]
        }
        SyntaxKind::ExprTuple => {
            vec![]
        }
        SyntaxKind::ExprListParenthesized => {
            vec![]
        }
        SyntaxKind::ExprFunctionCall => {
            vec![]
        }
        SyntaxKind::ExprStructCtorCall => {
            vec![]
        }
        SyntaxKind::ExprBlock => {
            vec![]
        }
        SyntaxKind::MatchArm => {
            vec![]
        }
        SyntaxKind::MatchArms => {
            vec![]
        }
        SyntaxKind::ExprMatch => {
            vec![]
        }
        SyntaxKind::TypeClause => {
            vec![]
        }
        SyntaxKind::NonOptionTypeClauseMissing => {
            vec![]
        }
        SyntaxKind::OptionTypeClauseEmpty => {
            vec![]
        }
        SyntaxKind::ReturnTypeClause => {
            vec![]
        }
        SyntaxKind::OptionReturnTypeClauseEmpty => {
            vec![]
        }
        SyntaxKind::StatementList => {
            vec![]
        }
        SyntaxKind::StatementMissing => {
            vec![]
        }
        SyntaxKind::StatementLet => {
            vec![/* name */ children[1]]
        }
        SyntaxKind::OptionSemicolonEmpty => {
            vec![]
        }
        SyntaxKind::StatementExpr => {
            vec![]
        }
        SyntaxKind::StatementReturn => {
            vec![]
        }
        SyntaxKind::Param => {
            vec![/* name */ children[0]]
        }
        SyntaxKind::ParamList => {
            vec![]
        }
        SyntaxKind::ParamListParenthesized => {
            vec![]
        }
        SyntaxKind::ParamListBraced => {
            vec![]
        }
        SyntaxKind::FunctionSignature => {
            vec![]
        }
        SyntaxKind::ItemList => {
            vec![]
        }
        SyntaxKind::ItemModule => {
            vec![/* name */ children[1]]
        }
        SyntaxKind::ItemFreeFunction => {
            vec![/* name */ children[1]]
        }
        SyntaxKind::ItemExternFunction => {
            vec![/* name */ children[2]]
        }
        SyntaxKind::ItemExternType => {
            vec![/* name */ children[2]]
        }
        SyntaxKind::ItemTrait => {
            vec![/* name */ children[1]]
        }
        SyntaxKind::ItemImpl => {
            vec![/* name */ children[1]]
        }
        SyntaxKind::ItemStruct => {
            vec![/* name */ children[1]]
        }
        SyntaxKind::ItemEnum => {
            vec![/* name */ children[1]]
        }
        SyntaxKind::ItemUse => {
            vec![/* name */ children[1]]
        }
        SyntaxKind::SyntaxFile => {
            vec![]
        }
    }
}
