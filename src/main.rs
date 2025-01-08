use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_roaring::RoaringPlugin;

fn main() {
    serve_plugin(&RoaringPlugin {}, MsgPackSerializer {})
}
