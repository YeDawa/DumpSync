# Skip Syntax

This document outlines the syntax for skipping specific tables or lines during database restore operations using pattern matching.

### Skip a specific table
This command will skip the specified tables (`table1` and `table2`) during the restore operation.

```
-- skip tables "table1, table2"
```

### Skip line matching a pattern
This command will skip the line that matches the specified pattern during the restore operation.

```
-- skip line
```
