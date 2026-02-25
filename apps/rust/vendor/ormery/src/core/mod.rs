#![allow(warnings)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            "TODO get console";
            let DEBUG__84: bool = false;
            Ok(())
    }).clone()
}
struct CollectorStruct<PART: Clone + std::marker::Send + std::marker::Sync + 'static> {
    parts_builder: temper_core::ListBuilder<Collected<PART>>, phantom_PART: std::marker::PhantomData<PART>
}
#[derive(Clone)]
pub struct Collector<PART: Clone + std::marker::Send + std::marker::Sync + 'static>(std::sync::Arc<CollectorStruct<PART>>);
impl<PART: Clone + std::marker::Send + std::marker::Sync + 'static> Collector<PART> {
    pub fn append_safe(& self, fixed__87: impl temper_core::ToArcString) {
        let fixed__87 = fixed__87.to_arc_string();
        let mut t___519: CollectedFixed<PART> = CollectedFixed::new(fixed__87.clone());
        temper_core::listed::add( & self.0.parts_builder, Collected::new(t___519.clone()), None);
    }
    pub fn append(& self, part__90: PART) {
        let mut t___517: CollectedPart<PART> = CollectedPart::new(part__90.clone());
        temper_core::listed::add( & self.0.parts_builder, Collected::new(t___517.clone()), None);
    }
    pub fn parts(& self) -> temper_core::List<Collected<PART>> {
        return temper_core::ListedTrait::to_list( & self.0.parts_builder);
    }
    pub fn new() -> Collector<PART> {
        let parts_builder;
        let mut t___515: temper_core::ListBuilder<Collected<PART>> = temper_core::listed::new_builder();
        parts_builder = t___515.clone();
        let selfish = Collector(std::sync::Arc::new(CollectorStruct {
                    parts_builder, phantom_PART: std::marker::PhantomData
        }));
        return selfish;
    }
}
temper_core::impl_any_value_trait!(Collector<PART>, []);
pub enum CollectedEnum {
    CollectedFixed(CollectedFixed), CollectedPart(CollectedPart)
}
pub trait CollectedTrait<PART: Clone + std::marker::Send + std::marker::Sync + 'static>: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> CollectedEnum;
    fn clone_boxed(& self) -> Collected<PART>;
}
#[derive(Clone)]
pub struct Collected<PART: Clone + std::marker::Send + std::marker::Sync + 'static>(std::sync::Arc<dyn CollectedTrait<PART>>);
impl<PART: Clone + std::marker::Send + std::marker::Sync + 'static> Collected<PART> {
    pub fn new(selfish: impl CollectedTrait<PART> + 'static) -> Collected<PART> {
        Collected(std::sync::Arc::new(selfish))
    }
}
temper_core::impl_any_value_trait_for_interface!(Collected<PART>);
impl<PART: Clone + std::marker::Send + std::marker::Sync + 'static> std::ops::Deref for Collected<PART> {
    type Target = dyn CollectedTrait<PART>;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct CollectedFixedStruct<PART: Clone + std::marker::Send + std::marker::Sync + 'static> {
    safe_text: std::sync::Arc<String>, phantom_PART: std::marker::PhantomData<PART>
}
#[derive(Clone)]
pub struct CollectedFixed<PART: Clone + std::marker::Send + std::marker::Sync + 'static>(std::sync::Arc<CollectedFixedStruct<PART>>);
impl<PART: Clone + std::marker::Send + std::marker::Sync + 'static> CollectedFixed<PART> {
    pub fn new(safeText__97: impl temper_core::ToArcString) -> CollectedFixed<PART> {
        let safeText__97 = safeText__97.to_arc_string();
        let safe_text;
        safe_text = safeText__97.clone();
        let selfish = CollectedFixed(std::sync::Arc::new(CollectedFixedStruct {
                    safe_text, phantom_PART: std::marker::PhantomData
        }));
        return selfish;
    }
    pub fn safe_text(& self) -> std::sync::Arc<String> {
        return self.0.safe_text.clone();
    }
}
impl<PART: Clone + std::marker::Send + std::marker::Sync + 'static> CollectedTrait<PART> for CollectedFixed<PART> {
    fn as_enum(& self) -> CollectedEnum {
        CollectedEnum::CollectedFixed(self.clone())
    }
    fn clone_boxed(& self) -> Collected<PART> {
        Collected::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(CollectedFixed<PART>, [Collected<PART>]);
struct CollectedPartStruct<PART: Clone + std::marker::Send + std::marker::Sync + 'static> {
    part: PART, phantom_PART: std::marker::PhantomData<PART>
}
#[derive(Clone)]
pub struct CollectedPart<PART: Clone + std::marker::Send + std::marker::Sync + 'static>(std::sync::Arc<CollectedPartStruct<PART>>);
impl<PART: Clone + std::marker::Send + std::marker::Sync + 'static> CollectedPart<PART> {
    pub fn new(part__100: PART) -> CollectedPart<PART> {
        let part;
        part = part__100.clone();
        let selfish = CollectedPart(std::sync::Arc::new(CollectedPartStruct {
                    part, phantom_PART: std::marker::PhantomData
        }));
        return selfish;
    }
    pub fn part(& self) -> PART {
        return self.0.part.clone();
    }
}
impl<PART: Clone + std::marker::Send + std::marker::Sync + 'static> CollectedTrait<PART> for CollectedPart<PART> {
    fn as_enum(& self) -> CollectedEnum {
        CollectedEnum::CollectedPart(self.clone())
    }
    fn clone_boxed(& self) -> Collected<PART> {
        Collected::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(CollectedPart<PART>, [Collected<PART>]);
pub trait ContextTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn clone_boxed(& self) -> Context;
    fn to_string(& self) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct Context(std::sync::Arc<dyn ContextTrait>);
impl Context {
    pub fn new(selfish: impl ContextTrait + 'static) -> Context {
        Context(std::sync::Arc::new(selfish))
    }
}
temper_core::impl_any_value_trait_for_interface!(Context);
impl std::ops::Deref for Context {
    type Target = dyn ContextTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct AutoescStateStruct<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static> {
    context: CONTEXT, subsidiary: Option<Subsidiary>, phantom_CONTEXT: std::marker::PhantomData<CONTEXT>
}
#[derive(Clone)]
pub struct AutoescState<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static>(std::sync::Arc<AutoescStateStruct<CONTEXT>>);
#[derive(Clone)]
pub struct AutoescStateBuilder<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static> {
    pub context: CONTEXT, pub subsidiary: Option<Subsidiary>
}
impl<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static> AutoescStateBuilder<CONTEXT> {
    pub fn build(self) -> AutoescState<CONTEXT> {
        AutoescState::new(self.context, self.subsidiary)
    }
}
impl<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static> AutoescState<CONTEXT> {
    pub fn new(context__106: CONTEXT, subsidiary__107: Option<Subsidiary>) -> AutoescState<CONTEXT> {
        let context;
        let subsidiary;
        context = context__106.clone();
        subsidiary = subsidiary__107.clone();
        let selfish = AutoescState(std::sync::Arc::new(AutoescStateStruct {
                    context, subsidiary, phantom_CONTEXT: std::marker::PhantomData
        }));
        return selfish;
    }
    pub fn context(& self) -> CONTEXT {
        return self.0.context.clone();
    }
    pub fn subsidiary(& self) -> Option<Subsidiary> {
        return self.0.subsidiary.clone();
    }
}
temper_core::impl_any_value_trait!(AutoescState<CONTEXT>, []);
struct AfterPropagateStruct<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static> {
    adjusted_string: std::sync::Arc<String>, consumed: usize, state_after: AutoescState<CONTEXT>, phantom_CONTEXT: std::marker::PhantomData<CONTEXT>
}
#[derive(Clone)]
pub struct AfterPropagate<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static>(std::sync::Arc<AfterPropagateStruct<CONTEXT>>);
#[derive(Clone)]
pub struct AfterPropagateBuilder<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static> {
    pub adjusted_string: std::sync::Arc<String>, pub consumed: usize, pub state_after: AutoescState<CONTEXT>
}
impl<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static> AfterPropagateBuilder<CONTEXT> {
    pub fn build(self) -> AfterPropagate<CONTEXT> {
        AfterPropagate::new(self.adjusted_string, self.consumed, self.state_after)
    }
}
impl<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static> AfterPropagate<CONTEXT> {
    pub fn push(& self, delegate__112: Delegate, codec__113: Codec) -> AfterPropagate<CONTEXT> {
        return AfterPropagate::new(self.0.adjusted_string.clone(), self.0.consumed, AutoescState::new(self.0.state_after.context(), Some(Subsidiary::new(delegate__112.clone(), codec__113.clone()))));
    }
    pub fn pop(& self) -> AfterPropagate<CONTEXT> {
        return AfterPropagate::new(self.0.adjusted_string.clone(), self.0.consumed, AutoescState::new(self.0.state_after.context(), None));
    }
    pub fn feed(& self, prepareForInterp__118: bool) -> AfterPropagate<CONTEXT> {
        let return__57: AfterPropagate<CONTEXT>;
        let subsidiary__120: Option<Subsidiary> = self.0.state_after.subsidiary();
        if ! subsidiary__120.is_none() {
            let subsidiary___237: Subsidiary = subsidiary__120.clone().unwrap();
            let adjustedFromDelegate__121: std::sync::Arc<String> = feedSubsidiary__83(subsidiary___237.clone(), self.0.adjusted_string.clone(), prepareForInterp__118);
            return__57 = AfterPropagate::new(adjustedFromDelegate__121.clone(), self.0.consumed, self.0.state_after.clone());
        } else {
            return__57 = self.clone();
        }
        return return__57.clone();
    }
    pub fn new(adjustedString__123: impl temper_core::ToArcString, consumed__124: usize, stateAfter__125: AutoescState<CONTEXT>) -> AfterPropagate<CONTEXT> {
        let adjustedString__123 = adjustedString__123.to_arc_string();
        let adjusted_string;
        let consumed;
        let state_after;
        adjusted_string = adjustedString__123.clone();
        consumed = consumed__124;
        state_after = stateAfter__125.clone();
        let selfish = AfterPropagate(std::sync::Arc::new(AfterPropagateStruct {
                    adjusted_string, consumed, state_after, phantom_CONTEXT: std::marker::PhantomData
        }));
        return selfish;
    }
    pub fn adjusted_string(& self) -> std::sync::Arc<String> {
        return self.0.adjusted_string.clone();
    }
    pub fn consumed(& self) -> usize {
        return self.0.consumed;
    }
    pub fn state_after(& self) -> AutoescState<CONTEXT> {
        return self.0.state_after.clone();
    }
}
temper_core::impl_any_value_trait!(AfterPropagate<CONTEXT>, []);
pub trait ContextPropagatorTrait<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static>: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn clone_boxed(& self) -> ContextPropagator<CONTEXT>;
    fn after(& self, before__127: AutoescState<CONTEXT>, literalPart__128: Option<std::sync::Arc<String>>) -> AfterPropagate<CONTEXT>;
}
#[derive(Clone)]
pub struct ContextPropagator<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static>(std::sync::Arc<dyn ContextPropagatorTrait<CONTEXT>>);
impl<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static> ContextPropagator<CONTEXT> {
    pub fn new(selfish: impl ContextPropagatorTrait<CONTEXT> + 'static) -> ContextPropagator<CONTEXT> {
        ContextPropagator(std::sync::Arc::new(selfish))
    }
}
temper_core::impl_any_value_trait_for_interface!(ContextPropagator<CONTEXT>);
impl<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static> std::ops::Deref for ContextPropagator<CONTEXT> {
    type Target = dyn ContextPropagatorTrait<CONTEXT>;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
pub trait DelegateTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn clone_boxed(& self) -> Delegate;
    fn process(& self, fixed__131: Option<std::sync::Arc<String>>) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct Delegate(std::sync::Arc<dyn DelegateTrait>);
impl Delegate {
    pub fn new(selfish: impl DelegateTrait + 'static) -> Delegate {
        Delegate(std::sync::Arc::new(selfish))
    }
}
temper_core::impl_any_value_trait_for_interface!(Delegate);
impl std::ops::Deref for Delegate {
    type Target = dyn DelegateTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
pub trait ContextDelegateTrait<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static>: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync + DelegateTrait {
    fn clone_boxed(& self) -> ContextDelegate<CONTEXT>;
    fn state(& self) -> AutoescState<CONTEXT>;
    fn set_state(& self, x__136: AutoescState<CONTEXT>);
    fn context_propagator(& self) -> ContextPropagator<CONTEXT>;
    fn process(& self, known__141: Option<std::sync::Arc<String>>) -> std::sync::Arc<String> {
        let after__143: AfterPropagate<CONTEXT> = propagateOver(self.context_propagator(), self.state(), known__141.clone());
        let mut t___479: AutoescState<CONTEXT> = after__143.state_after();
        self.set_state(t___479.clone());
        return after__143.adjusted_string();
    }
}
#[derive(Clone)]
pub struct ContextDelegate<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static>(std::sync::Arc<dyn ContextDelegateTrait<CONTEXT>>);
impl<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static> ContextDelegate<CONTEXT> {
    pub fn new(selfish: impl ContextDelegateTrait<CONTEXT> + 'static) -> ContextDelegate<CONTEXT> {
        ContextDelegate(std::sync::Arc::new(selfish))
    }
}
temper_core::impl_any_value_trait_for_interface!(ContextDelegate<CONTEXT>);
impl<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static> std::ops::Deref for ContextDelegate<CONTEXT> {
    type Target = dyn ContextDelegateTrait<CONTEXT>;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
pub trait EscaperTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn clone_boxed(& self) -> Escaper;
}
#[derive(Clone)]
pub struct Escaper(std::sync::Arc<dyn EscaperTrait>);
impl Escaper {
    pub fn new(selfish: impl EscaperTrait + 'static) -> Escaper {
        Escaper(std::sync::Arc::new(selfish))
    }
}
temper_core::impl_any_value_trait_for_interface!(Escaper);
impl std::ops::Deref for Escaper {
    type Target = dyn EscaperTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
pub trait EscaperPickerTrait<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static, ESC: EscaperTrait + Clone + std::marker::Send + std::marker::Sync + 'static>: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn clone_boxed(& self) -> EscaperPicker<CONTEXT, ESC>;
    fn escaper_for(& self, before__153: AutoescState<CONTEXT>) -> ESC;
}
#[derive(Clone)]
pub struct EscaperPicker<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static, ESC: EscaperTrait + Clone + std::marker::Send + std::marker::Sync + 'static>(std::sync::Arc<dyn EscaperPickerTrait<CONTEXT, ESC>>);
impl<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static, ESC: EscaperTrait + Clone + std::marker::Send + std::marker::Sync + 'static> EscaperPicker<CONTEXT, ESC> {
    pub fn new(selfish: impl EscaperPickerTrait<CONTEXT, ESC> + 'static) -> EscaperPicker<CONTEXT, ESC> {
        EscaperPicker(std::sync::Arc::new(selfish))
    }
}
temper_core::impl_any_value_trait_for_interface!(EscaperPicker<CONTEXT, ESC>);
impl<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static, ESC: EscaperTrait + Clone + std::marker::Send + std::marker::Sync + 'static> std::ops::Deref for EscaperPicker<CONTEXT, ESC> {
    type Target = dyn EscaperPickerTrait<CONTEXT, ESC>;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
pub trait ContextualAutoescapingAccumulatorTrait<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static, ESC: EscaperTrait + Clone + std::marker::Send + std::marker::Sync + 'static>: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn clone_boxed(& self) -> ContextualAutoescapingAccumulator<CONTEXT, ESC>;
    fn state(& self) -> AutoescState<CONTEXT>;
    fn set_state(& self, newState__158: AutoescState<CONTEXT>);
    fn escaper_picker(& self) -> EscaperPicker<CONTEXT, ESC>;
    fn context_propagator(& self) -> ContextPropagator<CONTEXT>;
    fn prepare_for_append(& self) -> ESC {
        let after__166: AfterPropagate<CONTEXT> = propagateOver(self.context_propagator(), self.state(), None::<std::sync::Arc<String>>);
        let mut t___467: AutoescState<CONTEXT> = after__166.state_after();
        self.set_state(t___467.clone());
        let adjusted__167: std::sync::Arc<String> = after__166.adjusted_string();
        if ! adjusted__167.is_empty() {
            self.collect_fixed(adjusted__167.clone());
        }
        return self.escaper_picker().escaper_for(self.state());
    }
    fn append_safe(& self, known__169: std::sync::Arc<String>) {
        let after__171: AfterPropagate<CONTEXT> = propagateOver(self.context_propagator(), self.state(), Some(known__169.clone()));
        let mut t___458: AutoescState<CONTEXT> = after__171.state_after();
        self.set_state(t___458.clone());
        let adjusted__172: std::sync::Arc<String> = after__171.adjusted_string();
        if ! adjusted__172.is_empty() {
            self.collect_fixed(adjusted__172.clone());
        }
    }
    fn collect_fixed(& self, fixedFragment__174: std::sync::Arc<String>);
}
#[derive(Clone)]
pub struct ContextualAutoescapingAccumulator<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static, ESC: EscaperTrait + Clone + std::marker::Send + std::marker::Sync + 'static>(std::sync::Arc<dyn ContextualAutoescapingAccumulatorTrait<CONTEXT, ESC>>);
impl<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static, ESC: EscaperTrait + Clone + std::marker::Send + std::marker::Sync + 'static> ContextualAutoescapingAccumulator<CONTEXT, ESC> {
    pub fn new(selfish: impl ContextualAutoescapingAccumulatorTrait<CONTEXT, ESC> + 'static) -> ContextualAutoescapingAccumulator<CONTEXT, ESC> {
        ContextualAutoescapingAccumulator(std::sync::Arc::new(selfish))
    }
}
temper_core::impl_any_value_trait_for_interface!(ContextualAutoescapingAccumulator<CONTEXT, ESC>);
impl<CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static, ESC: EscaperTrait + Clone + std::marker::Send + std::marker::Sync + 'static> std::ops::Deref for ContextualAutoescapingAccumulator<CONTEXT, ESC> {
    type Target = dyn ContextualAutoescapingAccumulatorTrait<CONTEXT, ESC>;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
