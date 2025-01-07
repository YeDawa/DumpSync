# DumpSync Command: Scan

The dumpsync scan command is used to scan a database table for Cross-Site Scripting (XSS) vulnerabilities. It allows you to provide a payload file containing scripts that will be tested on the input fields of the specified table.

```bash
dumpsync scan -t <table> -p <path_or_url_to_payload_file>
```

### Command Breakdown

- **dumpsync**: This is the main command to invoke the DumpSync tool.
- **scan**: This subcommand initiates the scanning process to check for XSS vulnerabilities in the specified table.

### Parameters

- **-t `<table>`**:  Specifies the name of the table or multiple tables (split by commas) to be scanned. The command will check the fields in the table for any inputs that could be vulnerable to XSS attacks.
- **-p <path_or_url_to_payload_file>** (Optional): Specifies the path or URL to the payload file containing XSS scripts that will be tested on the input fields of the table.
- **-l `<limit>`** (Optional): Specifies the limit of the number of rows to scan.
- **-o `<offset>`** (Optional): Specifies the offset from where to start scanning the rows.
- **-f <path_to_report_file>** (Optional): Specifies the path to save the scan results in CSV, JSON or HTML format.

### Example

Scan a table called example_table using a payload file `located at path/to/payload.txt`:

```bash
dumpsync scan -t example_table -p path/to/payload.txt
```

### Description

- The command will access the input fields of the specified table and attempt to inject each payload from the provided file.
- This process helps identify vulnerable points where XSS attacks could be performed, providing a way to improve the security of your application.

### Notes

- Ensure that the dump file exists and that you have the necessary permissions to read it.
- The transfer process will overwrite existing data in the database, so be cautious when using this command, especially if restoring to a production environment.
- It’s recommended to back up current data before performing an import to avoid accidental data loss.

## Report output file

It's possible to save the scan results to a file using the `-f` or `--file` option. The report will be saved in CSV, JSON or HTML format.

```bash
dumpsync scan -t example_table -f path/to/report.csv
```

Formats supported for the report file are:

- TXT
- CSV
- XML
- JSON
- HTML
