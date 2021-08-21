use scripttemplate::*;

struct Script;

impl ScriptFunctions for Script {}

fn main() {
    run_script_and_exit("type_b", "parent_b", &Script);
}
