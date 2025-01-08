use nu_plugin::PluginCommand;
use nu_protocol::{Category, LabeledError, PipelineData, Signature, SyntaxShape, Type, Value};

use crate::RoaringPlugin;

use super::{utils, RoaringCommand};

pub struct ListCommand;

impl nu_plugin::PluginCommand for ListCommand {
    type Plugin = RoaringPlugin;

    fn name(&self) -> &str {
        "roaring list"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(PluginCommand::name(self))
            .allow_variants_without_examples(true)
            .input_output_types(vec![
                (Type::Binary, Type::List(Box::new(Type::Int))),
                (
                    Type::Custom("RoaringBitmap".into()),
                    Type::List(Box::new(Type::Int)),
                ),
            ])
            .optional("range", SyntaxShape::Range, "the range of elements to list")
            .category(Category::Experimental)
            .filter()
    }

    fn description(&self) -> &str {
        "List the elements in a Roaring Bitmap"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let v = call.opt(0)?;
        let r = match v {
            Some(Value::Range { val, .. }) => Some(utils::to_u32_range(val)?),
            None => None,
            _ => return Err(LabeledError::new("Expected a range")),
        };
        RoaringCommand::list(call, input, r)
    }
}
