# ORMery C# Demo — ASP.NET Core + Microsoft.Data.Sqlite

A retro-styled todo list app built with ASP.NET Core Razor Pages and Microsoft.Data.Sqlite, using the **Temper-compiled ORMery query builder** for schema definition, SELECT queries, and INSERT operations.

Port: **5002**

## How ORMery Is Vendored

The compiled ORMery library lives in `TodoApp/vendor/` with three subdirectories:

```
TodoApp/vendor/
  ormery/          ← the query builder (has src/Ormery.csproj)
  std/             ← Temper standard library (has src/TemperLang.Std.csproj)
  temper-core/     ← Temper runtime (has TemperLang.Core.csproj)
```

Referenced in `TodoApp.csproj` via `<ProjectReference>`:

```xml
<ItemGroup>
  <!-- Exclude vendor source from this project's compilation -->
  <Compile Remove="vendor/**/*.cs" />
  <Content Remove="vendor/**" />
  <None Remove="vendor/**" />
</ItemGroup>

<ItemGroup>
  <PackageReference Include="Microsoft.Data.Sqlite" Version="10.0.3" />
  <ProjectReference Include="vendor/ormery/src/Ormery.csproj" />
</ItemGroup>
```

The `Ormery.csproj` internally references `TemperLang.Core` and `TemperLang.Std` via relative path, so the dependency chain resolves within `vendor/`.

No Entity Framework Core — just Microsoft.Data.Sqlite for execution.

## How ORMery Is Used

### Schema Definition

From `Data/TodoDb.cs` — schemas defined as static fields. The C# API uses PascalCase.

```csharp
using Ormery;
using TemperLang.Core;

public static readonly Schema ListSchema = OrmeryGlobal.Schema(
    "lists",
    Listed.CreateReadOnlyList<Field>(
        OrmeryGlobal.Field("name", "String", false, false),
        OrmeryGlobal.Field("created_at", "String", false, false)
    )
);

public static readonly Schema TodoSchema = OrmeryGlobal.Schema(
    "todos",
    Listed.CreateReadOnlyList<Field>(
        OrmeryGlobal.Field("title", "String", false, false),
        OrmeryGlobal.Field("completed", "Int", false, false),
        OrmeryGlobal.Field("list_id", "Int", false, false),
        OrmeryGlobal.Field("created_at", "String", false, false)
    )
);

private static readonly InMemoryStore _dummyStore = new InMemoryStore();
```

### SELECT Queries (ORMery)

ORMery generates the SQL, Microsoft.Data.Sqlite executes it:

```csharp
// GetAllLists — build SELECT * FROM lists ORDER BY created_at ASC
var listQuery = new Query(ListSchema, _dummyStore)
    .OrderBy("created_at", "asc");
string listSql = listQuery.ToSql().ToString();
cmd.CommandText = listSql;
using var reader = cmd.ExecuteReader();

// GetList — SELECT with WHERE clause
var query = new Query(ListSchema, _dummyStore)
    .Where("id", "=", id.ToString());
string sql = query.ToSql().ToString();

// GetTodosForList — SELECT with WHERE + multiple ORDER BY
var query = new Query(TodoSchema, _dummyStore)
    .Where("list_id", "=", listId.ToString())
    .OrderBy("completed", "asc")
    .OrderBy("created_at", "asc");
string sql = query.ToSql().ToString();

// HasAnyLists — SELECT with LIMIT for existence check
var query = new Query(ListSchema, _dummyStore).Limit(1);
string sql = query.ToSql().ToString();
```

### INSERT Operations (ORMery)

Value maps are built with `Mapped.ConstructMap` and `Listed.CreateReadOnlyList`:

```csharp
// InsertList — create a new list
var values = Mapped.ConstructMap(
    Listed.CreateReadOnlyList<KeyValuePair<string, string>>(
        new KeyValuePair<string, string>("name", name),
        new KeyValuePair<string, string>("created_at",
            DateTime.UtcNow.ToString("yyyy-MM-dd HH:mm:ss"))
    )
);
string sql = OrmeryGlobal.ToInsertSql(ListSchema, values).ToString();
cmd.CommandText = sql;
cmd.ExecuteNonQuery();

// InsertTodo — create a new todo
var values = Mapped.ConstructMap(
    Listed.CreateReadOnlyList<KeyValuePair<string, string>>(
        new KeyValuePair<string, string>("title", title),
        new KeyValuePair<string, string>("completed", "0"),
        new KeyValuePair<string, string>("list_id", listId.ToString()),
        new KeyValuePair<string, string>("created_at",
            DateTime.UtcNow.ToString("yyyy-MM-dd HH:mm:ss"))
    )
);
string sql = OrmeryGlobal.ToInsertSql(TodoSchema, values).ToString();
```

### Raw SQL (not supported by ORMery)

UPDATE and DELETE use ORMery's `SqlBuilder` for safe escaping:

```csharp
// ToggleTodo — UPDATE with SqlBuilder
var sb = new SqlBuilder();
sb.AppendSafe("UPDATE todos SET completed = CASE WHEN completed = 0 THEN 1 ELSE 0 END WHERE id = ");
sb.AppendInt32(todoId);
string sql = sb.Accumulated.ToString();

// UpdateList — UPDATE with safe string escaping
var sb = new SqlBuilder();
sb.AppendSafe("UPDATE lists SET name = ");
sb.AppendString(name);  // autoescapes single quotes
sb.AppendSafe(" WHERE id = ");
sb.AppendInt32(id);

// DeleteList — DELETE with PRAGMA for cascades
cmd.CommandText = "PRAGMA foreign_keys = ON";
cmd.ExecuteNonQuery();
var sb = new SqlBuilder();
sb.AppendSafe("DELETE FROM lists WHERE id = ");
sb.AppendInt32(id);
```

## Running

```bash
cd TodoApp
dotnet run
# → Todo app running at http://localhost:5002
```
