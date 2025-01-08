use nu_plugin::PluginCommand;
use nu_protocol::{Category, LabeledError, PipelineData, Signature, Type, Value};

use crate::RoaringPlugin;

use super::{RoaringCommand, RoaringCustomValue};

pub struct NewCommand;

impl nu_plugin::PluginCommand for NewCommand {
    type Plugin = RoaringPlugin;

    fn name(&self) -> &str {
        "roaring new"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(PluginCommand::name(self))
            .allow_variants_without_examples(true)
            .input_output_types(vec![
                (
                    Type::List(Box::new(Type::Any)),
                    Type::Custom("RoaringBitmap".into()),
                ),
                (Type::Range, Type::Custom("RoaringBitmap".into())),
                (Type::Binary, Type::Custom("RoaringBitmap".into())),
            ])
            .category(Category::Experimental)
            .filter()
    }

    fn description(&self) -> &str {
        "new roaring bitmap"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let (rb, span, pipeline_metadata) = RoaringCommand::new(call, input)?;
        Ok(PipelineData::Value(
            Value::custom(Box::new(RoaringCustomValue { rb }), span),
            pipeline_metadata,
        ))
    }
}
