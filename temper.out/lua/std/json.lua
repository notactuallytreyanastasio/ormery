local temper = require('temper-core');
local InterchangeContext, NullInterchangeContext, JsonProducer, JsonSyntaxTree, JsonObject, JsonArray, JsonBoolean, JsonNull, JsonString, JsonNumeric, JsonInt32, JsonInt64, JsonFloat64, JsonNumericToken, JsonTextProducer, JsonParseErrorReceiver, JsonSyntaxTreeProducer, parseJsonValue__356, JsonAdapter, BooleanJsonAdapter__164, Float64JsonAdapter__167, Int32JsonAdapter__170, Int64JsonAdapter__173, StringJsonAdapter__176, ListJsonAdapter__179, OrNullJsonAdapter, hexDigits__373, encodeHex4__352, encodeJsonString__351, storeJsonError__354, expectedTokenError__353, skipJsonSpaces__355, decodeHexUnsigned__361, parseJsonStringTo__360, parseJsonObject__357, parseJsonArray__358, parseJsonString__359, afterSubstring__364, parseJsonBoolean__362, parseJsonNull__363, parseJsonNumber__365, parseJsonToProducer, parseJson, booleanJsonAdapter, float64JsonAdapter, int32JsonAdapter, int64JsonAdapter, stringJsonAdapter, listJsonAdapter, exports;
InterchangeContext = temper.type('InterchangeContext');
InterchangeContext.methods.getHeader = function(this__78, headerName__376)
  temper.virtual();
end;
NullInterchangeContext = temper.type('NullInterchangeContext', InterchangeContext);
NullInterchangeContext.methods.getHeader = function(this__79, headerName__379)
  return temper.null;
end;
NullInterchangeContext.constructor = function(this__188)
  return nil;
end;
NullInterchangeContext.instance = NullInterchangeContext();
JsonProducer = temper.type('JsonProducer');
JsonProducer.get.interchangeContext = function(this_27)
end;
JsonProducer.methods.startObject = function(this__80)
  temper.virtual();
end;
JsonProducer.methods.endObject = function(this__81)
  temper.virtual();
end;
JsonProducer.methods.objectKey = function(this__82, key__389)
  temper.virtual();
end;
JsonProducer.methods.startArray = function(this__83)
  temper.virtual();
end;
JsonProducer.methods.endArray = function(this__84)
  temper.virtual();
end;
JsonProducer.methods.nullValue = function(this__85)
  temper.virtual();
end;
JsonProducer.methods.booleanValue = function(this__86, x__398)
  temper.virtual();
end;
JsonProducer.methods.int32Value = function(this__87, x__401)
  temper.virtual();
end;
JsonProducer.methods.int64Value = function(this__88, x__404)
  temper.virtual();
end;
JsonProducer.methods.float64Value = function(this__89, x__407)
  temper.virtual();
end;
JsonProducer.methods.numericTokenValue = function(this__90, x__410)
  temper.virtual();
end;
JsonProducer.methods.stringValue = function(this__91, x__413)
  temper.virtual();
end;
JsonProducer.get.parseErrorReceiver = function(this__92)
  return temper.null;
end;
JsonSyntaxTree = temper.type('JsonSyntaxTree');
JsonSyntaxTree.methods.produce = function(this__93, p__418)
  temper.virtual();
end;
JsonObject = temper.type('JsonObject', JsonSyntaxTree);
JsonObject.methods.propertyValueOrNull = function(this__94, propertyKey__422)
  local return__209, treeList__424, lastIndex__425;
  treeList__424 = temper.mapped_getor(this__94.properties__420, propertyKey__422, temper.listof());
  lastIndex__425 = temper.int32_sub(temper.list_length(treeList__424), 1);
  if (lastIndex__425 >= 0) then
    return__209 = temper.list_get(treeList__424, lastIndex__425);
  else
    return__209 = temper.null;
  end
  return return__209;
end;
JsonObject.methods.propertyValueOrBubble = function(this__95, propertyKey__427)
  local return__210, t_28;
  t_28 = this__95:propertyValueOrNull(propertyKey__427);
  if temper.is_null(t_28) then
    temper.bubble();
  else
    return__210 = t_28;
  end
  return return__210;
end;
JsonObject.methods.produce = function(this__96, p__430)
  local fn__2513;
  p__430:startObject();
  fn__2513 = function(k__432, vs__433)
    local fn__2510;
    fn__2510 = function(v__434)
      p__430:objectKey(k__432);
      v__434:produce(p__430);
      return nil;
    end;
    temper.list_foreach(vs__433, fn__2510);
    return nil;
  end;
  temper.mapped_foreach(this__96.properties__420, fn__2513);
  p__430:endObject();
  return nil;
end;
JsonObject.constructor = function(this__206, properties__436)
  this__206.properties__420 = properties__436;
  return nil;
end;
JsonObject.get.properties = function(this__875)
  return this__875.properties__420;
end;
JsonArray = temper.type('JsonArray', JsonSyntaxTree);
JsonArray.methods.produce = function(this__97, p__439)
  local fn__2503;
  p__439:startArray();
  fn__2503 = function(v__441)
    v__441:produce(p__439);
    return nil;
  end;
  temper.list_foreach(this__97.elements__437, fn__2503);
  p__439:endArray();
  return nil;
end;
JsonArray.constructor = function(this__212, elements__443)
  this__212.elements__437 = elements__443;
  return nil;
end;
JsonArray.get.elements = function(this__878)
  return this__878.elements__437;
end;
JsonBoolean = temper.type('JsonBoolean', JsonSyntaxTree);
JsonBoolean.methods.produce = function(this__98, p__446)
  p__446:booleanValue(this__98.content__444);
  return nil;
end;
JsonBoolean.constructor = function(this__216, content__449)
  this__216.content__444 = content__449;
  return nil;
end;
JsonBoolean.get.content = function(this__881)
  return this__881.content__444;
end;
JsonNull = temper.type('JsonNull', JsonSyntaxTree);
JsonNull.methods.produce = function(this__99, p__451)
  p__451:nullValue();
  return nil;
end;
JsonNull.constructor = function(this__219)
  return nil;
end;
JsonString = temper.type('JsonString', JsonSyntaxTree);
JsonString.methods.produce = function(this__100, p__456)
  p__456:stringValue(this__100.content__454);
  return nil;
end;
JsonString.constructor = function(this__222, content__459)
  this__222.content__454 = content__459;
  return nil;
end;
JsonString.get.content = function(this__884)
  return this__884.content__454;
end;
JsonNumeric = temper.type('JsonNumeric', JsonSyntaxTree);
JsonNumeric.methods.asJsonNumericToken = function(this__101)
  temper.virtual();
end;
JsonNumeric.methods.asInt32 = function(this__102)
  temper.virtual();
end;
JsonNumeric.methods.asInt64 = function(this__103)
  temper.virtual();
end;
JsonNumeric.methods.asFloat64 = function(this__104)
  temper.virtual();
end;
JsonInt32 = temper.type('JsonInt32', JsonNumeric);
JsonInt32.methods.produce = function(this__105, p__470)
  p__470:int32Value(this__105.content__468);
  return nil;
end;
JsonInt32.methods.asJsonNumericToken = function(this__106)
  return temper.int32_tostring(this__106.content__468);
end;
JsonInt32.methods.asInt32 = function(this__107)
  return this__107.content__468;
end;
JsonInt32.methods.asInt64 = function(this__108)
  return temper.int32_toint64(this__108.content__468);
end;
JsonInt32.methods.asFloat64 = function(this__109)
  return temper.int32_tofloat64(this__109.content__468);
end;
JsonInt32.constructor = function(this__229, content__481)
  this__229.content__468 = content__481;
  return nil;
end;
JsonInt32.get.content = function(this__887)
  return this__887.content__468;
end;
JsonInt64 = temper.type('JsonInt64', JsonNumeric);
JsonInt64.methods.produce = function(this__110, p__484)
  p__484:int64Value(this__110.content__482);
  return nil;
end;
JsonInt64.methods.asJsonNumericToken = function(this__111)
  return temper.int64_tostring(this__111.content__482);
end;
JsonInt64.methods.asInt32 = function(this__112)
  return temper.int64_toint32(this__112.content__482);
end;
JsonInt64.methods.asInt64 = function(this__113)
  return this__113.content__482;
end;
JsonInt64.methods.asFloat64 = function(this__114)
  return temper.int64_tofloat64(this__114.content__482);
