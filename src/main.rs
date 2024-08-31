use crate::control_flow::control_flow_start;
use crate::functions::functions_start;

mod data_types;
mod functions;
mod shadowing;
mod control_flow;

fn main() {
    functions_start();
    control_flow_start();
}
