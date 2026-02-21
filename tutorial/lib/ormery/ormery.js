import {
  SqlFragment as SqlFragment_1473, SqlBuilder as SqlBuilder_1509
} from "./sql.js";
import {
  SafeHtmlBuilder as SafeHtmlBuilder_1623, SafeHtml as SafeHtml_1624
} from "./html.js";
import {
  globalConsole as globalConsole__1187, type as type__1230, cmpString as cmpString__1505, listedMap as listedMap_1207, listedJoin as listedJoin_1210, listedGet as listedGet_1256, mapBuilderConstructor as mapBuilderConstructor_1293, mapBuilderSet as mapBuilderSet_1299, mappedGetOr as mappedGetOr_1304, mappedToMapBuilder as mappedToMapBuilder_1306, mappedToMap as mappedToMap_1308, listBuilderAdd as listBuilderAdd_1311, listBuilderToList as listBuilderToList_1316, mappedGet as mappedGet_1337, stringToInt32 as stringToInt32_1346, mappedToListWith as mappedToListWith_1352, panic as panic_1405, cmpGeneric as cmpGeneric_1446, listedFilter as listedFilter_1459, listedSorted as listedSorted_1465, listedSlice as listedSlice_1469, listBuilderAddAll as listBuilderAddAll_1637, pairConstructor as pairConstructor_1793, mapConstructor as mapConstructor_1792, stringSplit as stringSplit_1889
} from "@temperlang/core";
/** @type {Console_1188} */
const console_1186 = globalConsole__1187;
export class DemoController extends type__1230() {
  /** @type {Schema} */
  #schema_1189;
  /** @type {InMemoryStore} */
  #store_1190;
  /** @type {number} */
  #queryCount_1191;
  /**
   * @param {{
   *   schema: Schema, store: InMemoryStore
   * }}
   * props
   * @returns {DemoController}
   */
  static["new"](props) {
    return new DemoController(props.schema, props.store);
  }
  /**
   * @param {Schema} schema_1192
   * @param {InMemoryStore} store_1193
   */
  constructor(schema_1192, store_1193) {
    super ();
    this.#schema_1189 = schema_1192;
    this.#store_1190 = store_1193;
    this.#queryCount_1191 = 0;
    return;
  }
  /** @returns {number} */
  getRecordCount() {
    let t_1195 = this.#schema_1189.tableName;
    return this.#store_1190.count(t_1195);
  }
  /** @returns {number} */
  getAdultCount() {
    return new Query(this.#schema_1189, this.#store_1190).where("age", "\u003e=", "18").all().length;
  }
  /** @returns {number} */
  getQueryCount() {
    return this.#queryCount_1191;
  }
  #incrementQueryCount_1199() {
    const t_1200 = this.#queryCount_1191 + 1 | 0;
    this.#queryCount_1191 = t_1200;
    return;
  }
  /**
   * @param {Array<Record>} records_1203
   * @returns {string}
   */
  #formatRecords_1202(records_1203) {
    function fn_1204(record_1205) {
      return "  " + record_1205.describe();
    }
    const lines_1206 = listedMap_1207(records_1203, fn_1204);
    function fn_1208(s_1209) {
      return s_1209;
    }
    return listedJoin_1210(lines_1206, "\n", fn_1208);
  }
  /** @returns {string} */
  runDemo1() {
    this.#incrementQueryCount_1199();
    const results_1212 = new Query(this.#schema_1189, this.#store_1190).all();
    const formatted_1213 = this.#formatRecords_1202(results_1212);
    return "=== Demo 1: All Users ===" + "\n" + "\n" + "Query: new Query(userSchema, store).all()" + "\n" + "\n" + "Results:" + "\n" + formatted_1213 + "\n" + "\n" + "Total: " + results_1212.length.toString() + " records";
  }
  /** @returns {string} */
  runDemo2() {
    this.#incrementQueryCount_1199();
    const results_1215 = new Query(this.#schema_1189, this.#store_1190).where("age", "\u003e=", "18").all();
    const formatted_1216 = this.#formatRecords_1202(results_1215);
    return "=== Demo 2: Filter Adults ===" + "\n" + "\n" + "Query: new Query(userSchema, store)" + "\n" + "  .where(" + "\u0022" + "age" + "\u0022" + ", " + "\u0022" + "\u003e=" + "\u0022" + ", " + "\u0022" + "18" + "\u0022" + ")" + "\n" + "  .all()" + "\n" + "\n" + "Results:" + "\n" + formatted_1216 + "\n" + "\n" + "Total: " + results_1215.length.toString() + " adults found";
  }
  /** @returns {string} */
  runDemo3() {
    this.#incrementQueryCount_1199();
    const results_1218 = new Query(this.#schema_1189, this.#store_1190).orderBy("age", "desc").all();
    const formatted_1219 = this.#formatRecords_1202(results_1218);
    return "=== Demo 3: Sort by Age (Descending) ===" + "\n" + "\n" + "Query: new Query(userSchema, store)" + "\n" + "  .orderBy(" + "\u0022" + "age" + "\u0022" + ", " + "\u0022" + "desc" + "\u0022" + ")" + "\n" + "  .all()" + "\n" + "\n" + "Results (ordered by age, oldest first):" + "\n" + formatted_1219;
  }
  /** @returns {string} */
  runDemo4() {
    this.#incrementQueryCount_1199();
    const page1_1221 = new Query(this.#schema_1189, this.#store_1190).orderBy("id", "asc").limit(2).all();
    const page2_1222 = new Query(this.#schema_1189, this.#store_1190).orderBy("id", "asc").offset(2).limit(2).all();
    const formatted1_1223 = this.#formatRecords_1202(page1_1221);
    const formatted2_1224 = this.#formatRecords_1202(page2_1222);
    return "=== Demo 4: Pagination ===" + "\n" + "\n" + "Page 1: .orderBy(" + "\u0022" + "id" + "\u0022" + ", " + "\u0022" + "asc" + "\u0022" + ").limit(2)" + "\n" + "\n" + formatted1_1223 + "\n" + "\n" + "Page 2: .orderBy(" + "\u0022" + "id" + "\u0022" + ", " + "\u0022" + "asc" + "\u0022" + ").offset(2).limit(2)" + "\n" + "\n" + formatted2_1224;
  }
  /** @returns {string} */
  runDemo5() {
    this.#incrementQueryCount_1199();
    const results_1226 = new Query(this.#schema_1189, this.#store_1190).where("age", "\u003e=", "18").where("email", "!=", "").orderBy("age", "desc").select(Object.freeze(["name", "age"])).limit(2).all();
    const formatted_1227 = this.#formatRecords_1202(results_1226);
    return "=== Demo 5: Complex Query ===" + "\n" + "\n" + "Query: new Query(userSchema, store)" + "\n" + "  .where(" + "\u0022" + "age" + "\u0022" + ", " + "\u0022" + "\u003e=" + "\u0022" + ", " + "\u0022" + "18" + "\u0022" + ")" + "\n" + "  .where(" + "\u0022" + "email" + "\u0022" + ", " + "\u0022" + "!=" + "\u0022" + ", " + "\u0022" + "\u0022" + ")" + "\n" + "  .orderBy(" + "\u0022" + "age" + "\u0022" + ", " + "\u0022" + "desc" + "\u0022" + ")" + "\n" + "  .select([" + "\u0022" + "name" + "\u0022" + ", " + "\u0022" + "age" + "\u0022" + "])" + "\n" + "  .limit(2)" + "\n" + "  .all()" + "\n" + "\n" + "Results (adults with email, showing name/age only, oldest first, max 2):" + "\n" + formatted_1227;
  }
  /** @returns {Schema} */
  get schema() {
    return this.#schema_1189;
  }
  /** @returns {InMemoryStore} */
  get store() {
    return this.#store_1190;
  }
};
export class Field extends type__1230() {
  /** @type {string} */
  #name_1231;
  /** @type {string} */
  #fieldType_1232;
  /** @type {boolean} */
  #primaryKey_1233;
  /** @type {boolean} */
  #nullable_1234;
  /** @returns {string} */
  get description() {
    let pk_1236;
    if (this.#primaryKey_1233) {
      pk_1236 = " (PK)";
    } else {
      pk_1236 = "";
    }
    let null_1237;
    if (this.#nullable_1234) {
      null_1237 = " (nullable)";
    } else {
      null_1237 = "";
    }
    return String(this.#name_1231) + ": " + this.#fieldType_1232 + pk_1236 + null_1237;
  }
  /**
   * @param {{
   *   name: string, fieldType: string, primaryKey: boolean, nullable: boolean
   * }}
   * props
   * @returns {Field}
   */
  static["new"](props) {
    return new Field(props.name, props.fieldType, props.primaryKey, props.nullable);
  }
  /**
   * @param {string} name_1238
   * @param {string} fieldType_1239
   * @param {boolean} primaryKey_1240
   * @param {boolean} nullable_1241
   */
  constructor(name_1238, fieldType_1239, primaryKey_1240, nullable_1241) {
    super ();
    this.#name_1231 = name_1238;
    this.#fieldType_1232 = fieldType_1239;
    this.#primaryKey_1233 = primaryKey_1240;
    this.#nullable_1234 = nullable_1241;
    return;
  }
  /** @returns {string} */
  get name() {
    return this.#name_1231;
  }
  /** @returns {string} */
  get fieldType() {
    return this.#fieldType_1232;
  }
  /** @returns {boolean} */
  get primaryKey() {
    return this.#primaryKey_1233;
  }
  /** @returns {boolean} */
  get nullable() {
    return this.#nullable_1234;
  }
};
export class Schema extends type__1230() {
  /** @type {string} */
  #tableName_1246;
  /** @type {Array<Field>} */
  #fields_1247;
  /**
   * @param {string} name_1249
   * @returns {Field}
   */
  getField(name_1249) {
    let return_1250;
    fn_1251: {
      const this_1252 = this.#fields_1247;
      const n_1253 = this_1252.length;
      let i_1254 = 0;
      while (i_1254 < n_1253) {
        const el_1255 = listedGet_1256(this_1252, i_1254);
        i_1254 = i_1254 + 1 | 0;
        const field_1257 = el_1255;
        if (field_1257.name === name_1249) {
          return_1250 = field_1257;
          break fn_1251;
        }
      }
      throw Error();
    }
    return return_1250;
  }
  /**
   * @param {string} name_1259
   * @returns {boolean}
   */
  hasField(name_1259) {
    let return_1260;
    fn_1261: {
      const this_1262 = this.#fields_1247;
      const n_1263 = this_1262.length;
      let i_1264 = 0;
      while (i_1264 < n_1263) {
        const el_1265 = listedGet_1256(this_1262, i_1264);
        i_1264 = i_1264 + 1 | 0;
        const field_1266 = el_1265;
        if (field_1266.name === name_1259) {
          return_1260 = true;
          break fn_1261;
        }
      }
      return_1260 = false;
    }
    return return_1260;
  }
  /** @returns {Field} */
  get primaryKeyField() {
    let return_1268;
    fn_1269: {
      const this_1270 = this.#fields_1247;
      const n_1271 = this_1270.length;
      let i_1272 = 0;
      while (i_1272 < n_1271) {
        const el_1273 = listedGet_1256(this_1270, i_1272);
        i_1272 = i_1272 + 1 | 0;
        const field_1274 = el_1273;
        if (field_1274.primaryKey) {
          return_1268 = field_1274;
          break fn_1269;
        }
      }
      throw Error();
    }
    return return_1268;
  }
  /** @returns {Array<string>} */
  get fieldNames() {
    function fn_1276(f_1277) {
      return f_1277.name;
    }
    return listedMap_1207(this.#fields_1247, fn_1276);
  }
  /** @returns {string} */
  describe() {
    const header_1279 = "Schema: " + this.#tableName_1246 + "\n";
    function fn_1280(f_1281) {
      return "  - " + f_1281.description;
    }
    let t_1282 = listedMap_1207(this.#fields_1247, fn_1280);
    function fn_1283(s_1284) {
      return s_1284;
    }
    const fieldList_1285 = listedJoin_1210(t_1282, "\n", fn_1283);
    return String(header_1279) + fieldList_1285;
  }
  /**
   * @param {{
   *   tableName: string, fields: Array<Field>
   * }}
   * props
   * @returns {Schema}
   */
  static["new"](props) {
    return new Schema(props.tableName, props.fields);
  }
  /**
   * @param {string} tableName_1286
   * @param {Array<Field>} fields_1287
   */
  constructor(tableName_1286, fields_1287) {
    super ();
    this.#tableName_1246 = tableName_1286;
    this.#fields_1247 = fields_1287;
    return;
  }
  /** @returns {string} */
  get tableName() {
    return this.#tableName_1246;
  }
  /** @returns {Array<Field>} */
  get fields() {
    return this.#fields_1247;
  }
};
export class InMemoryStore extends type__1230() {
  /** @type {Map<string, Array<Record>>} */
  #tables_1290;
  /** @type {Map<string, number>} */
  #nextIds_1291;
  constructor() {
    super ();
    let t_1292 = mapBuilderConstructor_1293();
    this.#tables_1290 = t_1292;
    let t_1294 = mapBuilderConstructor_1293();
    this.#nextIds_1291 = t_1294;
    return;
  }
  /** @param {string} tableName_1297 */
  #ensureTable_1296(tableName_1297) {
    let t_1298;
    if (! this.#tables_1290.has(tableName_1297)) {
      t_1298 = [];
      mapBuilderSet_1299(this.#tables_1290, tableName_1297, t_1298);
      mapBuilderSet_1299(this.#nextIds_1291, tableName_1297, 1);
    }
    return;
  }
  /**
   * @param {string} tableName_1301
   * @param {Map<string, string>} data_1302
   * @returns {Record}
   */
  insert(tableName_1301, data_1302) {
    this.#ensureTable_1296(tableName_1301);
    const id_1303 = mappedGetOr_1304(this.#nextIds_1291, tableName_1301, 1);
    mapBuilderSet_1299(this.#nextIds_1291, tableName_1301, id_1303 + 1 | 0);
    const dataBuilder_1305 = mappedToMapBuilder_1306(data_1302);
    mapBuilderSet_1299(dataBuilder_1305, "id", id_1303.toString());
    const record_1307 = new Record(mappedToMap_1308(dataBuilder_1305));
    let t_1309 = [];
    const table_1310 = mappedGetOr_1304(this.#tables_1290, tableName_1301, t_1309);
    listBuilderAdd_1311(table_1310, record_1307);
    return record_1307;
  }
  /**
   * @param {string} tableName_1313
   * @returns {Array<Record>}
   */
  all(tableName_1313) {
    this.#ensureTable_1296(tableName_1313);
    let t_1314 = [];
    const table_1315 = mappedGetOr_1304(this.#tables_1290, tableName_1313, t_1314);
    return listBuilderToList_1316(table_1315);
  }
  /**
   * @param {string} tableName_1318
   * @param {number} id_1319
   * @returns {Record}
   */
  get(tableName_1318, id_1319) {
    let return_1320;
    let t_1321;
    fn_1322: {
      this.#ensureTable_1296(tableName_1318);
      t_1321 = [];
      const table_1323 = mappedGetOr_1304(this.#tables_1290, tableName_1318, t_1321);
      const this_1324 = listBuilderToList_1316(table_1323);
      const n_1325 = this_1324.length;
      let i_1326 = 0;
      while (i_1326 < n_1325) {
        const el_1327 = listedGet_1256(this_1324, i_1326);
        i_1326 = i_1326 + 1 | 0;
        const record_1328 = el_1327;
        let recordId_1329;
        try {
          recordId_1329 = record_1328.id;
        } catch {
          throw Error();
        }
        if (recordId_1329 === id_1319) {
          return_1320 = record_1328;
          break fn_1322;
        }
      }
      throw Error();
    }
    return return_1320;
  }
  /**
   * @param {string} tableName_1331
   * @returns {number}
   */
  count(tableName_1331) {
    this.#ensureTable_1296(tableName_1331);
    let t_1332 = [];
    const table_1333 = mappedGetOr_1304(this.#tables_1290, tableName_1331, t_1332);
    return table_1333.length;
  }
};
export class Record extends type__1230() {
  /** @type {Map<string, string>} */
  #data_1334;
  /**
   * @param {string} field_1336
   * @returns {string}
   */
  get(field_1336) {
    return mappedGet_1337(this.#data_1334, field_1336);
  }
  /**
   * @param {string} field_1339
   * @param {string} fallback_1340
   * @returns {string}
   */
  getOr(field_1339, fallback_1340) {
    return mappedGetOr_1304(this.#data_1334, field_1339, fallback_1340);
  }
  /**
   * @param {string} field_1342
   * @returns {boolean}
   */
  has(field_1342) {
    return this.#data_1334.has(field_1342);
  }
  /** @returns {number} */
  get id() {
    let return_1344;
    let idStr_1345;
    idStr_1345 = mappedGet_1337(this.#data_1334, "id");
    try {
      return_1344 = stringToInt32_1346(idStr_1345);
    } catch {
      throw Error();
    }
    return return_1344;
  }
  /** @returns {string} */
  describe() {
    function fn_1348(k_1349, v_1350) {
      return String(k_1349) + ": " + v_1350;
    }
    const pairs_1351 = mappedToListWith_1352(this.#data_1334, fn_1348);
    function fn_1353(s_1354) {
      return s_1354;
    }
    return listedJoin_1210(pairs_1351, ", ", fn_1353);
  }
  /** @param {Map<string, string>} data_1355 */
  constructor(data_1355) {
    super ();
    this.#data_1334 = data_1355;
    return;
  }
  /** @returns {Map<string, string>} */
  get data() {
    return this.#data_1334;
  }
};
export class Query extends type__1230() {
  /** @type {Schema} */
  #schema_1357;
  /** @type {InMemoryStore} */
  #store_1358;
  /** @type {Array<WhereClause>} */
  #whereClauses_1359;
  /** @type {Array<string>} */
  #selectFields_1360;
  /** @type {Array<OrderClause>} */
  #orderByClauses_1361;
  /** @type {number} */
  #limitValue_1362;
  /** @type {number} */
  #offsetValue_1363;
  /**
   * @param {{
   *   schema: Schema, store: InMemoryStore
   * }}
   * props
   * @returns {Query}
   */
  static["new"](props) {
    return new Query(props.schema, props.store);
  }
  /**
   * @param {Schema} schema_1364
   * @param {InMemoryStore} store_1365
   */
  constructor(schema_1364, store_1365) {
    super ();
    this.#schema_1357 = schema_1364;
    this.#store_1358 = store_1365;
    let t_1366 = [];
    this.#whereClauses_1359 = t_1366;
    let t_1367 = Object.freeze([]);
    this.#selectFields_1360 = t_1367;
    let t_1368 = [];
    this.#orderByClauses_1361 = t_1368;
    this.#limitValue_1362 = -1;
    this.#offsetValue_1363 = 0;
    return;
  }
  /**
   * @param {string} field_1370
   * @param {string} operator_1371
   * @param {string} value_1372
   * @returns {Query}
   */
  where(field_1370, operator_1371, value_1372) {
    let t_1373 = new WhereClause(field_1370, operator_1371, value_1372);
    listBuilderAdd_1311(this.#whereClauses_1359, t_1373);
    return this;
  }
  /**
   * @param {Array<string>} fields_1375
   * @returns {Query}
   */
  select(fields_1375) {
    this.#selectFields_1360 = fields_1375;
    return this;
  }
  /**
   * @param {string} field_1377
   * @param {string} direction_1378
   * @returns {Query}
   */
  orderBy(field_1377, direction_1378) {
    let t_1379 = new OrderClause(field_1377, direction_1378);
    listBuilderAdd_1311(this.#orderByClauses_1361, t_1379);
    return this;
  }
  /**
   * @param {number} n_1381
   * @returns {Query}
   */
  limit(n_1381) {
    this.#limitValue_1362 = n_1381;
    return this;
  }
  /**
   * @param {number} n_1383
   * @returns {Query}
   */
  offset(n_1383) {
    this.#offsetValue_1363 = n_1383;
    return this;
  }
  /**
   * @param {Record} record_1386
   * @returns {boolean}
   */
  #matchesWhere_1385(record_1386) {
    let return_1387;
    let t_1388;
    let t_1389;
    let t_1390;
    let t_1391;
    let t_1392;
    let t_1393;
    let t_1394;
    let t_1395;
    fn_1396: {
      const this_1397 = listBuilderToList_1316(this.#whereClauses_1359);
      const n_1398 = this_1397.length;
      let i_1399 = 0;
      while (i_1399 < n_1398) {
        const el_1400 = listedGet_1256(this_1397, i_1399);
        i_1399 = i_1399 + 1 | 0;
        const clause_1401 = el_1400;
        let t_1402;
        t_1388 = clause_1401.field;
        const recordValue_1403 = record_1386.getOr(t_1388, "");
        t_1389 = clause_1401.field;
        if (! this.#schema_1357.hasField(t_1389)) {
          return_1387 = false;
          break fn_1396;
        }
        let fieldInfo_1404;
        try {
          t_1390 = clause_1401.field;
          t_1402 = this.#schema_1357.getField(t_1390);
          fieldInfo_1404 = t_1402;
        } catch {
          fieldInfo_1404 = panic_1405();
        }
        const fieldType_1406 = fieldInfo_1404.fieldType;
        let matches_1407;
        if (fieldType_1406 === "Int") {
          t_1391 = clause_1401.operator;
          t_1392 = clause_1401.value;
          t_1393 = compareInt_1408(recordValue_1403, t_1391, t_1392);
          matches_1407 = t_1393;
        } else if (fieldType_1406 === "String") {
          t_1394 = clause_1401.operator;
          t_1395 = clause_1401.value;
          matches_1407 = compareString_1409(recordValue_1403, t_1394, t_1395);
        } else {
          matches_1407 = false;
        }
        if (! matches_1407) {
          return_1387 = false;
          break fn_1396;
        }
      }
      return_1387 = true;
    }
    return return_1387;
  }
  /**
   * @param {Record} record_1412
   * @returns {Record}
   */
  #projectRecord_1411(record_1412) {
    let return_1413;
    let t_1414;
    fn_1415: {
      if (this.#selectFields_1360.length === 0) {
        return_1413 = record_1412;
        break fn_1415;
      }
      const builder_1416 = mapBuilderConstructor_1293();
      function fn_1417(fieldName_1418) {
        const value_1419 = record_1412.getOr(fieldName_1418, "");
        mapBuilderSet_1299(builder_1416, fieldName_1418, value_1419);
        return;
      }
      this.#selectFields_1360.forEach(fn_1417);
      t_1414 = mappedToMap_1308(builder_1416);
      return_1413 = new Record(t_1414);
    }
    return return_1413;
  }
  /**
   * @param {Record} a_1422
   * @param {Record} b_1423
   * @param {Array<OrderClause>} orderClauses_1424
   * @returns {number}
   */
  #compareRecords_1421(a_1422, b_1423, orderClauses_1424) {
    let return_1425;
    let t_1426;
    let t_1427;
    let t_1428;
    let t_1429;
    fn_1430: {
      const this_1431 = orderClauses_1424;
      const n_1432 = this_1431.length;
      let i_1433 = 0;
      while (i_1433 < n_1432) {
        const el_1434 = listedGet_1256(this_1431, i_1433);
        i_1433 = i_1433 + 1 | 0;
        const clause_1435 = el_1434;
        let t_1436;
        let t_1437;
        let t_1438;
        t_1426 = clause_1435.field;
        const aVal_1439 = a_1422.getOr(t_1426, "");
        t_1427 = clause_1435.field;
        const bVal_1440 = b_1423.getOr(t_1427, "");
        t_1428 = clause_1435.field;
        if (! this.#schema_1357.hasField(t_1428)) {
          continue;
        }
        let fieldInfo_1441;
        try {
          t_1429 = clause_1435.field;
          t_1436 = this.#schema_1357.getField(t_1429);
          fieldInfo_1441 = t_1436;
        } catch {
          fieldInfo_1441 = panic_1405();
        }
        const fieldType_1442 = fieldInfo_1441.fieldType;
        let cmp_1443;
        if (fieldType_1442 === "Int") {
          let aInt_1444;
          try {
            t_1437 = stringToInt32_1346(aVal_1439);
            aInt_1444 = t_1437;
          } catch {
            aInt_1444 = 0;
          }
          let bInt_1445;
          try {
            t_1438 = stringToInt32_1346(bVal_1440);
            bInt_1445 = t_1438;
          } catch {
            bInt_1445 = 0;
          }
          cmp_1443 = cmpGeneric_1446(aInt_1444, bInt_1445);
        } else if (fieldType_1442 === "String") {
          cmp_1443 = cmpGeneric_1446(aVal_1439, bVal_1440);
        } else {
          cmp_1443 = 0;
        }
        if (cmp_1443 !== 0) {
          if (clause_1435.direction === "desc") {
            return_1425 = - cmp_1443 | 0;
          } else {
            return_1425 = cmp_1443;
          }
          break fn_1430;
        }
      }
      return_1425 = 0;
    }
    return return_1425;
  }
  /** @returns {Array<Record>} */
  all() {
    const this1457 = this;
    let t_1448;
    let t_1449;
    let t_1450;
    let t_1451;
    let t_1452;
    let t_1453 = this.#schema_1357.tableName;
    const allRecords_1454 = this.#store_1358.all(t_1453);
    function fn_1455(r_1456) {
      return this1457.#matchesWhere_1385(r_1456);
    }
    const filtered_1458 = listedFilter_1459(allRecords_1454, fn_1455);
    let sorted_1460;
    if (this.#orderByClauses_1361.length > 0) {
      const clauses_1461 = listBuilderToList_1316(this.#orderByClauses_1361);
      function fn_1462(a_1463, b_1464) {
        return this1457.#compareRecords_1421(a_1463, b_1464, clauses_1461);
      }
      t_1448 = listedSorted_1465(filtered_1458, fn_1462);
      sorted_1460 = t_1448;
    } else {
      sorted_1460 = filtered_1458;
    }
    let sliced_1466;
    if (this.#limitValue_1362 > 0) {
      const start_1467 = this.#offsetValue_1363;
      const end_1468 = this.#offsetValue_1363 + this.#limitValue_1362 | 0;
      t_1449 = listedSlice_1469(sorted_1460, start_1467, end_1468);
      sliced_1466 = t_1449;
    } else if (this.#offsetValue_1363 > 0) {
      t_1451 = this.#offsetValue_1363;
      t_1450 = sorted_1460.length;
      t_1452 = listedSlice_1469(sorted_1460, t_1451, t_1450);
      sliced_1466 = t_1452;
    } else {
      sliced_1466 = sorted_1460;
    }
    function fn_1470(r_1471) {
      return this1457.#projectRecord_1411(r_1471);
    }
    return listedMap_1207(sliced_1466, fn_1470);
  }
  /** @returns {SqlFragment_1473} */
  toSql() {
    return toSqlQuery(this.#schema_1357, this.#selectFields_1360, listBuilderToList_1316(this.#whereClauses_1359), listBuilderToList_1316(this.#orderByClauses_1361), this.#limitValue_1362, this.#offsetValue_1363);
  }
  /** @returns {Schema} */
  get schema() {
    return this.#schema_1357;
  }
  /** @returns {InMemoryStore} */
  get store() {
    return this.#store_1358;
  }
};
export class WhereClause extends type__1230() {
  /** @type {string} */
  #field_1476;
  /** @type {string} */
  #operator_1477;
  /** @type {string} */
  #value_1478;
  /** @returns {string} */
  describe() {
    return String(this.#field_1476) + " " + this.#operator_1477 + " " + this.#value_1478;
  }
  /**
   * @param {{
   *   field: string, operator: string, value: string
   * }}
   * props
   * @returns {WhereClause}
   */
  static["new"](props) {
    return new WhereClause(props.field, props.operator, props.value);
  }
  /**
   * @param {string} field_1480
   * @param {string} operator_1481
   * @param {string} value_1482
   */
  constructor(field_1480, operator_1481, value_1482) {
    super ();
    this.#field_1476 = field_1480;
    this.#operator_1477 = operator_1481;
    this.#value_1478 = value_1482;
    return;
  }
  /** @returns {string} */
  get field() {
    return this.#field_1476;
  }
  /** @returns {string} */
  get operator() {
    return this.#operator_1477;
  }
  /** @returns {string} */
  get value() {
    return this.#value_1478;
  }
};
export class OrderClause extends type__1230() {
  /** @type {string} */
  #field_1486;
  /** @type {string} */
  #direction_1487;
  /** @returns {string} */
  describe() {
    return String(this.#field_1486) + " " + this.#direction_1487;
  }
  /**
   * @param {{
   *   field: string, direction: string
   * }}
   * props
   * @returns {OrderClause}
   */
  static["new"](props) {
    return new OrderClause(props.field, props.direction);
  }
  /**
   * @param {string} field_1489
   * @param {string} direction_1490
   */
  constructor(field_1489, direction_1490) {
    super ();
    this.#field_1486 = field_1489;
    this.#direction_1487 = direction_1490;
    return;
  }
  /** @returns {string} */
  get field() {
    return this.#field_1486;
  }
  /** @returns {string} */
  get direction() {
    return this.#direction_1487;
  }
};
/**
 * @param {string} recordValue_1493
 * @param {string} operator_1494
 * @param {string} clauseValue_1495
 * @returns {boolean}
 */
function compareInt_1408(recordValue_1493, operator_1494, clauseValue_1495) {
  let return_1496;
  let t_1497;
  let t_1498;
  let rv_1499;
  try {
    t_1497 = stringToInt32_1346(recordValue_1493);
    rv_1499 = t_1497;
  } catch {
    rv_1499 = 0;
  }
  let cv_1500;
  try {
    t_1498 = stringToInt32_1346(clauseValue_1495);
    cv_1500 = t_1498;
  } catch {
    cv_1500 = 0;
  }
  if (operator_1494 === "==") {
    return_1496 = rv_1499 === cv_1500;
  } else if (operator_1494 === "!=") {
    return_1496 = rv_1499 !== cv_1500;
  } else if (operator_1494 === "\u003e") {
    return_1496 = rv_1499 > cv_1500;
  } else if (operator_1494 === "\u003c") {
    return_1496 = rv_1499 < cv_1500;
  } else if (operator_1494 === "\u003e=") {
    return_1496 = rv_1499 >= cv_1500;
  } else if (operator_1494 === "\u003c=") {
    return_1496 = rv_1499 <= cv_1500;
  } else {
    return_1496 = false;
  }
  return return_1496;
}
/**
 * @param {string} recordValue_1501
 * @param {string} operator_1502
 * @param {string} clauseValue_1503
 * @returns {boolean}
 */
function compareString_1409(recordValue_1501, operator_1502, clauseValue_1503) {
  let return_1504;
  if (operator_1502 === "==") {
    return_1504 = recordValue_1501 === clauseValue_1503;
  } else if (operator_1502 === "!=") {
    return_1504 = recordValue_1501 !== clauseValue_1503;
  } else if (operator_1502 === "\u003e") {
    return_1504 = cmpString__1505(recordValue_1501, clauseValue_1503) > 0;
  } else if (operator_1502 === "\u003c") {
    return_1504 = cmpString__1505(recordValue_1501, clauseValue_1503) < 0;
  } else if (operator_1502 === "\u003e=") {
    return_1504 = cmpString__1505(recordValue_1501, clauseValue_1503) >= 0;
  } else if (operator_1502 === "\u003c=") {
    return_1504 = cmpString__1505(recordValue_1501, clauseValue_1503) <= 0;
  } else {
    return_1504 = false;
  }
  return return_1504;
}
/**
 * @param {string} trusted_1507
 * @returns {SqlFragment_1473}
 */
function safeSql_1506(trusted_1507) {
  const b_1508 = new SqlBuilder_1509();
  b_1508.appendSafe(trusted_1507);
  return b_1508.accumulated;
}
/**
 * @param {Array<string>} selectFields_1511
 * @returns {SqlFragment_1473}
 */
function columnListSql_1510(selectFields_1511) {
  let return_1512;
  let t_1513;
  let t_1514;
  let t_1515;
  let t_1516;
  let t_1517;
  let t_1518;
  let t_1519;
  let t_1520;
  if (selectFields_1511.length === 0) {
    t_1513 = new SqlBuilder_1509();
    t_1513.appendSafe("*");
    return_1512 = t_1513.accumulated;
  } else {
    t_1514 = listedGet_1256(selectFields_1511, 0);
    const first_1521 = safeSql_1506(t_1514);
    t_1515 = new SqlBuilder_1509();
    t_1515.appendFragment(first_1521);
    t_1516 = t_1515.accumulated;
    let result_1522 = t_1516;
    let i_1523 = 1;
    while (true) {
      t_1517 = selectFields_1511.length;
      if (!(i_1523 < t_1517)) {
        break;
      }
      t_1518 = listedGet_1256(selectFields_1511, i_1523);
      const col_1524 = safeSql_1506(t_1518);
      t_1519 = new SqlBuilder_1509();
      t_1519.appendFragment(result_1522);
      t_1519.appendSafe(", ");
      t_1519.appendFragment(col_1524);
      t_1520 = t_1519.accumulated;
      result_1522 = t_1520;
      i_1523 = i_1523 + 1 | 0;
    }
    return_1512 = result_1522;
  }
  return return_1512;
}
/**
 * @param {string} op_1526
 * @returns {string}
 */
function validOperator_1525(op_1526) {
  let return_1527;
  if (op_1526 === "=") {
    return_1527 = "=";
  } else if (op_1526 === "==") {
    return_1527 = "=";
  } else if (op_1526 === "!=") {
    return_1527 = "!=";
  } else if (op_1526 === "\u003c\u003e") {
    return_1527 = "\u003c\u003e";
  } else if (op_1526 === "\u003e") {
    return_1527 = "\u003e";
  } else if (op_1526 === "\u003c") {
    return_1527 = "\u003c";
  } else if (op_1526 === "\u003e=") {
    return_1527 = "\u003e=";
  } else if (op_1526 === "\u003c=") {
    return_1527 = "\u003c=";
  } else {
    return_1527 = "=";
  }
  return return_1527;
}
/**
 * @param {WhereClause} clause_1529
 * @param {Schema} schema_1530
 * @returns {SqlFragment_1473}
 */
function whereConditionSql_1528(clause_1529, schema_1530) {
  let return_1531;
  let t_1532;
  let t_1533;
  let t_1534;
  let t_1535;
  let t_1536;
  let t_1537 = clause_1529.field;
  const col_1538 = safeSql_1506(t_1537);
  let t_1539 = clause_1529.operator;
  const op_1540 = safeSql_1506(validOperator_1525(t_1539));
  let fieldInfo_1541;
  try {
    t_1532 = clause_1529.field;
    t_1535 = schema_1530.getField(t_1532);
    fieldInfo_1541 = t_1535;
  } catch {
    fieldInfo_1541 = panic_1405();
  }
  if (fieldInfo_1541.fieldType === "Int") {
    let intVal_1542;
    try {
      t_1536 = stringToInt32_1346(clause_1529.value);
      intVal_1542 = t_1536;
    } catch {
      intVal_1542 = 0;
    }
    t_1533 = new SqlBuilder_1509();
    t_1533.appendFragment(col_1538);
    t_1533.appendSafe(" ");
    t_1533.appendFragment(op_1540);
    t_1533.appendSafe(" ");
    t_1533.appendInt32(intVal_1542);
    return_1531 = t_1533.accumulated;
  } else {
    const strVal_1543 = clause_1529.value;
    t_1534 = new SqlBuilder_1509();
    t_1534.appendFragment(col_1538);
    t_1534.appendSafe(" ");
    t_1534.appendFragment(op_1540);
    t_1534.appendSafe(" ");
    t_1534.appendString(strVal_1543);
    return_1531 = t_1534.accumulated;
  }
  return return_1531;
}
/**
 * @param {Array<OrderClause>} clauses_1545
 * @returns {SqlFragment_1473}
 */
function orderBySql_1544(clauses_1545) {
  let t_1546;
  let t_1547;
  let t_1548;
  let t_1549;
  let t_1550;
  let t_1551;
  let t_1552;
  let t_1553;
  let t_1554;
  let t_1555 = listedGet_1256(clauses_1545, 0).field;
  const first_1556 = safeSql_1506(t_1555);
  let firstDir_1557;
  if (listedGet_1256(clauses_1545, 0).direction === "desc") {
    t_1546 = safeSql_1506(" DESC");
    firstDir_1557 = t_1546;
  } else {
    t_1547 = safeSql_1506(" ASC");
    firstDir_1557 = t_1547;
  }
  let t_1558 = new SqlBuilder_1509();
  t_1558.appendFragment(first_1556);
  t_1558.appendFragment(firstDir_1557);
  let t_1559 = t_1558.accumulated;
  let result_1560 = t_1559;
  let i_1561 = 1;
  while (true) {
    t_1548 = clauses_1545.length;
    if (!(i_1561 < t_1548)) {
      break;
    }
    t_1549 = listedGet_1256(clauses_1545, i_1561).field;
    const col_1562 = safeSql_1506(t_1549);
    if (listedGet_1256(clauses_1545, i_1561).direction === "desc") {
      t_1550 = safeSql_1506(" DESC");
      t_1554 = t_1550;
    } else {
      t_1551 = safeSql_1506(" ASC");
      t_1554 = t_1551;
    }
    const dir_1563 = t_1554;
    t_1552 = new SqlBuilder_1509();
    t_1552.appendFragment(result_1560);
    t_1552.appendSafe(", ");
    t_1552.appendFragment(col_1562);
    t_1552.appendFragment(dir_1563);
    t_1553 = t_1552.accumulated;
    result_1560 = t_1553;
    i_1561 = i_1561 + 1 | 0;
  }
  return result_1560;
}
/**
 * @param {Schema} schema_1564
 * @param {Array<string>} selectFields_1565
 * @param {Array<WhereClause>} whereClauses_1566
 * @param {Array<OrderClause>} orderClauses_1567
 * @param {number} limitValue_1568
 * @param {number} offsetValue_1569
 * @returns {SqlFragment_1473}
 */
export function toSqlQuery(schema_1564, selectFields_1565, whereClauses_1566, orderClauses_1567, limitValue_1568, offsetValue_1569) {
  let t_1570;
  let t_1571;
  let t_1572;
  let t_1573;
  let t_1574;
  let t_1575;
  let t_1576;
  let t_1577;
  let t_1578;
  let t_1579;
  let t_1580;
  let t_1581;
  let t_1582;
  let t_1583;
  function fn_1584(f_1585) {
    return schema_1564.hasField(f_1585);
  }
  const validSelect_1586 = listedFilter_1459(selectFields_1565, fn_1584);
  function fn_1587(c_1588) {
    let t_1589 = c_1588.field;
    return schema_1564.hasField(t_1589);
  }
  const validWhere_1590 = listedFilter_1459(whereClauses_1566, fn_1587);
  function fn_1591(c_1592) {
    let t_1593 = c_1592.field;
    return schema_1564.hasField(t_1593);
  }
  const validOrder_1594 = listedFilter_1459(orderClauses_1567, fn_1591);
  let t_1595 = schema_1564.tableName;
  const table_1596 = safeSql_1506(t_1595);
  const cols_1597 = columnListSql_1510(validSelect_1586);
  let t_1598 = new SqlBuilder_1509();
  t_1598.appendSafe("SELECT ");
  t_1598.appendFragment(cols_1597);
  t_1598.appendSafe(" FROM ");
  t_1598.appendFragment(table_1596);
  let t_1599 = t_1598.accumulated;
  let result_1600 = t_1599;
  if (validWhere_1590.length > 0) {
    t_1570 = listedGet_1256(validWhere_1590, 0);
    t_1571 = whereConditionSql_1528(t_1570, schema_1564);
    let conditions_1601 = t_1571;
    let i_1602 = 1;
    while (true) {
      t_1572 = validWhere_1590.length;
      if (!(i_1602 < t_1572)) {
        break;
      }
      t_1573 = listedGet_1256(validWhere_1590, i_1602);
      const next_1603 = whereConditionSql_1528(t_1573, schema_1564);
      t_1574 = new SqlBuilder_1509();
      t_1574.appendFragment(conditions_1601);
      t_1574.appendSafe(" AND ");
      t_1574.appendFragment(next_1603);
      t_1575 = t_1574.accumulated;
      conditions_1601 = t_1575;
      i_1602 = i_1602 + 1 | 0;
    }
    t_1576 = new SqlBuilder_1509();
    t_1576.appendFragment(result_1600);
    t_1576.appendSafe(" WHERE ");
    t_1576.appendFragment(conditions_1601);
    t_1577 = t_1576.accumulated;
    result_1600 = t_1577;
  }
  if (validOrder_1594.length > 0) {
    const ordering_1604 = orderBySql_1544(validOrder_1594);
    t_1578 = new SqlBuilder_1509();
    t_1578.appendFragment(result_1600);
    t_1578.appendSafe(" ORDER BY ");
    t_1578.appendFragment(ordering_1604);
    t_1579 = t_1578.accumulated;
    result_1600 = t_1579;
  }
  if (limitValue_1568 > 0) {
    t_1580 = new SqlBuilder_1509();
    t_1580.appendFragment(result_1600);
    t_1580.appendSafe(" LIMIT ");
    t_1580.appendInt32(limitValue_1568);
    t_1581 = t_1580.accumulated;
    result_1600 = t_1581;
  }
  if (offsetValue_1569 > 0) {
    t_1582 = new SqlBuilder_1509();
    t_1582.appendFragment(result_1600);
    t_1582.appendSafe(" OFFSET ");
    t_1582.appendInt32(offsetValue_1569);
    t_1583 = t_1582.accumulated;
    result_1600 = t_1583;
  }
  return result_1600;
};
export class TokenType extends type__1230() {
  /** @type {string} */
  #name_1605;
  /** @returns {boolean} */
  get isKeyword() {
    return this.#name_1605 === "keyword";
  }
  /** @returns {boolean} */
  get isType() {
    return this.#name_1605 === "type";
  }
  /** @returns {boolean} */
  get isString() {
    return this.#name_1605 === "string";
  }
  /** @returns {boolean} */
  get isNumber() {
    return this.#name_1605 === "number";
  }
  /** @returns {boolean} */
  get isComment() {
    return this.#name_1605 === "comment";
  }
  /** @returns {boolean} */
  get isOperator() {
    return this.#name_1605 === "operator";
  }
  /** @returns {boolean} */
  get isIdentifier() {
    return this.#name_1605 === "identifier";
  }
  /** @param {string} name_1613 */
  constructor(name_1613) {
    super ();
    this.#name_1605 = name_1613;
    return;
  }
  /** @returns {string} */
  get name() {
    return this.#name_1605;
  }
};
export class Token extends type__1230() {
  /** @type {TokenType} */
  #tokenType_1615;
  /** @type {string} */
  #value_1616;
  /** @returns {string} */
  cssClass() {
    let return_1618;
    const name_1619 = this.#tokenType_1615.name;
    if (name_1619 === "keyword") {
      return_1618 = "kw";
    } else if (name_1619 === "type") {
      return_1618 = "typ";
    } else if (name_1619 === "string") {
      return_1618 = "str";
    } else if (name_1619 === "number") {
      return_1618 = "num";
    } else if (name_1619 === "comment") {
      return_1618 = "cmt";
    } else if (name_1619 === "operator") {
      return_1618 = "op";
    } else {
      return_1618 = "id";
    }
    return return_1618;
  }
  /** @returns {SafeHtml_1624} */
  toHtml() {
    const cls_1621 = this.cssClass();
    let t_1622 = new SafeHtmlBuilder_1623();
    t_1622.appendSafe("\u003cspan class='");
    t_1622.appendString(cls_1621);
    t_1622.appendSafe("'\u003e");
    t_1622.appendString(this.#value_1616);
    t_1622.appendSafe("\u003c/span\u003e");
    return t_1622.accumulated;
  }
  /**
   * @param {{
   *   tokenType: TokenType, value: string
   * }}
   * props
   * @returns {Token}
   */
  static["new"](props) {
    return new Token(props.tokenType, props.value);
  }
  /**
   * @param {TokenType} tokenType_1625
   * @param {string} value_1626
   */
  constructor(tokenType_1625, value_1626) {
    super ();
    this.#tokenType_1615 = tokenType_1625;
    this.#value_1616 = value_1626;
    return;
  }
  /** @returns {TokenType} */
  get tokenType() {
    return this.#tokenType_1615;
  }
  /** @returns {string} */
  get value() {
    return this.#value_1616;
  }
};
/**
 * @param {string} name_1629
 * @param {string} fieldType_1630
 * @param {boolean} primaryKey_1631
 * @param {boolean} nullable_1632
 * @returns {Field}
 */
export function field(name_1629, fieldType_1630, primaryKey_1631, nullable_1632) {
  return new Field(name_1629, fieldType_1630, primaryKey_1631, nullable_1632);
};
/**
 * @param {string} tableName_1633
 * @param {Array<Field>} fields_1634
 * @returns {Schema}
 */
export function schema(tableName_1633, fields_1634) {
  const idField_1635 = new Field("id", "Int", true, false);
  const allFields_1636 = [];
  listBuilderAdd_1311(allFields_1636, idField_1635);
  listBuilderAddAll_1637(allFields_1636, fields_1634);
  let t_1638 = listBuilderToList_1316(allFields_1636);
  return new Schema(tableName_1633, t_1638);
};
/**
 * @param {Schema} schema_1639
 * @param {Map<string, string>} values_1640
 * @returns {SqlFragment_1473}
 */
export function toInsertSql(schema_1639, values_1640) {
  let t_1641;
  let t_1642;
  let t_1643;
  let t_1644;
  let t_1645;
  let t_1646;
  let t_1647;
  let t_1648;
  let t_1649;
  let t_1650;
  let t_1651;
  let t_1652;
  let t_1653;
  let t_1654;
  let t_1655 = schema_1639.tableName;
  const table_1656 = safeSql_1506(t_1655);
  let t_1657 = schema_1639.fields;
  function fn_1658(f_1659) {
    let t_1660 = f_1659.name;
    return values_1640.has(t_1660);
  }
  const fieldList_1661 = listedFilter_1459(t_1657, fn_1658);
  function fn_1662(f_1663) {
    return f_1663.name;
  }
  let t_1664 = listedMap_1207(fieldList_1661, fn_1662);
  const colNames_1665 = columnListSql_1510(t_1664);
  let t_1666 = listedGet_1256(fieldList_1661, 0).name;
  const firstVal_1667 = mappedGetOr_1304(values_1640, t_1666, "");
  if (listedGet_1256(fieldList_1661, 0).fieldType === "Int") {
    let iv_1668;
    try {
      t_1651 = stringToInt32_1346(firstVal_1667);
      iv_1668 = t_1651;
    } catch {
      iv_1668 = 0;
    }
    t_1641 = new SqlBuilder_1509();
    t_1641.appendInt32(iv_1668);
    t_1642 = t_1641.accumulated;
    t_1652 = t_1642;
  } else {
    t_1643 = new SqlBuilder_1509();
    t_1643.appendString(firstVal_1667);
    t_1644 = t_1643.accumulated;
    t_1652 = t_1644;
  }
  let vals_1669 = t_1652;
  let i_1670 = 1;
  while (true) {
    t_1645 = fieldList_1661.length;
    if (!(i_1670 < t_1645)) {
      break;
    }
    t_1646 = listedGet_1256(fieldList_1661, i_1670).name;
    const val_1671 = mappedGetOr_1304(values_1640, t_1646, "");
    if (listedGet_1256(fieldList_1661, i_1670).fieldType === "Int") {
      try {
        t_1653 = stringToInt32_1346(val_1671);
        t_1654 = t_1653;
      } catch {
        t_1654 = 0;
      }
      const iv_1672 = t_1654;
      t_1647 = new SqlBuilder_1509();
      t_1647.appendFragment(vals_1669);
      t_1647.appendSafe(", ");
      t_1647.appendInt32(iv_1672);
      t_1648 = t_1647.accumulated;
      vals_1669 = t_1648;
    } else {
      t_1649 = new SqlBuilder_1509();
      t_1649.appendFragment(vals_1669);
      t_1649.appendSafe(", ");
      t_1649.appendString(val_1671);
      t_1650 = t_1649.accumulated;
      vals_1669 = t_1650;
    }
    i_1670 = i_1670 + 1 | 0;
  }
  let t_1673 = new SqlBuilder_1509();
  t_1673.appendSafe("INSERT INTO ");
  t_1673.appendFragment(table_1656);
  t_1673.appendSafe(" (");
  t_1673.appendFragment(colNames_1665);
  t_1673.appendSafe(") VALUES (");
  t_1673.appendFragment(vals_1669);
  t_1673.appendSafe(")");
  return t_1673.accumulated;
};
export function main() {
  console_1186.log("=== ORMery Demo ===\n");
  const userFields_1832 = Object.freeze([field("name", "String", false, false), field("age", "Int", false, false), field("email", "String", false, true)]);
  const userSchema_1833 = schema("users", userFields_1832);
  let t_1834 = userSchema_1833.describe();
  console_1186.log(t_1834);
  console_1186.log("");
  const store_1835 = new InMemoryStore();
  const rec1_1836 = store_1835.insert("users", mapConstructor_1792(Object.freeze([pairConstructor_1793("name", "Alice"), pairConstructor_1793("age", "25"), pairConstructor_1793("email", "alice@example.com")])));
  const rec2_1837 = store_1835.insert("users", mapConstructor_1792(Object.freeze([pairConstructor_1793("name", "Bob"), pairConstructor_1793("age", "30"), pairConstructor_1793("email", "bob@example.com")])));
  const rec3_1838 = store_1835.insert("users", mapConstructor_1792(Object.freeze([pairConstructor_1793("name", "Charlie"), pairConstructor_1793("age", "17"), pairConstructor_1793("email", "charlie@example.com")])));
  console_1186.log("Inserted 3 users:");
  let t_1839 = rec1_1836.describe();
  console_1186.log("  " + t_1839);
  let t_1840 = rec2_1837.describe();
  console_1186.log("  " + t_1840);
  let t_1841 = rec3_1838.describe();
  console_1186.log("  " + t_1841);
  console_1186.log("");
  console_1186.log("=== In-Memory Queries ===\n");
  console_1186.log("All users:");
  const allUsers_1842 = new Query(userSchema_1833, store_1835).all();
  function fn_1843(u_1844) {
    let t_1845 = u_1844.describe();
    console_1186.log("  " + t_1845);
    return;
  }
  allUsers_1842.forEach(fn_1843);
  console_1186.log("");
  console_1186.log("Adults (age \u003e= 18):");
  const adults_1846 = new Query(userSchema_1833, store_1835).where("age", "\u003e=", "18").all();
  function fn_1847(u_1848) {
    let t_1849 = u_1848.describe();
    console_1186.log("  " + t_1849);
    return;
  }
  adults_1846.forEach(fn_1847);
  console_1186.log("");
  console_1186.log("=== SQL Generation (secure-composition) ===\n");
  const q1_1850 = new Query(userSchema_1833, store_1835);
  let t_1851 = q1_1850.toSql().toString();
  console_1186.log("SELECT all: " + t_1851);
  const q2_1852 = new Query(userSchema_1833, store_1835).select(Object.freeze(["name", "age"])).where("age", "\u003e=", "18").orderBy("age", "desc").limit(10);
  let t_1853 = q2_1852.toSql().toString();
  console_1186.log("Complex:    " + t_1853);
  const bobby_1854 = "Robert'); DROP TABLE users;--";
  const q3_1855 = new Query(userSchema_1833, store_1835).where("name", "=", "Robert'); DROP TABLE users;--");
  let t_1856 = q3_1855.toSql().toString();
  console_1186.log("Injection:  " + t_1856);
  const insertVals_1857 = mapConstructor_1792(Object.freeze([pairConstructor_1793("name", "O'Malley"), pairConstructor_1793("age", "42")]));
  let t_1858 = toInsertSql(userSchema_1833, insertVals_1857).toString();
  console_1186.log("INSERT:     " + t_1858);
  console_1186.log("\n=== Demo Complete ===");
  return;
};
/** @type {Array<string>} */
const temperKeywords_1859 = Object.freeze(["if", "else", "for", "while", "do", "when", "break", "continue", "return", "let", "var", "class", "export", "import", "public", "private", "protected", "throws", "new", "this", "get", "set", "static", "extends", "implements", "true", "false", "null", "bubble", "orelse", "of"]);
/** @type {Array<string>} */
const temperTypes_1860 = Object.freeze(["String", "Int", "Boolean", "List", "Map", "Bubble", "Pair", "Float", "Double", "Byte", "Short", "Long", "Char", "Void", "Record", "Schema", "Field", "Query", "InMemoryStore", "ListBuilder", "MapBuilder", "WhereClause", "OrderClause"]);
/**
 * @param {string} word_1861
 * @returns {TokenType}
 */
export function classifyWord(word_1861) {
  let return_1862;
  fn_1863: {
    const this_1864 = temperKeywords_1859;
    const n_1865 = this_1864.length;
    let i_1866 = 0;
    while (i_1866 < n_1865) {
      const el_1867 = listedGet_1256(this_1864, i_1866);
      i_1866 = i_1866 + 1 | 0;
      const kw_1868 = el_1867;
      if (kw_1868 === word_1861) {
        return_1862 = new TokenType("keyword");
        break fn_1863;
      }
    }
    const this_1869 = temperTypes_1860;
    const n_1870 = this_1869.length;
    let i_1871 = 0;
    while (i_1871 < n_1870) {
      const el_1872 = listedGet_1256(this_1869, i_1871);
      i_1871 = i_1871 + 1 | 0;
      const tp_1873 = el_1872;
      if (tp_1873 === word_1861) {
        return_1862 = new TokenType("type");
        break fn_1863;
      }
    }
    return_1862 = new TokenType("identifier");
  }
  return return_1862;
};
/**
 * @param {string} word_1874
 * @returns {SafeHtml_1624}
 */
export function highlightWord(word_1874) {
  let return_1875;
  let t_1876;
  fn_1877: {
    if (word_1874 === "") {
      t_1876 = new SafeHtmlBuilder_1623();
      return_1875 = t_1876.accumulated;
      break fn_1877;
    }
    const tokenType_1878 = classifyWord(word_1874);
    const token_1879 = new Token(tokenType_1878, word_1874);
    return_1875 = token_1879.toHtml();
  }
  return return_1875;
};
/**
 * @param {string} line_1880
 * @returns {SafeHtml_1624}
 */
export function highlightLine(line_1880) {
  let return_1881;
  let t_1882;
  let t_1883;
  let t_1884;
  let t_1885;
  let t_1886;
  fn_1887: {
    const words_1888 = stringSplit_1889(line_1880, " ");
    if (words_1888.length === 0) {
      t_1882 = new SafeHtmlBuilder_1623();
      return_1881 = t_1882.accumulated;
      break fn_1887;
    }
    t_1883 = highlightWord(listedGet_1256(words_1888, 0));
    let result_1890 = t_1883;
    let i_1891 = 1;
    while (true) {
      t_1884 = words_1888.length;
      if (!(i_1891 < t_1884)) {
        break;
      }
      const word_1892 = highlightWord(listedGet_1256(words_1888, i_1891));
      t_1885 = new SafeHtmlBuilder_1623();
      t_1885.appendSafeHtml(result_1890);
      t_1885.appendSafe(" ");
      t_1885.appendSafeHtml(word_1892);
      t_1886 = t_1885.accumulated;
      result_1890 = t_1886;
      i_1891 = i_1891 + 1 | 0;
    }
    return_1881 = result_1890;
  }
  return return_1881;
};
/**
 * @param {string} source_1893
 * @returns {SafeHtml_1624}
 */
export function highlightSource(source_1893) {
  let return_1894;
  let t_1895;
  let t_1896;
  let t_1897;
  let t_1898;
  let t_1899;
  fn_1900: {
    const lines_1901 = stringSplit_1889(source_1893, "\n");
    if (lines_1901.length === 0) {
      t_1895 = new SafeHtmlBuilder_1623();
      return_1894 = t_1895.accumulated;
      break fn_1900;
    }
    t_1896 = highlightLine(listedGet_1256(lines_1901, 0));
    let result_1902 = t_1896;
    let i_1903 = 1;
    while (true) {
      t_1897 = lines_1901.length;
      if (!(i_1903 < t_1897)) {
        break;
      }
      const line_1904 = highlightLine(listedGet_1256(lines_1901, i_1903));
      t_1898 = new SafeHtmlBuilder_1623();
      t_1898.appendSafeHtml(result_1902);
      t_1898.appendSafe("\\n");
      t_1898.appendSafeHtml(line_1904);
      t_1899 = t_1898.accumulated;
      result_1902 = t_1899;
      i_1903 = i_1903 + 1 | 0;
    }
    return_1894 = result_1902;
  }
  return return_1894;
};
/**
 * @param {string} source_1905
 * @returns {SafeHtml_1624}
 */
export function highlightBlock(source_1905) {
  const highlighted_1906 = highlightSource(source_1905);
  let t_1907 = new SafeHtmlBuilder_1623();
  t_1907.appendSafe("\u003cpre class='temper-code'\u003e\u003ccode\u003e");
  t_1907.appendSafeHtml(highlighted_1906);
  t_1907.appendSafe("\u003c/code\u003e\u003c/pre\u003e");
  return t_1907.accumulated;
};
