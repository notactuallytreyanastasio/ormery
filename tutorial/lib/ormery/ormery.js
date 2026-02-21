import {
  globalConsole as globalConsole__1, type as type__18, cmpString as cmpString__288, listedGet as listedGet_29, listedMap as listedMap_51, listedJoin as listedJoin_60, mappedGet as mappedGet_68, mappedGetOr as mappedGetOr_72, stringToInt32 as stringToInt32_78, mappedToListWith as mappedToListWith_84, mapBuilderConstructor as mapBuilderConstructor_92, mapBuilderSet as mapBuilderSet_98, mappedToMapBuilder as mappedToMapBuilder_104, mappedToMap as mappedToMap_106, listBuilderAdd as listBuilderAdd_109, listBuilderToList as listBuilderToList_114, panic as panic_197, cmpGeneric as cmpGeneric_238, listedFilter as listedFilter_251, listedSorted as listedSorted_257, listedSlice as listedSlice_261, listBuilderAddAll as listBuilderAddAll_274, pairConstructor as pairConstructor_295, mapConstructor as mapConstructor_294
} from "@temperlang/core";
/** @type {Console_2} */
const console_0 = globalConsole__1;
export class Field extends type__18() {
  /** @type {string} */
  #name_3;
  /** @type {string} */
  #fieldType_4;
  /** @type {boolean} */
  #primaryKey_5;
  /** @type {boolean} */
  #nullable_6;
  /** @returns {string} */
  get description() {
    let pk_8;
    if (this.#primaryKey_5) {
      pk_8 = " (PK)";
    } else {
      pk_8 = "";
    }
    let null_9;
    if (this.#nullable_6) {
      null_9 = " (nullable)";
    } else {
      null_9 = "";
    }
    return String(this.#name_3) + ": " + this.#fieldType_4 + pk_8 + null_9;
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
   * @param {string} name_10
   * @param {string} fieldType_11
   * @param {boolean} primaryKey_12
   * @param {boolean} nullable_13
   */
  constructor(name_10, fieldType_11, primaryKey_12, nullable_13) {
    super ();
    this.#name_3 = name_10;
    this.#fieldType_4 = fieldType_11;
    this.#primaryKey_5 = primaryKey_12;
    this.#nullable_6 = nullable_13;
    return;
  }
  /** @returns {string} */
  get name() {
    return this.#name_3;
  }
  /** @returns {string} */
  get fieldType() {
    return this.#fieldType_4;
  }
  /** @returns {boolean} */
  get primaryKey() {
    return this.#primaryKey_5;
  }
  /** @returns {boolean} */
  get nullable() {
    return this.#nullable_6;
  }
};
export class Schema extends type__18() {
  /** @type {string} */
  #tableName_19;
  /** @type {Array<Field>} */
  #fields_20;
  /**
   * @param {string} name_22
   * @returns {Field}
   */
  getField(name_22) {
    let return_23;
    fn_24: {
      const this_25 = this.#fields_20;
      const n_26 = this_25.length;
      let i_27 = 0;
      while (i_27 < n_26) {
        const el_28 = listedGet_29(this_25, i_27);
        i_27 = i_27 + 1 | 0;
        const field_30 = el_28;
        if (field_30.name === name_22) {
          return_23 = field_30;
          break fn_24;
        }
      }
      throw Error();
    }
    return return_23;
  }
  /**
   * @param {string} name_32
   * @returns {boolean}
   */
  hasField(name_32) {
    let return_33;
    fn_34: {
      const this_35 = this.#fields_20;
      const n_36 = this_35.length;
      let i_37 = 0;
      while (i_37 < n_36) {
        const el_38 = listedGet_29(this_35, i_37);
        i_37 = i_37 + 1 | 0;
        const field_39 = el_38;
        if (field_39.name === name_32) {
          return_33 = true;
          break fn_34;
        }
      }
      return_33 = false;
    }
    return return_33;
  }
  /** @returns {Field} */
  get primaryKeyField() {
    let return_41;
    fn_42: {
      const this_43 = this.#fields_20;
      const n_44 = this_43.length;
      let i_45 = 0;
      while (i_45 < n_44) {
        const el_46 = listedGet_29(this_43, i_45);
        i_45 = i_45 + 1 | 0;
        const field_47 = el_46;
        if (field_47.primaryKey) {
          return_41 = field_47;
          break fn_42;
        }
      }
      throw Error();
    }
    return return_41;
  }
  /** @returns {Array<string>} */
  get fieldNames() {
    function fn_49(f_50) {
      return f_50.name;
    }
    return listedMap_51(this.#fields_20, fn_49);
  }
  /** @returns {string} */
  describe() {
    const header_53 = "Schema: " + this.#tableName_19 + "\n";
    function fn_54(f_55) {
      return "  - " + f_55.description;
    }
    let t_56 = listedMap_51(this.#fields_20, fn_54);
    function fn_57(s_58) {
      return s_58;
    }
    const fieldList_59 = listedJoin_60(t_56, "\n", fn_57);
    return String(header_53) + fieldList_59;
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
   * @param {string} tableName_61
   * @param {Array<Field>} fields_62
   */
  constructor(tableName_61, fields_62) {
    super ();
    this.#tableName_19 = tableName_61;
    this.#fields_20 = fields_62;
    return;
  }
  /** @returns {string} */
  get tableName() {
    return this.#tableName_19;
  }
  /** @returns {Array<Field>} */
  get fields() {
    return this.#fields_20;
  }
};
export class Record extends type__18() {
  /** @type {Map<string, string>} */
  #data_65;
  /**
   * @param {string} field_67
   * @returns {string}
   */
  get(field_67) {
    return mappedGet_68(this.#data_65, field_67);
  }
  /**
   * @param {string} field_70
   * @param {string} fallback_71
   * @returns {string}
   */
  getOr(field_70, fallback_71) {
    return mappedGetOr_72(this.#data_65, field_70, fallback_71);
  }
  /**
   * @param {string} field_74
   * @returns {boolean}
   */
  has(field_74) {
    return this.#data_65.has(field_74);
  }
  /** @returns {number} */
  get id() {
    let return_76;
    let idStr_77;
    idStr_77 = mappedGet_68(this.#data_65, "id");
    try {
      return_76 = stringToInt32_78(idStr_77);
    } catch {
      throw Error();
    }
    return return_76;
  }
  /** @returns {string} */
  describe() {
    function fn_80(k_81, v_82) {
      return String(k_81) + ": " + v_82;
    }
    const pairs_83 = mappedToListWith_84(this.#data_65, fn_80);
    function fn_85(s_86) {
      return s_86;
    }
    return listedJoin_60(pairs_83, ", ", fn_85);
  }
  /** @param {Map<string, string>} data_87 */
  constructor(data_87) {
    super ();
    this.#data_65 = data_87;
    return;
  }
  /** @returns {Map<string, string>} */
  get data() {
    return this.#data_65;
  }
};
export class InMemoryStore extends type__18() {
  /** @type {Map<string, Array<Record>>} */
  #tables_89;
  /** @type {Map<string, number>} */
  #nextIds_90;
  constructor() {
    super ();
    let t_91 = mapBuilderConstructor_92();
    this.#tables_89 = t_91;
    let t_93 = mapBuilderConstructor_92();
    this.#nextIds_90 = t_93;
    return;
  }
  /** @param {string} tableName_96 */
  #ensureTable_95(tableName_96) {
    let t_97;
    if (! this.#tables_89.has(tableName_96)) {
      t_97 = [];
      mapBuilderSet_98(this.#tables_89, tableName_96, t_97);
      mapBuilderSet_98(this.#nextIds_90, tableName_96, 1);
    }
    return;
  }
  /**
   * @param {string} tableName_100
   * @param {Map<string, string>} data_101
   * @returns {Record}
   */
  insert(tableName_100, data_101) {
    this.#ensureTable_95(tableName_100);
    const id_102 = mappedGetOr_72(this.#nextIds_90, tableName_100, 1);
    mapBuilderSet_98(this.#nextIds_90, tableName_100, id_102 + 1 | 0);
    const dataBuilder_103 = mappedToMapBuilder_104(data_101);
    mapBuilderSet_98(dataBuilder_103, "id", id_102.toString());
    const record_105 = new Record(mappedToMap_106(dataBuilder_103));
    let t_107 = [];
    const table_108 = mappedGetOr_72(this.#tables_89, tableName_100, t_107);
    listBuilderAdd_109(table_108, record_105);
    return record_105;
  }
  /**
   * @param {string} tableName_111
   * @returns {Array<Record>}
   */
  all(tableName_111) {
    this.#ensureTable_95(tableName_111);
    let t_112 = [];
    const table_113 = mappedGetOr_72(this.#tables_89, tableName_111, t_112);
    return listBuilderToList_114(table_113);
  }
  /**
   * @param {string} tableName_116
   * @param {number} id_117
   * @returns {Record}
   */
  get(tableName_116, id_117) {
    let return_118;
    let t_119;
    fn_120: {
      this.#ensureTable_95(tableName_116);
      t_119 = [];
      const table_121 = mappedGetOr_72(this.#tables_89, tableName_116, t_119);
      const this_122 = listBuilderToList_114(table_121);
      const n_123 = this_122.length;
      let i_124 = 0;
      while (i_124 < n_123) {
        const el_125 = listedGet_29(this_122, i_124);
        i_124 = i_124 + 1 | 0;
        const record_126 = el_125;
        let recordId_127;
        try {
          recordId_127 = record_126.id;
        } catch {
          throw Error();
        }
        if (recordId_127 === id_117) {
          return_118 = record_126;
          break fn_120;
        }
      }
      throw Error();
    }
    return return_118;
  }
  /**
   * @param {string} tableName_129
   * @returns {number}
   */
  count(tableName_129) {
    this.#ensureTable_95(tableName_129);
    let t_130 = [];
    const table_131 = mappedGetOr_72(this.#tables_89, tableName_129, t_130);
    return table_131.length;
  }
};
export class WhereClause extends type__18() {
  /** @type {string} */
  #field_132;
  /** @type {string} */
  #operator_133;
  /** @type {string} */
  #value_134;
  /** @returns {string} */
  describe() {
    return String(this.#field_132) + " " + this.#operator_133 + " " + this.#value_134;
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
   * @param {string} field_136
   * @param {string} operator_137
   * @param {string} value_138
   */
  constructor(field_136, operator_137, value_138) {
    super ();
    this.#field_132 = field_136;
    this.#operator_133 = operator_137;
    this.#value_134 = value_138;
    return;
  }
  /** @returns {string} */
  get field() {
    return this.#field_132;
  }
  /** @returns {string} */
  get operator() {
    return this.#operator_133;
  }
  /** @returns {string} */
  get value() {
    return this.#value_134;
  }
};
export class OrderClause extends type__18() {
  /** @type {string} */
  #field_142;
  /** @type {string} */
  #direction_143;
  /** @returns {string} */
  describe() {
    return String(this.#field_142) + " " + this.#direction_143;
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
   * @param {string} field_145
   * @param {string} direction_146
   */
  constructor(field_145, direction_146) {
    super ();
    this.#field_142 = field_145;
    this.#direction_143 = direction_146;
    return;
  }
  /** @returns {string} */
  get field() {
    return this.#field_142;
  }
  /** @returns {string} */
  get direction() {
    return this.#direction_143;
  }
};
export class Query extends type__18() {
  /** @type {Schema} */
  #schema_149;
  /** @type {InMemoryStore} */
  #store_150;
  /** @type {Array<WhereClause>} */
  #whereClauses_151;
  /** @type {Array<string>} */
  #selectFields_152;
  /** @type {Array<OrderClause>} */
  #orderByClauses_153;
  /** @type {number} */
  #limitValue_154;
  /** @type {number} */
  #offsetValue_155;
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
   * @param {Schema} schema_156
   * @param {InMemoryStore} store_157
   */
  constructor(schema_156, store_157) {
    super ();
    this.#schema_149 = schema_156;
    this.#store_150 = store_157;
    let t_158 = [];
    this.#whereClauses_151 = t_158;
    let t_159 = Object.freeze([]);
    this.#selectFields_152 = t_159;
    let t_160 = [];
    this.#orderByClauses_153 = t_160;
    this.#limitValue_154 = -1;
    this.#offsetValue_155 = 0;
    return;
  }
  /**
   * @param {string} field_162
   * @param {string} operator_163
   * @param {string} value_164
   * @returns {Query}
   */
  where(field_162, operator_163, value_164) {
    let t_165 = new WhereClause(field_162, operator_163, value_164);
    listBuilderAdd_109(this.#whereClauses_151, t_165);
    return this;
  }
  /**
   * @param {Array<string>} fields_167
   * @returns {Query}
   */
  select(fields_167) {
    this.#selectFields_152 = fields_167;
    return this;
  }
  /**
   * @param {string} field_169
   * @param {string} direction_170
   * @returns {Query}
   */
  orderBy(field_169, direction_170) {
    let t_171 = new OrderClause(field_169, direction_170);
    listBuilderAdd_109(this.#orderByClauses_153, t_171);
    return this;
  }
  /**
   * @param {number} n_173
   * @returns {Query}
   */
  limit(n_173) {
    this.#limitValue_154 = n_173;
    return this;
  }
  /**
   * @param {number} n_175
   * @returns {Query}
   */
  offset(n_175) {
    this.#offsetValue_155 = n_175;
    return this;
  }
  /**
   * @param {Record} record_178
   * @returns {boolean}
   */
  #matchesWhere_177(record_178) {
    let return_179;
    let t_180;
    let t_181;
    let t_182;
    let t_183;
    let t_184;
    let t_185;
    let t_186;
    let t_187;
    fn_188: {
      const this_189 = listBuilderToList_114(this.#whereClauses_151);
      const n_190 = this_189.length;
      let i_191 = 0;
      while (i_191 < n_190) {
        const el_192 = listedGet_29(this_189, i_191);
        i_191 = i_191 + 1 | 0;
        const clause_193 = el_192;
        let t_194;
        t_180 = clause_193.field;
        const recordValue_195 = record_178.getOr(t_180, "");
        t_181 = clause_193.field;
        if (! this.#schema_149.hasField(t_181)) {
          return_179 = false;
          break fn_188;
        }
        let fieldInfo_196;
        try {
          t_182 = clause_193.field;
          t_194 = this.#schema_149.getField(t_182);
          fieldInfo_196 = t_194;
        } catch {
          fieldInfo_196 = panic_197();
        }
        const fieldType_198 = fieldInfo_196.fieldType;
        let matches_199;
        if (fieldType_198 === "Int") {
          t_183 = clause_193.operator;
          t_184 = clause_193.value;
          t_185 = compareInt_200(recordValue_195, t_183, t_184);
          matches_199 = t_185;
        } else if (fieldType_198 === "String") {
          t_186 = clause_193.operator;
          t_187 = clause_193.value;
          matches_199 = compareString_201(recordValue_195, t_186, t_187);
        } else {
          matches_199 = false;
        }
        if (! matches_199) {
          return_179 = false;
          break fn_188;
        }
      }
      return_179 = true;
    }
    return return_179;
  }
  /**
   * @param {Record} record_204
   * @returns {Record}
   */
  #projectRecord_203(record_204) {
    let return_205;
    let t_206;
    fn_207: {
      if (this.#selectFields_152.length === 0) {
        return_205 = record_204;
        break fn_207;
      }
      const builder_208 = mapBuilderConstructor_92();
      function fn_209(fieldName_210) {
        const value_211 = record_204.getOr(fieldName_210, "");
        mapBuilderSet_98(builder_208, fieldName_210, value_211);
        return;
      }
      this.#selectFields_152.forEach(fn_209);
      t_206 = mappedToMap_106(builder_208);
      return_205 = new Record(t_206);
    }
    return return_205;
  }
  /**
   * @param {Record} a_214
   * @param {Record} b_215
   * @param {Array<OrderClause>} orderClauses_216
   * @returns {number}
   */
  #compareRecords_213(a_214, b_215, orderClauses_216) {
    let return_217;
    let t_218;
    let t_219;
    let t_220;
    let t_221;
    fn_222: {
      const this_223 = orderClauses_216;
      const n_224 = this_223.length;
      let i_225 = 0;
      while (i_225 < n_224) {
        const el_226 = listedGet_29(this_223, i_225);
        i_225 = i_225 + 1 | 0;
        const clause_227 = el_226;
        let t_228;
        let t_229;
        let t_230;
        t_218 = clause_227.field;
        const aVal_231 = a_214.getOr(t_218, "");
        t_219 = clause_227.field;
        const bVal_232 = b_215.getOr(t_219, "");
        t_220 = clause_227.field;
        if (! this.#schema_149.hasField(t_220)) {
          continue;
        }
        let fieldInfo_233;
        try {
          t_221 = clause_227.field;
          t_228 = this.#schema_149.getField(t_221);
          fieldInfo_233 = t_228;
        } catch {
          fieldInfo_233 = panic_197();
        }
        const fieldType_234 = fieldInfo_233.fieldType;
        let cmp_235;
        if (fieldType_234 === "Int") {
          let aInt_236;
          try {
            t_229 = stringToInt32_78(aVal_231);
            aInt_236 = t_229;
          } catch {
            aInt_236 = 0;
          }
          let bInt_237;
          try {
            t_230 = stringToInt32_78(bVal_232);
            bInt_237 = t_230;
          } catch {
            bInt_237 = 0;
          }
          cmp_235 = cmpGeneric_238(aInt_236, bInt_237);
        } else if (fieldType_234 === "String") {
          cmp_235 = cmpGeneric_238(aVal_231, bVal_232);
        } else {
          cmp_235 = 0;
        }
        if (cmp_235 !== 0) {
          if (clause_227.direction === "desc") {
            return_217 = - cmp_235 | 0;
          } else {
            return_217 = cmp_235;
          }
          break fn_222;
        }
      }
      return_217 = 0;
    }
    return return_217;
  }
  /** @returns {Array<Record>} */
  all() {
    const this249 = this;
    let t_240;
    let t_241;
    let t_242;
    let t_243;
    let t_244;
    let t_245 = this.#schema_149.tableName;
    const allRecords_246 = this.#store_150.all(t_245);
    function fn_247(r_248) {
      return this249.#matchesWhere_177(r_248);
    }
    const filtered_250 = listedFilter_251(allRecords_246, fn_247);
    let sorted_252;
    if (this.#orderByClauses_153.length > 0) {
      const clauses_253 = listBuilderToList_114(this.#orderByClauses_153);
      function fn_254(a_255, b_256) {
        return this249.#compareRecords_213(a_255, b_256, clauses_253);
      }
      t_240 = listedSorted_257(filtered_250, fn_254);
      sorted_252 = t_240;
    } else {
      sorted_252 = filtered_250;
    }
    let sliced_258;
    if (this.#limitValue_154 > 0) {
      const start_259 = this.#offsetValue_155;
      const end_260 = this.#offsetValue_155 + this.#limitValue_154 | 0;
      t_241 = listedSlice_261(sorted_252, start_259, end_260);
      sliced_258 = t_241;
    } else if (this.#offsetValue_155 > 0) {
      t_243 = this.#offsetValue_155;
      t_242 = sorted_252.length;
      t_244 = listedSlice_261(sorted_252, t_243, t_242);
      sliced_258 = t_244;
    } else {
      sliced_258 = sorted_252;
    }
    function fn_262(r_263) {
      return this249.#projectRecord_203(r_263);
    }
    return listedMap_51(sliced_258, fn_262);
  }
  /** @returns {Schema} */
  get schema() {
    return this.#schema_149;
  }
  /** @returns {InMemoryStore} */
  get store() {
    return this.#store_150;
  }
};
/**
 * @param {string} name_266
 * @param {string} fieldType_267
 * @param {boolean} primaryKey_268
 * @param {boolean} nullable_269
 * @returns {Field}
 */
export function field(name_266, fieldType_267, primaryKey_268, nullable_269) {
  return new Field(name_266, fieldType_267, primaryKey_268, nullable_269);
};
/**
 * @param {string} tableName_270
 * @param {Array<Field>} fields_271
 * @returns {Schema}
 */
export function schema(tableName_270, fields_271) {
  const idField_272 = new Field("id", "Int", true, false);
  const allFields_273 = [];
  listBuilderAdd_109(allFields_273, idField_272);
  listBuilderAddAll_274(allFields_273, fields_271);
  let t_275 = listBuilderToList_114(allFields_273);
  return new Schema(tableName_270, t_275);
};
/**
 * @param {string} recordValue_276
 * @param {string} operator_277
 * @param {string} clauseValue_278
 * @returns {boolean}
 */
function compareInt_200(recordValue_276, operator_277, clauseValue_278) {
  let return_279;
  let t_280;
  let t_281;
  let rv_282;
  try {
    t_280 = stringToInt32_78(recordValue_276);
    rv_282 = t_280;
  } catch {
    rv_282 = 0;
  }
  let cv_283;
  try {
    t_281 = stringToInt32_78(clauseValue_278);
    cv_283 = t_281;
  } catch {
    cv_283 = 0;
  }
  if (operator_277 === "==") {
    return_279 = rv_282 === cv_283;
  } else if (operator_277 === "!=") {
    return_279 = rv_282 !== cv_283;
  } else if (operator_277 === "\u003e") {
    return_279 = rv_282 > cv_283;
  } else if (operator_277 === "\u003c") {
    return_279 = rv_282 < cv_283;
  } else if (operator_277 === "\u003e=") {
    return_279 = rv_282 >= cv_283;
  } else if (operator_277 === "\u003c=") {
    return_279 = rv_282 <= cv_283;
  } else {
    return_279 = false;
  }
  return return_279;
}
/**
 * @param {string} recordValue_284
 * @param {string} operator_285
 * @param {string} clauseValue_286
 * @returns {boolean}
 */
function compareString_201(recordValue_284, operator_285, clauseValue_286) {
  let return_287;
  if (operator_285 === "==") {
    return_287 = recordValue_284 === clauseValue_286;
  } else if (operator_285 === "!=") {
    return_287 = recordValue_284 !== clauseValue_286;
  } else if (operator_285 === "\u003e") {
    return_287 = cmpString__288(recordValue_284, clauseValue_286) > 0;
  } else if (operator_285 === "\u003c") {
    return_287 = cmpString__288(recordValue_284, clauseValue_286) < 0;
  } else if (operator_285 === "\u003e=") {
    return_287 = cmpString__288(recordValue_284, clauseValue_286) >= 0;
  } else if (operator_285 === "\u003c=") {
    return_287 = cmpString__288(recordValue_284, clauseValue_286) <= 0;
  } else {
    return_287 = false;
  }
  return return_287;
}
export function main() {
  console_0.log("=== ORMery Demo ===\n");
  const userFields_289 = Object.freeze([field("name", "String", false, false), field("age", "Int", false, false), field("email", "String", false, true)]);
  const userSchema_290 = schema("users", userFields_289);
  let t_291 = userSchema_290.describe();
  console_0.log(t_291);
  console_0.log("");
  const store_292 = new InMemoryStore();
  const rec1_293 = store_292.insert("users", mapConstructor_294(Object.freeze([pairConstructor_295("name", "Alice"), pairConstructor_295("age", "25"), pairConstructor_295("email", "alice@example.com")])));
  const rec2_296 = store_292.insert("users", mapConstructor_294(Object.freeze([pairConstructor_295("name", "Bob"), pairConstructor_295("age", "30"), pairConstructor_295("email", "bob@example.com")])));
  const rec3_297 = store_292.insert("users", mapConstructor_294(Object.freeze([pairConstructor_295("name", "Charlie"), pairConstructor_295("age", "17"), pairConstructor_295("email", "charlie@example.com")])));
  console_0.log("Inserted 3 users:");
  let t_298 = rec1_293.describe();
  console_0.log("  " + t_298);
  let t_299 = rec2_296.describe();
  console_0.log("  " + t_299);
  let t_300 = rec3_297.describe();
  console_0.log("  " + t_300);
  console_0.log("");
  console_0.log("=== Query 1: All users ===");
  const allUsers_301 = new Query(userSchema_290, store_292).all();
  function fn_302(u_303) {
    let t_304 = u_303.describe();
    console_0.log("  " + t_304);
    return;
  }
  allUsers_301.forEach(fn_302);
  console_0.log("");
  console_0.log("=== Query 2: Users age \u003e= 18 ===");
  const adults_305 = new Query(userSchema_290, store_292).where("age", "\u003e=", "18").all();
  function fn_306(u_307) {
    let t_308 = u_307.describe();
    console_0.log("  " + t_308);
    return;
  }
  adults_305.forEach(fn_306);
  console_0.log("");
  console_0.log("=== Query 3: Just names and ages ===");
  const namesAndAges_309 = new Query(userSchema_290, store_292).select(Object.freeze(["name", "age"])).all();
  function fn_310(u_311) {
    let t_312 = u_311.describe();
    console_0.log("  " + t_312);
    return;
  }
  namesAndAges_309.forEach(fn_310);
  console_0.log("");
  console_0.log("=== Query 4: Adult names only ===");
  const adultNames_313 = new Query(userSchema_290, store_292).where("age", "\u003e=", "18").select(Object.freeze(["name"])).all();
  function fn_314(u_315) {
    let t_316 = u_315.describe();
    console_0.log("  " + t_316);
    return;
  }
  adultNames_313.forEach(fn_314);
  console_0.log("");
  console_0.log("=== Query 5: Age \u003e= 18 AND age \u003c 30 ===");
  const youngAdults_317 = new Query(userSchema_290, store_292).where("age", "\u003e=", "18").where("age", "\u003c", "30").all();
  function fn_318(u_319) {
    let t_320 = u_319.describe();
    console_0.log("  " + t_320);
    return;
  }
  youngAdults_317.forEach(fn_318);
  console_0.log("");
  console_0.log("=== Query 6: All users ordered by age (asc) ===");
  const byAgeAsc_321 = new Query(userSchema_290, store_292).orderBy("age", "asc").all();
  function fn_322(u_323) {
    let t_324 = u_323.describe();
    console_0.log("  " + t_324);
    return;
  }
  byAgeAsc_321.forEach(fn_322);
  console_0.log("");
  console_0.log("=== Query 7: All users ordered by age (desc) ===");
  const byAgeDesc_325 = new Query(userSchema_290, store_292).orderBy("age", "desc").all();
  function fn_326(u_327) {
    let t_328 = u_327.describe();
    console_0.log("  " + t_328);
    return;
  }
  byAgeDesc_325.forEach(fn_326);
  console_0.log("");
  console_0.log("=== Query 8: First 2 users (limit) ===");
  const first2_329 = new Query(userSchema_290, store_292).orderBy("id", "asc").limit(2).all();
  function fn_330(u_331) {
    let t_332 = u_331.describe();
    console_0.log("  " + t_332);
    return;
  }
  first2_329.forEach(fn_330);
  console_0.log("");
  console_0.log("=== Query 9: Skip first user (offset) ===");
  const skip1_333 = new Query(userSchema_290, store_292).orderBy("id", "asc").offset(1).all();
  function fn_334(u_335) {
    let t_336 = u_335.describe();
    console_0.log("  " + t_336);
    return;
  }
  skip1_333.forEach(fn_334);
  console_0.log("");
  console_0.log("=== Query 10: Page 2, size 1 (offset + limit) ===");
  const page2_337 = new Query(userSchema_290, store_292).orderBy("id", "asc").offset(1).limit(1).all();
  function fn_338(u_339) {
    let t_340 = u_339.describe();
    console_0.log("  " + t_340);
    return;
  }
  page2_337.forEach(fn_338);
  console_0.log("");
  console_0.log("=== Query 11: Oldest adult (where + orderBy + limit) ===");
  const oldestAdult_341 = new Query(userSchema_290, store_292).where("age", "\u003e=", "18").orderBy("age", "desc").limit(1).all();
  function fn_342(u_343) {
    let t_344 = u_343.describe();
    console_0.log("  " + t_344);
    return;
  }
  oldestAdult_341.forEach(fn_342);
  console_0.log("");
  console_0.log("=== Demo Complete ===");
  return;
};
