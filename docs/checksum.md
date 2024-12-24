# DumpSync Command: Checksum

The `checksum` command in DumpSync is used to verify the integrity of dump files by calculating and comparing checksums. This feature ensures that the dump files have not been tampered with or corrupted during transfer or storage.

```bash
dumpsync checksum -f <path_to_dump_file> -o <path_to_output_file>
```

### Command Breakdown

- **dumpsync**: This is the main command to invoke the DumpSync tool.
- **checksum**: This subcommand initiates the checksum verification process to ensure the integrity of the dump file.

### Parameters

- **-f <path_to_dump_file>**: Specifies the path to the dump file for which the checksum needs to be calculated and verified.
- **-o <path_to_output_file>**: Specifies the path to save the checksum output.

### Example

Calculate and verify the checksum of a dump file located at `path/to/dump.sql`:

```bash
dumpsync checksum -f path/to/dump.sql -o path/to/output.txt
```

### Description

- The `checksum` command calculates the checksum of the specified dump file and compares it with the original checksum stored in the file.
- If the checksums match, the dump file is considered intact and has not been altered.
- This process provides an additional layer of security and ensures the reliability of dump files for data restoration.
- The output file will contain the calculated checksum and the comparison result.