end;
JsonInt64.constructor = function(this__236, content__495)
  this__236.content__482 = content__495;
  return nil;
end;
JsonInt64.get.content = function(this__890)
  return this__890.content__482;
end;
JsonFloat64 = temper.type('JsonFloat64', JsonNumeric);
JsonFloat64.methods.produce = function(this__115, p__498)
  p__498:float64Value(this__115.content__496);
  return nil;
end;
JsonFloat64.methods.asJsonNumericToken = function(this__116)
  return temper.float64_tostring(this__116.content__496);
end;
JsonFloat64.methods.asInt32 = function(this__117)
  return temper.float64_toint32(this__117.content__496);
end;
JsonFloat64.methods.asInt64 = function(this__118)
  return temper.float64_toint64(this__118.content__496);
end;
JsonFloat64.methods.asFloat64 = function(this__119)
  return this__119.content__496;
end;
JsonFloat64.constructor = function(this__243, content__509)
  this__243.content__496 = content__509;
  return nil;
end;
JsonFloat64.get.content = function(this__893)
  return this__893.content__496;
end;
JsonNumericToken = temper.type('JsonNumericToken', JsonNumeric);
JsonNumericToken.methods.produce = function(this__120, p__512)
  p__512:numericTokenValue(this__120.content__510);
  return nil;
end;
JsonNumericToken.methods.asJsonNumericToken = function(this__121)
  return this__121.content__510;
end;
JsonNumericToken.methods.asInt32 = function(this__122)
  local return__254, t_29, t_30, local_31, local_32, local_33;
  local_31, local_32, local_33 = temper.pcall(function()
    t_29 = temper.string_toint32(this__122.content__510);
    return__254 = t_29;
  end);
  if local_31 then
  else
    t_30 = temper.string_tofloat64(this__122.content__510);
    return__254 = temper.float64_toint32(t_30);
  end
  return return__254;
end;
JsonNumericToken.methods.asInt64 = function(this__123)
  local return__255, t_35, t_36, local_37, local_38, local_39;
  local_37, local_38, local_39 = temper.pcall(function()
    t_35 = temper.string_toint64(this__123.content__510);
    return__255 = t_35;
  end);
  if local_37 then
  else
    t_36 = temper.string_tofloat64(this__123.content__510);
    return__255 = temper.float64_toint64(t_36);
  end
  return return__255;
end;
JsonNumericToken.methods.asFloat64 = function(this__124)
  return temper.string_tofloat64(this__124.content__510);
end;
JsonNumericToken.constructor = function(this__250, content__523)
  this__250.content__510 = content__523;
  return nil;
end;
JsonNumericToken.get.content = function(this__896)
  return this__896.content__510;
end;
JsonTextProducer = temper.type('JsonTextProducer', JsonProducer);
JsonTextProducer.constructor = function(this__125, interchangeContext__938)
  local interchangeContext__529, t_41, t_42;
  if temper.is_null(interchangeContext__938) then
    interchangeContext__529 = NullInterchangeContext.instance;
  else
    interchangeContext__529 = interchangeContext__938;
  end
  this__125.interchangeContext__524 = interchangeContext__529;
  t_41 = temper.stringbuilder_constructor();
  this__125.buffer__525 = t_41;
  t_42 = temper.listbuilder_constructor();
  this__125.stack__526 = t_42;
  temper.listbuilder_add(this__125.stack__526, 5);
  this__125.wellFormed__527 = true;
  return nil;
end;
JsonTextProducer.methods.state = function(this__126)
  local t_43;
  t_43 = temper.listbuilder_length(this__126.stack__526);
  return temper.listed_getor(this__126.stack__526, temper.int32_sub(t_43, 1), -1);
end;
JsonTextProducer.methods.beforeValue = function(this__127)
  local t_44, t_45, t_46, t_47, currentState__535;
  currentState__535 = this__127:state();
  if (currentState__535 == 3) then
    t_44 = temper.listbuilder_length(this__127.stack__526);
    temper.listbuilder_set(this__127.stack__526, temper.int32_sub(t_44, 1), 4);
  elseif (currentState__535 == 4) then
    temper.stringbuilder_append(this__127.buffer__525, ',');
  elseif (currentState__535 == 1) then
    t_45 = temper.listbuilder_length(this__127.stack__526);
    temper.listbuilder_set(this__127.stack__526, temper.int32_sub(t_45, 1), 2);
  elseif (currentState__535 == 5) then
    t_46 = temper.listbuilder_length(this__127.stack__526);
    temper.listbuilder_set(this__127.stack__526, temper.int32_sub(t_46, 1), 6);
  else
    if (currentState__535 == 6) then
      t_47 = true;
    else
      t_47 = (currentState__535 == 2);
    end
    if t_47 then
      this__127.wellFormed__527 = false;
    end
  end
  return nil;
end;
JsonTextProducer.methods.startObject = function(this__128)
  this__128:beforeValue();
  temper.stringbuilder_append(this__128.buffer__525, '{');
  temper.listbuilder_add(this__128.stack__526, 0);
  return nil;
end;
JsonTextProducer.methods.endObject = function(this__129)
  local t_48, currentState__540;
  temper.stringbuilder_append(this__129.buffer__525, '}');
  currentState__540 = this__129:state();
  if (0 == currentState__540) then
    t_48 = true;
  else
    t_48 = (2 == currentState__540);
  end
  if t_48 then
    temper.listbuilder_removelast(this__129.stack__526);
  else
    this__129.wellFormed__527 = false;
  end
  return nil;
end;
JsonTextProducer.methods.objectKey = function(this__130, key__542)
  local t_49, currentState__544;
  currentState__544 = this__130:state();
  if not (currentState__544 == 0) then
    if (currentState__544 == 2) then
      temper.stringbuilder_append(this__130.buffer__525, ',');
    else
      this__130.wellFormed__527 = false;
    end
  end
  encodeJsonString__351(key__542, this__130.buffer__525);
  temper.stringbuilder_append(this__130.buffer__525, ':');
  if (currentState__544 >= 0) then
    t_49 = temper.listbuilder_length(this__130.stack__526);
    temper.listbuilder_set(this__130.stack__526, temper.int32_sub(t_49, 1), 1);
  end
  return nil;
end;
JsonTextProducer.methods.startArray = function(this__131)
  this__131:beforeValue();
  temper.stringbuilder_append(this__131.buffer__525, '[');
  temper.listbuilder_add(this__131.stack__526, 3);
  return nil;
end;
JsonTextProducer.methods.endArray = function(this__132)
  local t_50, currentState__549;
  temper.stringbuilder_append(this__132.buffer__525, ']');
  currentState__549 = this__132:state();
  if (3 == currentState__549) then
    t_50 = true;
  else
    t_50 = (4 == currentState__549);
  end
  if t_50 then
    temper.listbuilder_removelast(this__132.stack__526);
  else
    this__132.wellFormed__527 = false;
  end
  return nil;
end;
JsonTextProducer.methods.nullValue = function(this__133)
  this__133:beforeValue();
  temper.stringbuilder_append(this__133.buffer__525, 'null');
  return nil;
end;
JsonTextProducer.methods.booleanValue = function(this__134, x__553)
  local t_51;
  this__134:beforeValue();
  if x__553 then
    t_51 = 'true';
  else
    t_51 = 'false';
  end
  temper.stringbuilder_append(this__134.buffer__525, t_51);
  return nil;
end;
JsonTextProducer.methods.int32Value = function(this__135, x__556)
  local t_52;
  this__135:beforeValue();
  t_52 = temper.int32_tostring(x__556);
  temper.stringbuilder_append(this__135.buffer__525, t_52);
  return nil;
end;
JsonTextProducer.methods.int64Value = function(this__136, x__559)
  local t_53;
  this__136:beforeValue();
  t_53 = temper.int64_tostring(x__559);
  temper.stringbuilder_append(this__136.buffer__525, t_53);
  return nil;
end;
JsonTextProducer.methods.float64Value = function(this__137, x__562)
  local t_54;
  this__137:beforeValue();
  t_54 = temper.float64_tostring(x__562);
  temper.stringbuilder_append(this__137.buffer__525, t_54);
  return nil;
end;
JsonTextProducer.methods.numericTokenValue = function(this__138, x__565)
  this__138:beforeValue();
  temper.stringbuilder_append(this__138.buffer__525, x__565);
  return nil;
