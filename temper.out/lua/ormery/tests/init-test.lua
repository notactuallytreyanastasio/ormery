local temper = require('temper-core');
local SqlBuilder, SafeHtmlBuilder, schema, field, InMemoryStore, Query, toInsertSql, toSqlQuery, WhereClause, OrderClause, isValidIdentifier, local_1016, local_1017, exports;
SqlBuilder = temper.import('ormery/sql', 'SqlBuilder');
SafeHtmlBuilder = temper.import('ormery/html', 'SafeHtmlBuilder');
schema = temper.import('ormery/init', 'schema');
field = temper.import('ormery/init', 'field');
InMemoryStore = temper.import('ormery/init', 'InMemoryStore');
Query = temper.import('ormery/init', 'Query');
toInsertSql = temper.import('ormery/init', 'toInsertSql');
toSqlQuery = temper.import('ormery/init', 'toSqlQuery');
WhereClause = temper.import('ormery/init', 'WhereClause');
OrderClause = temper.import('ormery/init', 'OrderClause');
isValidIdentifier = temper.import('ormery/init', 'isValidIdentifier');
local_1016 = (unpack or table.unpack);
local_1017 = require('luaunit');
local_1017.FAILURE_PREFIX = temper.test_failure_prefix;
Test_ = {};
Test_.test_toSqlSelectAll__830 = function()
  temper.test('toSql: select all', function(test_897)
    local s__527, store__528, q__529, actual_898, t_899, fn__3710;
    s__527 = schema('users', temper.listof(field('name', 'String', false, false)));
    store__528 = InMemoryStore();
    q__529 = Query(s__527, store__528);
    actual_898 = q__529:toSql():toString();
    t_899 = temper.str_eq(actual_898, 'SELECT * FROM users');
    fn__3710 = function()
      return temper.concat('expected q.toSql().toString() == (', 'SELECT * FROM users', ') not (', actual_898, ')');
    end;
    temper.test_assert(test_897, t_899, fn__3710);
    return nil;
  end);
end;
Test_.test_toSqlSelectColumns__833 = function()
  temper.test('toSql: select columns', function(test_900)
    local s__531, store__532, q__533, actual_901, t_902, fn__3700;
    s__531 = schema('users', temper.listof(field('name', 'String', false, false), field('age', 'Int', false, false)));
    store__532 = InMemoryStore();
    q__533 = Query(s__531, store__532):select(temper.listof('name', 'age'));
    actual_901 = q__533:toSql():toString();
    t_902 = temper.str_eq(actual_901, 'SELECT name, age FROM users');
    fn__3700 = function()
      return temper.concat('expected q.toSql().toString() == (', 'SELECT name, age FROM users', ') not (', actual_901, ')');
    end;
    temper.test_assert(test_900, t_902, fn__3700);
    return nil;
  end);
end;
Test_.test_toSqlWhereString__836 = function()
  temper.test('toSql: where string', function(test_903)
    local s__535, store__536, q__537, actual_904, t_905, fn__3690;
    s__535 = schema('users', temper.listof(field('name', 'String', false, false)));
    store__536 = InMemoryStore();
    q__537 = Query(s__535, store__536):where('name', '=', 'Alice');
    actual_904 = q__537:toSql():toString();
    t_905 = temper.str_eq(actual_904, "SELECT * FROM users WHERE name = 'Alice'");
    fn__3690 = function()
      return temper.concat('expected q.toSql().toString() == (', "SELECT * FROM users WHERE name = 'Alice'", ') not (', actual_904, ')');
    end;
    temper.test_assert(test_903, t_905, fn__3690);
    return nil;
  end);
end;
Test_.test_toSqlWhereInt__839 = function()
  temper.test('toSql: where int', function(test_906)
    local s__539, store__540, q__541, actual_907, t_908, fn__3680;
    s__539 = schema('users', temper.listof(field('age', 'Int', false, false)));
    store__540 = InMemoryStore();
    q__541 = Query(s__539, store__540):where('age', '>=', '18');
    actual_907 = q__541:toSql():toString();
    t_908 = temper.str_eq(actual_907, 'SELECT * FROM users WHERE age >= 18');
    fn__3680 = function()
      return temper.concat('expected q.toSql().toString() == (', 'SELECT * FROM users WHERE age >= 18', ') not (', actual_907, ')');
    end;
    temper.test_assert(test_906, t_908, fn__3680);
    return nil;
  end);
