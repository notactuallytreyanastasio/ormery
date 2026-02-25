local temper = require('temper-core');
local SqlBuilder, SafeHtmlBuilder, console_734, DemoController, Field, Schema, InMemoryStore, Record, Query, WhereClause, OrderClause, compareInt__195, compareString__196, safeSql__198, columnListSql__199, validOperator__197, whereConditionSql__200, orderBySql__201, toSqlQuery, TokenType, Token, field, isValidIdentifier, schema, toInsertSql, main, temperKeywords__204, temperTypes__205, classifyWord, highlightWord, highlightLine, highlightSource, highlightBlock, exports;
SqlBuilder = temper.import('ormery/sql', 'SqlBuilder');
SafeHtmlBuilder = temper.import('ormery/html', 'SafeHtmlBuilder');
console_734 = 0.0;
DemoController = temper.type('DemoController');
DemoController.constructor = function(this__47, schema__211, store__212)
  this__47.schema__207 = schema__211;
  this__47.store__208 = store__212;
  this__47.queryCount__209 = 0;
  return nil;
end;
DemoController.methods.getRecordCount = function(this__48)
  local t_735;
  t_735 = this__48.schema__207.tableName;
  return this__48.store__208:count(t_735);
end;
DemoController.methods.getAdultCount = function(this__49)
  return temper.list_length(Query(this__49.schema__207, this__49.store__208):where('age', '>=', '18'):all());
end;
DemoController.methods.getQueryCount = function(this__50)
  return this__50.queryCount__209;
end;
DemoController.methods.incrementQueryCount = function(this__51)
  local t_736;
  t_736 = temper.int32_add(this__51.queryCount__209, 1);
  this__51.queryCount__209 = t_736;
  return nil;
end;
DemoController.methods.formatRecords = function(this__52, records__223)
  local fn__3824, lines__225, fn__3823;
  fn__3824 = function(record__226)
    return temper.concat('  ', record__226:describe());
  end;
  lines__225 = temper.listed_map(records__223, fn__3824);
  fn__3823 = function(s__228)
    return s__228;
  end;
  return temper.listed_join(lines__225, '\n', fn__3823);
end;
DemoController.methods.runDemo1 = function(this__53)
  local results__232, formatted__233;
  this__53:incrementQueryCount();
  results__232 = Query(this__53.schema__207, this__53.store__208):all();
  formatted__233 = this__53:formatRecords(results__232);
  return temper.concat('=== Demo 1: All Users ===', '\n', '\n', 'Query: new Query(userSchema, store).all()', '\n', '\n', 'Results:', '\n', formatted__233, '\n', '\n', 'Total: ', temper.int32_tostring(temper.list_length(results__232)), ' records');
end;
DemoController.methods.runDemo2 = function(this__54)
  local results__236, formatted__237;
  this__54:incrementQueryCount();
  results__236 = Query(this__54.schema__207, this__54.store__208):where('age', '>=', '18'):all();
  formatted__237 = this__54:formatRecords(results__236);
  return temper.concat('=== Demo 2: Filter Adults ===', '\n', '\n', 'Query: new Query(userSchema, store)', '\n', '  .where(', '"', 'age', '"', ', ', '"', '>=', '"', ', ', '"', '18', '"', ')', '\n', '  .all()', '\n', '\n', 'Results:', '\n', formatted__237, '\n', '\n', 'Total: ', temper.int32_tostring(temper.list_length(results__236)), ' adults found');
end;
DemoController.methods.runDemo3 = function(this__55)
  local results__240, formatted__241;
  this__55:incrementQueryCount();
  results__240 = Query(this__55.schema__207, this__55.store__208):orderBy('age', 'desc'):all();
  formatted__241 = this__55:formatRecords(results__240);
  return temper.concat('=== Demo 3: Sort by Age (Descending) ===', '\n', '\n', 'Query: new Query(userSchema, store)', '\n', '  .orderBy(', '"', 'age', '"', ', ', '"', 'desc', '"', ')', '\n', '  .all()', '\n', '\n', 'Results (ordered by age, oldest first):', '\n', formatted__241);
end;
DemoController.methods.runDemo4 = function(this__56)
  local page1__244, page2__245, formatted1__246, formatted2__247;
  this__56:incrementQueryCount();
  page1__244 = Query(this__56.schema__207, this__56.store__208):orderBy('id', 'asc'):limit(2):all();
  page2__245 = Query(this__56.schema__207, this__56.store__208):orderBy('id', 'asc'):offset(2):limit(2):all();
  formatted1__246 = this__56:formatRecords(page1__244);
  formatted2__247 = this__56:formatRecords(page2__245);
  return temper.concat('=== Demo 4: Pagination ===', '\n', '\n', 'Page 1: .orderBy(', '"', 'id', '"', ', ', '"', 'asc', '"', ').limit(2)', '\n', '\n', formatted1__246, '\n', '\n', 'Page 2: .orderBy(', '"', 'id', '"', ', ', '"', 'asc', '"', ').offset(2).limit(2)', '\n', '\n', formatted2__247);
end;
DemoController.methods.runDemo5 = function(this__57)
  local results__250, formatted__251;
  this__57:incrementQueryCount();
  results__250 = Query(this__57.schema__207, this__57.store__208):where('age', '>=', '18'):where('email', '!=', ''):orderBy('age', 'desc'):select(temper.listof('name', 'age')):limit(2):all();
  formatted__251 = this__57:formatRecords(results__250);
  return temper.concat('=== Demo 5: Complex Query ===', '\n', '\n', 'Query: new Query(userSchema, store)', '\n', '  .where(', '"', 'age', '"', ', ', '"', '>=', '"', ', ', '"', '18', '"', ')', '\n', '  .where(', '"', 'email', '"', ', ', '"', '!=', '"', ', ', '"', '"', ')', '\n', '  .orderBy(', '"', 'age', '"', ', ', '"', 'desc', '"', ')', '\n', '  .select([', '"', 'name', '"', ', ', '"', 'age', '"', '])', '\n', '  .limit(2)', '\n', '  .all()', '\n', '\n', 'Results (adults with email, showing name/age only, oldest first, max 2):', '\n', formatted__251);
end;
DemoController.get.schema = function(this__819)
  return this__819.schema__207;
end;
DemoController.get.store = function(this__822)
  return this__822.store__208;
end;
Field = temper.type('Field');
Field.get.description = function(this__58)
  local pk__258, null__259;
  if this__58.primaryKey__254 then
    pk__258 = ' (PK)';
  else
    pk__258 = '';
  end
  if this__58.nullable__255 then
    null__259 = ' (nullable)';
  else
    null__259 = '';
  end
  return temper.concat(this__58.name__252, ': ', this__58.fieldType__253, pk__258, null__259);
