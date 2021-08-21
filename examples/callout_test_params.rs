use scripttemplate::*;

struct Script;

const DEFAULT_UUID: &str = "976d8cc2-4bfc-43b9-b9f9-f4af2de91ab9";

impl ScriptFunctions for Script {
    fn pre_start(&self, opts: ScriptOpts) -> i32 {
        assert_eq!(opts.mdev_type, "test_type_start");
        assert_eq!(opts.state, CommandState::None);
        assert_eq!(opts.uuid.to_string(), DEFAULT_UUID);
        assert_eq!(opts.parent, "test_parent_start");
        return 0;
    }
    fn post_start(&self, opts: ScriptOpts) -> i32 {
        assert_eq!(opts.mdev_type, "test_type_start");
        assert_eq!(opts.state, CommandState::Success);
        assert_eq!(opts.uuid.to_string(), DEFAULT_UUID);
        assert_eq!(opts.parent, "test_parent_start");
        return 0;
    }
    fn pre_define(&self, opts: ScriptOpts) -> i32 {
        assert_eq!(opts.mdev_type, "test_type_define");
        assert_eq!(opts.state, CommandState::None);
        assert_eq!(opts.uuid.to_string(), DEFAULT_UUID);
        assert_eq!(opts.parent, "test_parent_define");
        return 0;
    }
    fn post_define(&self, opts: ScriptOpts) -> i32 {
        assert_eq!(opts.mdev_type, "test_type_define");
        assert_eq!(opts.state, CommandState::Success);
        assert_eq!(opts.uuid.to_string(), DEFAULT_UUID);
        assert_eq!(opts.parent, "test_parent_define");
        return 0;
    }
    fn pre_modify(&self, opts: ScriptOpts) -> i32 {
        assert_eq!(opts.mdev_type, "test_type_modify");
        assert_eq!(opts.state, CommandState::None);
        assert_eq!(opts.uuid.to_string(), DEFAULT_UUID);
        assert_eq!(opts.parent, "test_parent_modify");
        return 0;
    }
    fn post_modify(&self, opts: ScriptOpts) -> i32 {
        assert_eq!(opts.mdev_type, "test_type_modify");
        assert_eq!(opts.state, CommandState::Success);
        assert_eq!(opts.uuid.to_string().to_string(), DEFAULT_UUID);
        assert_eq!(opts.parent, "test_parent_modify");
        return 0;
    }
    fn pre_stop(&self, opts: ScriptOpts) -> i32 {
        assert_eq!(opts.mdev_type, "test_type_stop");
        assert_eq!(opts.state, CommandState::None);
        assert_eq!(opts.uuid.to_string(), DEFAULT_UUID);
        assert_eq!(opts.parent, "test_parent_stop");
        return 0;
    }
    fn post_stop(&self, opts: ScriptOpts) -> i32 {
        assert_eq!(opts.mdev_type, "test_type_stop");
        assert_eq!(opts.state, CommandState::Success);
        assert_eq!(opts.uuid.to_string(), DEFAULT_UUID);
        assert_eq!(opts.parent, "test_parent_stop");
        return 0;
    }
    fn pre_undefine(&self, opts: ScriptOpts) -> i32 {
        assert_eq!(opts.mdev_type, "test_type_undefine");
        assert_eq!(opts.state, CommandState::None);
        assert_eq!(opts.uuid.to_string(), DEFAULT_UUID);
        assert_eq!(opts.parent, "test_parent_undefine");
        return 0;
    }
    fn post_undefine(&self, opts: ScriptOpts) -> i32 {
        assert_eq!(opts.mdev_type, "test_type_undefine");
        assert_eq!(opts.state, CommandState::Success);
        assert_eq!(opts.uuid.to_string(), DEFAULT_UUID);
        assert_eq!(opts.parent, "test_parent_undefine");
        return 0;
    }
}

fn main() {
    run_script_and_exit("", "", &Script);
}
