# DumpSync Command: Export

To create a database dump, you can use the following command:

```bash
dumpsync export -d my_database -i 3600 -f /path/to/
```

### Command Breakdown

- **dumpsync**: This is the main command to invoke the DumpSync tool.

- **export**: This subcommand initiates the export process to create a dump of the specified database.

### Options

- **-d my_database**: Specifies the name of the database you want to export. Replace `my_database` with the actual name of your database.

- **-i 3600**: Sets the interval (in seconds) for the dump process. In this example, the interval is set to 3600 seconds (1 hour). You can adjust this value based on your requirements.

- **-f /path/to/**: Indicates the file path where the dump will be saved. Replace `/path/to/` with the desired directory path on your system.

### Example

To create a dump of a database named `example_db` with an interval of 2 hours and save it to the `/backups/` directory, you would run:

```bash
dumpsync export -d example_db -i 7200 -f /backups/
```

### Notes

- Ensure that the specified path for the dump exists and that you have the necessary permissions to write to that directory.
- Adjust the interval according to your backup strategy to ensure that you have up-to-date dumps without overwhelming your database resources.
