mod contains;
mod len;
mod list;
mod new;
mod serialize;
mod utils;

use roaring::RoaringBitmap;
use std::ops;

pub use contains::ContainsCommand;
pub use len::LenCommand;
pub use list::ListCommand;
pub use new::NewCommand;
use nu_protocol::{
    CustomValue, LabeledError, PipelineData, PipelineMetadata, ShellError, Span, Value,
};
use serde::{Deserialize, Serialize};
pub use serialize::SerializeCommand;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RoaringCustomValue {
    rb: roaring::RoaringBitmap,
}

#[typetag::serde]
impl CustomValue for RoaringCustomValue {
    fn clone_value(&self, span: Span) -> Value {
        Value::custom(Box::new(self.clone()), span)
    }

    fn type_name(&self) -> String {
        "RoaringBitmap".to_string()
    }

    fn to_base_value(&self, span: Span) -> Result<Value, ShellError> {
        let mut bytes = vec![];

        self.rb
            .serialize_into(&mut bytes)
            .map_err(|e| ShellError::GenericError {
                error: e.to_string(),
                msg: "Failed to serialize Roaring Bitmap".to_string(),
                span: Some(span),
                help: None,
                inner: vec![],
            })?;
        Ok(Value::binary(bytes, span))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

struct RoaringCommand;

impl RoaringCommand {
    fn new(
        call: &nu_plugin::EvaluatedCall,
        input: PipelineData,
    ) -> Result<(RoaringBitmap, Span, Option<PipelineMetadata>), LabeledError> {
        match input {
            PipelineData::Value(Value::Range { val, internal_span }, pipeline_metadata) => {
                let rb = RoaringBitmap::from_iter(utils::to_u32_range(val)?);
                Ok((rb, internal_span, pipeline_metadata))
            }
            PipelineData::Value(
                Value::List {
                    vals,
                    internal_span,
                },
                pipeline_metadata,
            ) => {
                let mut rb = RoaringBitmap::new();
                for v in vals {
                    match v {
                        Value::Range { val, .. } => {
                            rb.insert_range(utils::to_u32_range(val)?);
                        }
                        Value::Int { val, .. } => {
                            rb.insert(val as u32);
                        }
                        _ => {}
                    }
                }
                Ok((rb, internal_span, pipeline_metadata))
            }
            PipelineData::ByteStream(byte_stream, pipeline_metadata) => {
                let span = byte_stream.span();
                let reader = byte_stream
                    .reader()
                    .ok_or_else(|| LabeledError::new("Failed to get reader from byte stream"))?;

                let rb = RoaringBitmap::deserialize_from(reader).map_err(|e| {
                    LabeledError::new(e.to_string())
                        .with_label("Failed to deserialize Roaring Bitmap", span)
                })?;
                Ok((rb, span, pipeline_metadata))
            }

            PipelineData::Value(Value::Binary { val, internal_span }, pipeline_metadata) => {
                let rb = RoaringBitmap::deserialize_from(val.as_slice()).map_err(|e| {
                    LabeledError::new(e.to_string())
                        .with_label("Failed to deserialize Roaring Bitmap", internal_span)
                })?;
                Ok((rb, internal_span, pipeline_metadata))
            }

            PipelineData::Value(Value::Custom { val, internal_span }, pipeline_metadata) => {
                if let Some(custom_value) = val.as_any().downcast_ref::<RoaringCustomValue>() {
                    Ok((custom_value.rb.clone(), internal_span, pipeline_metadata))
                } else {
                    Err(
                        LabeledError::new("Failed to downcast to RoaringCustomValue")
                            .with_label("Invalid custom value", internal_span),
                    )
                }
            }
            v => {
                return Err(LabeledError::new(format!(
                    "requires binary input, got {}",
                    v.get_type()
                ))
                .with_label("Expected binary from pipeline", call.head))
            }
        }
    }

    fn len(
        call: &nu_plugin::EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, nu_protocol::LabeledError> {
        let (rb, span, pipeline_metadata) = Self::new(call, input)?;
        Ok(PipelineData::Value(
            Value::int(rb.len() as i64, span),
            pipeline_metadata,
        ))
    }

    fn list(
        call: &nu_plugin::EvaluatedCall,
        input: PipelineData,
        range: Option<ops::Range<u32>>,
    ) -> Result<PipelineData, nu_protocol::LabeledError> {
        let (rb, span, pipeline_metadata) = Self::new(call, input)?;
        let values = match range {
            Some(range) => rb
                .range(range)
                .map(|x| Value::int(x as i64, span))
                .collect(),
            None => rb.iter().map(|x| Value::int(x as i64, span)).collect(),
        };
        Ok(PipelineData::Value(
            Value::list(values, span),
            pipeline_metadata,
        ))
    }

    fn contains(
        call: &nu_plugin::EvaluatedCall,
        input: PipelineData,
        range: ops::Range<u32>,
    ) -> Result<PipelineData, nu_protocol::LabeledError> {
        let (rb, span, pipeline_metadata) = Self::new(call, input)?;
        Ok(PipelineData::Value(
            Value::bool(rb.contains_range(range), span),
            pipeline_metadata,
        ))
    }

    fn serialize(
        call: &nu_plugin::EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, nu_protocol::LabeledError> {
        match input {
            PipelineData::Value(Value::Custom { val, internal_span }, pipeline_metadata) => {
                if let Some(custom_value) = val.as_any().downcast_ref::<RoaringCustomValue>() {
                    let mut bytes = vec![];
                    custom_value.rb.serialize_into(&mut bytes).map_err(|e| {
                        LabeledError::new(e.to_string())
                            .with_label("Failed to serialize Roaring Bitmap", internal_span)
                    })?;
                    Ok(PipelineData::Value(
                        Value::binary(bytes, internal_span),
                        pipeline_metadata,
                    ))
                } else {
                    Err(
                        LabeledError::new("Failed to downcast to RoaringCustomValue")
                            .with_label("Invalid custom value", internal_span),
                    )
                }
            }
            v => {
                return Err(LabeledError::new(format!(
                    "requires RoaringCustomValue input, got {}",
                    v.get_type()
                ))
                .with_label("Expected RoaringCustomValue from pipeline", call.head))
            }
        }
    }
}
