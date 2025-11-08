<div align='center'>
<img src='https://i.imgur.com/DTR2L2Y.png'/>
<p></p>
</div>

<div align='center'>
<img src='https://img.shields.io/crates/v/dumpsync?style=for-the-badge&logo=rust'/>
<img src='https://img.shields.io/crates/l/dumpsync?style=for-the-badge'/>
<img src='https://img.shields.io/crates/d/dumpsync?style=for-the-badge&logo=rust'/>
</div>

DumpSync is a lightweight tool designed for efficiently dumping and restoring MySQL databases. Whether you need to create backups, restore databases, or transfer data between servers, DumpSync provides a simple and automated solution.

<div align='center'>
<img src='https://i.imgur.com/R8VOwQB.png'>
</div>

## Features

Some of the key features of DumpSync include:

- **Automated Backups**: Schedule regular database dumps at configurable intervals.
- **Flexible Configuration**: Easily configure settings using environment variables or command-line arguments.
- **Simple Restoration**: Quickly restore your database from previously created dump files.
- **Cross-Server Transfers**: Seamlessly transfer databases between different MySQL servers.
- **XSS Scanner**: Scan for XSS vulnerabilities in tables to enhance security.
- **XSS Report**: Generate a detailed report of XSS vulnerabilities found in the database.
- **Share**: Share your dump or scan results with others.
- **Schema**: Generate a schema file for the database.
- **Encryption**: Encrypt your dump files for added security.
- **Compression**: Compress your dump files to save disk space.
- **Checksum**: Verify the integrity of your dump files using checksums.
- **PDF Report**: Generate PDF reports for your dump files and your settings.
- **Truncate**: Truncate tables to remove all data from the specified table.
- **Visual**: Visualize the table structure of a database.
- **Pull**: Pull the latest changes from a remote database to your local environment.
- **History**: View the history of actions performed by the DumpSync tool.
- **Pattern Matching**: Use patterns to exclude specific tables or lines during restore operations.
- **Lock tables**: Lock tables during dump operations to ensure data consistency.

## Installation

To install DumpSync, use the following command:

> Make sure you have Rust installed on your system. If not, you can install it from [here](https://docs.dumpsync.com/install).

```bash
cargo install dumpsync
```

To install using Scoop, run:

```bash
scoop bucket add dumpsync https://github.com/YeDawa/DumpSync.git

scoop install dumpsync
```

## Documentation

For more help and document, see our documentation:

- [Overview](https://docs.dumpsync.com)
- [Install](https://docs.dumpsync.com/install)
- [Init](https://docs.dumpsync.com/init)
- [Connection](https://docs.dumpsync.com/connection)
- [Settings](https://docs.dumpsync.com/settings)
- [Export](https://docs.dumpsync.com/export)
- [Import](https://docs.dumpsync.com/import)
- [Transfer](https://docs.dumpsync.com/transfer)
- [Truncate](https://docs.dumpsync.com/truncate)
- [Pull](https://docs.dumpsync.com/pull)
- [Scan XSS](https://docs.dumpsync.com/scan)
- [Share](https://docs.dumpsync.com/share)
- [Schema](https://docs.dumpsync.com/schema)
- [Checksum](https://docs.dumpsync.com/checksum)
- [Visual](https://docs.dumpsync.com/visual)
- [History](https://docs.dumpsync.com/history)
- [Skip Syntax](https://docs.dumpsync.com/skip-syntax)
- [Writing patterns](https://docs.dumpsync.com/writing-patterns)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
