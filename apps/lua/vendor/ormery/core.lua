local temper = require('temper-core');
local Collector, Collected, CollectedFixed, CollectedPart, Context, AutoescState, AfterPropagate, ContextPropagator, Delegate, ContextDelegate, Escaper, EscaperPicker, ContextualAutoescapingAccumulator, Codec, Subsidiary, feedSubsidiary__83, propagateOver, exports;
Collector = temper.type('Collector');
Collector.methods.appendSafe = function(this__3, fixed__87)
  local t_99;
  t_99 = CollectedFixed(fixed__87);
  temper.listbuilder_add(this__3.partsBuilder__85, t_99);
  return nil;
end;
Collector.methods.append = function(this__4, part__90)
  local t_100;
  t_100 = CollectedPart(part__90);
  temper.listbuilder_add(this__4.partsBuilder__85, t_100);
  return nil;
end;
Collector.get.parts = function(this__5)
  return temper.listbuilder_tolist(this__5.partsBuilder__85);
end;
Collector.constructor = function(this__39)
  local t_101;
  t_101 = temper.listbuilder_constructor();
  this__39.partsBuilder__85 = t_101;
  return nil;
end;
Collected = temper.type('Collected');
CollectedFixed = temper.type('CollectedFixed', Collected);
CollectedFixed.constructor = function(this__44, safeText__97)
  this__44.safeText__95 = safeText__97;
  return nil;
end;
CollectedFixed.get.safeText = function(this__209)
  return this__209.safeText__95;
end;
CollectedPart = temper.type('CollectedPart', Collected);
CollectedPart.constructor = function(this__46, part__100)
  this__46.part__98 = part__100;
  return nil;
end;
CollectedPart.get.part = function(this__212)
  return this__212.part__98;
end;
Context = temper.type('Context');
Context.methods.toString = function(this__9)
  temper.virtual();
end;
AutoescState = temper.type('AutoescState');
AutoescState.constructor = function(this__49, context__106, subsidiary__107)
  if (subsidiary__107 == nil) then
    subsidiary__107 = temper.null;
  end
  this__49.context__103 = context__106;
  this__49.subsidiary__104 = subsidiary__107;
  return nil;
end;
AutoescState.get.context = function(this__221)
  return this__221.context__103;
end;
AutoescState.get.subsidiary = function(this__224)
  return this__224.subsidiary__104;
end;
AfterPropagate = temper.type('AfterPropagate');
AfterPropagate.methods.push = function(this__12, delegate__112, codec__113)
  return AfterPropagate(this__12.adjustedString__108, this__12.consumed__109, AutoescState(this__12.stateAfter__110.context, Subsidiary(delegate__112, codec__113)));
end;
AfterPropagate.methods.pop = function(this__13)
  return AfterPropagate(this__13.adjustedString__108, this__13.consumed__109, AutoescState(this__13.stateAfter__110.context, temper.null));
end;
AfterPropagate.methods.feed = function(this__14, prepareForInterp__118)
  local return__57, subsidiary__120;
  subsidiary__120 = this__14.stateAfter__110.subsidiary;
  if not temper.is_null(subsidiary__120) then
    local subsidiary_102, adjustedFromDelegate__121;
    subsidiary_102 = subsidiary__120;
    adjustedFromDelegate__121 = feedSubsidiary__83(subsidiary_102, this__14.adjustedString__108, prepareForInterp__118);
    return__57 = AfterPropagate(adjustedFromDelegate__121, this__14.consumed__109, this__14.stateAfter__110);
  else
    return__57 = this__14;
  end
  return return__57;
end;
AfterPropagate.constructor = function(this__52, adjustedString__123, consumed__124, stateAfter__125)
  this__52.adjustedString__108 = adjustedString__123;
  this__52.consumed__109 = consumed__124;
  this__52.stateAfter__110 = stateAfter__125;
  return nil;
end;
AfterPropagate.get.adjustedString = function(this__227)
  return this__227.adjustedString__108;
end;
AfterPropagate.get.consumed = function(this__230)
  return this__230.consumed__109;
end;
AfterPropagate.get.stateAfter = function(this__233)
  return this__233.stateAfter__110;
end;
ContextPropagator = temper.type('ContextPropagator');
ContextPropagator.methods.after = function(this__16, before__127, literalPart__128)
  if (literalPart__128 == nil) then
    literalPart__128 = temper.null;
  end
  temper.virtual();
end;
Delegate = temper.type('Delegate');
Delegate.methods.process = function(this__17, fixed__131)
  if (fixed__131 == nil) then
    fixed__131 = temper.null;
  end
  temper.virtual();
end;
ContextDelegate = temper.type('ContextDelegate', Delegate);
ContextDelegate.get.state = function(this__19)
  temper.virtual();
end;
ContextDelegate.set.state = function(this__20, x__136)
  temper.virtual();
end;
ContextDelegate.get.contextPropagator = function(this__21)
  temper.virtual();
end;
ContextDelegate.methods.process = function(this__22, known__141)
  local after__143, t_103;
  if (known__141 == nil) then
    known__141 = temper.null;
  end
  after__143 = propagateOver(this__22.contextPropagator, this__22.state, known__141);
  t_103 = after__143.stateAfter;
  this__22['state'] = t_103;
  return after__143.adjustedString;