end;
Test_.test_toSqlSqlInjectionBlocked__842 = function()
  temper.test('toSql: SQL injection blocked', function(test_909)
    local s__543, store__544, bobby__545, q__546, result__547, actual_910, t_911, fn__3670;
    s__543 = schema('users', temper.listof(field('name', 'String', false, false)));
    store__544 = InMemoryStore();
    bobby__545 = "Robert'); DROP TABLE users;--";
    q__546 = Query(s__543, store__544):where('name', '=', "Robert'); DROP TABLE users;--");
    result__547 = q__546:toSql():toString();
    actual_910 = result__547;
    t_911 = temper.str_eq(actual_910, "SELECT * FROM users WHERE name = 'Robert''); DROP TABLE users;--'");
    fn__3670 = function()
      return temper.concat('expected result == (', "SELECT * FROM users WHERE name = 'Robert''); DROP TABLE users;--'", ') not (', actual_910, ')');
    end;
    temper.test_assert(test_909, t_911, fn__3670);
    return nil;
  end);
end;
Test_.test_toSqlOperatorNormalization__845 = function()
  temper.test('toSql: operator normalization', function(test_912)
    local s__549, store__550, q__551, actual_913, t_914, fn__3660;
    s__549 = schema('users', temper.listof(field('name', 'String', false, false)));
    store__550 = InMemoryStore();
    q__551 = Query(s__549, store__550):where('name', '==', 'Alice');
    actual_913 = q__551:toSql():toString();
    t_914 = temper.str_eq(actual_913, "SELECT * FROM users WHERE name = 'Alice'");
    fn__3660 = function()
      return temper.concat('expected q.toSql().toString() == (', "SELECT * FROM users WHERE name = 'Alice'", ') not (', actual_913, ')');
    end;
    temper.test_assert(test_912, t_914, fn__3660);
    return nil;
  end);
end;
Test_.test_toSqlInvalidOperatorFallback__848 = function()
  temper.test('toSql: invalid operator fallback', function(test_915)
    local s__553, store__554, q__555, actual_916, t_917, fn__3650;
    s__553 = schema('users', temper.listof(field('name', 'String', false, false)));
    store__554 = InMemoryStore();
    q__555 = Query(s__553, store__554):where('name', 'LIKE', 'Alice');
    actual_916 = q__555:toSql():toString();
    t_917 = temper.str_eq(actual_916, "SELECT * FROM users WHERE name = 'Alice'");
    fn__3650 = function()
      return temper.concat('expected q.toSql().toString() == (', "SELECT * FROM users WHERE name = 'Alice'", ') not (', actual_916, ')');
    end;
    temper.test_assert(test_915, t_917, fn__3650);
    return nil;
  end);
end;
Test_.test_toSqlMultipleWhere__851 = function()
  temper.test('toSql: multiple where', function(test_918)
    local s__557, store__558, q__559, actual_919, t_920, fn__3639;
    s__557 = schema('users', temper.listof(field('age', 'Int', false, false)));
    store__558 = InMemoryStore();
    q__559 = Query(s__557, store__558):where('age', '>=', '18'):where('age', '<', '30');
    actual_919 = q__559:toSql():toString();
    t_920 = temper.str_eq(actual_919, 'SELECT * FROM users WHERE age >= 18 AND age < 30');
    fn__3639 = function()
      return temper.concat('expected q.toSql().toString() == (', 'SELECT * FROM users WHERE age >= 18 AND age < 30', ') not (', actual_919, ')');
    end;
    temper.test_assert(test_918, t_920, fn__3639);
    return nil;
  end);