end;
Field.constructor = function(this__109, name__261, fieldType__262, primaryKey__263, nullable__264)
  this__109.name__252 = name__261;
  this__109.fieldType__253 = fieldType__262;
  this__109.primaryKey__254 = primaryKey__263;
  this__109.nullable__255 = nullable__264;
  return nil;
end;
Field.get.name = function(this__739)
  return this__739.name__252;
end;
Field.get.fieldType = function(this__742)
  return this__742.fieldType__253;
end;
Field.get.primaryKey = function(this__745)
  return this__745.primaryKey__254;
end;
Field.get.nullable = function(this__748)
  return this__748.nullable__255;
end;
Schema = temper.type('Schema');
Schema.methods.getField = function(this__59, name__268)
  local return__117, this__2501, n__2502, i__2503;
  ::continue_11::this__2501 = this__59.fields__266;
  n__2502 = temper.list_length(this__2501);
  i__2503 = 0;
  while (i__2503 < n__2502) do
    local el__2504, field__270;
    el__2504 = temper.list_get(this__2501, i__2503);
    i__2503 = temper.int32_add(i__2503, 1);
    field__270 = el__2504;
    if temper.str_eq(field__270.name, name__268) then
      return__117 = field__270;
      goto break_10;
    end
  end
  temper.bubble();
  ::break_10::return return__117;
end;
Schema.methods.hasField = function(this__60, name__272)
  local return__118, this__2506, n__2507, i__2508;
  ::continue_13::this__2506 = this__60.fields__266;
  n__2507 = temper.list_length(this__2506);
  i__2508 = 0;
  while (i__2508 < n__2507) do
    local el__2509, field__274;
    el__2509 = temper.list_get(this__2506, i__2508);
    i__2508 = temper.int32_add(i__2508, 1);
    field__274 = el__2509;
    if temper.str_eq(field__274.name, name__272) then
      return__118 = true;
      goto break_12;
    end
  end
  return__118 = false;
  ::break_12::return return__118;
end;
Schema.get.primaryKeyField = function(this__61)
  local return__119, this__2511, n__2512, i__2513;
  ::continue_15::this__2511 = this__61.fields__266;
  n__2512 = temper.list_length(this__2511);
  i__2513 = 0;
  while (i__2513 < n__2512) do
    local el__2514, field__277;
    el__2514 = temper.list_get(this__2511, i__2513);
    i__2513 = temper.int32_add(i__2513, 1);
    field__277 = el__2514;
    if field__277.primaryKey then
      return__119 = field__277;
      goto break_14;
    end
  end
  temper.bubble();
  ::break_14::return return__119;
end;
Schema.get.fieldNames = function(this__62)
  local fn__4076;
  fn__4076 = function(f__280)
    return f__280.name;
  end;
  return temper.listed_map(this__62.fields__266, fn__4076);
end;
Schema.methods.describe = function(this__63)
  local header__284, fn__4070, t_737, fn__4069, fieldList__285;
  header__284 = temper.concat('Schema: ', this__63.tableName__265, '\n');
  fn__4070 = function(f__286)
    return temper.concat('  - ', f__286.description);
  end;
  t_737 = temper.listed_map(this__63.fields__266, fn__4070);
  fn__4069 = function(s__288)
    return s__288;
  end;
  fieldList__285 = temper.listed_join(t_737, '\n', fn__4069);
  return temper.concat(header__284, fieldList__285);
end;
Schema.constructor = function(this__114, tableName__291, fields__292)
  this__114.tableName__265 = tableName__291;
  this__114.fields__266 = fields__292;
  return nil;
end;
Schema.get.tableName = function(this__751)
  return this__751.tableName__265;
end;
Schema.get.fields = function(this__754)
  return this__754.fields__266;
end;
InMemoryStore = temper.type('InMemoryStore');
InMemoryStore.constructor = function(this__69)
  local t_738, t_739;
  t_738 = temper.mapbuilder_constructor();
  this__69.tables__330 = t_738;
  t_739 = temper.mapbuilder_constructor();
  this__69.nextIds__331 = t_739;
  return nil;
end;
InMemoryStore.methods.ensureTable = function(this__70, tableName__335)
  local t_740;
  if not temper.mapped_has(this__70.tables__330, tableName__335) then
    t_740 = temper.listbuilder_constructor();
    temper.mapbuilder_set(this__70.tables__330, tableName__335, t_740);
    temper.mapbuilder_set(this__70.nextIds__331, tableName__335, 1);
  end
  return nil;
end;
InMemoryStore.methods.insert = function(this__71, tableName__338, data__339)
  local id__341, dataBuilder__342, record__343, t_741, table__344;
  this__71:ensureTable(tableName__338);
  id__341 = temper.mapped_getor(this__71.nextIds__331, tableName__338, 1);
  temper.mapbuilder_set(this__71.nextIds__331, tableName__338, temper.int32_add(id__341, 1));
  dataBuilder__342 = temper.mapped_tomapbuilder(data__339);
  temper.mapbuilder_set(dataBuilder__342, 'id', temper.int32_tostring(id__341));
  record__343 = Record(temper.mapped_tomap(dataBuilder__342));
  t_741 = temper.listbuilder_constructor();
  table__344 = temper.mapped_getor(this__71.tables__330, tableName__338, t_741);
  temper.listbuilder_add(table__344, record__343);
  return record__343;
end;
InMemoryStore.methods.all = function(this__72, tableName__346)
  local t_742, table__348;
  this__72:ensureTable(tableName__346);
  t_742 = temper.listbuilder_constructor();
  table__348 = temper.mapped_getor(this__72.tables__330, tableName__346, t_742);
  return temper.listbuilder_tolist(table__348);
end;
InMemoryStore.methods.get = function(this__73, tableName__350, id__351)
  local return__138, t_743, table__353, this__2516, n__2517, i__2518;
  ::continue_17::this__73:ensureTable(tableName__350);
  t_743 = temper.listbuilder_constructor();
  table__353 = temper.mapped_getor(this__73.tables__330, tableName__350, t_743);
  this__2516 = temper.listbuilder_tolist(table__353);
  n__2517 = temper.list_length(this__2516);
  i__2518 = 0;
  while (i__2518 < n__2517) do
    local el__2519, record__354, recordId__355, local_744, local_745, local_746;
    el__2519 = temper.list_get(this__2516, i__2518);
    i__2518 = temper.int32_add(i__2518, 1);
    record__354 = el__2519;
    local_744, local_745, local_746 = temper.pcall(function()
      recordId__355 = record__354.id;
    end);
    if local_744 then
    else
      temper.bubble();
    end
    if (recordId__355 == id__351) then
      return__138 = record__354;
      goto break_16;
    end
  end
  temper.bubble();
  ::break_16::return return__138;
