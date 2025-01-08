use nu_plugin::PluginCommand;
use nu_protocol::{Category, LabeledError, PipelineData, Signature, Type};

use crate::RoaringPlugin;

use super::RoaringCommand;

pub struct SerializeCommand;

impl nu_plugin::PluginCommand for SerializeCommand {
    type Plugin = RoaringPlugin;

    fn name(&self) -> &str {
        "roaring ser"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(PluginCommand::name(self))
            .allow_variants_without_examples(true)
            .input_output_types(vec![(Type::Custom("RoaringBitmap".into()), Type::Binary)])
            .category(Category::Experimental)
            .filter()
    }

    fn description(&self) -> &str {
        "Serialize a Roaring Bitmap"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        RoaringCommand::serialize(call, input)
    }
}