end;
Test_.test_toSqlOrderBy__854 = function()
  temper.test('toSql: order by', function(test_921)
    local s__561, store__562, q__563, actual_922, t_923, fn__3629;
    s__561 = schema('users', temper.listof(field('name', 'String', false, false)));
    store__562 = InMemoryStore();
    q__563 = Query(s__561, store__562):orderBy('name', 'asc');
    actual_922 = q__563:toSql():toString();
    t_923 = temper.str_eq(actual_922, 'SELECT * FROM users ORDER BY name ASC');
    fn__3629 = function()
      return temper.concat('expected q.toSql().toString() == (', 'SELECT * FROM users ORDER BY name ASC', ') not (', actual_922, ')');
    end;
    temper.test_assert(test_921, t_923, fn__3629);
    return nil;
  end);
end;
Test_.test_toSqlOrderByDesc__857 = function()
  temper.test('toSql: order by desc', function(test_924)
    local s__565, store__566, q__567, actual_925, t_926, fn__3619;
    s__565 = schema('users', temper.listof(field('age', 'Int', false, false)));
    store__566 = InMemoryStore();
    q__567 = Query(s__565, store__566):orderBy('age', 'desc');
    actual_925 = q__567:toSql():toString();
    t_926 = temper.str_eq(actual_925, 'SELECT * FROM users ORDER BY age DESC');
    fn__3619 = function()
      return temper.concat('expected q.toSql().toString() == (', 'SELECT * FROM users ORDER BY age DESC', ') not (', actual_925, ')');
    end;
    temper.test_assert(test_924, t_926, fn__3619);
    return nil;
  end);
end;
Test_.test_toSqlLimit__860 = function()
  temper.test('toSql: limit', function(test_927)
    local s__569, store__570, q__571, actual_928, t_929, fn__3609;
    s__569 = schema('users', temper.listof(field('name', 'String', false, false)));
    store__570 = InMemoryStore();
    q__571 = Query(s__569, store__570):limit(10);
    actual_928 = q__571:toSql():toString();
    t_929 = temper.str_eq(actual_928, 'SELECT * FROM users LIMIT 10');
    fn__3609 = function()
      return temper.concat('expected q.toSql().toString() == (', 'SELECT * FROM users LIMIT 10', ') not (', actual_928, ')');
    end;
    temper.test_assert(test_927, t_929, fn__3609);
    return nil;
  end);
end;
Test_.test_toSqlOffset__863 = function()
  temper.test('toSql: offset', function(test_930)
    local s__573, store__574, q__575, actual_931, t_932, fn__3599;
    s__573 = schema('users', temper.listof(field('name', 'String', false, false)));
    store__574 = InMemoryStore();
    q__575 = Query(s__573, store__574):offset(5);
    actual_931 = q__575:toSql():toString();
    t_932 = temper.str_eq(actual_931, 'SELECT * FROM users OFFSET 5');
    fn__3599 = function()
      return temper.concat('expected q.toSql().toString() == (', 'SELECT * FROM users OFFSET 5', ') not (', actual_931, ')');
    end;
    temper.test_assert(test_930, t_932, fn__3599);
    return nil;
  end);
end;
Test_.test_toSqlComplexQuery__866 = function()
  temper.test('toSql: complex query', function(test_933)
    local s__577, store__578, q__579, actual_934, t_935, fn__3585;
    s__577 = schema('users', temper.listof(field('name', 'String', false, false), field('age', 'Int', false, false)));
    store__578 = InMemoryStore();
    q__579 = Query(s__577, store__578):select(temper.listof('name', 'age')):where('age', '>=', '18'):orderBy('age', 'desc'):limit(10):offset(20);
    actual_934 = q__579:toSql():toString();
    t_935 = temper.str_eq(actual_934, 'SELECT name, age FROM users WHERE age >= 18 ORDER BY age DESC LIMIT 10 OFFSET 20');
    fn__3585 = function()
      return temper.concat('expected q.toSql().toString() == (', 'SELECT name, age FROM users WHERE age >= 18 ORDER BY age DESC LIMIT 10 OFFSET 20', ') not (', actual_934, ')');
    end;
    temper.test_assert(test_933, t_935, fn__3585);
    return nil;
  end);