end;
InMemoryStore.methods.count = function(this__74, tableName__357)
  local t_748, table__359;
  this__74:ensureTable(tableName__357);
  t_748 = temper.listbuilder_constructor();
  table__359 = temper.mapped_getor(this__74.tables__330, tableName__357, t_748);
  return temper.listbuilder_length(table__359);
end;
Record = temper.type('Record');
Record.methods.get = function(this__64, field__308)
  return temper.mapped_get(this__64.data__306, field__308);
end;
Record.methods.getOr = function(this__65, field__311, fallback__312)
  return temper.mapped_getor(this__65.data__306, field__311, fallback__312);
end;
Record.methods.has = function(this__66, field__315)
  return temper.mapped_has(this__66.data__306, field__315);
end;
Record.get.id = function(this__67)
  local return__132, idStr__319, local_749, local_750, local_751;
  idStr__319 = temper.mapped_get(this__67.data__306, 'id');
  local_749, local_750, local_751 = temper.pcall(function()
    return__132 = temper.string_toint32(idStr__319);
  end);
  if local_749 then
  else
    temper.bubble();
  end
  return return__132;
end;
Record.methods.describe = function(this__68)
  local fn__4061, pairs__322, fn__4060;
  fn__4061 = function(k__323, v__324)
    return temper.concat(k__323, ': ', v__324);
  end;
  pairs__322 = temper.mapped_tolistwith(this__68.data__306, fn__4061);
  fn__4060 = function(s__326)
    return s__326;
  end;
  return temper.listed_join(pairs__322, ', ', fn__4060);
end;
Record.constructor = function(this__126, data__329)
  this__126.data__306 = data__329;
  return nil;
end;
Record.get.data = function(this__757)
  return this__757.data__306;
end;
Query = temper.type('Query');
Query.constructor = function(this__77, schema__384, store__385)
  local t_753, t_754, t_755;
  this__77.schema__376 = schema__384;
  this__77.store__377 = store__385;
  t_753 = temper.listbuilder_constructor();
  this__77.whereClauses__378 = t_753;
  t_754 = temper.listof();
  this__77.selectFields__379 = t_754;
  t_755 = temper.listbuilder_constructor();
  this__77.orderByClauses__380 = t_755;
  this__77.limitValue__381 = -1;
  this__77.offsetValue__382 = 0;
  return nil;
end;
Query.methods.where = function(this__78, field__388, operator__389, value__390)
  local t_756;
  t_756 = WhereClause(field__388, operator__389, value__390);
  temper.listbuilder_add(this__78.whereClauses__378, t_756);
  return this__78;
end;
Query.methods.select = function(this__79, fields__393)
  this__79.selectFields__379 = fields__393;
  return this__79;
end;
Query.methods.orderBy = function(this__80, field__396, direction__397)
  local t_757;
  t_757 = OrderClause(field__396, direction__397);
  temper.listbuilder_add(this__80.orderByClauses__380, t_757);
  return this__80;
end;
Query.methods.limit = function(this__81, n__400)
  local t_758;
  if (n__400 < 0) then
    t_758 = 0;
  else
    t_758 = n__400;
  end
  this__81.limitValue__381 = t_758;
  return this__81;
end;
Query.methods.offset = function(this__82, n__403)
  this__82.offsetValue__382 = n__403;
  return this__82;
end;
Query.methods.matchesWhere = function(this__83, record__406)
  local return__152, t_759, t_760, t_761, t_762, t_763, t_764, t_765, t_766, this__2521, n__2522, i__2523;
  ::continue_19::this__2521 = temper.listbuilder_tolist(this__83.whereClauses__378);
  n__2522 = temper.list_length(this__2521);
  i__2523 = 0;
  while (i__2523 < n__2522) do
    local el__2524, clause__408, t_767, recordValue__409, fieldInfo__410, local_768, local_769, local_770, fieldType__411, matches__412;
    el__2524 = temper.list_get(this__2521, i__2523);
    i__2523 = temper.int32_add(i__2523, 1);
    clause__408 = el__2524;
    t_759 = clause__408.field;
    recordValue__409 = record__406:getOr(t_759, '');
    t_760 = clause__408.field;
    if not this__83.schema__376:hasField(t_760) then
      return__152 = false;
      goto break_18;
    end
    local_768, local_769, local_770 = temper.pcall(function()
      t_761 = clause__408.field;
      t_767 = this__83.schema__376:getField(t_761);
      fieldInfo__410 = t_767;
    end);
    if local_768 then
    else
      fieldInfo__410 = temper.bubble();
    end
    fieldType__411 = fieldInfo__410.fieldType;
    if temper.str_eq(fieldType__411, 'Int') then
      t_762 = clause__408.operator;
      t_763 = clause__408.value;
      t_764 = compareInt__195(recordValue__409, t_762, t_763);
      matches__412 = t_764;
    elseif temper.str_eq(fieldType__411, 'String') then
      t_765 = clause__408.operator;
      t_766 = clause__408.value;
      matches__412 = compareString__196(recordValue__409, t_765, t_766);
    else
      matches__412 = false;
    end
    if not matches__412 then
      return__152 = false;
      goto break_18;
    end
  end
  return__152 = true;
  ::break_18::return return__152;
end;
Query.methods.projectRecord = function(this__84, record__414)
  local return__153, t_772, builder__416, fn__3873;
  ::continue_21::
  if (temper.list_length(this__84.selectFields__379) == 0) then
    return__153 = record__414;
    goto break_20;
  end
  builder__416 = temper.mapbuilder_constructor();
  fn__3873 = function(fieldName__417)
    local value__418;
    value__418 = record__414:getOr(fieldName__417, '');
    temper.mapbuilder_set(builder__416, fieldName__417, value__418);
    return nil;
  end;
  temper.list_foreach(this__84.selectFields__379, fn__3873);
  t_772 = temper.mapped_tomap(builder__416);
  return__153 = Record(t_772);
  ::break_20::return return__153;
