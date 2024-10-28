# DumpSync

A simple tool for dump and restore a MySQL database. It can be used for backup and restore purposes, or for transferring a database from one server to another.

## Overview

This tool allows for automatic dumps of a MySQL database at configurable time intervals, saving the files in a specified directory. Configuration can be done via environment variables or command-line arguments, providing flexibility for different usage contexts.

## Installation

To install the tool, you can use the following command:

```bash
cargo install dumpsync
```

### Parameters

The tool uses three main parameters, which can be configured via arguments or environment variables.

1. **Database Name (`dbname`)** The name of the database to be dumped. This parameter is required for the dump to execute successfully.

   - **Command-Line Argument**: `--database`
   - **Environment Variable**: `DB_NAME`
   - **Behavior**: If the `--database` argument is provided, it will be used. Otherwise, the `DB_NAME` environment variable will be used.
2. **Dump Interval (`interval`)** Time interval, in seconds, between each database dump. This parameter defines the frequency of automatic backups.

   - **Command-Line Argument**: `--interval`
   - **Environment Variable**: `DS_DUMP_INTERVAL`
   - **Behavior**: If the `--interval` argument is provided, it will be used. Otherwise, the `DS_DUMP_INTERVAL` environment variable will be used.
3. **Backup Path (`backup_path`)** Directory where dump files will be saved. You can specify a local or remote folder as needed.

   - **Command-Line Argument**: `--folder`
   - **Environment Variable**: `DS_DUMP_PATH`
   - **Behavior**: If the `--folder` argument is provided, it will be used. Otherwise, the `DS_DUMP_PATH` environment variable will be used.

#### Usage Example

**Command-Line Arguments**:

```bash
dumpsync -d my_database -i 3600 -f /path/to/
```

**Environment Variables**:

```bash
DB_USER="YOUR_USERNAME"
DB_PASSWORD="YOUR_PASSWORD"
DB_NAME="YOUR_DATABASE"

DS_DUMP_INTERVAL="3600"
DS_DUMP_PATH="/path/to/"
```

#### Notes

- The tool requires read and write permissions in the directory set for `backup_path`.
- The MySQL database must be accessible for the dump to proceed.
- It is recommended to test the initial configuration to ensure that parameters are correct and the backup executes as expected.