end;
Test_.test_toSqlUnicodeEscaping__869 = function()
  temper.test('toSql: unicode escaping', function(test_936)
    local s__581, store__582, q__583, actual_937, t_938, fn__3575;
    s__581 = schema('users', temper.listof(field('name', 'String', false, false)));
    store__582 = InMemoryStore();
    q__583 = Query(s__581, store__582):where('name', '=', 'Hello \xe4\xb8\x96\xe7\x95\x8c');
    actual_937 = q__583:toSql():toString();
    t_938 = temper.str_eq(actual_937, "SELECT * FROM users WHERE name = 'Hello \xe4\xb8\x96\xe7\x95\x8c'");
    fn__3575 = function()
      return temper.concat('expected q.toSql().toString() == (', "SELECT * FROM users WHERE name = 'Hello \xe4\xb8\x96\xe7\x95\x8c'", ') not (', actual_937, ')');
    end;
    temper.test_assert(test_936, t_938, fn__3575);
    return nil;
  end);
end;
Test_.test_toSqlEmbeddedQuotes__872 = function()
  temper.test('toSql: embedded quotes', function(test_939)
    local s__585, store__586, q__587, actual_940, t_941, fn__3565;
    s__585 = schema('users', temper.listof(field('name', 'String', false, false)));
    store__586 = InMemoryStore();
    q__587 = Query(s__585, store__586):where('name', '=', "O'Brien");
    actual_940 = q__587:toSql():toString();
    t_941 = temper.str_eq(actual_940, "SELECT * FROM users WHERE name = 'O''Brien'");
    fn__3565 = function()
      return temper.concat('expected q.toSql().toString() == (', "SELECT * FROM users WHERE name = 'O''Brien'", ') not (', actual_940, ')');
    end;
    temper.test_assert(test_939, t_941, fn__3565);
    return nil;
  end);
end;
Test_.test_toSqlEmptyString__875 = function()
  temper.test('toSql: empty string', function(test_942)
    local s__589, store__590, q__591, actual_943, t_944, fn__3555;
    s__589 = schema('users', temper.listof(field('name', 'String', false, false)));
    store__590 = InMemoryStore();
    q__591 = Query(s__589, store__590):where('name', '=', '');
    actual_943 = q__591:toSql():toString();
    t_944 = temper.str_eq(actual_943, "SELECT * FROM users WHERE name = ''");
    fn__3555 = function()
      return temper.concat('expected q.toSql().toString() == (', "SELECT * FROM users WHERE name = ''", ') not (', actual_943, ')');
    end;
    temper.test_assert(test_942, t_944, fn__3555);
    return nil;
  end);
end;
Test_.test_toInsertSqlBasicInsert__878 = function()
  temper.test('toInsertSql: basic insert', function(test_945)
    local s__593, vals__594, result__595, actual_946, t_947, fn__3545;
    s__593 = schema('users', temper.listof(field('name', 'String', false, false), field('age', 'Int', false, false)));
    vals__594 = temper.map_constructor(temper.listof(temper.pair_constructor('name', 'Alice'), temper.pair_constructor('age', '25')));
    result__595 = toInsertSql(s__593, vals__594);
    actual_946 = result__595:toString();
    t_947 = temper.str_eq(actual_946, "INSERT INTO users (name, age) VALUES ('Alice', 25)");
    fn__3545 = function()
      return temper.concat('expected result.toString() == (', "INSERT INTO users (name, age) VALUES ('Alice', 25)", ') not (', actual_946, ')');
    end;
    temper.test_assert(test_945, t_947, fn__3545);
    return nil;
  end);
end;
Test_.test_toInsertSqlInjectionBlocked__881 = function()
  temper.test('toInsertSql: injection blocked', function(test_948)
    local s__597, vals__598, result__599, actual_949, t_950, fn__3536;
    s__597 = schema('users', temper.listof(field('name', 'String', false, false)));
    vals__598 = temper.map_constructor(temper.listof(temper.pair_constructor('name', "Robert'); DROP TABLE users;--")));
    result__599 = toInsertSql(s__597, vals__598);
    actual_949 = result__599:toString();
    t_950 = temper.str_eq(actual_949, "INSERT INTO users (name) VALUES ('Robert''); DROP TABLE users;--')");
    fn__3536 = function()
      return temper.concat('expected result.toString() == (', "INSERT INTO users (name) VALUES ('Robert''); DROP TABLE users;--')", ') not (', actual_949, ')');
    end;
    temper.test_assert(test_948, t_950, fn__3536);
    return nil;
  end);
