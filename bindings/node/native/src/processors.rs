extern crate tokenizers as tk;

use crate::container::Container;
use neon::prelude::*;

/// Processor
pub struct Processor {
    pub processor: Container<dyn tk::tokenizer::PostProcessor>,
}

declare_types! {
    pub class JsPostProcessor for Processor {
        init(_) {
            // This should not be called from JS
            Ok(Processor {
                processor: Container::Empty
            })
        }
    }
}

/// bert_processing(sep: [String, number], cls: [String, number])
fn bert_processing(mut cx: FunctionContext) -> JsResult<JsPostProcessor> {
    let sep = cx.argument::<JsArray>(0)?;
    let cls = cx.argument::<JsArray>(1)?;
    if sep.len() != 2 || cls.len() != 2 {
        return cx.throw_error("SEP and CLS must be of the form: [String, number]");
    }
    let sep: (String, u32) = (
        sep.get(&mut cx, 0)?
            .downcast::<JsString>()
            .or_throw(&mut cx)?
            .value(),
        sep.get(&mut cx, 1)?
            .downcast::<JsNumber>()
            .or_throw(&mut cx)?
            .value() as u32,
    );
    let cls: (String, u32) = (
        cls.get(&mut cx, 0)?
            .downcast::<JsString>()
            .or_throw(&mut cx)?
            .value(),
        cls.get(&mut cx, 1)?
            .downcast::<JsNumber>()
            .or_throw(&mut cx)?
            .value() as u32,
    );

    let mut processor = JsPostProcessor::new::<_, JsPostProcessor, _>(&mut cx, vec![])?;
    let guard = cx.lock();
    processor.borrow_mut(&guard).processor.to_owned(Box::new(
        tk::processors::bert::BertProcessing::new(sep, cls),
    ));
    Ok(processor)
}

/// roberta_processing(
///   sep: [String, number],
///   cls: [String, number],
///   trimOffsets: boolean = true,
///   addPrefixSpace: boolean = true
/// )
fn roberta_processing(mut cx: FunctionContext) -> JsResult<JsPostProcessor> {
    let sep = cx.argument::<JsArray>(0)?;
    let cls = cx.argument::<JsArray>(1)?;
    if sep.len() != 2 || cls.len() != 2 {
        return cx.throw_error("SEP and CLS must be of the form: [String, number]");
    }
    let sep: (String, u32) = (
        sep.get(&mut cx, 0)?
            .downcast::<JsString>()
            .or_throw(&mut cx)?
            .value(),
        sep.get(&mut cx, 1)?
            .downcast::<JsNumber>()
            .or_throw(&mut cx)?
            .value() as u32,
    );
    let cls: (String, u32) = (
        cls.get(&mut cx, 0)?
            .downcast::<JsString>()
            .or_throw(&mut cx)?
            .value(),
        cls.get(&mut cx, 1)?
            .downcast::<JsNumber>()
            .or_throw(&mut cx)?
            .value() as u32,
    );

    let mut processor = tk::processors::roberta::RobertaProcessing::new(sep, cls);
    if let Some(args) = cx.argument_opt(2) {
        if args.downcast::<JsUndefined>().is_err() {
            processor =
                processor.trim_offsets(args.downcast::<JsBoolean>().or_throw(&mut cx)?.value());
        }
    }
    if let Some(args) = cx.argument_opt(3) {
        if args.downcast::<JsUndefined>().is_err() {
            processor =
                processor.add_prefix_space(args.downcast::<JsBoolean>().or_throw(&mut cx)?.value());
        }
    }

    let mut js_processor = JsPostProcessor::new::<_, JsPostProcessor, _>(&mut cx, vec![])?;
    let guard = cx.lock();
    js_processor
        .borrow_mut(&guard)
        .processor
        .to_owned(Box::new(processor));
    Ok(js_processor)
}

/// bytelevel(trimOffsets?: boolean)
fn bytelevel(mut cx: FunctionContext) -> JsResult<JsPostProcessor> {
    let mut byte_level = tk::processors::byte_level::ByteLevel::default();

    if let Some(args) = cx.argument_opt(0) {
        if args.downcast::<JsUndefined>().is_err() {
            byte_level =
                byte_level.trim_offsets(args.downcast::<JsBoolean>().or_throw(&mut cx)?.value());
        }
    }

    let mut processor = JsPostProcessor::new::<_, JsPostProcessor, _>(&mut cx, vec![])?;
    let guard = cx.lock();
    processor
        .borrow_mut(&guard)
        .processor
        .to_owned(Box::new(byte_level));
    Ok(processor)
}

/// Register everything here
pub fn register(m: &mut ModuleContext, prefix: &str) -> NeonResult<()> {
    m.export_function(&format!("{}_BertProcessing", prefix), bert_processing)?;
    m.export_function(&format!("{}_RobertaProcessing", prefix), roberta_processing)?;
    m.export_function(&format!("{}_ByteLevel", prefix), bytelevel)?;
    Ok(())
}
