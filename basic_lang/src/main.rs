extern crate rand;

mod vars;
mod functions;
mod structfun;
mod enumfun;
mod listsfun;
mod core_data_types;
mod unsafe_global;
mod if_stmt;
mod loops;
mod match_stmt;
mod union_dt;
mod option_t;
mod array_fun;
mod vector_fun;
mod slice_fun;
mod strings_fun;
mod tuples_fun;
mod pattern_matecher_fun;
mod generics_fun;
mod closure_fun;
mod high_order_functions;
mod traits_fun;
mod dynamic_dispatch_fun;

fn main() {
    println!("Hello, world!");
    vars::execute();
    functions::execute();
    structfun::print_struct();
    enumfun::execute();
    listsfun::execute();
    core_data_types::execute();
    unsafe_global::execute();
    if_stmt::execute();
    loops::execute();
    match_stmt::execute();
    union_dt::execute();
    option_t::execute();
    array_fun::execute();
    vector_fun::execute();
    slice_fun::execute();
    strings_fun::execute();
    tuples_fun::execute();
    pattern_matecher_fun::execute();
    generics_fun::execute();
    closure_fun::execute();
    high_order_functions::execute();
    traits_fun::execute();
    dynamic_dispatch_fun::execute();
}