end;
JsonTextProducer.methods.stringValue = function(this__139, x__568)
  this__139:beforeValue();
  encodeJsonString__351(x__568, this__139.buffer__525);
  return nil;
end;
JsonTextProducer.methods.toJsonString = function(this__140)
  local return__272, t_55, t_56, t_57;
  if this__140.wellFormed__527 then
    if (temper.listbuilder_length(this__140.stack__526) == 1) then
      t_55 = this__140:state();
      t_56 = (t_55 == 6);
    else
      t_56 = false;
    end
    t_57 = t_56;
  else
    t_57 = false;
  end
  if t_57 then
    return__272 = temper.stringbuilder_tostring(this__140.buffer__525);
  else
    temper.bubble();
  end
  return return__272;
end;
JsonTextProducer.get.interchangeContext = function(this__906)
  return this__906.interchangeContext__524;
end;
JsonParseErrorReceiver = temper.type('JsonParseErrorReceiver');
JsonParseErrorReceiver.methods.explainJsonError = function(this__141, explanation__588)
  temper.virtual();
end;
JsonSyntaxTreeProducer = temper.type('JsonSyntaxTreeProducer', JsonProducer, JsonParseErrorReceiver);
JsonSyntaxTreeProducer.get.interchangeContext = function(this__142)
  return NullInterchangeContext.instance;
end;
JsonSyntaxTreeProducer.constructor = function(this__143)
  local t_58, t_59;
  t_58 = temper.listbuilder_constructor();
  this__143.stack__590 = t_58;
  t_59 = temper.listbuilder_constructor();
  temper.listbuilder_add(this__143.stack__590, t_59);
  this__143.error__591 = temper.null;
  return nil;
end;
JsonSyntaxTreeProducer.methods.storeValue = function(this__144, v__597)
  local t_60;
  if not temper.listed_isempty(this__144.stack__590) then
    t_60 = temper.listbuilder_length(this__144.stack__590);
    temper.listbuilder_add(temper.listed_get(this__144.stack__590, temper.int32_sub(t_60, 1)), v__597);
  end
  return nil;
end;
JsonSyntaxTreeProducer.methods.startObject = function(this__145)
  local t_61;
  t_61 = temper.listbuilder_constructor();
  temper.listbuilder_add(this__145.stack__590, t_61);
  return nil;
end;
JsonSyntaxTreeProducer.methods.endObject = function(this__146)
  local return__283, t_62, t_63, t_64, t_65, t_66, t_67, t_68, t_69, t_70, t_71, ls__603, m__604, multis__605, i__606, n__607, multis__612;
  ::continue_3::
  if temper.listed_isempty(this__146.stack__590) then
    return__283 = nil;
    goto break_2;
  end
  ls__603 = temper.listbuilder_removelast(this__146.stack__590);
  m__604 = temper.mapbuilder_constructor();
  multis__605 = temper.null;
  i__606 = 0;
  n__607 = temper.band(temper.listbuilder_length(ls__603), -2);
  while (i__606 < n__607) do
    local postfixReturn_72, keyTree__608, local_73, local_74, local_75, key__609, postfixReturn_77, value__610;
    postfixReturn_72 = i__606;
    i__606 = temper.int32_add(i__606, 1);
    keyTree__608 = temper.listed_get(ls__603, postfixReturn_72);
    if not temper.instance_of(keyTree__608, JsonString) then
      break;
    end
    local_73, local_74, local_75 = temper.pcall(function()
      t_64 = temper.cast_to(keyTree__608, JsonString);
      t_65 = t_64;
    end);
    if local_73 then
    else
      t_65 = temper.bubble();
    end
    key__609 = t_65.content;
    postfixReturn_77 = i__606;
    i__606 = temper.int32_add(i__606, 1);
    value__610 = temper.listed_get(ls__603, postfixReturn_77);
    if temper.mapped_has(m__604, key__609) then
      local local_78, local_79, local_80, mb__611, local_86, local_87, local_88;
      if temper.is_null(multis__605) then
        t_62 = temper.mapbuilder_constructor();
        multis__605 = t_62;
      end
      local_78, local_79, local_80 = temper.pcall(function()
        if temper.is_null(multis__605) then
          temper.bubble();
        else
          t_66 = multis__605;
        end
        t_67 = t_66;
      end);
      if local_78 then
      else
        t_67 = temper.bubble();
      end
      mb__611 = t_67;
      if not temper.mapped_has(mb__611, key__609) then
        local local_82, local_83, local_84;
        local_82, local_83, local_84 = temper.pcall(function()
          t_68 = temper.mapped_get(m__604, key__609);
          t_69 = t_68;
        end);
        if local_82 then
        else
          t_69 = temper.bubble();
        end
        temper.mapbuilder_set(mb__611, key__609, temper.list_tolistbuilder(t_69));
      end
      local_86, local_87, local_88 = temper.pcall(function()
        t_70 = temper.mapped_get(mb__611, key__609);
        t_71 = t_70;
      end);
      if local_86 then
      else
        t_71 = temper.bubble();
      end
      temper.listbuilder_add(t_71, value__610);
    else
      temper.mapbuilder_set(m__604, key__609, temper.listof(value__610));
    end
  end
  multis__612 = multis__605;
  if not temper.is_null(multis__612) then
    local fn__2392;
    fn__2392 = function(k__613, vs__614)
      local t_90;
      t_90 = temper.listbuilder_tolist(vs__614);
      temper.mapbuilder_set(m__604, k__613, t_90);
      return nil;
    end;
    temper.mapped_foreach(multis__612, fn__2392);
  end
  t_63 = JsonObject(temper.mapped_tomap(m__604));
  this__146:storeValue(t_63);
  return__283 = nil;
  ::break_2::return return__283;
end;
JsonSyntaxTreeProducer.methods.objectKey = function(this__147, key__616)
  local t_91;
  t_91 = JsonString(key__616);
  this__147:storeValue(t_91);
  return nil;
end;
JsonSyntaxTreeProducer.methods.startArray = function(this__148)
  local t_92;
  t_92 = temper.listbuilder_constructor();
  temper.listbuilder_add(this__148.stack__590, t_92);
  return nil;
end;
JsonSyntaxTreeProducer.methods.endArray = function(this__149)
  local return__286, t_93, ls__622;
  ::continue_5::
  if temper.listed_isempty(this__149.stack__590) then
    return__286 = nil;
    goto break_4;
  end
  ls__622 = temper.listbuilder_removelast(this__149.stack__590);
  t_93 = JsonArray(temper.listbuilder_tolist(ls__622));
  this__149:storeValue(t_93);
  return__286 = nil;
  ::break_4::return return__286;
end;
JsonSyntaxTreeProducer.methods.nullValue = function(this__150)
  local t_94;
  t_94 = JsonNull();
  this__150:storeValue(t_94);
  return nil;
end;
JsonSyntaxTreeProducer.methods.booleanValue = function(this__151, x__626)
  local t_95;
  t_95 = JsonBoolean(x__626);
  this__151:storeValue(t_95);
  return nil;
end;
JsonSyntaxTreeProducer.methods.int32Value = function(this__152, x__629)
  local t_96;
  t_96 = JsonInt32(x__629);
  this__152:storeValue(t_96);
  return nil;
end;
JsonSyntaxTreeProducer.methods.int64Value = function(this__153, x__632)
  local t_97;
  t_97 = JsonInt64(x__632);
  this__153:storeValue(t_97);
  return nil;
end;
JsonSyntaxTreeProducer.methods.float64Value = function(this__154, x__635)
  local t_98;
  t_98 = JsonFloat64(x__635);
  this__154:storeValue(t_98);
  return nil;
end;
JsonSyntaxTreeProducer.methods.numericTokenValue = function(this__155, x__638)
  local t_99;
  t_99 = JsonNumericToken(x__638);
  this__155:storeValue(t_99);
  return nil;
end;
JsonSyntaxTreeProducer.methods.stringValue = function(this__156, x__641)
  local t_100;
  t_100 = JsonString(x__641);
  this__156:storeValue(t_100);
  return nil;
end;
JsonSyntaxTreeProducer.methods.toJsonSyntaxTree = function(this__157)
  local t_101, ls__645;
  if (temper.listbuilder_length(this__157.stack__590) ~= 1) then
    t_101 = true;
  else
    t_101 = not temper.is_null(this__157.error__591);
  end
  if t_101 then
    temper.bubble();
  end
  ls__645 = temper.listed_get(this__157.stack__590, 0);
  if (temper.listbuilder_length(ls__645) ~= 1) then
    temper.bubble();
  end
  return temper.listed_get(ls__645, 0);
