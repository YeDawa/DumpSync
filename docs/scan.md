# DumpSync Command: Scan

The dumpsync scan command is used to scan a database table for Cross-Site Scripting (XSS) vulnerabilities. It allows you to provide a payload file containing scripts that will be tested on the input fields of the specified table.

```bash
dumpsync scan --table <table> --payload <path_or_url_to_payload_file>
```

### Command Breakdown

- **dumpsync**: This is the main command to invoke the DumpSync tool.
- **scan**: This subcommand initiates the scanning process to check for XSS vulnerabilities in the specified table.

### Parameters

- **-t `<table>`**: Specifies the name of the table to be scanned. The command will check the fields in the table for any inputs that could be vulnerable to XSS attacks.
- **-p <path_or_url_to_payload_file>**: Specifies the path or URL to the payload file containing XSS scripts that will be tested on the input fields of the table.

### Example

Scan a table called example_table using a payload file `located at path/to/payload.txt`:

```bash
dumpsync scan --table example_table --payload path/to/payload.txt
```

### Description

- The command will access the input fields of the specified table and attempt to inject each payload from the provided file.
- This process helps identify vulnerable points where XSS attacks could be performed, providing a way to improve the security of your application.

### Notes

- Ensure that the dump file exists and that you have the necessary permissions to read it.
- The transfer process will overwrite existing data in the database, so be cautious when using this command, especially if restoring to a production environment.
- It’s recommended to back up current data before performing an import to avoid accidental data loss.