pub trait CodecTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn clone_boxed(& self) -> Codec;
    fn encode(& self, s__184: std::sync::Arc<String>) -> std::sync::Arc<String>;
    fn decode(& self, s__187: std::sync::Arc<String>) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct Codec(std::sync::Arc<dyn CodecTrait>);
impl Codec {
    pub fn new(selfish: impl CodecTrait + 'static) -> Codec {
        Codec(std::sync::Arc::new(selfish))
    }
}
temper_core::impl_any_value_trait_for_interface!(Codec);
impl std::ops::Deref for Codec {
    type Target = dyn CodecTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct SubsidiaryStruct {
    delegate: Delegate, codec: Codec
}
#[derive(Clone)]
pub struct Subsidiary(std::sync::Arc<SubsidiaryStruct>);
#[derive(Clone)]
pub struct SubsidiaryBuilder {
    pub delegate: Delegate, pub codec: Codec
}
impl SubsidiaryBuilder {
    pub fn build(self) -> Subsidiary {
        Subsidiary::new(self.delegate, self.codec)
    }
}
impl Subsidiary {
    pub fn new(delegate__192: Delegate, codec__193: Codec) -> Subsidiary {
        let delegate;
        let codec;
        delegate = delegate__192.clone();
        codec = codec__193.clone();
        let selfish = Subsidiary(std::sync::Arc::new(SubsidiaryStruct {
                    delegate, codec
        }));
        return selfish;
    }
    pub fn delegate(& self) -> Delegate {
        return self.0.delegate.clone();
    }
    pub fn codec(& self) -> Codec {
        return self.0.codec.clone();
    }
}
temper_core::impl_any_value_trait!(Subsidiary, []);
fn feedSubsidiary__83(subsidiary__176: Subsidiary, adjustedStr__177: impl temper_core::ToArcString, prepareForInterp__178: bool) -> std::sync::Arc<String> {
    let adjustedStr__177 = adjustedStr__177.to_arc_string();
    let mut t___513: std::sync::Arc<String>;
    let mut str__180: std::sync::Arc<String> = adjustedStr__177.clone();
    let delegate__181: Delegate = subsidiary__176.delegate();
    let codec__182: Codec = subsidiary__176.codec();
    let mut t___511: std::sync::Arc<String> = codec__182.decode(str__180.clone());
    str__180 = t___511.clone();
    let mut t___512: std::sync::Arc<String> = delegate__181.process(Some(str__180.clone()));
    str__180 = t___512.clone();
    if prepareForInterp__178 {
        t___513 = delegate__181.process(None);
        str__180 = std::sync::Arc::new(format!("{}{}", str__180, t___513.clone()));
    }
    let mut t___514: std::sync::Arc<String> = codec__182.encode(str__180.clone());
    str__180 = t___514.clone();
    return str__180.clone();
}
pub fn propagate_over<CONTEXT>(contextPropagator__144: ContextPropagator<CONTEXT>, before__145: AutoescState<CONTEXT>, known__146: Option<impl temper_core::ToArcString>) -> AfterPropagate<CONTEXT> where CONTEXT: ContextTrait + Clone + std::marker::Send + std::marker::Sync + 'static {
    let known__146 = known__146.map(| x | x.to_arc_string());
    let return__66: AfterPropagate<CONTEXT>;
    let mut t___488: AutoescState<CONTEXT>;
    let mut t___489: usize;
    let mut t___490: usize;
    let mut t___491: std::sync::Arc<String>;
    let mut t___492: std::sync::Arc<String>;
    let mut t___493: usize;
    if known__146.is_none() {
        return__66 = contextPropagator__144.after(before__145.clone(), None);
    } else {
        let known___238: std::sync::Arc<String> = known__146.clone().unwrap();
        let mut state__148: AutoescState<CONTEXT> = before__145.clone();
        let mut remainder__149: std::sync::Arc<String> = known___238.clone();
        let adjusted__150: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        'loop___5913: loop {
            if ! ( ! remainder__149.is_empty()) {
                break;
            }
            let after__151: AfterPropagate<CONTEXT> = contextPropagator__144.after(state__148.clone(), Some(remainder__149.clone()));
            temper_core::string::builder::append( & adjusted__150, after__151.adjusted_string());
            t___488 = after__151.state_after();
            state__148 = t___488.clone();
            t___489 = after__151.consumed();
            t___490 = remainder__149.len();
            t___491 = temper_core::string::slice( & remainder__149, t___489, t___490);
            remainder__149 = t___491.clone();
        }
        t___492 = temper_core::string::builder::to_string( & adjusted__150);
        t___493 = known___238.len();
        return__66 = AfterPropagate::new(t___492.clone(), t___493, state__148.clone());
    }
    return return__66.clone();
}