end;
JsonSyntaxTreeProducer.get.jsonError = function(this__158)
  return this__158.error__591;
end;
JsonSyntaxTreeProducer.get.parseErrorReceiver = function(this__159)
  return this__159;
end;
JsonSyntaxTreeProducer.methods.explainJsonError = function(this__160, error__651)
  this__160.error__591 = error__651;
  return nil;
end;
parseJsonValue__356 = function(sourceText__670, i__671, out__672)
  local return__302, t_102, t_103, t_104;
  ::continue_7::t_102 = skipJsonSpaces__355(sourceText__670, i__671);
  i__671 = t_102;
  if not temper.string_hasindex(sourceText__670, i__671) then
    expectedTokenError__353(sourceText__670, i__671, out__672, 'JSON value');
    return__302 = -1.0;
    goto break_6;
  end
  t_103 = temper.string_get(sourceText__670, i__671);
  if (t_103 == 123) then
    return__302 = parseJsonObject__357(sourceText__670, i__671, out__672);
  elseif (t_103 == 91) then
    return__302 = parseJsonArray__358(sourceText__670, i__671, out__672);
  elseif (t_103 == 34) then
    return__302 = parseJsonString__359(sourceText__670, i__671, out__672);
  else
    if (t_103 == 116) then
      t_104 = true;
    else
      t_104 = (t_103 == 102);
    end
    if t_104 then
      return__302 = parseJsonBoolean__362(sourceText__670, i__671, out__672);
    elseif (t_103 == 110) then
      return__302 = parseJsonNull__363(sourceText__670, i__671, out__672);
    else
      return__302 = parseJsonNumber__365(sourceText__670, i__671, out__672);
    end
  end
  ::break_6::return return__302;
end;
JsonAdapter = temper.type('JsonAdapter');
JsonAdapter.methods.encodeToJson = function(this__162, x__765, p__766)
  temper.virtual();
end;
JsonAdapter.methods.decodeFromJson = function(this__163, t__769, ic__770)
  temper.virtual();
end;
BooleanJsonAdapter__164 = temper.type('BooleanJsonAdapter__164', JsonAdapter);
BooleanJsonAdapter__164.methods.encodeToJson = function(this__165, x__773, p__774)
  p__774:booleanValue(x__773);
  return nil;
end;
BooleanJsonAdapter__164.methods.decodeFromJson = function(this__166, t__777, ic__778)
  local t_105;
  t_105 = temper.cast_to(t__777, JsonBoolean);
  return t_105.content;
end;
BooleanJsonAdapter__164.constructor = function(this__315)
  return nil;
end;
Float64JsonAdapter__167 = temper.type('Float64JsonAdapter__167', JsonAdapter);
Float64JsonAdapter__167.methods.encodeToJson = function(this__168, x__783, p__784)
  p__784:float64Value(x__783);
  return nil;
end;
Float64JsonAdapter__167.methods.decodeFromJson = function(this__169, t__787, ic__788)
  local t_106;
  t_106 = temper.cast_to(t__787, JsonNumeric);
  return t_106:asFloat64();
end;
Float64JsonAdapter__167.constructor = function(this__320)
  return nil;
end;
Int32JsonAdapter__170 = temper.type('Int32JsonAdapter__170', JsonAdapter);
Int32JsonAdapter__170.methods.encodeToJson = function(this__171, x__793, p__794)
  p__794:int32Value(x__793);
  return nil;
end;
Int32JsonAdapter__170.methods.decodeFromJson = function(this__172, t__797, ic__798)
  local t_107;
  t_107 = temper.cast_to(t__797, JsonNumeric);
  return t_107:asInt32();
end;
Int32JsonAdapter__170.constructor = function(this__325)
  return nil;
end;
Int64JsonAdapter__173 = temper.type('Int64JsonAdapter__173', JsonAdapter);
Int64JsonAdapter__173.methods.encodeToJson = function(this__174, x__803, p__804)
  p__804:int64Value(x__803);
  return nil;
end;
Int64JsonAdapter__173.methods.decodeFromJson = function(this__175, t__807, ic__808)
  local t_108;
  t_108 = temper.cast_to(t__807, JsonNumeric);
  return t_108:asInt64();
end;
Int64JsonAdapter__173.constructor = function(this__330)
  return nil;
end;
StringJsonAdapter__176 = temper.type('StringJsonAdapter__176', JsonAdapter);
StringJsonAdapter__176.methods.encodeToJson = function(this__177, x__813, p__814)
  p__814:stringValue(x__813);
  return nil;
end;
StringJsonAdapter__176.methods.decodeFromJson = function(this__178, t__817, ic__818)
  local t_109;
  t_109 = temper.cast_to(t__817, JsonString);
  return t_109.content;
end;
StringJsonAdapter__176.constructor = function(this__335)
  return nil;
end;
ListJsonAdapter__179 = temper.type('ListJsonAdapter__179', JsonAdapter);
ListJsonAdapter__179.methods.encodeToJson = function(this__181, x__824, p__825)
  local fn__2174;
  p__825:startArray();
  fn__2174 = function(el__827)
    this__181.adapterForT__822:encodeToJson(el__827, p__825);
    return nil;
  end;
  temper.list_foreach(x__824, fn__2174);
  p__825:endArray();
  return nil;
end;
ListJsonAdapter__179.methods.decodeFromJson = function(this__182, t__829, ic__830)
  local t_110, b__832, t_111, elements__833, n__834, i__835;
  b__832 = temper.listbuilder_constructor();
  t_111 = temper.cast_to(t__829, JsonArray);
  elements__833 = t_111.elements;
  n__834 = temper.list_length(elements__833);
  i__835 = 0;
  while (i__835 < n__834) do
    local el__836;
    el__836 = temper.list_get(elements__833, i__835);
    i__835 = temper.int32_add(i__835, 1);
    t_110 = this__182.adapterForT__822:decodeFromJson(el__836, ic__830);
    temper.listbuilder_add(b__832, t_110);
  end
  return temper.listbuilder_tolist(b__832);
end;
ListJsonAdapter__179.constructor = function(this__340, adapterForT__838)
  this__340.adapterForT__822 = adapterForT__838;
  return nil;
end;
OrNullJsonAdapter = temper.type('OrNullJsonAdapter', JsonAdapter);
OrNullJsonAdapter.methods.encodeToJson = function(this__185, x__843, p__844)
  if (x__843 == nil) then
    x__843 = temper.null;
  end
  if temper.is_null(x__843) then
    p__844:nullValue();
  else
    local x_112;
    x_112 = x__843;
    this__185.adapterForT__841:encodeToJson(x_112, p__844);
  end
  return nil;
end;
OrNullJsonAdapter.methods.decodeFromJson = function(this__186, t__847, ic__848)
  local return__350;
  if temper.instance_of(t__847, JsonNull) then
    return__350 = temper.null;
  else
    return__350 = this__186.adapterForT__841:decodeFromJson(t__847, ic__848);
  end
  return return__350;
end;
OrNullJsonAdapter.constructor = function(this__346, adapterForT__851)
  this__346.adapterForT__841 = adapterForT__851;
  return nil;
end;
hexDigits__373 = temper.listof('0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f');
encodeHex4__352 = function(cp__580, buffer__581)
  local b0__583, b1__584, b2__585, b3__586, t_113, t_114, t_115, t_116;
  b0__583 = temper.band(temper.int32_div(cp__580, 4096), 15);
  b1__584 = temper.band(temper.int32_div(cp__580, 256), 15);
  b2__585 = temper.band(temper.int32_div(cp__580, 16), 15);
  b3__586 = temper.band(cp__580, 15);
  t_113 = temper.list_get(hexDigits__373, b0__583);
  temper.stringbuilder_append(buffer__581, t_113);
  t_114 = temper.list_get(hexDigits__373, b1__584);
  temper.stringbuilder_append(buffer__581, t_114);
  t_115 = temper.list_get(hexDigits__373, b2__585);
  temper.stringbuilder_append(buffer__581, t_115);
  t_116 = temper.list_get(hexDigits__373, b3__586);
  temper.stringbuilder_append(buffer__581, t_116);
  return nil;
