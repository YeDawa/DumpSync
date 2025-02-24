# DumpSync Command: Truncate

To truncate a database, you can use the following command:

```bash
dumpsync truncate --table table1
```    

### Command Breakdown

- **dumpsync**: This is the main command to invoke the DumpSync tool.
- **truncate**: This subcommand initiates the truncation process to remove all data from the specified table.

### Options

- **-t table1**: Specifies the name of the table that you want to truncate. Replace `table1` with the actual name of the table you want to truncate.
- **-p**: Optional flag to specify path of dump file. If not specified, the default path will be used.
- **--encrypt**: Optional flag to encrypt the dump file. If specified, the dump file will be encrypted.

### Example

To truncate a table named `table1`, you would run:

```bash
dumpsync truncate --table table1
```

### Notes

- Truncating a table will remove all data from the specified table.
- Before truncating a table, will be generated a backup of the table.
- Be cautious when using this command, especially if truncating tables in a production environment.
