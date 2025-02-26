# DumpSync Command: Visual

The `checksum` command in DumpSync is used to view the table structure of a database. This feature allows you to visualize the table structure.

```bash
dumpsync visual --table table1
```

### Command Breakdown

- **dumpsync**: This is the main command to invoke the DumpSync tool.
- **visual**: This subcommand initiates the visual process to view the table structure of a database.

### Flags

- **--table table1**: Specifies the table name for which you want to view the structure.

### Example

To view the table structure of a table named `users`, you would run:

```bash
dumpsync visual --table users
```

```bash
TABLE: users
+----------+--------------+-----+
| Column   | Type         | Key |
+----------+--------------+-----+
| id       | int          | PK  |
| username | varchar(255) |     |
| email    | varchar(255) |     |
| profile  | text         |     |
+----------+--------------+-----+
```

### Description

- The `visual` command allows you to visualize the table structure of a database.