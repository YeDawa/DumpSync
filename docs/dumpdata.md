# DumpSync Command: Dump Data

To create a dump data, you can use the following command:

```bash
dumpsync dump-data
```

For connectting to a server, read the [Connecting to a Server](connection.md) guide.

### Command Breakdown

- **dumpsync**: This is the main command to invoke the DumpSync tool.
- **dump-data**: This subcommand initiates the dump data process to create a dump of the specified database.

### Options

- **-f**: (Optional) Indicates the file path where the dump will be saved. Replace `/path/to/` with the desired directory path on your system.
- **--table**: (Optional) Specifies the table to dump. If not provided, all tables will be dumped.

### Example

```bash
dumpsync dump-data
```

### Exporting only table

To export a specific table from the database, you can use the `--table` option:

```bash
dumpsync dump-data --table table1
```