end;
Query.methods.compareRecords = function(this__85, a__420, b__421, orderClauses__422)
  local return__154, t_773, t_774, t_775, t_776, this__2526, n__2527, i__2528;
  ::continue_23::this__2526 = orderClauses__422;
  n__2527 = temper.list_length(this__2526);
  i__2528 = 0;
  ::local_780::while (i__2528 < n__2527) do
    local el__2529, clause__424, t_777, t_778, t_779, aVal__425, bVal__426, fieldInfo__427, local_781, local_782, local_783, fieldType__428, cmp__429;
    el__2529 = temper.list_get(this__2526, i__2528);
    i__2528 = temper.int32_add(i__2528, 1);
    clause__424 = el__2529;
    t_773 = clause__424.field;
    aVal__425 = a__420:getOr(t_773, '');
    t_774 = clause__424.field;
    bVal__426 = b__421:getOr(t_774, '');
    t_775 = clause__424.field;
    if not this__85.schema__376:hasField(t_775) then
      goto local_780;
    end
    local_781, local_782, local_783 = temper.pcall(function()
      t_776 = clause__424.field;
      t_777 = this__85.schema__376:getField(t_776);
      fieldInfo__427 = t_777;
    end);
    if local_781 then
    else
      fieldInfo__427 = temper.bubble();
    end
    fieldType__428 = fieldInfo__427.fieldType;
    if temper.str_eq(fieldType__428, 'Int') then
      local aInt__430, local_785, local_786, local_787, bInt__431, local_789, local_790, local_791;
      local_785, local_786, local_787 = temper.pcall(function()
        t_778 = temper.string_toint32(aVal__425);
        aInt__430 = t_778;
      end);
      if local_785 then
      else
        aInt__430 = 0;
      end
      local_789, local_790, local_791 = temper.pcall(function()
        t_779 = temper.string_toint32(bVal__426);
        bInt__431 = t_779;
      end);
      if local_789 then
      else
        bInt__431 = 0;
      end
      cmp__429 = temper.generic_cmp(aInt__430, bInt__431);
    elseif temper.str_eq(fieldType__428, 'String') then
      cmp__429 = temper.generic_cmp(aVal__425, bVal__426);
    else
      cmp__429 = 0;
    end
    if (cmp__429 ~= 0) then
      if temper.str_eq(clause__424.direction, 'desc') then
        return__154 = temper.int32_unm(cmp__429);
      else
        return__154 = cmp__429;
      end
      goto break_22;
    end
  end
  return__154 = 0;
  ::break_22::return return__154;
end;
Query.methods.all = function(this__86)
  local t_793, t_794, t_795, t_796, t_797, t_798, allRecords__434, fn__3845, filtered__435, sorted__438, sliced__443, fn__3843;
  t_798 = this__86.schema__376.tableName;
  allRecords__434 = this__86.store__377:all(t_798);
  fn__3845 = function(r__436)
    return this__86:matchesWhere(r__436);
  end;
  filtered__435 = temper.listed_filter(allRecords__434, fn__3845);
  if (temper.listbuilder_length(this__86.orderByClauses__380) > 0) then
    local clauses__439, fn__3844;
    clauses__439 = temper.listbuilder_tolist(this__86.orderByClauses__380);
    fn__3844 = function(a__440, b__441)
      return this__86:compareRecords(a__440, b__441, clauses__439);
    end;
    t_793 = temper.listed_sorted(filtered__435, fn__3844);
    sorted__438 = t_793;
  else
    sorted__438 = filtered__435;
  end
  if (this__86.limitValue__381 >= 0) then
    local start__444, end__445;
    start__444 = this__86.offsetValue__382;
    end__445 = temper.int32_add(this__86.offsetValue__382, this__86.limitValue__381);
    t_794 = temper.listed_slice(sorted__438, start__444, end__445);
    sliced__443 = t_794;
  elseif (this__86.offsetValue__382 > 0) then
    t_796 = this__86.offsetValue__382;
    t_795 = temper.list_length(sorted__438);
    t_797 = temper.listed_slice(sorted__438, t_796, t_795);
    sliced__443 = t_797;
  else
    sliced__443 = sorted__438;
  end
  fn__3843 = function(r__446)
    return this__86:projectRecord(r__446);
  end;
  return temper.listed_map(sliced__443, fn__3843);
end;
Query.methods.toSql = function(this__87)
  return toSqlQuery(this__87.schema__376, this__87.selectFields__379, temper.listbuilder_tolist(this__87.whereClauses__378), temper.listbuilder_tolist(this__87.orderByClauses__380), this__87.limitValue__381, this__87.offsetValue__382);
end;
Query.get.schema = function(this__809)
  return this__809.schema__376;
end;
Query.get.store = function(this__812)
  return this__812.store__377;
end;
WhereClause = temper.type('WhereClause');
WhereClause.methods.describe = function(this__75)
  return temper.concat(this__75.field__360, ' ', this__75.operator__361, ' ', this__75.value__362);
end;
WhereClause.constructor = function(this__140, field__366, operator__367, value__368)
  this__140.field__360 = field__366;
  this__140.operator__361 = operator__367;
  this__140.value__362 = value__368;
  return nil;
end;
WhereClause.get.field = function(this__760)
  return this__760.field__360;
end;
WhereClause.get.operator = function(this__763)
  return this__763.operator__361;
end;
WhereClause.get.value = function(this__766)
  return this__766.value__362;
end;
OrderClause = temper.type('OrderClause');
OrderClause.methods.describe = function(this__76)
  return temper.concat(this__76.field__369, ' ', this__76.direction__370);
end;
OrderClause.constructor = function(this__143, field__374, direction__375)
  this__143.field__369 = field__374;
  this__143.direction__370 = direction__375;
  return nil;
end;
OrderClause.get.field = function(this__769)
  return this__769.field__369;
end;
OrderClause.get.direction = function(this__772)
  return this__772.direction__370;
end;
compareInt__195 = function(recordValue__450, operator__451, clauseValue__452)
  local return__157, t_799, t_800, t_801, rv__454, local_802, local_803, local_804, cv__455, local_806, local_807, local_808;
  ::continue_25::local_802, local_803, local_804 = temper.pcall(function()
    t_800 = temper.string_toint32(recordValue__450);
    rv__454 = t_800;
  end);
  if local_802 then
  else
    rv__454 = 0;
  end
  local_806, local_807, local_808 = temper.pcall(function()
    t_801 = temper.string_toint32(clauseValue__452);
    cv__455 = t_801;
  end);
  if local_806 then
  else
    cv__455 = 0;
  end
  t_799 = temper.int32_tostring(cv__455);
  if temper.str_ne(clauseValue__452, t_799) then
    return__157 = false;
    goto break_24;
  end
  if temper.str_eq(operator__451, '==') then
    return__157 = (rv__454 == cv__455);
  elseif temper.str_eq(operator__451, '!=') then
    return__157 = (rv__454 ~= cv__455);
  elseif temper.str_eq(operator__451, '>') then
    return__157 = (rv__454 > cv__455);
  elseif temper.str_eq(operator__451, '<') then
    return__157 = (rv__454 < cv__455);
  elseif temper.str_eq(operator__451, '>=') then
    return__157 = (rv__454 >= cv__455);
  elseif temper.str_eq(operator__451, '<=') then
    return__157 = (rv__454 <= cv__455);
  else
    return__157 = false;
  end
  ::break_24::return return__157;
