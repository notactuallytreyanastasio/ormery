#![allow(warnings)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
mod support;
pub use support::*;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            CODES__AMPERSAND.set(38).unwrap_or_else(| _ | panic!());
            CODES__BACKSLASH.set(92).unwrap_or_else(| _ | panic!());
            CODES__CARET.set(94).unwrap_or_else(| _ | panic!());
            CODES__CARRIAGE_RETURN.set(13).unwrap_or_else(| _ | panic!());
            CODES__CURLY_LEFT.set(123).unwrap_or_else(| _ | panic!());
            CODES__CURLY_RIGHT.set(125).unwrap_or_else(| _ | panic!());
            CODES__DASH.set(45).unwrap_or_else(| _ | panic!());
            CODES__DOT.set(46).unwrap_or_else(| _ | panic!());
            CODES__HIGH_CONTROL_MIN.set(127).unwrap_or_else(| _ | panic!());
            CODES__HIGH_CONTROL_MAX.set(159).unwrap_or_else(| _ | panic!());
            CODES__DIGIT0.set(48).unwrap_or_else(| _ | panic!());
            CODES__DIGIT9.set(57).unwrap_or_else(| _ | panic!());
            CODES__LOWER_A.set(97).unwrap_or_else(| _ | panic!());
            CODES__LOWER_Z.set(122).unwrap_or_else(| _ | panic!());
            CODES__NEWLINE.set(10).unwrap_or_else(| _ | panic!());
            CODES__PESO.set(36).unwrap_or_else(| _ | panic!());
            CODES__PIPE.set(124).unwrap_or_else(| _ | panic!());
            CODES__PLUS.set(43).unwrap_or_else(| _ | panic!());
            CODES__QUESTION.set(63).unwrap_or_else(| _ | panic!());
            CODES__ROUND_LEFT.set(40).unwrap_or_else(| _ | panic!());
            CODES__ROUND_RIGHT.set(41).unwrap_or_else(| _ | panic!());
            CODES__SLASH.set(47).unwrap_or_else(| _ | panic!());
            CODES__SQUARE_LEFT.set(91).unwrap_or_else(| _ | panic!());
            CODES__SQUARE_RIGHT.set(93).unwrap_or_else(| _ | panic!());
            CODES__STAR.set(42).unwrap_or_else(| _ | panic!());
            CODES__TAB.set(9).unwrap_or_else(| _ | panic!());
            CODES__TILDE.set(42).unwrap_or_else(| _ | panic!());
            CODES__UPPER_A.set(65).unwrap_or_else(| _ | panic!());
            CODES__UPPER_Z.set(90).unwrap_or_else(| _ | panic!());
            CODES__SPACE.set(32).unwrap_or_else(| _ | panic!());
            CODES__SURROGATE_MIN.set(55296).unwrap_or_else(| _ | panic!());
            CODES__SURROGATE_MAX.set(57343).unwrap_or_else(| _ | panic!());
            CODES__SUPPLEMENTAL_MIN.set(65536).unwrap_or_else(| _ | panic!());
            CODES__UINT16_MAX.set(65535).unwrap_or_else(| _ | panic!());
            CODES__UNDERSCORE.set(95).unwrap_or_else(| _ | panic!());
            let return__190: Special = Special::new(Begin::new());
            BEGIN.set(return__190.clone()).unwrap_or_else(| _ | panic!());
            let return__192: Special = Special::new(Dot::new());
            DOT.set(return__192.clone()).unwrap_or_else(| _ | panic!());
            let return__194: Special = Special::new(End::new());
            END.set(return__194.clone()).unwrap_or_else(| _ | panic!());
            let return__196: Special = Special::new(WordBoundary::new());
            WORD_BOUNDARY.set(return__196.clone()).unwrap_or_else(| _ | panic!());
            let return__198: SpecialSet = SpecialSet::new(Digit::new());
            DIGIT.set(return__198.clone()).unwrap_or_else(| _ | panic!());
            let return__200: SpecialSet = SpecialSet::new(Space::new());
            SPACE.set(return__200.clone()).unwrap_or_else(| _ | panic!());
            let return__202: SpecialSet = SpecialSet::new(Word::new());
            WORD.set(return__202.clone()).unwrap_or_else(| _ | panic!());
            let needsNoEscape__164: i32 = 0;
            let needsSimpleEscape__166: i32 = 2;
            let needsNumericEscape__165: i32 = 1;
            let return__990: temper_core::List<i32> = buildEscapeNeeds__161();
            ESCAPE_NEEDS.set(return__990.clone()).unwrap_or_else(| _ | panic!());
            let return__991: RegexRefs = RegexRefs::new(None, None, None, None);
            REGEX_REFS.set(return__991.clone()).unwrap_or_else(| _ | panic!());
            Ok(())
    }).clone()
}
static BEGIN: std::sync::OnceLock<Special> = std::sync::OnceLock::new();
pub fn begin() -> Special {
    ( * BEGIN.get().unwrap()).clone()
}
static DOT: std::sync::OnceLock<Special> = std::sync::OnceLock::new();
pub fn dot() -> Special {
    ( * DOT.get().unwrap()).clone()
}
static END: std::sync::OnceLock<Special> = std::sync::OnceLock::new();
pub fn end() -> Special {
    ( * END.get().unwrap()).clone()
}
static WORD_BOUNDARY: std::sync::OnceLock<Special> = std::sync::OnceLock::new();
pub fn word_boundary() -> Special {
    ( * WORD_BOUNDARY.get().unwrap()).clone()
}
static DIGIT: std::sync::OnceLock<SpecialSet> = std::sync::OnceLock::new();
pub fn digit() -> SpecialSet {
    ( * DIGIT.get().unwrap()).clone()
}
static SPACE: std::sync::OnceLock<SpecialSet> = std::sync::OnceLock::new();
pub fn space() -> SpecialSet {
    ( * SPACE.get().unwrap()).clone()
}
static WORD: std::sync::OnceLock<SpecialSet> = std::sync::OnceLock::new();
pub fn word() -> SpecialSet {
    ( * WORD.get().unwrap()).clone()
}
static ESCAPE_NEEDS: std::sync::OnceLock<temper_core::List<i32>> = std::sync::OnceLock::new();
fn escape_needs() -> temper_core::List<i32> {
    ( * ESCAPE_NEEDS.get().unwrap()).clone()
}
static REGEX_REFS: std::sync::OnceLock<RegexRefs> = std::sync::OnceLock::new();
fn regex_refs() -> RegexRefs {
    ( * REGEX_REFS.get().unwrap()).clone()
}
pub trait RegexNodeTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn clone_boxed(& self) -> RegexNode;
    fn compiled(& self) -> Regex {
        return Regex::new(self.clone_boxed());
    }
    fn found(& self, text__170: std::sync::Arc<String>) -> bool {
        return self.compiled().found(text__170.clone());
    }
    fn find(& self, text__173: std::sync::Arc<String>) -> temper_core::Result<Match> {
        let return__84: Match;
        return__84 = self.compiled().find(text__173.clone(), None) ? ;
        return Ok(return__84.clone());
    }
    fn replace(& self, text__176: std::sync::Arc<String>, format__177: std::sync::Arc<dyn Fn (Match) -> std::sync::Arc<String> + std::marker::Send + std::marker::Sync>) -> std::sync::Arc<String> {
        return self.compiled().replace(text__176.clone(), format__177.clone());
    }
    fn split(& self, text__180: std::sync::Arc<String>) -> temper_core::List<std::sync::Arc<String>> {
        return self.compiled().split(text__180.clone());
    }
}
#[derive(Clone)]
pub struct RegexNode(std::sync::Arc<dyn RegexNodeTrait>);
impl RegexNode {
    pub fn new(selfish: impl RegexNodeTrait + 'static) -> RegexNode {
        RegexNode(std::sync::Arc::new(selfish))
    }
}
temper_core::impl_any_value_trait_for_interface!(RegexNode);
impl std::ops::Deref for RegexNode {
    type Target = dyn RegexNodeTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct CaptureStruct {
    name: std::sync::Arc<String>, item: RegexNode
}
#[derive(Clone)]
pub struct Capture(std::sync::Arc<CaptureStruct>);
#[derive(Clone)]
pub struct CaptureBuilder {
    pub name: std::sync::Arc<String>, pub item: RegexNode
}
impl CaptureBuilder {
    pub fn build(self) -> Capture {
        Capture::new(self.name, self.item)
    }
}
impl Capture {
    pub fn new(name__185: impl temper_core::ToArcString, item__186: RegexNode) -> Capture {
        let name__185 = name__185.to_arc_string();
        let name;
        let item;
        name = name__185.clone();
        item = item__186.clone();
        let selfish = Capture(std::sync::Arc::new(CaptureStruct {
                    name, item
        }));
        return selfish;
    }
    pub fn name(& self) -> std::sync::Arc<String> {
        return self.0.name.clone();
    }
    pub fn item(& self) -> RegexNode {
        return self.0.item.clone();
    }
}
impl RegexNodeTrait for Capture {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Capture, [RegexNode]);
pub trait CodePartTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync + RegexNodeTrait {
    fn clone_boxed(& self) -> CodePart;
}
#[derive(Clone)]
pub struct CodePart(std::sync::Arc<dyn CodePartTrait>);
impl CodePart {
    pub fn new(selfish: impl CodePartTrait + 'static) -> CodePart {
        CodePart(std::sync::Arc::new(selfish))
    }
}
temper_core::impl_any_value_trait_for_interface!(CodePart);
impl std::ops::Deref for CodePart {
    type Target = dyn CodePartTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct CodePointsStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct CodePoints(std::sync::Arc<CodePointsStruct>);
impl CodePoints {
    pub fn new(value__189: impl temper_core::ToArcString) -> CodePoints {
        let value__189 = value__189.to_arc_string();
        let value;
        value = value__189.clone();
        let selfish = CodePoints(std::sync::Arc::new(CodePointsStruct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> std::sync::Arc<String> {
        return self.0.value.clone();
    }
}
impl CodePartTrait for CodePoints {
    fn clone_boxed(& self) -> CodePart {
        CodePart::new(self.clone())
    }
}
impl RegexNodeTrait for CodePoints {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(CodePoints, [CodePart, RegexNode]);
pub trait SpecialTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync + RegexNodeTrait {
    fn clone_boxed(& self) -> Special;
}
#[derive(Clone)]
pub struct Special(std::sync::Arc<dyn SpecialTrait>);
impl Special {
    pub fn new(selfish: impl SpecialTrait + 'static) -> Special {
        Special(std::sync::Arc::new(selfish))
    }
}
temper_core::impl_any_value_trait_for_interface!(Special);
impl std::ops::Deref for Special {
    type Target = dyn SpecialTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
pub trait SpecialSetTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync + CodePartTrait + SpecialTrait {
    fn clone_boxed(& self) -> SpecialSet;
}
#[derive(Clone)]
pub struct SpecialSet(std::sync::Arc<dyn SpecialSetTrait>);
impl SpecialSet {
    pub fn new(selfish: impl SpecialSetTrait + 'static) -> SpecialSet {
        SpecialSet(std::sync::Arc::new(selfish))
    }
}
temper_core::impl_any_value_trait_for_interface!(SpecialSet);
impl std::ops::Deref for SpecialSet {
    type Target = dyn SpecialSetTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct CodeRangeStruct {
    min: i32, max: i32
}
#[derive(Clone)]
pub struct CodeRange(std::sync::Arc<CodeRangeStruct>);
#[derive(Clone)]
pub struct CodeRangeBuilder {
    pub min: i32, pub max: i32
}
impl CodeRangeBuilder {
    pub fn build(self) -> CodeRange {
        CodeRange::new(self.min, self.max)
    }
}
impl CodeRange {
    pub fn new(min__207: i32, max__208: i32) -> CodeRange {
        let min;
        let max;
        min = min__207;
        max = max__208;
        let selfish = CodeRange(std::sync::Arc::new(CodeRangeStruct {
                    min, max
        }));
        return selfish;
    }
    pub fn min(& self) -> i32 {
        return self.0.min;
    }
    pub fn max(& self) -> i32 {
        return self.0.max;
    }
}
impl CodePartTrait for CodeRange {
    fn clone_boxed(& self) -> CodePart {
        CodePart::new(self.clone())
    }
}
impl RegexNodeTrait for CodeRange {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(CodeRange, [CodePart, RegexNode]);
struct CodeSetStruct {
    items: temper_core::List<CodePart>, negated: bool
}
#[derive(Clone)]
pub struct CodeSet(std::sync::Arc<CodeSetStruct>);
#[derive(Clone, Default)]
pub struct CodeSetOptions {
    pub negated: Option<bool>
}
#[derive(Clone)]
pub struct CodeSetBuilder {
    pub items: temper_core::List<CodePart>
}
impl CodeSetBuilder {
    pub fn build(self) -> CodeSet {
        self.build_with(std::default::Default::default())
    }
    pub fn build_with(self, options: CodeSetOptions) -> CodeSet {
        CodeSet::new(self.items, options.negated)
    }
}
impl CodeSet {
    pub fn new(items__212: impl temper_core::ToList<CodePart>, negated__542: Option<bool>) -> CodeSet {
        let items__212 = items__212.to_list();
        let items;
        let negated;
        let negated__213: bool;
        if negated__542.is_none() {
            negated__213 = false;
        } else {
            negated__213 = negated__542.unwrap();
        }
        items = items__212.clone();
        negated = negated__213;
        let selfish = CodeSet(std::sync::Arc::new(CodeSetStruct {
                    items, negated
        }));
        return selfish;
    }
    pub fn items(& self) -> temper_core::List<CodePart> {
        return self.0.items.clone();
    }
    pub fn negated(& self) -> bool {
        return self.0.negated;
    }
}
impl RegexNodeTrait for CodeSet {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(CodeSet, [RegexNode]);
struct OrStruct {
    items: temper_core::List<RegexNode>
}
#[derive(Clone)]
pub struct Or(std::sync::Arc<OrStruct>);
impl Or {
    pub fn new(items__216: impl temper_core::ToList<RegexNode>) -> Or {
        let items__216 = items__216.to_list();
        let items;
        items = items__216.clone();
        let selfish = Or(std::sync::Arc::new(OrStruct {
                    items
        }));
        return selfish;
    }
    pub fn items(& self) -> temper_core::List<RegexNode> {
        return self.0.items.clone();
    }
}
impl RegexNodeTrait for Or {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Or, [RegexNode]);
struct RepeatStruct {
    item: RegexNode, min: i32, max: Option<i32>, reluctant: bool
}
#[derive(Clone)]
pub struct Repeat(std::sync::Arc<RepeatStruct>);
#[derive(Clone, Default)]
pub struct RepeatOptions {
    pub reluctant: Option<bool>
}
#[derive(Clone)]
pub struct RepeatBuilder {
    pub item: RegexNode, pub min: i32, pub max: Option<i32>
}
impl RepeatBuilder {
    pub fn build(self) -> Repeat {
        self.build_with(std::default::Default::default())
    }
    pub fn build_with(self, options: RepeatOptions) -> Repeat {
        Repeat::new(self.item, self.min, self.max, options.reluctant)
    }
}
impl Repeat {
    pub fn new(item__222: RegexNode, min__223: i32, max__224: Option<i32>, reluctant__544: Option<bool>) -> Repeat {
        let item;
        let min;
        let max;
        let reluctant;
        let reluctant__225: bool;
        if reluctant__544.is_none() {
            reluctant__225 = false;
        } else {
            reluctant__225 = reluctant__544.unwrap();
        }
        item = item__222.clone();
        min = min__223;
        max = max__224;
        reluctant = reluctant__225;
        let selfish = Repeat(std::sync::Arc::new(RepeatStruct {
                    item, min, max, reluctant
        }));
        return selfish;
    }
    pub fn item(& self) -> RegexNode {
        return self.0.item.clone();
    }
    pub fn min(& self) -> i32 {
        return self.0.min;
    }
    pub fn max(& self) -> Option<i32> {
        return self.0.max;
    }
    pub fn reluctant(& self) -> bool {
        return self.0.reluctant;
    }
}
impl RegexNodeTrait for Repeat {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Repeat, [RegexNode]);
struct SequenceStruct {
    items: temper_core::List<RegexNode>
}
#[derive(Clone)]
pub struct Sequence(std::sync::Arc<SequenceStruct>);
impl Sequence {
    pub fn new(items__236: impl temper_core::ToList<RegexNode>) -> Sequence {
        let items__236 = items__236.to_list();
        let items;
        items = items__236.clone();
        let selfish = Sequence(std::sync::Arc::new(SequenceStruct {
                    items
        }));
        return selfish;
    }
    pub fn items(& self) -> temper_core::List<RegexNode> {
        return self.0.items.clone();
    }
}
impl RegexNodeTrait for Sequence {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Sequence, [RegexNode]);
struct MatchStruct {
    full: Group, groups: temper_core::Map<std::sync::Arc<String>, Group>
}
#[derive(Clone)]
pub struct Match(std::sync::Arc<MatchStruct>);
#[derive(Clone)]
pub struct MatchBuilder {
    pub full: Group, pub groups: temper_core::Map<std::sync::Arc<String>, Group>
}
impl MatchBuilder {
    pub fn build(self) -> Match {
        Match::new(self.full, self.groups)
    }
}
impl Match {
    pub fn new(full__240: Group, groups__241: temper_core::Map<std::sync::Arc<String>, Group>) -> Match {
        let full;
        let groups;
        full = full__240.clone();
        groups = groups__241.clone();
        let selfish = Match(std::sync::Arc::new(MatchStruct {
                    full, groups
        }));
        return selfish;
    }
    pub fn full(& self) -> Group {
        return self.0.full.clone();
    }
    pub fn groups(& self) -> temper_core::Map<std::sync::Arc<String>, Group> {
        return self.0.groups.clone();
    }
}
temper_core::impl_any_value_trait!(Match, []);
struct GroupStruct {
    name: std::sync::Arc<String>, value: std::sync::Arc<String>, begin: usize, end: usize
}
#[derive(Clone)]
pub struct Group(std::sync::Arc<GroupStruct>);
#[derive(Clone)]
pub struct GroupBuilder {
    pub name: std::sync::Arc<String>, pub value: std::sync::Arc<String>, pub begin: usize, pub end: usize
}
impl GroupBuilder {
    pub fn build(self) -> Group {
        Group::new(self.name, self.value, self.begin, self.end)
    }
}
impl Group {
    pub fn new(name__247: impl temper_core::ToArcString, value__248: impl temper_core::ToArcString, begin__249: usize, end__250: usize) -> Group {
        let name__247 = name__247.to_arc_string();
        let value__248 = value__248.to_arc_string();
        let name;
        let value;
        let begin;
        let end;
        name = name__247.clone();
        value = value__248.clone();
        begin = begin__249;
        end = end__250;
        let selfish = Group(std::sync::Arc::new(GroupStruct {
                    name, value, begin, end
        }));
        return selfish;
    }
    pub fn name(& self) -> std::sync::Arc<String> {
        return self.0.name.clone();
    }
    pub fn value(& self) -> std::sync::Arc<String> {
        return self.0.value.clone();
    }
    pub fn begin(& self) -> usize {
        return self.0.begin;
    }
    pub fn end(& self) -> usize {
        return self.0.end;
    }
}
temper_core::impl_any_value_trait!(Group, []);
struct RegexRefsStruct {
    code_points: CodePoints, group: Group, r#match: Match, or_object: Or
}
#[derive(Clone)]
pub (crate) struct RegexRefs(std::sync::Arc<RegexRefsStruct>);
impl RegexRefs {
    pub fn new(codePoints__546: Option<CodePoints>, group__548: Option<Group>, match__550: Option<Match>, orObject__552: Option<Or>) -> RegexRefs {
        let code_points;
        let group;
        let r#match;
        let or_object;
        let mut t___1272: CodePoints;
        let mut t___1273: Group;
        let mut t___1275: temper_core::Map<std::sync::Arc<String>, Group>;
        let mut t___1276: Match;
        let mut t___1277: Or;
        let codePoints__256: CodePoints;
        if codePoints__546.is_none() {
            t___1272 = CodePoints::new("");
            codePoints__256 = t___1272.clone();
        } else {
            codePoints__256 = codePoints__546.clone().unwrap();
        }
        let group__257: Group;
        if group__548.is_none() {
            t___1273 = Group::new("", "", 0usize, 0usize);
            group__257 = t___1273.clone();
        } else {
            group__257 = group__548.clone().unwrap();
        }
        let match__258: Match;
        if match__550.is_none() {
            t___1275 = temper_core::Map::new( & [(std::sync::Arc::new("".to_string()), group__257.clone())]);
            t___1276 = Match::new(group__257.clone(), t___1275.clone());
            match__258 = t___1276.clone();
        } else {
            match__258 = match__550.clone().unwrap();
        }
        let orObject__259: Or;
        if orObject__552.is_none() {
            t___1277 = Or::new([]);
            orObject__259 = t___1277.clone();
        } else {
            orObject__259 = orObject__552.clone().unwrap();
        }
        code_points = codePoints__256.clone();
        group = group__257.clone();
        r#match = match__258.clone();
        or_object = orObject__259.clone();
        let selfish = RegexRefs(std::sync::Arc::new(RegexRefsStruct {
                    code_points, group, r#match, or_object
        }));
        return selfish;
    }
    pub fn code_points(& self) -> CodePoints {
        return self.0.code_points.clone();
    }
    pub fn group(& self) -> Group {
        return self.0.group.clone();
    }
    pub fn r#match(& self) -> Match {
        return self.0.r#match.clone();
    }
    pub fn or_object(& self) -> Or {
        return self.0.or_object.clone();
    }
}
temper_core::impl_any_value_trait!(RegexRefs, []);
struct RegexStruct {
    data: RegexNode, compiled: temper_core::AnyValue
}
#[derive(Clone)]
pub struct Regex(std::sync::Arc<RegexStruct>);
impl Regex {
    pub fn new(data__262: RegexNode) -> Regex {
        let data;
        let compiled;
        let t___419: RegexNode = data__262.clone();
        data = t___419.clone();
        let formatted__264: std::sync::Arc<String> = RegexFormatter::regex_format(data__262.clone());
        let mut t___1156: temper_core::AnyValue = compile_formatted( & ( * data__262), formatted__264.clone());
        compiled = t___1156.clone();
        let selfish = Regex(std::sync::Arc::new(RegexStruct {
                    data, compiled
        }));
        return selfish;
    }
    pub fn found(& self, text__266: impl temper_core::ToArcString) -> bool {
        let text__266 = text__266.to_arc_string();
        return compiled_found( & self, self.0.compiled.clone(), text__266.clone());
    }
    pub fn find(& self, text__269: impl temper_core::ToArcString, begin__554: Option<usize>) -> temper_core::Result<Match> {
        let text__269 = text__269.to_arc_string();
        let return__131: Match;
        let begin__270: usize;
        if begin__554.is_none() {
            begin__270 = 0usize;
        } else {
            begin__270 = begin__554.unwrap();
        }
        return__131 = compiled_find( & self, self.0.compiled.clone(), text__269.clone(), begin__270, regex_refs().clone()) ? ;
        return Ok(return__131.clone());
    }
    pub fn replace(& self, text__273: impl temper_core::ToArcString, format__274: std::sync::Arc<dyn Fn (Match) -> std::sync::Arc<String> + std::marker::Send + std::marker::Sync>) -> std::sync::Arc<String> {
        let text__273 = text__273.to_arc_string();
        return compiled_replace( & self, self.0.compiled.clone(), text__273.clone(), & ( * format__274.clone()), regex_refs().clone());
    }
    pub fn split(& self, text__277: impl temper_core::ToArcString) -> temper_core::List<std::sync::Arc<String>> {
        let text__277 = text__277.to_arc_string();
        return compiled_split( & self, self.0.compiled.clone(), text__277.clone(), regex_refs().clone());
    }
    pub fn data(& self) -> RegexNode {
        return self.0.data.clone();
    }
}
temper_core::impl_any_value_trait!(Regex, []);
struct RegexFormatterStruct {
    out: std::sync::Arc<std::sync::RwLock<String>>
}
#[derive(Clone)]
pub (crate) struct RegexFormatter(std::sync::Arc<RegexFormatterStruct>);
impl RegexFormatter {
    pub fn regex_format(data__307: RegexNode) -> std::sync::Arc<String> {
        return RegexFormatter::new().format(data__307.clone());
    }
    pub fn format(& self, regex__310: RegexNode) -> std::sync::Arc<String> {
        self.push_regex(regex__310.clone());
        return temper_core::string::builder::to_string( & self.0.out);
    }
    fn push_regex(& self, regex__313: RegexNode) {
        let mut t___884: Capture;
        let mut t___885: CodePoints;
        let mut t___886: CodeRange;
        let mut t___887: CodeSet;
        let mut t___888: Or;
        let mut t___889: Repeat;
        let mut t___890: Sequence;
        if temper_core::is::<Capture>(regex__313.clone()) {
            t___884 = temper_core::cast::<Capture>(regex__313.clone()).unwrap();
            self.push_capture(t___884.clone());
        } else {
            if temper_core::is::<CodePoints>(regex__313.clone()) {
                t___885 = temper_core::cast::<CodePoints>(regex__313.clone()).unwrap();
                self.push_code_points(t___885.clone(), false);
            } else {
                if temper_core::is::<CodeRange>(regex__313.clone()) {
                    t___886 = temper_core::cast::<CodeRange>(regex__313.clone()).unwrap();
                    self.push_code_range(t___886.clone());
                } else {
                    if temper_core::is::<CodeSet>(regex__313.clone()) {
                        t___887 = temper_core::cast::<CodeSet>(regex__313.clone()).unwrap();
                        self.push_code_set(t___887.clone());
                    } else {
                        if temper_core::is::<Or>(regex__313.clone()) {
                            t___888 = temper_core::cast::<Or>(regex__313.clone()).unwrap();
                            self.push_or(t___888.clone());
                        } else {
                            if temper_core::is::<Repeat>(regex__313.clone()) {
                                t___889 = temper_core::cast::<Repeat>(regex__313.clone()).unwrap();
                                self.push_repeat(t___889.clone());
                            } else {
                                if temper_core::is::<Sequence>(regex__313.clone()) {
                                    t___890 = temper_core::cast::<Sequence>(regex__313.clone()).unwrap();
                                    self.push_sequence(t___890.clone());
                                } else {
                                    if regex__313.ptr_id() == begin().ptr_id() {
                                        temper_core::string::builder::append( & self.0.out, "^");
                                    } else {
                                        if regex__313.ptr_id() == dot().ptr_id() {
                                            temper_core::string::builder::append( & self.0.out, ".");
                                        } else {
                                            if regex__313.ptr_id() == end().ptr_id() {
                                                temper_core::string::builder::append( & self.0.out, "$");
                                            } else {
                                                if regex__313.ptr_id() == word_boundary().ptr_id() {
                                                    temper_core::string::builder::append( & self.0.out, "\\b");
                                                } else {
                                                    if regex__313.ptr_id() == digit().ptr_id() {
                                                        temper_core::string::builder::append( & self.0.out, "\\d");
                                                    } else {
                                                        if regex__313.ptr_id() == space().ptr_id() {
                                                            temper_core::string::builder::append( & self.0.out, "\\s");
                                                        } else {
                                                            if regex__313.ptr_id() == word().ptr_id() {
                                                                temper_core::string::builder::append( & self.0.out, "\\w");
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn push_capture(& self, capture__316: Capture) {
        temper_core::string::builder::append( & self.0.out, "(");
        let mut t___858: std::sync::Arc<std::sync::RwLock<String>> = self.0.out.clone();
        let mut t___1242: std::sync::Arc<String> = capture__316.name();
        self.push_capture_name(t___858.clone(), t___1242.clone());
        let mut t___1244: RegexNode = capture__316.item();
        self.push_regex(t___1244.clone());
        temper_core::string::builder::append( & self.0.out, ")");
    }
    fn push_capture_name(& self, out__319: std::sync::Arc<std::sync::RwLock<String>>, name__320: impl temper_core::ToArcString) {
        let name__320 = name__320.to_arc_string();
        temper_core::string::builder::append( & out__319, std::sync::Arc::new(format!("?<{}>", name__320.clone())));
    }
    fn push_code(& self, code__323: i32, insideCodeSet__324: bool) {
        let return__146: ();
        let mut t___846: bool;
        let mut t___847: bool;
        let mut t___848: std::sync::Arc<String>;
        let mut t___850: std::sync::Arc<String>;
        let mut t___851: bool;
        let mut t___852: bool;
        let mut t___853: bool;
        let mut t___854: bool;
        let mut t___855: std::sync::Arc<String>;
        'fn__325: {
            'ok___2629: {
                'orelse___560: {
                    let specialEscape__326: std::sync::Arc<String>;
                    if Some(code__323) == Some(Codes::carriage_return()) {
                        specialEscape__326 = std::sync::Arc::new("r".to_string());
                    } else {
                        if Some(code__323) == Some(Codes::newline()) {
                            specialEscape__326 = std::sync::Arc::new("n".to_string());
                        } else {
                            if Some(code__323) == Some(Codes::tab()) {
                                specialEscape__326 = std::sync::Arc::new("t".to_string());
                            } else {
                                specialEscape__326 = std::sync::Arc::new("".to_string());
                            }
                        }
                    }
                    if Some(specialEscape__326.as_str()) != Some("") {
                        temper_core::string::builder::append( & self.0.out, "\\");
                        temper_core::string::builder::append( & self.0.out, specialEscape__326.clone());
                        return__146 = ();
                        break 'fn__325;
                    }
                    if Some(code__323) <= Some(127) {
                        let escapeNeed__327: i32 = temper_core::ListedTrait::get( & escape_needs(), code__323);
                        if escapeNeed__327 == 2{
                            t___847 = true;
                        } else {
                            if insideCodeSet__324 {
                                t___846 = Some(code__323) == Some(Codes::dash());
                            } else {
                                t___846 = false;
                            }
                            t___847 = t___846;
                        }
                        if t___847 {
                            temper_core::string::builder::append( & self.0.out, "\\");
                            t___848 = match temper_core::string::from_code_point(code__323) {
                                Ok(x) => x,
                                _ => break 'orelse___560
                            };
                            temper_core::string::builder::append( & self.0.out, t___848.clone());
                            return__146 = ();
                            break 'fn__325;
                        } else {
                            if escapeNeed__327 == 0{
                                t___850 = match temper_core::string::from_code_point(code__323) {
                                    Ok(x) => x,
                                    _ => break 'orelse___560
                                };
                                temper_core::string::builder::append( & self.0.out, t___850.clone());
                                return__146 = ();
                                break 'fn__325;
                            }
                        }
                    }
                    if Some(code__323) >= Some(Codes::supplemental_min()) {
                        t___854 = true;
                    } else {
                        if Some(code__323) > Some(Codes::high_control_max()) {
                            if Some(Codes::surrogate_min()) <= Some(code__323) {
                                t___851 = Some(code__323) <= Some(Codes::surrogate_max());
                            } else {
                                t___851 = false;
                            }
                            if t___851 {
                                t___852 = true;
                            } else {
                                t___852 = Some(code__323) == Some(Codes::uint16_max());
                            }
                            t___853 = ! t___852;
                        } else {
                            t___853 = false;
                        }
                        t___854 = t___853;
                    }
                    if t___854 {
                        t___855 = match temper_core::string::from_code_point(code__323) {
                            Ok(x) => x,
                            _ => break 'orelse___560
                        };
                        temper_core::string::builder::append( & self.0.out, t___855.clone());
                    } else {
                        push_code_to( & self, self.0.out.clone(), code__323, insideCodeSet__324);
                    }
                    break 'ok___2629;
                }
                return panic!();
            }
            return__146 = ();
        }
        return return__146;
    }
    fn push_code_points(& self, codePoints__334: CodePoints, insideCodeSet__335: bool) {
        let mut t___1229: i32;
        let mut t___1231: usize;
        let value__337: std::sync::Arc<String> = codePoints__334.value();
        let mut index__338: usize = 0usize;
        'loop___2669: loop {
            if ! temper_core::string::has_index( & value__337, index__338) {
                break;
            }
            t___1229 = temper_core::string::get( & value__337, index__338);
            self.push_code(t___1229, insideCodeSet__335);
            t___1231 = temper_core::string::next( & value__337, index__338);
            index__338 = t___1231;
        }
    }
    fn push_code_range(& self, codeRange__340: CodeRange) {
        temper_core::string::builder::append( & self.0.out, "[");
        self.push_code_range_unwrapped(codeRange__340.clone());
        temper_core::string::builder::append( & self.0.out, "]");
    }
    fn push_code_range_unwrapped(& self, codeRange__343: CodeRange) {
        let mut t___1219: i32 = codeRange__343.min();
        self.push_code(t___1219, true);
        temper_core::string::builder::append( & self.0.out, "-");
        let mut t___1222: i32 = codeRange__343.max();
        self.push_code(t___1222, true);
    }
    fn push_code_set(& self, codeSet__346: CodeSet) {
        let mut t___1213: i32;
        let mut t___1215: CodePart;
        let mut t___831: CodeSet;
        let adjusted__348: RegexNode = self.adjust_code_set(codeSet__346.clone(), regex_refs().clone());
        if temper_core::is::<CodeSet>(adjusted__348.clone()) {
            t___831 = temper_core::cast::<CodeSet>(adjusted__348.clone()).unwrap();
            temper_core::string::builder::append( & self.0.out, "[");
            if t___831.negated() {
                temper_core::string::builder::append( & self.0.out, "^");
            }
            let mut i__349: i32 = 0;
            'loop___2670: loop {
                t___1213 = temper_core::ListedTrait::len( & t___831.items());
                if ! (Some(i__349) < Some(t___1213)) {
                    break;
                }
                t___1215 = temper_core::ListedTrait::get( & t___831.items(), i__349);
                self.push_code_set_item(t___1215.clone());
                i__349 = i__349.wrapping_add(1);
            }
            temper_core::string::builder::append( & self.0.out, "]");
        } else {
            self.push_regex(adjusted__348.clone());
        }
    }
    fn adjust_code_set(& self, codeSet__351: CodeSet, regexRefs__352: RegexRefs) -> RegexNode {
        return RegexNode::new(codeSet__351.clone());
    }
    fn push_code_set_item(& self, codePart__355: CodePart) {
        let mut t___819: CodePoints;
        let mut t___820: CodeRange;
        let mut t___821: SpecialSet;
        if temper_core::is::<CodePoints>(codePart__355.clone()) {
            t___819 = temper_core::cast::<CodePoints>(codePart__355.clone()).unwrap();
            self.push_code_points(t___819.clone(), true);
        } else {
            if temper_core::is::<CodeRange>(codePart__355.clone()) {
                t___820 = temper_core::cast::<CodeRange>(codePart__355.clone()).unwrap();
                self.push_code_range_unwrapped(t___820.clone());
            } else {
                if temper_core::is::<SpecialSet>(codePart__355.clone()) {
                    t___821 = temper_core::cast::<SpecialSet>(codePart__355.clone()).unwrap();
                    self.push_regex(temper_core::cast::<RegexNode>(t___821.clone()).unwrap());
                }
            }
        }
    }
    fn push_or(& self, or__358: Or) {
        let mut t___1192: RegexNode;
        let mut t___1195: i32;
        let mut t___1198: RegexNode;
        if ! temper_core::ListedTrait::is_empty( & or__358.items()) {
            temper_core::string::builder::append( & self.0.out, "(?:");
            t___1192 = temper_core::ListedTrait::get( & or__358.items(), 0);
            self.push_regex(t___1192.clone());
            let mut i__360: i32 = 1;
            'loop___2671: loop {
                t___1195 = temper_core::ListedTrait::len( & or__358.items());
                if ! (Some(i__360) < Some(t___1195)) {
                    break;
                }
                temper_core::string::builder::append( & self.0.out, "|");
                t___1198 = temper_core::ListedTrait::get( & or__358.items(), i__360);
                self.push_regex(t___1198.clone());
                i__360 = i__360.wrapping_add(1);
            }
            temper_core::string::builder::append( & self.0.out, ")");
        }
    }
    fn push_repeat(& self, repeat__362: Repeat) {
        let mut t___1180: std::sync::Arc<String>;
        let mut t___1183: std::sync::Arc<String>;
        let mut t___796: bool;
        let mut t___797: bool;
        let mut t___798: bool;
        temper_core::string::builder::append( & self.0.out, "(?:");
        let mut t___1172: RegexNode = repeat__362.item();
        self.push_regex(t___1172.clone());
        temper_core::string::builder::append( & self.0.out, ")");
        let min__364: i32 = repeat__362.min();
        let max__365: Option<i32> = repeat__362.max();
        if Some(min__364) == Some(0) {
            t___796 = max__365 == Some(1);
        } else {
            t___796 = false;
        }
        if t___796 {
            temper_core::string::builder::append( & self.0.out, "?");
        } else {
            if Some(min__364) == Some(0) {
                t___797 = max__365.is_none();
            } else {
                t___797 = false;
            }
            if t___797 {
                temper_core::string::builder::append( & self.0.out, "*");
            } else {
                if Some(min__364) == Some(1) {
                    t___798 = max__365.is_none();
                } else {
                    t___798 = false;
                }
                if t___798 {
                    temper_core::string::builder::append( & self.0.out, "+");
                } else {
                    t___1180 = temper_core::int_to_string(min__364, None);
                    temper_core::string::builder::append( & self.0.out, std::sync::Arc::new(format!("{{{}", t___1180.clone())));
                    if Some(min__364) != max__365 {
                        temper_core::string::builder::append( & self.0.out, ",");
                        if ! max__365.is_none() {
                            t___1183 = temper_core::int_to_string(max__365.unwrap(), None);
                            temper_core::string::builder::append( & self.0.out, t___1183.clone());
                        }
                    }
                    temper_core::string::builder::append( & self.0.out, "}");
                }
            }
        }
        if repeat__362.reluctant() {
            temper_core::string::builder::append( & self.0.out, "?");
        }
    }
    fn push_sequence(& self, sequence__367: Sequence) {
        let mut t___1167: i32;
        let mut t___1169: RegexNode;
        let mut i__369: i32 = 0;
        'loop___2675: loop {
            t___1167 = temper_core::ListedTrait::len( & sequence__367.items());
            if ! (Some(i__369) < Some(t___1167)) {
                break;
            }
            t___1169 = temper_core::ListedTrait::get( & sequence__367.items(), i__369);
            self.push_regex(t___1169.clone());
            i__369 = i__369.wrapping_add(1);
        }
    }
    pub fn max_code(& self, codePart__371: CodePart) -> Option<i32> {
        let return__157: Option<i32>;
        let mut t___1163: usize;
        let mut t___784: CodePoints;
        if temper_core::is::<CodePoints>(codePart__371.clone()) {
            t___784 = temper_core::cast::<CodePoints>(codePart__371.clone()).unwrap();
            let value__373: std::sync::Arc<String> = t___784.value();
            if value__373.is_empty() {
                return__157 = None;
            } else {
                let mut max__374: i32 = 0;
                let mut index__375: usize = 0usize;
                'loop___2676: loop {
                    if ! temper_core::string::has_index( & value__373, index__375) {
                        break;
                    }
                    let next__376: i32 = temper_core::string::get( & value__373, index__375);
                    if Some(next__376) > Some(max__374) {
                        max__374 = next__376;
                    }
                    t___1163 = temper_core::string::next( & value__373, index__375);
                    index__375 = t___1163;
                }
                return__157 = Some(max__374);
            }
        } else {
            if temper_core::is::<CodeRange>(codePart__371.clone()) {
                return__157 = Some(temper_core::cast::<CodeRange>(codePart__371.clone()).unwrap().max());
            } else {
                if codePart__371.ptr_id() == digit().ptr_id() {
                    return__157 = Some(Codes::digit9());
                } else {
                    if codePart__371.ptr_id() == space().ptr_id() {
                        return__157 = Some(Codes::space());
                    } else {
                        if codePart__371.ptr_id() == word().ptr_id() {
                            return__157 = Some(Codes::lower_z());
                        } else {
                            return__157 = None;
                        }
                    }
                }
            }
        }
        return return__157;
    }
    pub fn new() -> RegexFormatter {
        let out;
        let mut t___1157: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        out = t___1157.clone();
        let selfish = RegexFormatter(std::sync::Arc::new(RegexFormatterStruct {
                    out
        }));
        return selfish;
    }
}
temper_core::impl_any_value_trait!(RegexFormatter, []);
struct CodesStruct {}
#[derive(Clone)]
pub (crate) struct Codes(std::sync::Arc<CodesStruct>);
static CODES__AMPERSAND: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__BACKSLASH: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__CARET: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__CARRIAGE_RETURN: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__CURLY_LEFT: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__CURLY_RIGHT: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__DASH: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__DOT: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__HIGH_CONTROL_MIN: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__HIGH_CONTROL_MAX: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__DIGIT0: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__DIGIT9: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__LOWER_A: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__LOWER_Z: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__NEWLINE: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__PESO: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__PIPE: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__PLUS: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__QUESTION: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__ROUND_LEFT: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__ROUND_RIGHT: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__SLASH: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__SQUARE_LEFT: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__SQUARE_RIGHT: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__STAR: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__TAB: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__TILDE: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__UPPER_A: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__UPPER_Z: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__SPACE: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__SURROGATE_MIN: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__SURROGATE_MAX: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__SUPPLEMENTAL_MIN: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__UINT16_MAX: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__UNDERSCORE: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
impl Codes {
    pub fn ampersand() -> i32 {
        * CODES__AMPERSAND.get().unwrap()
    }
    pub fn backslash() -> i32 {
        * CODES__BACKSLASH.get().unwrap()
    }
    pub fn caret() -> i32 {
        * CODES__CARET.get().unwrap()
    }
    pub fn carriage_return() -> i32 {
        * CODES__CARRIAGE_RETURN.get().unwrap()
    }
    pub fn curly_left() -> i32 {
        * CODES__CURLY_LEFT.get().unwrap()
    }
    pub fn curly_right() -> i32 {
        * CODES__CURLY_RIGHT.get().unwrap()
    }
    pub fn dash() -> i32 {
        * CODES__DASH.get().unwrap()
    }
    pub fn dot() -> i32 {
        * CODES__DOT.get().unwrap()
    }
    pub fn high_control_min() -> i32 {
        * CODES__HIGH_CONTROL_MIN.get().unwrap()
    }
    pub fn high_control_max() -> i32 {
        * CODES__HIGH_CONTROL_MAX.get().unwrap()
    }
    pub fn digit0() -> i32 {
        * CODES__DIGIT0.get().unwrap()
    }
    pub fn digit9() -> i32 {
        * CODES__DIGIT9.get().unwrap()
    }
    pub fn lower_a() -> i32 {
        * CODES__LOWER_A.get().unwrap()
    }
    pub fn lower_z() -> i32 {
        * CODES__LOWER_Z.get().unwrap()
    }
    pub fn newline() -> i32 {
        * CODES__NEWLINE.get().unwrap()
    }
    pub fn peso() -> i32 {
        * CODES__PESO.get().unwrap()
    }
    pub fn pipe() -> i32 {
        * CODES__PIPE.get().unwrap()
    }
    pub fn plus() -> i32 {
        * CODES__PLUS.get().unwrap()
    }
    pub fn question() -> i32 {
        * CODES__QUESTION.get().unwrap()
    }
    pub fn round_left() -> i32 {
        * CODES__ROUND_LEFT.get().unwrap()
    }
    pub fn round_right() -> i32 {
        * CODES__ROUND_RIGHT.get().unwrap()
    }
    pub fn slash() -> i32 {
        * CODES__SLASH.get().unwrap()
    }
    pub fn square_left() -> i32 {
        * CODES__SQUARE_LEFT.get().unwrap()
    }
    pub fn square_right() -> i32 {
        * CODES__SQUARE_RIGHT.get().unwrap()
    }
    pub fn star() -> i32 {
        * CODES__STAR.get().unwrap()
    }
    pub fn tab() -> i32 {
        * CODES__TAB.get().unwrap()
    }
    pub fn tilde() -> i32 {
        * CODES__TILDE.get().unwrap()
    }
    pub fn upper_a() -> i32 {
        * CODES__UPPER_A.get().unwrap()
    }
    pub fn upper_z() -> i32 {
        * CODES__UPPER_Z.get().unwrap()
    }
    pub fn space() -> i32 {
        * CODES__SPACE.get().unwrap()
    }
    pub fn surrogate_min() -> i32 {
        * CODES__SURROGATE_MIN.get().unwrap()
    }
    pub fn surrogate_max() -> i32 {
        * CODES__SURROGATE_MAX.get().unwrap()
    }
    pub fn supplemental_min() -> i32 {
        * CODES__SUPPLEMENTAL_MIN.get().unwrap()
    }
    pub fn uint16_max() -> i32 {
        * CODES__UINT16_MAX.get().unwrap()
    }
    pub fn underscore() -> i32 {
        * CODES__UNDERSCORE.get().unwrap()
    }
    pub fn new() -> Codes {
        let selfish = Codes(std::sync::Arc::new(CodesStruct {}));
        return selfish;
    }
}
temper_core::impl_any_value_trait!(Codes, []);
struct BeginStruct {}
#[derive(Clone)]
pub (crate) struct Begin(std::sync::Arc<BeginStruct>);
impl Begin {
    pub fn new() -> Begin {
        let selfish = Begin(std::sync::Arc::new(BeginStruct {}));
        return selfish;
    }
}
impl SpecialTrait for Begin {
    fn clone_boxed(& self) -> Special {
        Special::new(self.clone())
    }
}
impl RegexNodeTrait for Begin {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Begin, [Special, RegexNode]);
struct DotStruct {}
#[derive(Clone)]
pub (crate) struct Dot(std::sync::Arc<DotStruct>);
impl Dot {
    pub fn new() -> Dot {
        let selfish = Dot(std::sync::Arc::new(DotStruct {}));
        return selfish;
    }
}
impl SpecialTrait for Dot {
    fn clone_boxed(& self) -> Special {
        Special::new(self.clone())
    }
}
impl RegexNodeTrait for Dot {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Dot, [Special, RegexNode]);
struct EndStruct {}
#[derive(Clone)]
pub (crate) struct End(std::sync::Arc<EndStruct>);
impl End {
    pub fn new() -> End {
        let selfish = End(std::sync::Arc::new(EndStruct {}));
        return selfish;
    }
}
impl SpecialTrait for End {
    fn clone_boxed(& self) -> Special {
        Special::new(self.clone())
    }
}
impl RegexNodeTrait for End {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(End, [Special, RegexNode]);
struct WordBoundaryStruct {}
#[derive(Clone)]
pub (crate) struct WordBoundary(std::sync::Arc<WordBoundaryStruct>);
impl WordBoundary {
    pub fn new() -> WordBoundary {
        let selfish = WordBoundary(std::sync::Arc::new(WordBoundaryStruct {}));
        return selfish;
    }
}
impl SpecialTrait for WordBoundary {
    fn clone_boxed(& self) -> Special {
        Special::new(self.clone())
    }
}
impl RegexNodeTrait for WordBoundary {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(WordBoundary, [Special, RegexNode]);
struct DigitStruct {}
#[derive(Clone)]
pub (crate) struct Digit(std::sync::Arc<DigitStruct>);
impl Digit {
    pub fn new() -> Digit {
        let selfish = Digit(std::sync::Arc::new(DigitStruct {}));
        return selfish;
    }
}
impl SpecialSetTrait for Digit {
    fn clone_boxed(& self) -> SpecialSet {
        SpecialSet::new(self.clone())
    }
}
impl CodePartTrait for Digit {
    fn clone_boxed(& self) -> CodePart {
        CodePart::new(self.clone())
    }
}
impl RegexNodeTrait for Digit {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
impl SpecialTrait for Digit {
    fn clone_boxed(& self) -> Special {
        Special::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Digit, [SpecialSet, CodePart, RegexNode, Special]);
struct SpaceStruct {}
#[derive(Clone)]
pub (crate) struct Space(std::sync::Arc<SpaceStruct>);
impl Space {
    pub fn new() -> Space {
        let selfish = Space(std::sync::Arc::new(SpaceStruct {}));
        return selfish;
    }
}
impl SpecialSetTrait for Space {
    fn clone_boxed(& self) -> SpecialSet {
        SpecialSet::new(self.clone())
    }
}
impl CodePartTrait for Space {
    fn clone_boxed(& self) -> CodePart {
        CodePart::new(self.clone())
    }
}
impl RegexNodeTrait for Space {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
impl SpecialTrait for Space {
    fn clone_boxed(& self) -> Special {
        Special::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Space, [SpecialSet, CodePart, RegexNode, Special]);
struct WordStruct {}
#[derive(Clone)]
pub (crate) struct Word(std::sync::Arc<WordStruct>);
impl Word {
    pub fn new() -> Word {
        let selfish = Word(std::sync::Arc::new(WordStruct {}));
        return selfish;
    }
}
impl SpecialSetTrait for Word {
    fn clone_boxed(& self) -> SpecialSet {
        SpecialSet::new(self.clone())
    }
}
impl CodePartTrait for Word {
    fn clone_boxed(& self) -> CodePart {
        CodePart::new(self.clone())
    }
}
impl RegexNodeTrait for Word {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
impl SpecialTrait for Word {
    fn clone_boxed(& self) -> Special {
        Special::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Word, [SpecialSet, CodePart, RegexNode, Special]);
fn buildEscapeNeeds__161() -> temper_core::List<i32> {
    let mut t___925: bool;
    let mut t___926: bool;
    let mut t___927: bool;
    let mut t___928: bool;
    let mut t___929: bool;
    let mut t___930: bool;
    let mut t___931: bool;
    let mut t___932: bool;
    let mut t___933: bool;
    let mut t___934: bool;
    let mut t___935: bool;
    let mut t___936: bool;
    let mut t___937: bool;
    let mut t___938: bool;
    let mut t___939: bool;
    let mut t___940: bool;
    let mut t___941: bool;
    let mut t___942: bool;
    let mut t___943: bool;
    let mut t___944: bool;
    let mut t___945: bool;
    let mut t___946: bool;
    let mut t___947: bool;
    let mut t___948: bool;
    let mut t___949: i32;
    let escapeNeeds__379: temper_core::ListBuilder<i32> = temper_core::listed::new_builder();
    let mut code__380: i32 = 0;
    'loop___2678: while Some(code__380) <= Some(127) {
        if Some(code__380) == Some(Codes::dash()) {
            t___932 = true;
        } else {
            if Some(code__380) == Some(Codes::space()) {
                t___931 = true;
            } else {
                if Some(code__380) == Some(Codes::underscore()) {
                    t___930 = true;
                } else {
                    if Some(Codes::digit0()) <= Some(code__380) {
                        t___925 = Some(code__380) <= Some(Codes::digit9());
                    } else {
                        t___925 = false;
                    }
                    if t___925 {
                        t___929 = true;
                    } else {
                        if Some(Codes::upper_a()) <= Some(code__380) {
                            t___926 = Some(code__380) <= Some(Codes::upper_z());
                        } else {
                            t___926 = false;
                        }
                        if t___926 {
                            t___928 = true;
                        } else {
                            if Some(Codes::lower_a()) <= Some(code__380) {
                                t___927 = Some(code__380) <= Some(Codes::lower_z());
                            } else {
                                t___927 = false;
                            }
                            t___928 = t___927;
                        }
                        t___929 = t___928;
                    }
                    t___930 = t___929;
                }
                t___931 = t___930;
            }
            t___932 = t___931;
        }
        if t___932 {
            t___949 = 0;
        } else {
            if Some(code__380) == Some(Codes::ampersand()) {
                t___948 = true;
            } else {
                if Some(code__380) == Some(Codes::backslash()) {
                    t___947 = true;
                } else {
                    if Some(code__380) == Some(Codes::caret()) {
                        t___946 = true;
                    } else {
                        if Some(code__380) == Some(Codes::curly_left()) {
                            t___945 = true;
                        } else {
                            if Some(code__380) == Some(Codes::curly_right()) {
                                t___944 = true;
                            } else {
                                if Some(code__380) == Some(Codes::dot()) {
                                    t___943 = true;
                                } else {
                                    if Some(code__380) == Some(Codes::peso()) {
                                        t___942 = true;
                                    } else {
                                        if Some(code__380) == Some(Codes::pipe()) {
                                            t___941 = true;
                                        } else {
                                            if Some(code__380) == Some(Codes::plus()) {
                                                t___940 = true;
                                            } else {
                                                if Some(code__380) == Some(Codes::question()) {
                                                    t___939 = true;
                                                } else {
                                                    if Some(code__380) == Some(Codes::round_left()) {
                                                        t___938 = true;
                                                    } else {
                                                        if Some(code__380) == Some(Codes::round_right()) {
                                                            t___937 = true;
                                                        } else {
                                                            if Some(code__380) == Some(Codes::slash()) {
                                                                t___936 = true;
                                                            } else {
                                                                if Some(code__380) == Some(Codes::square_left()) {
                                                                    t___935 = true;
                                                                } else {
                                                                    if Some(code__380) == Some(Codes::square_right()) {
                                                                        t___934 = true;
                                                                    } else {
                                                                        if Some(code__380) == Some(Codes::star()) {
                                                                            t___933 = true;
                                                                        } else {
                                                                            t___933 = Some(code__380) == Some(Codes::tilde());
                                                                        }
                                                                        t___934 = t___933;
                                                                    }
                                                                    t___935 = t___934;
                                                                }
                                                                t___936 = t___935;
                                                            }
                                                            t___937 = t___936;
                                                        }
                                                        t___938 = t___937;
                                                    }
                                                    t___939 = t___938;
                                                }
                                                t___940 = t___939;
                                            }
                                            t___941 = t___940;
                                        }
                                        t___942 = t___941;
                                    }
                                    t___943 = t___942;
                                }
                                t___944 = t___943;
                            }
                            t___945 = t___944;
                        }
                        t___946 = t___945;
                    }
                    t___947 = t___946;
                }
                t___948 = t___947;
            }
            if t___948 {
                t___949 = 2;
            } else {
                t___949 = 1;
            }
        }
        temper_core::listed::add( & escapeNeeds__379, t___949, None);
        code__380 = code__380.wrapping_add(1);
    }
    return temper_core::ListedTrait::to_list( & escapeNeeds__379);
}
pub fn entire(item__226: RegexNode) -> RegexNode {
    return RegexNode::new(Sequence::new([temper_core::cast::<RegexNode>(begin().clone()).unwrap(), item__226.clone(), temper_core::cast::<RegexNode>(end().clone()).unwrap()]));
}
pub fn one_or_more(item__228: RegexNode, reluctant__556: Option<bool>) -> Repeat {
    let reluctant__229: bool;
    if reluctant__556.is_none() {
        reluctant__229 = false;
    } else {
        reluctant__229 = reluctant__556.unwrap();
    }
    return Repeat::new(item__228.clone(), 1, None, Some(reluctant__229));
}
pub fn optional(item__231: RegexNode, reluctant__558: Option<bool>) -> Repeat {
    let reluctant__232: bool;
    if reluctant__558.is_none() {
        reluctant__232 = false;
    } else {
        reluctant__232 = reluctant__558.unwrap();
    }
    return Repeat::new(item__231.clone(), 0, Some(1), Some(reluctant__232));
}