end;
Test_.test_toSqlQueryStandalone__884 = function()
  temper.test('toSqlQuery: standalone', function(test_951)
    local s__601, result__602, actual_952, t_953, fn__3525;
    s__601 = schema('users', temper.listof(field('name', 'String', false, false), field('age', 'Int', false, false)));
    result__602 = toSqlQuery(s__601, temper.listof('name'), temper.listof(WhereClause('age', '>', '21')), temper.listof(OrderClause('name', 'asc')), 5, 0);
    actual_952 = result__602:toString();
    t_953 = temper.str_eq(actual_952, 'SELECT name FROM users WHERE age > 21 ORDER BY name ASC LIMIT 5');
    fn__3525 = function()
      return temper.concat('expected result.toString() == (', 'SELECT name FROM users WHERE age > 21 ORDER BY name ASC LIMIT 5', ') not (', actual_952, ')');
    end;
    temper.test_assert(test_951, t_953, fn__3525);
    return nil;
  end);
end;
Test_.test_toSqlAdversarialFieldNameBlocked__887 = function()
  temper.test('toSql: adversarial field name blocked', function(test_954)
    local s__604, store__605, q__606, actual_955, t_956, fn__3515;
    s__604 = schema('users', temper.listof(field('name', 'String', false, false)));
    store__605 = InMemoryStore();
    q__606 = Query(s__604, store__605):where('1=1; DROP TABLE users; --', '=', 'Alice');
    actual_955 = q__606:toSql():toString();
    t_956 = temper.str_eq(actual_955, 'SELECT * FROM users');
    fn__3515 = function()
      return temper.concat('expected q.toSql().toString() == (', 'SELECT * FROM users', ') not (', actual_955, ')');
    end;
    temper.test_assert(test_954, t_956, fn__3515);
    return nil;
  end);
end;
Test_.test_toSqlAdversarialSelectColumnBlocked__890 = function()
  temper.test('toSql: adversarial select column blocked', function(test_957)
    local s__608, store__609, q__610, actual_958, t_959, fn__3505;
    s__608 = schema('users', temper.listof(field('name', 'String', false, false)));
    store__609 = InMemoryStore();
    q__610 = Query(s__608, store__609):select(temper.listof('name', '1; DROP TABLE users'));
    actual_958 = q__610:toSql():toString();
    t_959 = temper.str_eq(actual_958, 'SELECT name FROM users');
    fn__3505 = function()
      return temper.concat('expected q.toSql().toString() == (', 'SELECT name FROM users', ') not (', actual_958, ')');
    end;
    temper.test_assert(test_957, t_959, fn__3505);
    return nil;
  end);
end;
Test_.test_toSqlAdversarialOrderByBlocked__893 = function()
  temper.test('toSql: adversarial order by blocked', function(test_960)
    local s__612, store__613, q__614, actual_961, t_962, fn__3495;
    s__612 = schema('users', temper.listof(field('name', 'String', false, false)));
    store__613 = InMemoryStore();
    q__614 = Query(s__612, store__613):orderBy('1; DROP TABLE users', 'asc');
    actual_961 = q__614:toSql():toString();
    t_962 = temper.str_eq(actual_961, 'SELECT * FROM users');
    fn__3495 = function()
      return temper.concat('expected q.toSql().toString() == (', 'SELECT * FROM users', ') not (', actual_961, ')');
    end;
    temper.test_assert(test_960, t_962, fn__3495);
    return nil;
  end);