end;
Escaper = temper.type('Escaper');
EscaperPicker = temper.type('EscaperPicker');
EscaperPicker.methods.escaperFor = function(this__26, before__153)
  temper.virtual();
end;
ContextualAutoescapingAccumulator = temper.type('ContextualAutoescapingAccumulator');
ContextualAutoescapingAccumulator.get.state = function(this__29)
  temper.virtual();
end;
ContextualAutoescapingAccumulator.set.state = function(this__30, newState__158)
  temper.virtual();
end;
ContextualAutoescapingAccumulator.get.escaperPicker = function(this__31)
  temper.virtual();
end;
ContextualAutoescapingAccumulator.get.contextPropagator = function(this__32)
  temper.virtual();
end;
ContextualAutoescapingAccumulator.methods.prepareForAppend = function(this__33)
  local after__166, t_104, adjusted__167;
  after__166 = propagateOver(this__33.contextPropagator, this__33.state, temper.null);
  t_104 = after__166.stateAfter;
  this__33['state'] = t_104;
  adjusted__167 = after__166.adjustedString;
  if not temper.string_isempty(adjusted__167) then
    this__33:collectFixed(adjusted__167);
  end
  return this__33.escaperPicker:escaperFor(this__33.state);
end;
ContextualAutoescapingAccumulator.methods.appendSafe = function(this__34, known__169)
  local after__171, t_105, adjusted__172;
  after__171 = propagateOver(this__34.contextPropagator, this__34.state, known__169);
  t_105 = after__171.stateAfter;
  this__34['state'] = t_105;
  adjusted__172 = after__171.adjustedString;
  if not temper.string_isempty(adjusted__172) then
    this__34:collectFixed(adjusted__172);
  end
  return nil;
end;
ContextualAutoescapingAccumulator.methods.collectFixed = function(this__35, fixedFragment__174)
  temper.virtual();
end;
Codec = temper.type('Codec');
Codec.methods.encode = function(this__36, s__184)
  temper.virtual();
end;
Codec.methods.decode = function(this__37, s__187)
  temper.virtual();
end;
Subsidiary = temper.type('Subsidiary');
Subsidiary.constructor = function(this__81, delegate__192, codec__193)
  this__81.delegate__189 = delegate__192;
  this__81.codec__190 = codec__193;
  return nil;
end;
Subsidiary.get.delegate = function(this__215)
  return this__215.delegate__189;
end;
Subsidiary.get.codec = function(this__218)
  return this__218.codec__190;
end;
feedSubsidiary__83 = function(subsidiary__176, adjustedStr__177, prepareForInterp__178)
  local t_106, str__180, delegate__181, codec__182, t_107, t_108, t_109;
  str__180 = adjustedStr__177;
  delegate__181 = subsidiary__176.delegate;
  codec__182 = subsidiary__176.codec;
  t_107 = codec__182:decode(str__180);
  str__180 = t_107;
  t_108 = delegate__181:process(str__180);
  str__180 = t_108;
  if prepareForInterp__178 then
    t_106 = delegate__181:process(temper.null);
    str__180 = temper.concat(str__180, t_106);
  end
  t_109 = codec__182:encode(str__180);
  str__180 = t_109;
  return str__180;
end;
propagateOver = function(contextPropagator__144, before__145, known__146)
  local return__66, t_110, t_111, t_112, t_113, t_114, t_115;
  if (known__146 == nil) then
    known__146 = temper.null;
  end
  if temper.is_null(known__146) then
    return__66 = contextPropagator__144:after(before__145, temper.null);
  else
    local known_116, state__148, remainder__149, adjusted__150;
    known_116 = known__146;
    state__148 = before__145;
    remainder__149 = known_116;
    adjusted__150 = temper.stringbuilder_constructor();
    while true do
      local after__151;
      if not not temper.string_isempty(remainder__149) then
        break;
      end
      after__151 = contextPropagator__144:after(state__148, remainder__149);
      temper.stringbuilder_append(adjusted__150, after__151.adjustedString);
      t_110 = after__151.stateAfter;
      state__148 = t_110;
      t_111 = after__151.consumed;
      t_112 = temper.string_end(remainder__149);
      t_113 = temper.string_slice(remainder__149, t_111, t_112);
      remainder__149 = t_113;
    end
    t_114 = temper.stringbuilder_tostring(adjusted__150);
    t_115 = temper.string_end(known_116);
    return__66 = AfterPropagate(t_114, t_115, state__148);
  end
  return return__66;
end;
exports = {};
exports.Collector = Collector;
exports.Collected = Collected;
exports.CollectedFixed = CollectedFixed;
exports.CollectedPart = CollectedPart;
exports.Context = Context;
exports.AutoescState = AutoescState;
exports.AfterPropagate = AfterPropagate;
exports.ContextPropagator = ContextPropagator;
exports.Delegate = Delegate;
exports.ContextDelegate = ContextDelegate;
exports.Escaper = Escaper;
exports.EscaperPicker = EscaperPicker;
exports.ContextualAutoescapingAccumulator = ContextualAutoescapingAccumulator;
exports.Codec = Codec;
exports.Subsidiary = Subsidiary;
exports.propagateOver = propagateOver;
return exports;
