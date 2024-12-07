# DumpSync Command: Transfer

To transfer a database from one server to another, you can use the following command:

```bash
dumpsync transfer -f /path/to/dump.sql
```

### Command Breakdown

- **dumpsync**: This is the main command to invoke the DumpSync tool.
- **transfer**: This subcommand initiates the transfer process to restore the database from the specified dump file.

### Options

- **-f /path/to/dump.sql**: Specifies the path to the dump file that you want to restore. Replace `/path/to/dump.sql` with the actual file path to your SQL dump file.

### Example

To transfer a database from a dump file located at `/backups/example_dump.sql`, you would run:

```bash
dumpsync transfer -f /backups/example_dump.sql
```

### Notes

- Ensure that the dump file exists and that you have the necessary permissions to read it.
- The transfer process will overwrite existing data in the database, so be cautious when using this command, especially if restoring to a production environment.
- It’s recommended to back up current data before performing an import to avoid accidental data loss.
