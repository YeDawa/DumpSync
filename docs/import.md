# DumpSync Command: Import

To restore a database dump, you can use the following command:

```bash
dumpsync import -f /path/to/dump.sql
```

For connectting to a server, read the [Connecting to a Server](connection.md) guide.

### Command Breakdown

- **dumpsync**: This is the main command to invoke the DumpSync tool.
- **import**: This subcommand initiates the import process to restore the database from the specified dump file.

### Options

- **-f /path/to/dump.sql**: Specifies the path to the dump file that you want to restore. Replace `/path/to/dump.sql` with the actual file path to your SQL dump file.

### Example

To restore a database from a dump file located at `/backups/example_dump.sql`, you would run:

```bash
dumpsync import -f /backups/example_dump.sql
```

### Encrypted Dumps

```bash
dumpsync import -f /backups/example_dump.sql.aes
```

DumpSync detects encrypted dump files and automatically decrypts them during the import process. If your dump file is encrypted, you can still use the `import` command as shown above.

### Notes

- Ensure that the dump file exists and that you have the necessary permissions to read it.
- The import process will overwrite existing data in the database, so be cautious when using this command, especially if restoring to a production environment.
- It’s recommended to back up current data before performing an import to avoid accidental data loss.
