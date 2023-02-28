use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::test_utils::{setup_test_module, SemanticDatabaseForTesting};
use cairo_lang_utils::extract_matches;
use indoc::indoc;
use pretty_assertions::assert_eq;

use crate::abi::AbiBuilder;

#[test]
fn test_abi() {
    let mut db_val = SemanticDatabaseForTesting::default();
    let module_id = setup_test_module(
        &mut db_val,
        indoc! {"
            trait MyAbi {
                fn foo(a: felt, b: u128) -> Option::<()>;

                #[external]
                fn foo_external(a: felt, b: u128) -> Option::<()>;

                #[view]
                fn foo_view(a: felt, b: u128) -> Option::<()>;

                #[external]
                fn empty();

                #[event]
                fn foo_event(a: felt, b: u128);
            }
        "},
    )
    .unwrap()
    .module_id;

    let db = &db_val;
    let trait_id = extract_matches!(
        db.module_item_by_name(module_id, "MyAbi".into()).unwrap().unwrap(),
        ModuleItemId::Trait
    );
    let abi = AbiBuilder::from_trait(db, trait_id).unwrap();
    let actual_serialization = serde_json::to_string_pretty(&abi).unwrap();
    assert_eq!(
        actual_serialization,
        indoc! {
        r#"[
            {
              "type": "function",
              "name": "foo",
              "inputs": [
                {
                  "name": "a",
                  "type": "core::felt"
                },
                {
                  "name": "b",
                  "type": "core::integer::u128"
                }
              ],
              "outputs": [
                {
                  "type": "core::option::Option::<()>"
                }
              ],
              "state_mutability": "external"
            },
            {
              "type": "function",
              "name": "foo_external",
              "inputs": [
                {
                  "name": "a",
                  "type": "core::felt"
                },
                {
                  "name": "b",
                  "type": "core::integer::u128"
                }
              ],
              "outputs": [
                {
                  "type": "core::option::Option::<()>"
                }
              ],
              "state_mutability": "external"
            },
            {
              "type": "function",
              "name": "foo_view",
              "inputs": [
                {
                  "name": "a",
                  "type": "core::felt"
                },
                {
                  "name": "b",
                  "type": "core::integer::u128"
                }
              ],
              "outputs": [
                {
                  "type": "core::option::Option::<()>"
                }
              ],
              "state_mutability": "view"
            },
            {
              "type": "function",
              "name": "empty",
              "inputs": [],
              "outputs": [],
              "state_mutability": "external"
            },
            {
              "type": "event",
              "name": "foo_event",
              "inputs": [
                {
                  "name": "a",
                  "type": "core::felt"
                },
                {
                  "name": "b",
                  "type": "core::integer::u128"
                }
              ]
            }
          ]"#}
    );
}
