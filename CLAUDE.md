# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Development Setup
```bash
# Create and activate virtual environment (using Python 3.10+)
uv venv .venv
source .venv/bin/activate

# Install dependencies 
uv pip install -e .
```

### Build
```bash
# Build the Rust core library
cargo build --manifest-path fauxgen-core/Cargo.toml

# Build the Python package
uv pip install -e .
```

### Run Tests
```bash
# Run Python tests
pytest

# Run Rust tests
cargo test --manifest-path fauxgen-core/Cargo.toml

# Run a specific test
pytest tests/test_unset.py::test_function_name
cargo test --manifest-path fauxgen-core/Cargo.toml -- test_name
```

### Linting and Type Checking
```bash
# Run ruff linter
ruff check .

# Run mypy type checking
mypy .
```

### Run the CLI Tool
```bash
# Generate factory methods from a module directory
fauxgen gen --module-dir <your_module_path>

# Generate factory methods and specify custom output directory
fauxgen gen --module-dir <your_module_path> --output-dir <custom_output_path>
```

## Architecture

### Overview

Fauxgen is a factory method generator that streamlines test data creation by automating field value generation. It consists of two main components:

1. **Rust Core (fauxgen-core)**: A CLI tool that parses Python code, analyzes pandera DataFrameModel definitions, and generates factory methods.
2. **Python Library (python/fauxgen)**: Provides utilities for generating random values and working with factory methods.

### Key Components

#### Rust Core Components

- **main.rs**: CLI entry point that processes commands and arguments
- **generator.rs**: Core logic for generating factory code
- **fields.rs**: Handles field parsing and type detection
- **visitor.rs**: AST visitor for parsing Python code

#### Python Components

- **generator.py**: Utilities for generating random values of various types
- **unset.py**: Implements the UNSET pattern for optional parameters

### Code Generation Workflow

1. The user runs the `fauxgen gen` command targeting a module directory
2. The Rust core scans Python files for pandera DataFrameModel definitions
3. For each model, it:
   - Analyzes field types and constraints
   - Generates a TypedDict representation
   - Creates factory methods with appropriate generators
4. Generated code is written to the `testing/fauxgen` directory by default
5. Generated factory methods respect the validation rules defined in the models

### Testing Pattern

Tests use the generated factory methods to create test data with specific field values:

```python
# Example test using a generated factory method
def test_some_functionality():
    df = pd.DataFrame([
        user_schema_record(age=25),  # Override specific fields for testing
        user_schema_record(age=30, name="John")
    ]).pipe(DataFrame[UserSchema])
    
    # Test code using the DataFrame with the test data
```

When working with this codebase, focus on maintaining the separation between the Rust code generation logic and the Python runtime functionality. Follow test-driven development practices by using existing tests as examples when adding new features.