end;
compareString__196 = function(recordValue__456, operator__457, clauseValue__458)
  local return__158;
  if temper.str_eq(operator__457, '==') then
    return__158 = temper.str_eq(recordValue__456, clauseValue__458);
  elseif temper.str_eq(operator__457, '!=') then
    return__158 = temper.str_ne(recordValue__456, clauseValue__458);
  elseif temper.str_eq(operator__457, '>') then
    return__158 = temper.str_gt(recordValue__456, clauseValue__458);
  elseif temper.str_eq(operator__457, '<') then
    return__158 = temper.str_lt(recordValue__456, clauseValue__458);
  elseif temper.str_eq(operator__457, '>=') then
    return__158 = temper.str_ge(recordValue__456, clauseValue__458);
  elseif temper.str_eq(operator__457, '<=') then
    return__158 = temper.str_le(recordValue__456, clauseValue__458);
  else
    return__158 = false;
  end
  return return__158;
end;
safeSql__198 = function(trusted__462)
  local b__464;
  b__464 = SqlBuilder();
  b__464:appendSafe(trusted__462);
  return b__464.accumulated;
end;
columnListSql__199 = function(selectFields__465)
  local return__161, t_810, t_811, t_812, t_813, t_814, t_815, t_816, t_817;
  if (temper.list_length(selectFields__465) == 0) then
    t_810 = SqlBuilder();
    t_810:appendSafe('*');
    return__161 = t_810.accumulated;
  else
    local first__467, result__468, i__469;
    t_811 = temper.list_get(selectFields__465, 0);
    first__467 = safeSql__198(t_811);
    t_812 = SqlBuilder();
    t_812:appendFragment(first__467);
    t_813 = t_812.accumulated;
    result__468 = t_813;
    i__469 = 1;
    while true do
      local col__470;
      t_814 = temper.list_length(selectFields__465);
      if not (i__469 < t_814) then
        break;
      end
      t_815 = temper.list_get(selectFields__465, i__469);
      col__470 = safeSql__198(t_815);
      t_816 = SqlBuilder();
      t_816:appendFragment(result__468);
      t_816:appendSafe(', ');
      t_816:appendFragment(col__470);
      t_817 = t_816.accumulated;
      result__468 = t_817;
      i__469 = temper.int32_add(i__469, 1);
    end
    return__161 = result__468;
  end
  return return__161;
end;
validOperator__197 = function(op__460)
  local return__159;
  if temper.str_eq(op__460, '=') then
    return__159 = '=';
  elseif temper.str_eq(op__460, '==') then
    return__159 = '=';
  elseif temper.str_eq(op__460, '!=') then
    return__159 = '!=';
  elseif temper.str_eq(op__460, '<>') then
    return__159 = '<>';
  elseif temper.str_eq(op__460, '>') then
    return__159 = '>';
  elseif temper.str_eq(op__460, '<') then
    return__159 = '<';
  elseif temper.str_eq(op__460, '>=') then
    return__159 = '>=';
  elseif temper.str_eq(op__460, '<=') then
    return__159 = '<=';
  else
    return__159 = '=';
  end
  return return__159;
end;
whereConditionSql__200 = function(clause__471, schema__472)
  local return__162, t_818, t_819, t_820, t_821, t_822, t_823, t_824, col__474, t_825, op__475, fieldInfo__476, local_826, local_827, local_828;
  t_824 = clause__471.field;
  col__474 = safeSql__198(t_824);
  t_825 = clause__471.operator;
  op__475 = safeSql__198(validOperator__197(t_825));
  local_826, local_827, local_828 = temper.pcall(function()
    t_818 = clause__471.field;
    t_822 = schema__472:getField(t_818);
    fieldInfo__476 = t_822;
  end);
  if local_826 then
  else
    fieldInfo__476 = temper.bubble();
  end
  if temper.str_eq(fieldInfo__476.fieldType, 'Int') then
    local intVal__477, local_830, local_831, local_832;
    local_830, local_831, local_832 = temper.pcall(function()
      t_823 = temper.string_toint32(clause__471.value);
      intVal__477 = t_823;
    end);
    if local_830 then
    else
      intVal__477 = 0;
    end
    if temper.str_ne(clause__471.value, temper.int32_tostring(intVal__477)) then
      t_819 = SqlBuilder();
      t_819:appendSafe('1 = 0');
      return__162 = t_819.accumulated;
    else
      t_820 = SqlBuilder();
      t_820:appendFragment(col__474);
      t_820:appendSafe(' ');
      t_820:appendFragment(op__475);
      t_820:appendSafe(' ');
      t_820:appendInt32(intVal__477);
      return__162 = t_820.accumulated;
    end
  else
    local strVal__478;
    strVal__478 = clause__471.value;
    t_821 = SqlBuilder();
    t_821:appendFragment(col__474);
    t_821:appendSafe(' ');
    t_821:appendFragment(op__475);
    t_821:appendSafe(' ');
    t_821:appendString(strVal__478);
    return__162 = t_821.accumulated;
  end
  return return__162;
end;
orderBySql__201 = function(clauses__479)
  local t_834, t_835, t_836, t_837, t_838, t_839, t_840, t_841, t_842, t_843, first__481, firstDir__482, t_844, t_845, result__483, i__484;
  t_843 = (temper.list_get(clauses__479, 0)).field;
  first__481 = safeSql__198(t_843);
  if temper.str_eq((temper.list_get(clauses__479, 0)).direction, 'desc') then
    t_834 = safeSql__198(' DESC');
    firstDir__482 = t_834;
  else
    t_835 = safeSql__198(' ASC');
    firstDir__482 = t_835;
  end
  t_844 = SqlBuilder();
  t_844:appendFragment(first__481);
  t_844:appendFragment(firstDir__482);
  t_845 = t_844.accumulated;
  result__483 = t_845;
  i__484 = 1;
  while true do
    local col__485, dir__486;
    t_836 = temper.list_length(clauses__479);
    if not (i__484 < t_836) then
      break;
    end
    t_837 = (temper.list_get(clauses__479, i__484)).field;
    col__485 = safeSql__198(t_837);
    if temper.str_eq((temper.list_get(clauses__479, i__484)).direction, 'desc') then
      t_838 = safeSql__198(' DESC');
      t_842 = t_838;
    else
      t_839 = safeSql__198(' ASC');
      t_842 = t_839;
    end
    dir__486 = t_842;
    t_840 = SqlBuilder();
    t_840:appendFragment(result__483);
    t_840:appendSafe(', ');
    t_840:appendFragment(col__485);
    t_840:appendFragment(dir__486);
    t_841 = t_840.accumulated;
    result__483 = t_841;
    i__484 = temper.int32_add(i__484, 1);
  end
  return result__483;
