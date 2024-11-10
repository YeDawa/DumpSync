# Settings File for DumpSync

The configuration file `dumpsync.yml` allows you to customize the options for the dump process. 

It should be saved as `dumpsync.yml` in the same directory where your project is running.

## File Structure

Example structure:

```yaml
exports:
  dump_data: true
  insert_ignore_into: false
  drop_table_if_exists: true
  database_if_not_exists: true
  ignore_tables:
    - YOUR_TABLE_NAME
    - ANOTHER_TABLE_NAME

connection:
  max_retries: 3
  retry_connection_interval: 5
```

### Properties of `exports`

- **dump_data** (`boolean`): Determines whether all data in your database should be exported. If set to `true`, the content of all tables will be included in the dump. If `false`, only the structures of the tables will be exported.
- **drop_table_if_exists** (`boolean`): Specifies whether existing tables should be dropped before being recreated during the import process. Setting this to `true` will include a `DROP TABLE IF EXISTS` statement before the `CREATE TABLE` statement, preventing table duplication conflicts.
- **insert_ignore_into** (`boolean`): The `insert_ignore_into` property specifies whether data should be inserted using the `INSERT IGNORE INTO` statement during the import process. Setting it to `true` will ensure that duplicate records are ignored, preventing errors from duplicate entries in the table.
- **database_if_not_exists** (`boolean`): Indicates whether the database should be created only if it does not exist. If set to `true`, a `CREATE DATABASE IF NOT EXISTS` statement will be included in the dump, avoiding errors if the database is already present.
- **ignore_tables** (`array` of `strings`): A list of tables to be ignored during the dump. Tables listed here will not have their structure or data exported. Example:

```yaml
ignore_tables:
  - table
```

### Properties of `connection`

- **max_retries** (`integer`): Defines the maximum number of retry attempts to establish a connection to the database. If a connection attempt fails, the application will retry up to this number.
- **retry_connection_interval** (`integer`): Specifies the interval (in seconds) to wait between each retry attempt when attempting to connect to the database. This allows for gradual retries rather than immediate retries.