end;
Test_.test_isValidIdentifierValidNames__896 = function()
  temper.test('isValidIdentifier: valid names', function(test_963)
    local t_964, fn__3479, t_965, fn__3478, t_966, fn__3477, t_967, fn__3476, t_968, fn__3475;
    t_964 = isValidIdentifier('users');
    fn__3479 = function()
      return 'expected `-work/src//`.isValidIdentifier("users")';
    end;
    temper.test_assert(test_963, t_964, fn__3479);
    t_965 = isValidIdentifier('user_table');
    fn__3478 = function()
      return 'expected `-work/src//`.isValidIdentifier("user_table")';
    end;
    temper.test_assert(test_963, t_965, fn__3478);
    t_966 = isValidIdentifier('Table1');
    fn__3477 = function()
      return 'expected `-work/src//`.isValidIdentifier("Table1")';
    end;
    temper.test_assert(test_963, t_966, fn__3477);
    t_967 = isValidIdentifier('_private');
    fn__3476 = function()
      return 'expected `-work/src//`.isValidIdentifier("_private")';
    end;
    temper.test_assert(test_963, t_967, fn__3476);
    t_968 = isValidIdentifier('a');
    fn__3475 = function()
      return 'expected `-work/src//`.isValidIdentifier("a")';
    end;
    temper.test_assert(test_963, t_968, fn__3475);
    return nil;
  end);
end;
Test_.test_isValidIdentifierInvalidNames__897 = function()
  temper.test('isValidIdentifier: invalid names', function(test_969)
    local t_970, fn__3450, t_971, fn__3449, t_972, fn__3448, t_973, fn__3447, t_974, fn__3446, t_975, fn__3445;
    t_970 = not isValidIdentifier('');
    fn__3450 = function()
      return 'expected !`-work/src//`.isValidIdentifier("")';
    end;
    temper.test_assert(test_969, t_970, fn__3450);
    t_971 = not isValidIdentifier('users; DROP TABLE');
    fn__3449 = function()
      return 'expected !`-work/src//`.isValidIdentifier("users; DROP TABLE")';
    end;
    temper.test_assert(test_969, t_971, fn__3449);
    t_972 = not isValidIdentifier('users--');
    fn__3448 = function()
      return 'expected !`-work/src//`.isValidIdentifier("users--")';
    end;
    temper.test_assert(test_969, t_972, fn__3448);
    t_973 = not isValidIdentifier('ta ble');
    fn__3447 = function()
      return 'expected !`-work/src//`.isValidIdentifier("ta ble")';
    end;
    temper.test_assert(test_969, t_973, fn__3447);
    t_974 = not isValidIdentifier('table.name');
    fn__3446 = function()
      return 'expected !`-work/src//`.isValidIdentifier("table.name")';
    end;
    temper.test_assert(test_969, t_974, fn__3446);
    t_975 = not isValidIdentifier("Robert'); DROP TABLE users;--");
    fn__3445 = function()
      return "expected !`-work/src//`.isValidIdentifier(\"Robert'); DROP TABLE users;--\")";
    end;
    temper.test_assert(test_969, t_975, fn__3445);
    return nil;
  end);
end;
Test_.test_toSqlNonNumericIntValueProducesAlwaysFalse__898 = function()
  temper.test('toSql: non-numeric Int value produces always-false', function(test_976)
    local s__618, store__619, q__620, actual_977, t_978, fn__3435;
    s__618 = schema('users', temper.listof(field('age', 'Int', false, false)));
    store__619 = InMemoryStore();
    q__620 = Query(s__618, store__619):where('age', '=', 'admin');
    actual_977 = q__620:toSql():toString();
    t_978 = temper.str_eq(actual_977, 'SELECT * FROM users WHERE 1 = 0');
    fn__3435 = function()
      return temper.concat('expected q.toSql().toString() == (', 'SELECT * FROM users WHERE 1 = 0', ') not (', actual_977, ')');
    end;
    temper.test_assert(test_976, t_978, fn__3435);
    return nil;
  end);
