use nu_plugin::Plugin;
mod command;

pub struct RoaringPlugin;

impl Plugin for RoaringPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(command::NewCommand),
            Box::new(command::LenCommand),
            Box::new(command::ContainsCommand),
            Box::new(command::ListCommand),
            Box::new(command::SerializeCommand),
        ]
    }
}