end;
encodeJsonString__351 = function(x__572, buffer__573)
  local t_117, t_118, t_119, t_120, i__575, emitted__576;
  temper.stringbuilder_append(buffer__573, '"');
  i__575 = 1.0;
  emitted__576 = i__575;
  while true do
    local cp__577, replacement__578, nextI__579;
    if not temper.string_hasindex(x__572, i__575) then
      break;
    end
    cp__577 = temper.string_get(x__572, i__575);
    if (cp__577 == 8) then
      t_120 = '\\b';
    elseif (cp__577 == 9) then
      t_120 = '\\t';
    elseif (cp__577 == 10) then
      t_120 = '\\n';
    elseif (cp__577 == 12) then
      t_120 = '\\f';
    elseif (cp__577 == 13) then
      t_120 = '\\r';
    elseif (cp__577 == 34) then
      t_120 = '\\"';
    elseif (cp__577 == 92) then
      t_120 = '\\\\';
    else
      if (cp__577 < 32) then
        t_118 = true;
      else
        if (55296 <= cp__577) then
          t_117 = (cp__577 <= 57343);
        else
          t_117 = false;
        end
        t_118 = t_117;
      end
      if t_118 then
        t_119 = '\\u';
      else
        t_119 = '';
      end
      t_120 = t_119;
    end
    replacement__578 = t_120;
    nextI__579 = temper.string_next(x__572, i__575);
    if temper.str_ne(replacement__578, '') then
      temper.stringbuilder_appendbetween(buffer__573, x__572, emitted__576, i__575);
      temper.stringbuilder_append(buffer__573, replacement__578);
      if temper.str_eq(replacement__578, '\\u') then
        encodeHex4__352(cp__577, buffer__573);
      end
      emitted__576 = nextI__579;
    end
    i__575 = nextI__579;
  end
  temper.stringbuilder_appendbetween(buffer__573, x__572, emitted__576, i__575);
  temper.stringbuilder_append(buffer__573, '"');
  return nil;
end;
storeJsonError__354 = function(out__659, explanation__660)
  local t_121;
  t_121 = out__659.parseErrorReceiver;
  if not temper.is_null(t_121) then
    t_121:explainJsonError(explanation__660);
  end
  return nil;
end;
expectedTokenError__353 = function(sourceText__653, i__654, out__655, shortExplanation__656)
  local t_122, t_123, gotten__658;
  if temper.string_hasindex(sourceText__653, i__654) then
    t_122 = temper.string_end(sourceText__653);
    t_123 = temper.string_slice(sourceText__653, i__654, t_122);
    gotten__658 = temper.concat('`', t_123, '`');
  else
    gotten__658 = 'end-of-file';
  end
  storeJsonError__354(out__655, temper.concat('Expected ', shortExplanation__656, ', but got ', gotten__658));
  return nil;
end;
skipJsonSpaces__355 = function(sourceText__667, i__668)
  local t_124, t_125, t_126, t_127, t_128;
  while true do
    if not temper.string_hasindex(sourceText__667, i__668) then
      break;
    end
    t_124 = temper.string_get(sourceText__667, i__668);
    if (t_124 == 9) then
      t_128 = true;
    else
      if (t_124 == 10) then
        t_127 = true;
      else
        if (t_124 == 13) then
          t_126 = true;
        else
          t_126 = (t_124 == 32);
        end
        t_127 = t_126;
      end
      t_128 = t_127;
    end
    if not t_128 then
      break;
    end
    t_125 = temper.string_next(sourceText__667, i__668);
    i__668 = t_125;
  end
  return i__668;
end;
decodeHexUnsigned__361 = function(sourceText__708, start__709, limit__710)
  local return__307, t_129, t_130, t_131, t_132, t_133, n__712, i__713;
  ::continue_9::n__712 = 0;
  i__713 = start__709;
  while true do
    local cp__714, digit__715;
    if not (temper.stringindexoption_compareto(i__713, limit__710) < 0) then
      break;
    end
    cp__714 = temper.string_get(sourceText__708, i__713);
    if (48 <= cp__714) then
      t_130 = (cp__714 <= 48);
    else
      t_130 = false;
    end
    if t_130 then
      t_133 = temper.int32_sub(cp__714, 48);
    else
      if (65 <= cp__714) then
        t_131 = (cp__714 <= 70);
      else
        t_131 = false;
      end
      if t_131 then
        t_133 = temper.int32_add(temper.int32_sub(cp__714, 65), 10);
      else
        if (97 <= cp__714) then
          t_132 = (cp__714 <= 102);
        else
          t_132 = false;
        end
        if t_132 then
          t_133 = temper.int32_add(temper.int32_sub(cp__714, 97), 10);
        else
          return__307 = -1;
          goto break_8;
        end
      end
    end
    digit__715 = t_133;
    n__712 = temper.int32_add(temper.int32_mul(n__712, 16), digit__715);
    t_129 = temper.string_next(sourceText__708, i__713);
    i__713 = t_129;
  end
  return__307 = n__712;
  ::break_8::return return__307;
end;
parseJsonStringTo__360 = function(sourceText__692, i__693, sb__694, errOut__695)
  local return__306, t_134, t_135, t_136, t_137, t_138, t_139, t_140, t_141, t_142, t_143, t_144, t_145, t_146, t_147, t_148, t_149, t_150, t_151, t_152, t_153, t_154, t_155, leadSurrogate__697, consumed__698;
  ::continue_11::
  if not temper.string_hasindex(sourceText__692, i__693) then
    t_145 = true;
  else
    t_134 = temper.string_get(sourceText__692, i__693);
    t_145 = (t_134 ~= 34);
  end
  if t_145 then
    expectedTokenError__353(sourceText__692, i__693, errOut__695, '"');
    return__306 = -1.0;
    goto break_10;
  end
  t_135 = temper.string_next(sourceText__692, i__693);
  i__693 = t_135;
  leadSurrogate__697 = -1;
  consumed__698 = i__693;
  while true do
    local cp__699, iNext__700, end__701, needToFlush__702, decodedCp__703;
    if not temper.string_hasindex(sourceText__692, i__693) then
      break;
    end
    cp__699 = temper.string_get(sourceText__692, i__693);
    if (cp__699 == 34) then
      break;
    end
    t_136 = temper.string_next(sourceText__692, i__693);
    iNext__700 = t_136;
    end__701 = temper.string_end(sourceText__692);
    needToFlush__702 = false;
    if (cp__699 ~= 92) then
      t_151 = cp__699;
    else
      local esc0__704, hex__705;
      needToFlush__702 = true;
      if not temper.string_hasindex(sourceText__692, iNext__700) then
        expectedTokenError__353(sourceText__692, iNext__700, errOut__695, 'escape sequence');
        return__306 = -1.0;
        goto break_10;
      end
      esc0__704 = temper.string_get(sourceText__692, iNext__700);
      t_137 = temper.string_next(sourceText__692, iNext__700);
      iNext__700 = t_137;
      if (esc0__704 == 34) then
        t_147 = true;
      else
        if (esc0__704 == 92) then
          t_146 = true;
        else
          t_146 = (esc0__704 == 47);
        end
        t_147 = t_146;
      end
      if t_147 then
        t_150 = esc0__704;
      elseif (esc0__704 == 98) then
        t_150 = 8;
      elseif (esc0__704 == 102) then
        t_150 = 12;
      elseif (esc0__704 == 110) then
        t_150 = 10;
      elseif (esc0__704 == 114) then
        t_150 = 13;
      elseif (esc0__704 == 116) then
        t_150 = 9;
      elseif (esc0__704 == 117) then
        if temper.string_hasatleast(sourceText__692, iNext__700, end__701, 4) then
          local startHex__706;
          startHex__706 = iNext__700;
          t_138 = temper.string_next(sourceText__692, iNext__700);
          iNext__700 = t_138;
          t_139 = temper.string_next(sourceText__692, iNext__700);
          iNext__700 = t_139;
          t_140 = temper.string_next(sourceText__692, iNext__700);
          iNext__700 = t_140;
          t_141 = temper.string_next(sourceText__692, iNext__700);
          iNext__700 = t_141;
          t_142 = decodeHexUnsigned__361(sourceText__692, startHex__706, iNext__700);
          t_148 = t_142;
        else
          t_148 = -1;
        end
        hex__705 = t_148;
        if (hex__705 < 0) then
          expectedTokenError__353(sourceText__692, iNext__700, errOut__695, 'four hex digits');
          return__306 = -1.0;
          goto break_10;
        end
        t_149 = hex__705;
        t_150 = t_149;
      else
        expectedTokenError__353(sourceText__692, iNext__700, errOut__695, 'escape sequence');
        return__306 = -1.0;
        goto break_10;
      end
      t_151 = t_150;
    end
    decodedCp__703 = t_151;
    if (leadSurrogate__697 >= 0) then
      local lead__707;
      needToFlush__702 = true;
      lead__707 = leadSurrogate__697;
      if (56320 <= decodedCp__703) then
        t_152 = (decodedCp__703 <= 57343);
      else
        t_152 = false;
      end
      if t_152 then
        leadSurrogate__697 = -1;
        decodedCp__703 = temper.int32_add(65536, temper.bor(temper.int32_mul(temper.int32_sub(lead__707, 55296), 1024), temper.int32_sub(decodedCp__703, 56320)));
      end
    else
      if (55296 <= decodedCp__703) then
        t_153 = (decodedCp__703 <= 56319);
      else
        t_153 = false;
      end
      if t_153 then
        needToFlush__702 = true;
      end
    end
    if needToFlush__702 then
      temper.stringbuilder_appendbetween(sb__694, sourceText__692, consumed__698, i__693);
      if (leadSurrogate__697 >= 0) then
        local local_156, local_157, local_158;
        local_156, local_157, local_158 = temper.pcall(function()
          temper.stringbuilder_appendcodepoint(sb__694, leadSurrogate__697);
        end);
        if local_156 then
        else
          temper.bubble();
        end
      end
      if (55296 <= decodedCp__703) then
        t_154 = (decodedCp__703 <= 56319);
      else
        t_154 = false;
      end
      if t_154 then
        leadSurrogate__697 = decodedCp__703;
      else
        local local_160, local_161, local_162;
        leadSurrogate__697 = -1;
        local_160, local_161, local_162 = temper.pcall(function()
          temper.stringbuilder_appendcodepoint(sb__694, decodedCp__703);
        end);
        if local_160 then
        else
          temper.bubble();
        end
      end
      consumed__698 = iNext__700;
    end
    i__693 = iNext__700;
  end
  if not temper.string_hasindex(sourceText__692, i__693) then
    t_155 = true;
  else
    t_143 = temper.string_get(sourceText__692, i__693);
    t_155 = (t_143 ~= 34);
  end
  if t_155 then
    expectedTokenError__353(sourceText__692, i__693, errOut__695, '"');
    return__306 = -1.0;
  else
    if (leadSurrogate__697 >= 0) then
      local local_164, local_165, local_166;
      local_164, local_165, local_166 = temper.pcall(function()
        temper.stringbuilder_appendcodepoint(sb__694, leadSurrogate__697);
      end);
      if local_164 then
      else
        temper.bubble();
      end
    else
      temper.stringbuilder_appendbetween(sb__694, sourceText__692, consumed__698, i__693);
    end
    t_144 = temper.string_next(sourceText__692, i__693);
    i__693 = t_144;
    return__306 = i__693;
  end
  ::break_10::return return__306;
