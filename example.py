#!/usr/bin/env python3
"""
Example script demonstrating how to use the Rust extension from Python.

To run this example:
1. Install maturin: pip install maturin
2. Build the extension: maturin develop
3. Run this script: python example.py
"""

import rustmod

def main():
    print("=== Rust-Python Integration Demo ===")
    print()
    
    # Test simple functions
    print("1. Testing simple functions:")
    result = rustmod.sum_as_string(5, 3)
    print(f"   sum_as_string(5, 3) = {result} (type: {type(result)})")
    
    product = rustmod.multiply(4.5, 2.0)
    print(f"   multiply(4.5, 2.0) = {product} (type: {type(product)})")
    print()
    
    # Test list processing
    print("2. Testing list processing:")
    numbers = [1, 2, 3, 4, 5]
    doubled = rustmod.process_list(numbers)
    print(f"   Original list: {numbers}")
    print(f"   Doubled list:  {doubled}")
    print()
    
    # Test Calculator class
    print("3. Testing Calculator class:")
    calc = rustmod.Calculator(10.0)
    print(f"   Initial calculator: {calc}")
    
    result = calc.add(5.0)
    print(f"   After adding 5.0: {calc} (returned: {result})")
    
    result = calc.multiply(2.0)
    print(f"   After multiplying by 2.0: {calc} (returned: {result})")
    
    # Access the value directly
    print(f"   Direct value access: calc.value = {calc.value}")
    
    # Set value directly
    calc.value = 100.0
    print(f"   After setting value to 100.0: {calc}")
    
    calc.reset()
    print(f"   After reset: {calc}")
    print()

if __name__ == "__main__":
    main()