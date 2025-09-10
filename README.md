# Alon Public - Testing Stop Gap

This repository provides a basic testing infrastructure setup that serves as a "stop gap" solution - a temporary foundation for testing practices that can be built upon.

## Project Structure

```
├── src/                    # Source code
│   ├── __init__.py
│   └── calculator.py       # Sample module with basic functions
├── tests/                  # Test files
│   ├── __init__.py
│   └── test_calculator.py  # Sample tests demonstrating testing patterns
├── .github/workflows/      # CI/CD workflows
│   └── test.yml           # GitHub Actions workflow for running tests
├── requirements.txt        # Python dependencies
├── pytest.ini            # Pytest configuration
├── .gitignore            # Git ignore patterns
└── README.md             # This file
```

## Quick Start

### Prerequisites
- Python 3.8 or higher
- pip (Python package installer)

### Setup
1. Clone the repository
2. Install dependencies:
   ```bash
   pip install -r requirements.txt
   ```

### Running Tests
```bash
# Run all tests
pytest

# Run tests with coverage report
pytest --cov=src --cov-report=html

# Run specific test file
pytest tests/test_calculator.py

# Run tests with verbose output
pytest -v
```

## Testing Features

This stop gap testing setup includes:

- **pytest framework**: Modern Python testing framework
- **Test organization**: Clear separation of source code and tests
- **Coverage reporting**: Code coverage analysis with pytest-cov
- **CI/CD integration**: GitHub Actions workflow for automated testing
- **Multiple Python versions**: Testing across Python 3.8-3.11
- **Configuration**: Centralized pytest configuration
- **Sample tests**: Comprehensive examples including edge cases and error handling

## Test Patterns Demonstrated

The sample tests demonstrate several important testing patterns:

1. **Basic unit tests**: Testing individual functions
2. **Edge case testing**: Boundary conditions and special values
3. **Exception testing**: Proper error handling verification
4. **Parametric testing**: Testing multiple scenarios efficiently
5. **Test organization**: Logical grouping with test classes
6. **Documentation**: Well-documented test purposes

## Extending the Tests

To add new tests:

1. Create test files in the `tests/` directory following the `test_*.py` naming convention
2. Import your modules from the `src/` directory
3. Use descriptive test names starting with `test_`
4. Group related tests in classes starting with `Test`
5. Document your test purposes with docstrings

## CI/CD Integration

The GitHub Actions workflow (`.github/workflows/test.yml`) automatically:

- Runs tests on push to main/develop branches
- Runs tests on pull requests to main
- Tests across multiple Python versions
- Uploads coverage reports

## Configuration

The `pytest.ini` file configures:

- Test discovery patterns
- Output formatting
- Coverage reporting
- Custom markers for test categorization

This setup serves as a foundation that can be expanded with additional testing tools and practices as needed.