end;
parseJsonObject__357 = function(sourceText__674, i__675, out__676)
  local return__303, t_168, t_169, t_170, t_171, t_172, t_173, t_174, t_175, t_176, t_177, t_178, t_179, t_180, t_181, t_182, t_183, t_184, t_185, t_186, t_187, t_188, local_189, local_190, local_191;
  ::continue_13::local_189, local_190, local_191 = temper.pcall(function()
    if not temper.string_hasindex(sourceText__674, i__675) then
      t_181 = true;
    else
      t_168 = temper.string_get(sourceText__674, i__675);
      t_181 = (t_168 ~= 123);
    end
    if t_181 then
      expectedTokenError__353(sourceText__674, i__675, out__676, "'{'");
      return__303 = -1.0;
      return 'break_12', 'flow';
    end
    out__676:startObject();
    t_169 = temper.string_next(sourceText__674, i__675);
    t_170 = skipJsonSpaces__355(sourceText__674, t_169);
    i__675 = t_170;
    if temper.string_hasindex(sourceText__674, i__675) then
      t_171 = temper.string_get(sourceText__674, i__675);
      t_182 = (t_171 ~= 125);
    else
      t_182 = false;
    end
    if t_182 then
      while true do
        local keyBuffer__678, afterKey__679, local_192, local_193, local_194;
        keyBuffer__678 = temper.stringbuilder_constructor();
        afterKey__679 = parseJsonStringTo__360(sourceText__674, i__675, keyBuffer__678, out__676);
        if not temper.is_string_index(afterKey__679) then
          return__303 = -1.0;
          return 'break_12', 'flow';
        end
        t_172 = temper.stringbuilder_tostring(keyBuffer__678);
        out__676:objectKey(t_172);
        local_192, local_193, local_194 = temper.pcall(function()
          t_183 = temper.require_string_index(afterKey__679);
          t_184 = t_183;
        end);
        if local_192 then
        else
          t_184 = temper.bubble();
        end
        t_173 = skipJsonSpaces__355(sourceText__674, t_184);
        i__675 = t_173;
        if temper.string_hasindex(sourceText__674, i__675) then
          t_174 = temper.string_get(sourceText__674, i__675);
          t_185 = (t_174 == 58);
        else
          t_185 = false;
        end
        if t_185 then
          local afterPropertyValue__680;
          t_175 = temper.string_next(sourceText__674, i__675);
          i__675 = t_175;
          afterPropertyValue__680 = parseJsonValue__356(sourceText__674, i__675, out__676);
          if not temper.is_string_index(afterPropertyValue__680) then
            return__303 = -1.0;
            return 'break_12', 'flow';
          end
          t_186 = temper.require_string_index(afterPropertyValue__680);
          i__675 = t_186;
        else
          expectedTokenError__353(sourceText__674, i__675, out__676, "':'");
          return__303 = -1.0;
          return 'break_12', 'flow';
        end
        t_176 = skipJsonSpaces__355(sourceText__674, i__675);
        i__675 = t_176;
        if temper.string_hasindex(sourceText__674, i__675) then
          t_177 = temper.string_get(sourceText__674, i__675);
          t_187 = (t_177 == 44);
        else
          t_187 = false;
        end
        if t_187 then
          t_178 = temper.string_next(sourceText__674, i__675);
          t_179 = skipJsonSpaces__355(sourceText__674, t_178);
          i__675 = t_179;
        else
          break;
        end
      end
    end
    if temper.string_hasindex(sourceText__674, i__675) then
      t_180 = temper.string_get(sourceText__674, i__675);
      t_188 = (t_180 == 125);
    else
      t_188 = false;
    end
    if t_188 then
      out__676:endObject();
      return__303 = temper.string_next(sourceText__674, i__675);
    else
      expectedTokenError__353(sourceText__674, i__675, out__676, "'}'");
      return__303 = -1.0;
    end
  end);
  if local_189 then
    if (local_191 == 'flow') then
      if (local_190 == nil) then
      elseif (local_190 == 'break_12') then
        goto break_12;
      end
    end
  else
    return__303 = temper.bubble();
  end
  ::break_12::return return__303;
