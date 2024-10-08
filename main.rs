#![allow(unused_variables)]
pub mod attributes;
pub mod conversion;
pub mod custom_types;
pub mod error_handling;
pub mod flow_control;
pub mod functions;
pub mod generics;
pub mod hello_world;
pub mod macro_rules;
pub mod modules;
pub mod primitives;
pub mod scoping_rules;
pub mod std_lib_types;
pub mod std_misc;
pub mod testing;
pub mod traits;
pub mod types;
fn main() {
    // hello_world::formatted_print::main();

    // primitives::literals_operators::main();
    // primitives::tuples::main();
    // primitives::arrays_slices::main();

    // custom_types::structures::main();
    // custom_types::enums::main();
    // custom_types::linked_list::main();
    // custom_types::constants::main();

    // types::casting::main();
    // types::literals::main();
    // types::aliasing::main();

    // conversion::from_into::main();
    // conversion::tryfrom_tryinto::main();
    // conversion::to_from_string::main();

    // flow_control::for_range::main();
    // flow_control::match_pattern::main();
    // flow_control::match_pattern::desturcturing::tuples::main();
    // flow_control::match_pattern::desturcturing::arrays::main();
    // flow_control::match_pattern::desturcturing::slices::main();
    // flow_control::match_pattern::desturcturing::enums::main();
    // flow_control::match_pattern::desturcturing::pointers_refs::main();
    // flow_control::match_pattern::desturcturing::structs::main();
    // flow_control::match_pattern::guards::main();
    // flow_control::match_pattern::binding::main();
    // flow_control::if_let::main();
    // flow_control::let_else::main();
    // flow_control::while_let::main();

    // functions::main();
    // functions::methods::main();
    // functions::closures::capturing::main();
    // functions::closures::parameters::main();
    // functions::closures::examples::main();
    // functions::high_order_functions::main();
    // functions::diverging_functions::main();

    // modules::visibility::main();
    // modules::struct_visibility::main();
    // modules::use_declaration::main();
    // modules::self_super::main();

    // attributes::main();
    // attributes::cfg::main();

    // generics::main();
    // generics::functions::main();
    // generics::implementation::main();
    // generics::traits::main();
    // generics::bounds::main();
    // generics::multi_bounds::main();
    // generics::where_clauses::main();
    // generics::newtype::main();

    // generics::associated_items::problem::main();
    // generics::associated_items::associated_types::main();
    // generics::phantom_parameter::main();
    // generics::phantom_parameter::test_case::main();

    // scoping_rules::raii::main();
    // scoping_rules::ownership_moves::main();
    // scoping_rules::ownership_moves::mutability::main();
    // scoping_rules::ownership_moves::partial_moves::main();
    // scoping_rules::borrowing::main();
    // scoping_rules::borrowing::mutability::main();
    // scoping_rules::borrowing::aliasing::main();
    // scoping_rules::borrowing::ref_pattern::main();
    // scoping_rules::lifetimes::explicit_annotations::main();
    // scoping_rules::lifetimes::functions::main();
    // scoping_rules::lifetimes::methods::main();
    // scoping_rules::lifetimes::structs::main();
    // scoping_rules::lifetimes::traits::main();
    // scoping_rules::lifetimes::bounds::main();
    // scoping_rules::lifetimes::coercion::main();
    // scoping_rules::lifetimes::statics::main();
    // scoping_rules::lifetimes::elisions::main();

    // traits::main();
    // traits::derive::main();
    // traits::return_traits_dyn::main();
    // traits::operator_overloadings::main();
    // traits::drop::main();
    // traits::iterators::main();
    // traits::impl_trait::main();
    // traits::clone::main();
    // traits::supertraits::main();
    // traits::operator_overloadings::main();
    // macro_rules::main();
    // macro_rules::syntax::designators::main();
    // macro_rules::syntax::overload::main();
    // macro_rules::syntax::repeat::main();
    // macro_rules::dsl::main();
    // macro_rules::variadic_interfaces::main();
    // error_handling::panics::main();
    // error_handling::abort_unwind::main();
    // error_handling::option_unwrap::main();
    // error_handling::option_unwrap::unpack_option::main();
    // error_handling::option_unwrap::combinator_map::main();
    // error_handling::option_unwrap::combinator_and_then::main();
    // error_handling::option_unwrap::option_default::main();
    // let _a = error_handling::results::main();
    // error_handling::results::map_for_result::main();
    // error_handling::results::aliases_for_result::main();
    // error_handling::results::early_return::main();
    // error_handling::multiple_error_types::main();
    // error_handling::multiple_error_types::pull_result_from_option::main();
    // error_handling::multiple_error_types::own_error_type::main();
    // error_handling::multiple_error_types::boxing_errors::main();
    // error_handling::multiple_error_types::use_question_mark::main();
    // error_handling::multiple_error_types::wrapping_errors::main();
    // error_handling::multiple_error_types::iterate_over_results::main();
    // std_lib_types::box_stack_heap::main();
    // std_lib_types::vectors::main();
    // std_lib_types::strings::main();
    // std_lib_types::options::main();
    // std_lib_types::results::main();
    // std_lib_types::panics::main();
    // std_lib_types::hashmaps::main();
    // std_lib_types::hashmaps::key_types::main();
    // std_lib_types::hashmaps::hashsets::main();
    // std_lib_types::reference_count::main();
    // std_lib_types::atomically_ref_counted::main();
    // std_misc::threads::main();
    // std_misc::threads::testcase::main();
    // std_misc::channels::main();
    // std_misc::paths::main();
    // std_misc::file_io::open::main();
    // std_misc::file_io::create::main();
    // std_misc::file_io::read_lines::main();
    // std_misc::child_process::main();
    // std_misc::child_process::pipes::main();
    // std_misc::child_process::wait::main();
    // std_misc::filesystem_operations::main();
    // std_misc::program_arguments::main();
    // std_misc::foreign_function_interface::main();
    testing::unit_tests::main();
}