end;
toSqlQuery = function(schema__487, selectFields__488, whereClauses__489, orderClauses__490, limitValue__491, offsetValue__492)
  local t_846, t_847, t_848, t_849, t_850, t_851, t_852, t_853, t_854, t_855, t_856, t_857, t_858, t_859, fn__3907, validSelect__494, fn__3906, validWhere__497, fn__3905, validOrder__500, t_862, table__503, cols__504, t_863, t_864, result__505;
  fn__3907 = function(f__495)
    return schema__487:hasField(f__495);
  end;
  validSelect__494 = temper.listed_filter(selectFields__488, fn__3907);
  fn__3906 = function(c__498)
    local t_860;
    t_860 = c__498.field;
    return schema__487:hasField(t_860);
  end;
  validWhere__497 = temper.listed_filter(whereClauses__489, fn__3906);
  fn__3905 = function(c__501)
    local t_861;
    t_861 = c__501.field;
    return schema__487:hasField(t_861);
  end;
  validOrder__500 = temper.listed_filter(orderClauses__490, fn__3905);
  t_862 = schema__487.tableName;
  table__503 = safeSql__198(t_862);
  cols__504 = columnListSql__199(validSelect__494);
  t_863 = SqlBuilder();
  t_863:appendSafe('SELECT ');
  t_863:appendFragment(cols__504);
  t_863:appendSafe(' FROM ');
  t_863:appendFragment(table__503);
  t_864 = t_863.accumulated;
  result__505 = t_864;
  if (temper.list_length(validWhere__497) > 0) then
    local conditions__506, i__507;
    t_846 = temper.list_get(validWhere__497, 0);
    t_847 = whereConditionSql__200(t_846, schema__487);
    conditions__506 = t_847;
    i__507 = 1;
    while true do
      local next__508;
      t_848 = temper.list_length(validWhere__497);
      if not (i__507 < t_848) then
        break;
      end
      t_849 = temper.list_get(validWhere__497, i__507);
      next__508 = whereConditionSql__200(t_849, schema__487);
      t_850 = SqlBuilder();
      t_850:appendFragment(conditions__506);
      t_850:appendSafe(' AND ');
      t_850:appendFragment(next__508);
      t_851 = t_850.accumulated;
      conditions__506 = t_851;
      i__507 = temper.int32_add(i__507, 1);
    end
    t_852 = SqlBuilder();
    t_852:appendFragment(result__505);
    t_852:appendSafe(' WHERE ');
    t_852:appendFragment(conditions__506);
    t_853 = t_852.accumulated;
    result__505 = t_853;
  end
  if (temper.list_length(validOrder__500) > 0) then
    local ordering__509;
    ordering__509 = orderBySql__201(validOrder__500);
    t_854 = SqlBuilder();
    t_854:appendFragment(result__505);
    t_854:appendSafe(' ORDER BY ');
    t_854:appendFragment(ordering__509);
    t_855 = t_854.accumulated;
    result__505 = t_855;
  end
  if (limitValue__491 >= 0) then
    t_856 = SqlBuilder();
    t_856:appendFragment(result__505);
    t_856:appendSafe(' LIMIT ');
    t_856:appendInt32(limitValue__491);
    t_857 = t_856.accumulated;
    result__505 = t_857;
  end
  if (offsetValue__492 > 0) then
    t_858 = SqlBuilder();
    t_858:appendFragment(result__505);
    t_858:appendSafe(' OFFSET ');
    t_858:appendInt32(offsetValue__492);
    t_859 = t_858.accumulated;
    result__505 = t_859;
  end
  return result__505;
end;
TokenType = temper.type('TokenType');
TokenType.get.isKeyword = function(this__88)
  return temper.str_eq(this__88.name__657, 'keyword');
end;
TokenType.get.isType = function(this__89)
  return temper.str_eq(this__89.name__657, 'type');
end;
TokenType.get.isString = function(this__90)
  return temper.str_eq(this__90.name__657, 'string');
end;
TokenType.get.isNumber = function(this__91)
  return temper.str_eq(this__91.name__657, 'number');
end;
TokenType.get.isComment = function(this__92)
  return temper.str_eq(this__92.name__657, 'comment');
end;
TokenType.get.isOperator = function(this__93)
  return temper.str_eq(this__93.name__657, 'operator');
end;
TokenType.get.isIdentifier = function(this__94)
  return temper.str_eq(this__94.name__657, 'identifier');
end;
TokenType.constructor = function(this__174, name__673)
  this__174.name__657 = name__673;
  return nil;
end;
TokenType.get.name = function(this__917)
  return this__917.name__657;
end;
Token = temper.type('Token');
Token.methods.cssClass = function(this__95)
  local return__185, name__678;
  name__678 = this__95.tokenType__674.name;
  if temper.str_eq(name__678, 'keyword') then
    return__185 = 'kw';
  elseif temper.str_eq(name__678, 'type') then
    return__185 = 'typ';
  elseif temper.str_eq(name__678, 'string') then
    return__185 = 'str';
  elseif temper.str_eq(name__678, 'number') then
    return__185 = 'num';
  elseif temper.str_eq(name__678, 'comment') then
    return__185 = 'cmt';
  elseif temper.str_eq(name__678, 'operator') then
    return__185 = 'op';
  else
    return__185 = 'id';
  end
  return return__185;
end;
Token.methods.toHtml = function(this__96)
  local cls__681, t_865;
  cls__681 = this__96:cssClass();
  t_865 = SafeHtmlBuilder();
  t_865:appendSafe("<span class='");
  t_865:appendString(cls__681);
  t_865:appendSafe("'>");
  t_865:appendString(this__96.value__675);
  t_865:appendSafe('</span>');
  return t_865.accumulated;
end;
Token.constructor = function(this__183, tokenType__683, value__684)
  this__183.tokenType__674 = tokenType__683;
  this__183.value__675 = value__684;
  return nil;
end;
Token.get.tokenType = function(this__920)
  return this__920.tokenType__674;
end;
Token.get.value = function(this__923)
  return this__923.value__675;
end;
field = function(name__293, fieldType__294, primaryKey__295, nullable__296)
  return Field(name__293, fieldType__294, primaryKey__295, nullable__296);
end;
isValidIdentifier = function(name__298)
  local return__123, t_866, this__2531, index__2533;
  ::continue_27::
  if temper.string_isempty(name__298) then
    return__123 = false;
    goto break_26;
  end
  this__2531 = name__298;
  index__2533 = 1.0;
  while true do
    local codePoint__2534, c__300;
    if not temper.string_hasindex(this__2531, index__2533) then
      break;
    end
    codePoint__2534 = temper.string_get(this__2531, index__2533);
    c__300 = codePoint__2534;
    if (c__300 ~= 95) then
      if (c__300 >= 97) then
        if (c__300 > 122) then
          return__123 = false;
          goto break_26;
        end
      elseif (c__300 >= 65) then
        if (c__300 > 90) then
          return__123 = false;
          goto break_26;
        end
      elseif (c__300 >= 48) then
        if (c__300 > 57) then
          return__123 = false;
          goto break_26;
        end
      else
        return__123 = false;
        goto break_26;
      end
    end
    t_866 = temper.string_next(this__2531, index__2533);
    index__2533 = t_866;
  end
  return__123 = true;
  ::break_26::return return__123;
