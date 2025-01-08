use nu_plugin::PluginCommand;
use nu_protocol::{Category, LabeledError, PipelineData, Signature, Type};

use crate::RoaringPlugin;

use super::RoaringCommand;

pub struct LenCommand;

impl nu_plugin::PluginCommand for LenCommand {
    type Plugin = RoaringPlugin;

    fn name(&self) -> &str {
        "roaring len"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(PluginCommand::name(self))
            .allow_variants_without_examples(true)
            .input_output_types(vec![
                (Type::Binary, Type::Int),
                (Type::Custom("RoaringBitmap".into()), Type::Int),
            ])
            .category(Category::Experimental)
            .filter()
    }

    fn description(&self) -> &str {
        "Get the number of elements in a Roaring Bitmap"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        RoaringCommand::len(call, input)
    }
}