end;
parseJsonArray__358 = function(sourceText__681, i__682, out__683)
  local return__304, t_197, t_198, t_199, t_200, t_201, t_202, t_203, t_204, t_205, t_206, t_207, t_208, t_209, t_210, local_211, local_212, local_213;
  ::continue_15::local_211, local_212, local_213 = temper.pcall(function()
    if not temper.string_hasindex(sourceText__681, i__682) then
      t_206 = true;
    else
      t_197 = temper.string_get(sourceText__681, i__682);
      t_206 = (t_197 ~= 91);
    end
    if t_206 then
      expectedTokenError__353(sourceText__681, i__682, out__683, "'['");
      return__304 = -1.0;
      return 'break_14', 'flow';
    end
    out__683:startArray();
    t_198 = temper.string_next(sourceText__681, i__682);
    t_199 = skipJsonSpaces__355(sourceText__681, t_198);
    i__682 = t_199;
    if temper.string_hasindex(sourceText__681, i__682) then
      t_200 = temper.string_get(sourceText__681, i__682);
      t_207 = (t_200 ~= 93);
    else
      t_207 = false;
    end
    if t_207 then
      while true do
        local afterElementValue__685;
        afterElementValue__685 = parseJsonValue__356(sourceText__681, i__682, out__683);
        if not temper.is_string_index(afterElementValue__685) then
          return__304 = -1.0;
          return 'break_14', 'flow';
        end
        t_208 = temper.require_string_index(afterElementValue__685);
        i__682 = t_208;
        t_201 = skipJsonSpaces__355(sourceText__681, i__682);
        i__682 = t_201;
        if temper.string_hasindex(sourceText__681, i__682) then
          t_202 = temper.string_get(sourceText__681, i__682);
          t_209 = (t_202 == 44);
        else
          t_209 = false;
        end
        if t_209 then
          t_203 = temper.string_next(sourceText__681, i__682);
          t_204 = skipJsonSpaces__355(sourceText__681, t_203);
          i__682 = t_204;
        else
          break;
        end
      end
    end
    if temper.string_hasindex(sourceText__681, i__682) then
      t_205 = temper.string_get(sourceText__681, i__682);
      t_210 = (t_205 == 93);
    else
      t_210 = false;
    end
    if t_210 then
      out__683:endArray();
      return__304 = temper.string_next(sourceText__681, i__682);
    else
      expectedTokenError__353(sourceText__681, i__682, out__683, "']'");
      return__304 = -1.0;
    end
  end);
  if local_211 then
    if (local_213 == 'flow') then
      if (local_212 == nil) then
      elseif (local_212 == 'break_14') then
        goto break_14;
      end
    end
  else
    return__304 = temper.bubble();
  end
  ::break_14::return return__304;
end;
parseJsonString__359 = function(sourceText__686, i__687, out__688)
  local t_215, sb__690, after__691;
  sb__690 = temper.stringbuilder_constructor();
  after__691 = parseJsonStringTo__360(sourceText__686, i__687, sb__690, out__688);
  if temper.is_string_index(after__691) then
    t_215 = temper.stringbuilder_tostring(sb__690);
    out__688:stringValue(t_215);
  end
  return after__691;
end;
afterSubstring__364 = function(string__730, inString__731, substring__732)
  local return__310, t_216, t_217, i__734, j__735;
  ::continue_17::i__734 = inString__731;
  j__735 = 1.0;
  while true do
    if not temper.string_hasindex(substring__732, j__735) then
      break;
    end
    if not temper.string_hasindex(string__730, i__734) then
      return__310 = -1.0;
      goto break_16;
    end
    if (temper.string_get(string__730, i__734) ~= temper.string_get(substring__732, j__735)) then
      return__310 = -1.0;
      goto break_16;
    end
    t_216 = temper.string_next(string__730, i__734);
    i__734 = t_216;
    t_217 = temper.string_next(substring__732, j__735);
    j__735 = t_217;
  end
  return__310 = i__734;
  ::break_16::return return__310;
end;
parseJsonBoolean__362 = function(sourceText__716, i__717, out__718)
  local return__308, t_218, ch0__720, end__721, keyword__722, n__723;
  ::continue_19::
  if temper.string_hasindex(sourceText__716, i__717) then
    t_218 = temper.string_get(sourceText__716, i__717);
    ch0__720 = t_218;
  else
    ch0__720 = 0;
  end
  end__721 = temper.string_end(sourceText__716);
  if (ch0__720 == 102) then
    keyword__722 = 'false';
    n__723 = 5;
  elseif (ch0__720 == 116) then
    keyword__722 = 'true';
    n__723 = 4;
  else
    keyword__722 = temper.null;
    n__723 = 0;
  end
  if not temper.is_null(keyword__722) then
    local keyword_219;
    keyword_219 = keyword__722;
    if temper.string_hasatleast(sourceText__716, i__717, end__721, n__723) then
      local after__724;
      after__724 = afterSubstring__364(sourceText__716, i__717, keyword_219);
      if temper.is_string_index(after__724) then
        return__308 = temper.require_string_index(after__724);
        out__718:booleanValue((n__723 == 4));
        goto break_18;
      end
    end
  end
  expectedTokenError__353(sourceText__716, i__717, out__718, '`false` or `true`');
  return__308 = -1.0;
  ::break_18::return return__308;
end;
parseJsonNull__363 = function(sourceText__725, i__726, out__727)
  local return__309, after__729;
  ::continue_21::after__729 = afterSubstring__364(sourceText__725, i__726, 'null');
  if temper.is_string_index(after__729) then
    return__309 = temper.require_string_index(after__729);
    out__727:nullValue();
    goto break_20;
  end
  expectedTokenError__353(sourceText__725, i__726, out__727, '`null`');
  return__309 = -1.0;
  ::break_20::return return__309;