end;
schema = function(tableName__301, fields__302)
  local idField__304, allFields__305, t_867;
  if not isValidIdentifier(tableName__301) then
    temper.bubble();
  end
  idField__304 = Field('id', 'Int', true, false);
  allFields__305 = temper.listbuilder_constructor();
  temper.listbuilder_add(allFields__305, idField__304);
  temper.listbuilder_addall(allFields__305, fields__302);
  t_867 = temper.listbuilder_tolist(allFields__305);
  return Schema(tableName__301, t_867);
end;
toInsertSql = function(schema__510, values__511)
  local return__165, t_868, t_869, t_870, t_871, t_872, t_873, t_874, t_875, t_876, t_877, t_878, t_879, t_880, t_881, t_882, t_883, t_884, t_885, t_886, t_887, table__513, fn__3723, fieldList__514, fn__3722, colNames__517, firstVal__520, vals__521, i__523;
  ::continue_29::t_868 = schema__510.tableName;
  table__513 = safeSql__198(t_868);
  t_869 = schema__510.fields;
  fn__3723 = function(f__515)
    local t_888;
    t_888 = f__515.name;
    return temper.mapped_has(values__511, t_888);
  end;
  fieldList__514 = temper.listed_filter(t_869, fn__3723);
  if (temper.list_length(fieldList__514) == 0) then
    t_870 = SqlBuilder();
    return__165 = t_870.accumulated;
    goto break_28;
  end
  fn__3722 = function(f__518)
    return f__518.name;
  end;
  t_871 = temper.listed_map(fieldList__514, fn__3722);
  colNames__517 = columnListSql__199(t_871);
  t_872 = (temper.list_get(fieldList__514, 0)).name;
  firstVal__520 = temper.mapped_getor(values__511, t_872, '');
  if temper.str_eq((temper.list_get(fieldList__514, 0)).fieldType, 'Int') then
    local iv__522, local_889, local_890, local_891;
    local_889, local_890, local_891 = temper.pcall(function()
      t_884 = temper.string_toint32(firstVal__520);
      iv__522 = t_884;
    end);
    if local_889 then
    else
      iv__522 = 0;
    end
    t_873 = SqlBuilder();
    t_873:appendInt32(iv__522);
    t_874 = t_873.accumulated;
    t_885 = t_874;
  else
    t_875 = SqlBuilder();
    t_875:appendString(firstVal__520);
    t_876 = t_875.accumulated;
    t_885 = t_876;
  end
  vals__521 = t_885;
  i__523 = 1;
  while true do
    local val__524;
    t_877 = temper.list_length(fieldList__514);
    if not (i__523 < t_877) then
      break;
    end
    t_878 = (temper.list_get(fieldList__514, i__523)).name;
    val__524 = temper.mapped_getor(values__511, t_878, '');
    if temper.str_eq((temper.list_get(fieldList__514, i__523)).fieldType, 'Int') then
      local local_893, local_894, local_895, iv__525;
      local_893, local_894, local_895 = temper.pcall(function()
        t_886 = temper.string_toint32(val__524);
        t_887 = t_886;
      end);
      if local_893 then
      else
        t_887 = 0;
      end
      iv__525 = t_887;
      t_879 = SqlBuilder();
      t_879:appendFragment(vals__521);
      t_879:appendSafe(', ');
      t_879:appendInt32(iv__525);
      t_880 = t_879.accumulated;
      vals__521 = t_880;
    else
      t_881 = SqlBuilder();
      t_881:appendFragment(vals__521);
      t_881:appendSafe(', ');
      t_881:appendString(val__524);
      t_882 = t_881.accumulated;
      vals__521 = t_882;
    end
    i__523 = temper.int32_add(i__523, 1);
  end
  t_883 = SqlBuilder();
  t_883:appendSafe('INSERT INTO ');
  t_883:appendFragment(table__513);
  t_883:appendSafe(' (');
  t_883:appendFragment(colNames__517);
  t_883:appendSafe(') VALUES (');
  t_883:appendFragment(vals__521);
  t_883:appendSafe(')');
  return__165 = t_883.accumulated;
  ::break_28::return return__165;
end;
main = function()
  local userFields__642, userSchema__643, t_994, store__644, rec1__645, rec2__646, rec3__647, t_995, t_996, t_997, allUsers__648, fn__3301, adults__650, fn__3300, q1__652, t_1000, q2__653, t_1001, bobby__654, q3__655, t_1002, insertVals__656, t_1003;
  temper.log('=== ORMery Demo ===\n');
  userFields__642 = temper.listof(field('name', 'String', false, false), field('age', 'Int', false, false), field('email', 'String', false, true));
  userSchema__643 = schema('users', userFields__642);
  t_994 = userSchema__643:describe();
  temper.log(t_994);
  temper.log('');
  store__644 = InMemoryStore();
  rec1__645 = store__644:insert('users', temper.map_constructor(temper.listof(temper.pair_constructor('name', 'Alice'), temper.pair_constructor('age', '25'), temper.pair_constructor('email', 'alice@example.com'))));
  rec2__646 = store__644:insert('users', temper.map_constructor(temper.listof(temper.pair_constructor('name', 'Bob'), temper.pair_constructor('age', '30'), temper.pair_constructor('email', 'bob@example.com'))));
  rec3__647 = store__644:insert('users', temper.map_constructor(temper.listof(temper.pair_constructor('name', 'Charlie'), temper.pair_constructor('age', '17'), temper.pair_constructor('email', 'charlie@example.com'))));
  temper.log('Inserted 3 users:');
  t_995 = rec1__645:describe();
  temper.log(temper.concat('  ', t_995));
  t_996 = rec2__646:describe();
  temper.log(temper.concat('  ', t_996));
  t_997 = rec3__647:describe();
  temper.log(temper.concat('  ', t_997));
  temper.log('');
  temper.log('=== In-Memory Queries ===\n');
  temper.log('All users:');
  allUsers__648 = Query(userSchema__643, store__644):all();
  fn__3301 = function(u__649)
    local t_998;
    t_998 = u__649:describe();
    temper.log(temper.concat('  ', t_998));
    return nil;
  end;
  temper.list_foreach(allUsers__648, fn__3301);
  temper.log('');
  temper.log('Adults (age >= 18):');
  adults__650 = Query(userSchema__643, store__644):where('age', '>=', '18'):all();
  fn__3300 = function(u__651)
    local t_999;
    t_999 = u__651:describe();
    temper.log(temper.concat('  ', t_999));
    return nil;
  end;
  temper.list_foreach(adults__650, fn__3300);
  temper.log('');
  temper.log('=== SQL Generation (secure-composition) ===\n');
  q1__652 = Query(userSchema__643, store__644);
  t_1000 = q1__652:toSql():toString();
  temper.log(temper.concat('SELECT all: ', t_1000));
  q2__653 = Query(userSchema__643, store__644):select(temper.listof('name', 'age')):where('age', '>=', '18'):orderBy('age', 'desc'):limit(10);
  t_1001 = q2__653:toSql():toString();
  temper.log(temper.concat('Complex:    ', t_1001));
  bobby__654 = "Robert'); DROP TABLE users;--";
  q3__655 = Query(userSchema__643, store__644):where('name', '=', "Robert'); DROP TABLE users;--");
  t_1002 = q3__655:toSql():toString();
  temper.log(temper.concat('Injection:  ', t_1002));
  insertVals__656 = temper.map_constructor(temper.listof(temper.pair_constructor('name', "O'Malley"), temper.pair_constructor('age', '42')));
  t_1003 = toInsertSql(userSchema__643, insertVals__656):toString();
  temper.log(temper.concat('INSERT:     ', t_1003));
  temper.log('\n=== Demo Complete ===');
  return nil;
