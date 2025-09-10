"""
Test cases for the calculator module.
This demonstrates basic testing patterns for the testing stop gap implementation.
"""

import pytest
import sys
import os

# Add src directory to Python path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from calculator import add, subtract, multiply, divide, power


class TestCalculator:
    """Test class for calculator functions."""

    def test_add(self):
        """Test addition function."""
        assert add(2, 3) == 5
        assert add(-1, 1) == 0
        assert add(0, 0) == 0
        assert add(1.5, 2.5) == 4.0

    def test_subtract(self):
        """Test subtraction function."""
        assert subtract(5, 3) == 2
        assert subtract(1, 1) == 0
        assert subtract(0, 5) == -5
        assert subtract(2.5, 1.5) == 1.0

    def test_multiply(self):
        """Test multiplication function."""
        assert multiply(3, 4) == 12
        assert multiply(-2, 3) == -6
        assert multiply(0, 10) == 0
        assert multiply(2.5, 2) == 5.0

    def test_divide(self):
        """Test division function."""
        assert divide(6, 2) == 3
        assert divide(5, 2) == 2.5
        assert divide(-10, 2) == -5
        assert divide(0, 5) == 0

    def test_divide_by_zero(self):
        """Test division by zero raises ValueError."""
        with pytest.raises(ValueError, match="Cannot divide by zero"):
            divide(5, 0)

    def test_power(self):
        """Test power function."""
        assert power(2, 3) == 8
        assert power(5, 0) == 1
        assert power(2, -1) == 0.5
        assert power(9, 0.5) == 3.0


class TestEdgeCases:
    """Test edge cases and boundary conditions."""

    def test_large_numbers(self):
        """Test with large numbers."""
        large = 10**10
        assert add(large, large) == 2 * large
        assert multiply(large, 0) == 0

    def test_float_precision(self):
        """Test floating point operations."""
        result = add(0.1, 0.2)
        assert abs(result - 0.3) < 1e-10  # Account for floating point precision

    def test_negative_numbers(self):
        """Test operations with negative numbers."""
        assert add(-5, -3) == -8
        assert subtract(-5, -3) == -2
        assert multiply(-2, -3) == 6
        assert divide(-6, -2) == 3