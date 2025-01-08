use nu_plugin::PluginCommand;
use nu_protocol::{Category, LabeledError, PipelineData, Signature, SyntaxShape, Type, Value};

use crate::RoaringPlugin;

use super::{utils, RoaringCommand};

pub struct ContainsCommand;

impl nu_plugin::PluginCommand for ContainsCommand {
    type Plugin = RoaringPlugin;

    fn name(&self) -> &str {
        "roaring contains"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(PluginCommand::name(self))
            .allow_variants_without_examples(true)
            .input_output_types(vec![
                (Type::Binary, Type::Bool),
                (Type::Custom("RoaringBitmap".into()), Type::Bool),
            ])
            .required(
                "value",
                SyntaxShape::OneOf(vec![SyntaxShape::Int, SyntaxShape::Range]),
                "the value to check for in the Roaring Bitmap",
            )
            .category(Category::Experimental)
            .filter()
    }

    fn description(&self) -> &str {
        "Check if a Roaring Bitmap contains a value"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let v = call.req(0)?;
        let r = match v {
            Value::Int {
                val,
                internal_span: _,
            } => val as u32..(val + 1) as u32,
            Value::Range {
                val,
                internal_span: _,
            } => utils::to_u32_range(val)?,
            _ => return Err(LabeledError::new("Expected an integer or range")),
        };

        RoaringCommand::contains(call, input, r)
    }
}