end;
Test_.test_inMemoryNonNumericIntValueMatchesNothing__901 = function()
  temper.test('in-memory: non-numeric Int value matches nothing', function(test_979)
    local s__622, store__623, results__624, actual_980, t_981, fn__3421;
    s__622 = schema('users', temper.listof(field('name', 'String', false, false), field('age', 'Int', false, false)));
    store__623 = InMemoryStore();
    store__623:insert('users', temper.map_constructor(temper.listof(temper.pair_constructor('name', 'Alice'), temper.pair_constructor('age', '0'))));
    results__624 = Query(s__622, store__623):where('age', '=', 'admin'):all();
    actual_980 = temper.list_length(results__624);
    t_981 = (actual_980 == 0);
    fn__3421 = function()
      return temper.concat('expected results.length == (', temper.int32_tostring(0), ') not (', temper.int32_tostring(actual_980), ')');
    end;
    temper.test_assert(test_979, t_981, fn__3421);
    return nil;
  end);
end;
Test_.test_toSqlLimitZeroEmitsLimit0__904 = function()
  temper.test('toSql: limit zero emits LIMIT 0', function(test_982)
    local s__626, store__627, q__628, actual_983, t_984, fn__3409;
    s__626 = schema('users', temper.listof(field('name', 'String', false, false)));
    store__627 = InMemoryStore();
    q__628 = Query(s__626, store__627):limit(0);
    actual_983 = q__628:toSql():toString();
    t_984 = temper.str_eq(actual_983, 'SELECT * FROM users LIMIT 0');
    fn__3409 = function()
      return temper.concat('expected q.toSql().toString() == (', 'SELECT * FROM users LIMIT 0', ') not (', actual_983, ')');
    end;
    temper.test_assert(test_982, t_984, fn__3409);
    return nil;
  end);
end;
Test_.test_inMemoryLimitZeroReturnsEmpty__907 = function()
  temper.test('in-memory: limit zero returns empty', function(test_985)
    local s__630, store__631, results__632, actual_986, t_987, fn__3396;
    s__630 = schema('users', temper.listof(field('name', 'String', false, false)));
    store__631 = InMemoryStore();
    store__631:insert('users', temper.map_constructor(temper.listof(temper.pair_constructor('name', 'Alice'))));
    results__632 = Query(s__630, store__631):limit(0):all();
    actual_986 = temper.list_length(results__632);
    t_987 = (actual_986 == 0);
    fn__3396 = function()
      return temper.concat('expected results.length == (', temper.int32_tostring(0), ') not (', temper.int32_tostring(actual_986), ')');
    end;
    temper.test_assert(test_985, t_987, fn__3396);
    return nil;
  end);
end;
Test_.test_inMemoryNegativeLimitClampedToZero__910 = function()
  temper.test('in-memory: negative limit clamped to zero', function(test_988)
    local s__634, store__635, results__636, actual_989, t_990, fn__3381;
    s__634 = schema('users', temper.listof(field('name', 'String', false, false)));
    store__635 = InMemoryStore();
    store__635:insert('users', temper.map_constructor(temper.listof(temper.pair_constructor('name', 'Alice'))));
    results__636 = Query(s__634, store__635):limit(-5):all();
    actual_989 = temper.list_length(results__636);
    t_990 = (actual_989 == 0);
    fn__3381 = function()
      return temper.concat('expected results.length == (', temper.int32_tostring(0), ') not (', temper.int32_tostring(actual_989), ')');
    end;
    temper.test_assert(test_988, t_990, fn__3381);
    return nil;
  end);
end;
Test_.test_toInsertSqlNoMatchingFieldsReturnsEmpty__913 = function()
  temper.test('toInsertSql: no matching fields returns empty', function(test_991)
    local s__638, vals__639, result__640, actual_992, t_993, fn__3370;
    s__638 = schema('users', temper.listof(field('name', 'String', false, false)));
    vals__639 = temper.map_constructor(temper.listof(temper.pair_constructor('nonexistent', 'value')));
    result__640 = toInsertSql(s__638, vals__639);
    actual_992 = result__640:toString();
    t_993 = temper.str_eq(actual_992, '');
    fn__3370 = function()
      return temper.concat('expected result.toString() == (', '', ') not (', actual_992, ')');
    end;
    temper.test_assert(test_991, t_993, fn__3370);
    return nil;
  end);
end;
exports = {};
local_1017.LuaUnit.run(local_1016({'--pattern', '^Test_%.', local_1016(arg)}));
return exports;
