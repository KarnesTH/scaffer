# Scaffer

A simple command-line tool for creating project structures from predefined templates.

## Overview

Scaffer is a Rust-based tool that enables quick creation of project structures for various programming languages. It automatically generates directories and files based on predefined templates.

## Features

- Support for multiple programming languages (Rust, Python, Java, PHP, C, C++, HTML, Go)
- Interactive user guidance
- Automatic generation of project structures
- Optional: Automatic addition of .gitignore files
- Customizable project templates in JSON format

## Usage

```bash
# Create project with interactive user guidance
scaffer create

# Create project with specific parameters
scaffer create --language rust --name my-project --path /path/to/project
```

### Available Options

- `-l, --language`: The desired programming language
- `-n, --name`: The name of the project
- `-p, --path`: The installation path for the project

## Project Structure

The tool uses JSON templates stored in a `templates` directory. Each template defines:
- The directories to be created
- The files to be created with their content
- Commands to start the project

---

**Note**: This project was developed for practice purposes and is not intended for production use. It mainly serves for learning and experimenting with Rust and CLI application development.