end;
temperKeywords__204 = temper.listof('if', 'else', 'for', 'while', 'do', 'when', 'break', 'continue', 'return', 'let', 'var', 'class', 'export', 'import', 'public', 'private', 'protected', 'throws', 'new', 'this', 'get', 'set', 'static', 'extends', 'implements', 'true', 'false', 'null', 'bubble', 'orelse', 'of');
temperTypes__205 = temper.listof('String', 'Int', 'Boolean', 'List', 'Map', 'Bubble', 'Pair', 'Float', 'Double', 'Byte', 'Short', 'Long', 'Char', 'Void', 'Record', 'Schema', 'Field', 'Query', 'InMemoryStore', 'ListBuilder', 'MapBuilder', 'WhereClause', 'OrderClause');
classifyWord = function(word__685)
  local return__187, this__2536, n__2537, i__2538, this__2541, n__2542, i__2543;
  ::continue_31::this__2536 = temperKeywords__204;
  n__2537 = temper.list_length(this__2536);
  i__2538 = 0;
  while (i__2538 < n__2537) do
    local el__2539, kw__687;
    el__2539 = temper.list_get(this__2536, i__2538);
    i__2538 = temper.int32_add(i__2538, 1);
    kw__687 = el__2539;
    if temper.str_eq(kw__687, word__685) then
      return__187 = TokenType('keyword');
      goto break_30;
    end
  end
  this__2541 = temperTypes__205;
  n__2542 = temper.list_length(this__2541);
  i__2543 = 0;
  while (i__2543 < n__2542) do
    local el__2544, tp__688;
    el__2544 = temper.list_get(this__2541, i__2543);
    i__2543 = temper.int32_add(i__2543, 1);
    tp__688 = el__2544;
    if temper.str_eq(tp__688, word__685) then
      return__187 = TokenType('type');
      goto break_30;
    end
  end
  return__187 = TokenType('identifier');
  ::break_30::return return__187;
end;
highlightWord = function(word__689)
  local return__188, t_1004, tokenType__691, token__692;
  ::continue_33::
  if temper.str_eq(word__689, '') then
    t_1004 = SafeHtmlBuilder();
    return__188 = t_1004.accumulated;
    goto break_32;
  end
  tokenType__691 = classifyWord(word__689);
  token__692 = Token(tokenType__691, word__689);
  return__188 = token__692:toHtml();
  ::break_32::return return__188;
end;
highlightLine = function(line__693)
  local return__189, t_1005, t_1006, t_1007, t_1008, t_1009, words__695, result__696, i__697;
  ::continue_35::words__695 = temper.string_split(line__693, ' ');
  if (temper.list_length(words__695) == 0) then
    t_1005 = SafeHtmlBuilder();
    return__189 = t_1005.accumulated;
    goto break_34;
  end
  t_1006 = highlightWord(temper.list_get(words__695, 0));
  result__696 = t_1006;
  i__697 = 1;
  while true do
    local word__698;
    t_1007 = temper.list_length(words__695);
    if not (i__697 < t_1007) then
      break;
    end
    word__698 = highlightWord(temper.list_get(words__695, i__697));
    t_1008 = SafeHtmlBuilder();
    t_1008:appendSafeHtml(result__696);
    t_1008:appendSafe(' ');
    t_1008:appendSafeHtml(word__698);
    t_1009 = t_1008.accumulated;
    result__696 = t_1009;
    i__697 = temper.int32_add(i__697, 1);
  end
  return__189 = result__696;
  ::break_34::return return__189;
end;
highlightSource = function(source__699)
  local return__190, t_1010, t_1011, t_1012, t_1013, t_1014, lines__701, result__702, i__703;
  ::continue_37::lines__701 = temper.string_split(source__699, '\n');
  if (temper.list_length(lines__701) == 0) then
    t_1010 = SafeHtmlBuilder();
    return__190 = t_1010.accumulated;
    goto break_36;
  end
  t_1011 = highlightLine(temper.list_get(lines__701, 0));
  result__702 = t_1011;
  i__703 = 1;
  while true do
    local line__704;
    t_1012 = temper.list_length(lines__701);
    if not (i__703 < t_1012) then
      break;
    end
    line__704 = highlightLine(temper.list_get(lines__701, i__703));
    t_1013 = SafeHtmlBuilder();
    t_1013:appendSafeHtml(result__702);
    t_1013:appendSafe('\\n');
    t_1013:appendSafeHtml(line__704);
    t_1014 = t_1013.accumulated;
    result__702 = t_1014;
    i__703 = temper.int32_add(i__703, 1);
  end
  return__190 = result__702;
  ::break_36::return return__190;
end;
highlightBlock = function(source__705)
  local highlighted__707, t_1015;
  highlighted__707 = highlightSource(source__705);
  t_1015 = SafeHtmlBuilder();
  t_1015:appendSafe("<pre class='temper-code'><code>");
  t_1015:appendSafeHtml(highlighted__707);
  t_1015:appendSafe('</code></pre>');
  return t_1015.accumulated;
end;
exports = {};
exports.DemoController = DemoController;
exports.Field = Field;
exports.Schema = Schema;
exports.InMemoryStore = InMemoryStore;
exports.Record = Record;
exports.Query = Query;
exports.WhereClause = WhereClause;
exports.OrderClause = OrderClause;
exports.toSqlQuery = toSqlQuery;
exports.TokenType = TokenType;
exports.Token = Token;
exports.field = field;
exports.isValidIdentifier = isValidIdentifier;
exports.schema = schema;
exports.toInsertSql = toInsertSql;
exports.main = main;
exports.classifyWord = classifyWord;
exports.highlightWord = highlightWord;
exports.highlightLine = highlightLine;
exports.highlightSource = highlightSource;
exports.highlightBlock = highlightBlock;
return exports;
