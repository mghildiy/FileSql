## FileSQL
```
FileSQL is a lightweight query engine for structured files (CSV, JSON) that allows running SQL-like queries directly on file data without requiring a database.
```

### Features
```
- Parse CSV and JSON files efficiently.
- Support SQL-like queries: SELECT, WHERE, ORDER BY, aggregations.
- Stream large files without loading everything into memory.
- Return results in table, JSON, or CSV format.
```

### Example Usage
```bash
  filesql data.csv "SELECT name, age FROM data.csv WHERE age > 30"
```

### Program flow
```
input sql -> lexer -> tokens -> parser(sql grammar check) -> ast -> semantic analysis(sql semantics check) -> query planner -> executor
AST
SELECT [columns] FROM [source] WHERE [condition] GROUP BY [cols] ORDER BY [cols]
```