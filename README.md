# Elliptic_Curve_Point_Addition_and_Point_Doubling_over_a_Finite_Field

## Overview
This project implements **Point Addition** and **Point Doubling** on an elliptic curve defined over a finite field, specifically modulo a prime number. These operations are fundamental in elliptic curve cryptography (ECC) and are used for various cryptographic algorithms. The implementation involves:
- **Point Addition**: Adding two distinct points on the curve.
- **Point Doubling**: Doubling a point on the curve, which is a special case of addition where both points are the same.
- **Curve Check**: Verifying if a point lies on the given elliptic curve.
- **Modular Inverse**: Calculating modular inverses used in the point addition and doubling formulas.

## Structure

### `Point` Struct
Represents a point on the elliptic curve with `x` and `y` coordinates. It also includes a method to check if a point is the "point at infinity", which is the identity element for elliptic curve operations.

### `EllipticCurve` Struct
Defines an elliptic curve with parameters:
- `a`: coefficient for the curve equation.
- `b`: coefficient for the curve equation.
- `p`: prime modulus over which the curve is defined.

The `EllipticCurve` struct includes methods for:
- `add_points`: Adding two points on the curve.
- `double_point`: Doubling a point on the curve.
- `is_point_on_curve`: Verifying if a point satisfies the elliptic curve equation.

### `mod_inverse` Function
A helper function that computes the modular inverse of a number `a` modulo `m`. This is used in both point addition and doubling to handle division in modular arithmetic.

## Key Concepts
- **Point Addition**: The process of adding two distinct points $P$ and $Q$ on the elliptic curve.
- **Point Doubling**: When adding a point to itself, resulting in a new point on the curve.
- **Finite Field**: Operations are performed modulo a prime number to keep values within a finite field, ensuring the calculations remain efficient and well-defined.

## Example Usage

In the `main` function, the code demonstrates how to:
1. Create an elliptic curve defined by the equation $y^2 = x^3 + ax + b \mod p$.
2. Verify if two points $p1$ and $p2$ lie on the curve.
3. Perform point addition on the two points.
4. Double the first point $p1$.

```rust
fn main() {
    let curve = EllipticCurve { a: 4, b: 4, p: 313 };

    let p1 = Point { x: Some(274), y: Some(288) };
    let p2 = Point { x: Some(159), y: Some(45) };

    // Check if points are on the curve
    println!("Is p1 on the curve? {}", curve.is_point_on_curve(p1));
    println!("Is p2 on the curve? {}", curve.is_point_on_curve(p2));

    let result = curve.add_points(p1, p2);
    println!("p1 + p2 = {:?}", result);

    let result_double = curve.double_point(p1);
    println!("2 * p1 = {:?}", result_double);
}