end;
parseJsonNumber__365 = function(sourceText__736, i__737, out__738)
  local return__311, t_220, t_221, t_222, t_223, t_224, t_225, t_226, t_227, t_228, t_229, t_230, t_231, t_232, t_233, t_234, t_235, t_236, t_237, t_238, t_239, t_240, t_241, t_242, t_243, t_244, t_245, t_246, t_247, t_248, t_249, t_250, t_251, t_252, t_253, t_254, t_255, t_256, t_257, t_258, isNegative__740, startOfNumber__741, digit0__742, nDigits__744, tentativeFloat64__745, tentativeInt64__746, overflowInt64__747, nDigitsAfterPoint__751, nExponentDigits__754, afterExponent__757, numericTokenString__759, doubleValue__760;
  ::continue_23::isNegative__740 = false;
  startOfNumber__741 = i__737;
  if temper.string_hasindex(sourceText__736, i__737) then
    t_220 = temper.string_get(sourceText__736, i__737);
    t_238 = (t_220 == 45);
  else
    t_238 = false;
  end
  if t_238 then
    isNegative__740 = true;
    t_221 = temper.string_next(sourceText__736, i__737);
    i__737 = t_221;
  end
  if temper.string_hasindex(sourceText__736, i__737) then
    t_222 = temper.string_get(sourceText__736, i__737);
    digit0__742 = t_222;
  else
    digit0__742 = -1;
  end
  if (digit0__742 < 48) then
    t_239 = true;
  else
    t_239 = (57 < digit0__742);
  end
  if t_239 then
    local error__743;
    if not isNegative__740 then
      t_240 = (digit0__742 ~= 46);
    else
      t_240 = false;
    end
    if t_240 then
      error__743 = 'JSON value';
    else
      error__743 = 'digit';
    end
    expectedTokenError__353(sourceText__736, i__737, out__738, error__743);
    return__311 = -1.0;
    goto break_22;
  end
  t_223 = temper.string_next(sourceText__736, i__737);
  i__737 = t_223;
  nDigits__744 = 1;
  t_224 = temper.int32_tofloat64(temper.int32_sub(digit0__742, 48));
  tentativeFloat64__745 = t_224;
  t_225 = temper.int32_toint64(temper.int32_sub(digit0__742, 48));
  tentativeInt64__746 = t_225;
  overflowInt64__747 = false;
  if (48 ~= digit0__742) then
    while true do
      local possibleDigit__748;
      if not temper.string_hasindex(sourceText__736, i__737) then
        break;
      end
      possibleDigit__748 = temper.string_get(sourceText__736, i__737);
      if (48 <= possibleDigit__748) then
        t_241 = (possibleDigit__748 <= 57);
      else
        t_241 = false;
      end
      if t_241 then
        local nextDigit__749, oldInt64__750;
        t_226 = temper.string_next(sourceText__736, i__737);
        i__737 = t_226;
        nDigits__744 = temper.int32_add(nDigits__744, 1);
        nextDigit__749 = temper.int32_sub(possibleDigit__748, 48);
        t_242 = temper.mul(tentativeFloat64__745, 10.0);
        t_227 = temper.int32_tofloat64(nextDigit__749);
        tentativeFloat64__745 = temper.add(t_242, t_227);
        oldInt64__750 = tentativeInt64__746;
        t_243 = (tentativeInt64__746 * temper.int64_constructor(10));
        t_228 = temper.int32_toint64(nextDigit__749);
        tentativeInt64__746 = (t_243 + t_228);
        if (tentativeInt64__746 < oldInt64__750) then
          if ((temper.int64_constructor(-2147483648, 0) - oldInt64__750) == temper.int64_unm(temper.int32_toint64(nextDigit__749))) then
            if isNegative__740 then
              t_244 = (oldInt64__750 > temper.int64_constructor(0));
            else
              t_244 = false;
            end
            t_245 = t_244;
          else
            t_245 = false;
          end
          if not t_245 then
            overflowInt64__747 = true;
          end
        end
      else
        break;
      end
    end
  end
  nDigitsAfterPoint__751 = 0;
  if temper.string_hasindex(sourceText__736, i__737) then
    t_229 = temper.string_get(sourceText__736, i__737);
    t_246 = (46 == t_229);
  else
    t_246 = false;
  end
  if t_246 then
    local afterPoint__752;
    t_230 = temper.string_next(sourceText__736, i__737);
    i__737 = t_230;
    afterPoint__752 = i__737;
    while true do
      local possibleDigit__753;
      if not temper.string_hasindex(sourceText__736, i__737) then
        break;
      end
      possibleDigit__753 = temper.string_get(sourceText__736, i__737);
      if (48 <= possibleDigit__753) then
        t_247 = (possibleDigit__753 <= 57);
      else
        t_247 = false;
      end
      if t_247 then
        t_231 = temper.string_next(sourceText__736, i__737);
        i__737 = t_231;
        nDigits__744 = temper.int32_add(nDigits__744, 1);
        nDigitsAfterPoint__751 = temper.int32_add(nDigitsAfterPoint__751, 1);
        t_248 = temper.mul(tentativeFloat64__745, 10.0);
        t_232 = temper.int32_tofloat64(temper.int32_sub(possibleDigit__753, 48));
        tentativeFloat64__745 = temper.add(t_248, t_232);
      else
        break;
      end
    end
    if temper.stringindexoption_compareto_eq(i__737, afterPoint__752) then
      expectedTokenError__353(sourceText__736, i__737, out__738, 'digit');
      return__311 = -1.0;
      goto break_22;
    end
  end
  nExponentDigits__754 = 0;
  if temper.string_hasindex(sourceText__736, i__737) then
    t_233 = temper.string_get(sourceText__736, i__737);
    t_249 = (101 == temper.bor(t_233, 32));
  else
    t_249 = false;
  end
  if t_249 then
    local afterE__755;
    t_234 = temper.string_next(sourceText__736, i__737);
    i__737 = t_234;
    if not temper.string_hasindex(sourceText__736, i__737) then
      expectedTokenError__353(sourceText__736, i__737, out__738, 'sign or digit');
      return__311 = -1.0;
      goto break_22;
    end
    afterE__755 = temper.string_get(sourceText__736, i__737);
    if (afterE__755 == 43) then
      t_250 = true;
    else
      t_250 = (afterE__755 == 45);
    end
    if t_250 then
      t_235 = temper.string_next(sourceText__736, i__737);
      i__737 = t_235;
    end
    while true do
      local possibleDigit__756;
      if not temper.string_hasindex(sourceText__736, i__737) then
        break;
      end
      possibleDigit__756 = temper.string_get(sourceText__736, i__737);
      if (48 <= possibleDigit__756) then
        t_251 = (possibleDigit__756 <= 57);
      else
        t_251 = false;
      end
      if t_251 then
        t_236 = temper.string_next(sourceText__736, i__737);
        i__737 = t_236;
        nExponentDigits__754 = temper.int32_add(nExponentDigits__754, 1);
      else
        break;
      end
    end
    if (nExponentDigits__754 == 0) then
      expectedTokenError__353(sourceText__736, i__737, out__738, 'exponent digit');
      return__311 = -1.0;
      goto break_22;
    end
  end
  afterExponent__757 = i__737;
  if (nExponentDigits__754 == 0) then
    if (nDigitsAfterPoint__751 == 0) then
      t_252 = not overflowInt64__747;
    else
      t_252 = false;
    end
    t_253 = t_252;
  else
    t_253 = false;
  end
  if t_253 then
    local value__758;
    if isNegative__740 then
      value__758 = temper.int64_unm(tentativeInt64__746);
    else
      value__758 = tentativeInt64__746;
    end
    if (temper.int64_constructor(-2147483648) <= value__758) then
      t_254 = (value__758 <= temper.int64_constructor(2147483647));
    else
      t_254 = false;
    end
    if t_254 then
      t_237 = temper.int64_toint32unsafe(value__758);
      out__738:int32Value(t_237);
    else
      out__738:int64Value(value__758);
    end
    return__311 = i__737;
    goto break_22;
  end
  numericTokenString__759 = temper.string_slice(sourceText__736, startOfNumber__741, i__737);
  doubleValue__760 = temper.nan;
  if (nExponentDigits__754 ~= 0) then
    t_255 = true;
  else
    t_255 = (nDigitsAfterPoint__751 ~= 0);
  end
  if t_255 then
    local local_259, local_260, local_261;
    local_259, local_260, local_261 = temper.pcall(function()
      t_256 = temper.string_tofloat64(numericTokenString__759);
      doubleValue__760 = t_256;
    end);
    if local_259 then
    end
  end
  if temper.float_ne(doubleValue__760, temper.neg_inf) then
    if temper.float_ne(doubleValue__760, temper.pos_inf) then
      t_257 = temper.float_ne(doubleValue__760, temper.nan);
    else
      t_257 = false;
    end
    t_258 = t_257;
  else
    t_258 = false;
  end
  if t_258 then
    out__738:float64Value(doubleValue__760);
  else
    out__738:numericTokenValue(numericTokenString__759);
  end
  return__311 = i__737;
  ::break_22::return return__311;
end;
parseJsonToProducer = function(sourceText__662, out__663)
  local t_263, t_264, t_265, t_266, t_267, t_268, i__665, afterValue__666;
  i__665 = 1.0;
  afterValue__666 = parseJsonValue__356(sourceText__662, i__665, out__663);
  if temper.is_string_index(afterValue__666) then
    t_268 = temper.require_string_index(afterValue__666);
    t_263 = skipJsonSpaces__355(sourceText__662, t_268);
    i__665 = t_263;
    if temper.string_hasindex(sourceText__662, i__665) then
      t_264 = out__663.parseErrorReceiver;
      t_267 = not temper.is_null(t_264);
    else
      t_267 = false;
    end
    if t_267 then
      t_265 = temper.string_end(sourceText__662);
      t_266 = temper.string_slice(sourceText__662, i__665, t_265);
      storeJsonError__354(out__663, temper.concat('Extraneous JSON `', t_266, '`'));
    end
  end
  return nil;
end;
parseJson = function(sourceText__761)
  local p__763;
  p__763 = JsonSyntaxTreeProducer();
  parseJsonToProducer(sourceText__761, p__763);
  return p__763:toJsonSyntaxTree();
end;
booleanJsonAdapter = function()
  return BooleanJsonAdapter__164();
end;
float64JsonAdapter = function()
  return Float64JsonAdapter__167();
end;
int32JsonAdapter = function()
  return Int32JsonAdapter__170();
end;
int64JsonAdapter = function()
  return Int64JsonAdapter__173();
end;
stringJsonAdapter = function()
  return StringJsonAdapter__176();
end;
listJsonAdapter = function(adapterForT__839)
  return ListJsonAdapter__179(adapterForT__839);
end;
exports = {};
exports.InterchangeContext = InterchangeContext;
exports.NullInterchangeContext = NullInterchangeContext;
exports.JsonProducer = JsonProducer;
exports.JsonSyntaxTree = JsonSyntaxTree;
exports.JsonObject = JsonObject;
exports.JsonArray = JsonArray;
exports.JsonBoolean = JsonBoolean;
exports.JsonNull = JsonNull;
exports.JsonString = JsonString;
exports.JsonNumeric = JsonNumeric;
exports.JsonInt32 = JsonInt32;
exports.JsonInt64 = JsonInt64;
exports.JsonFloat64 = JsonFloat64;
exports.JsonNumericToken = JsonNumericToken;
exports.JsonTextProducer = JsonTextProducer;
exports.JsonParseErrorReceiver = JsonParseErrorReceiver;
exports.JsonSyntaxTreeProducer = JsonSyntaxTreeProducer;
exports.JsonAdapter = JsonAdapter;
exports.OrNullJsonAdapter = OrNullJsonAdapter;
exports.parseJsonToProducer = parseJsonToProducer;
exports.parseJson = parseJson;
exports.booleanJsonAdapter = booleanJsonAdapter;
exports.float64JsonAdapter = float64JsonAdapter;
exports.int32JsonAdapter = int32JsonAdapter;
exports.int64JsonAdapter = int64JsonAdapter;
exports.stringJsonAdapter = stringJsonAdapter;
exports.listJsonAdapter = listJsonAdapter;
return exports;
