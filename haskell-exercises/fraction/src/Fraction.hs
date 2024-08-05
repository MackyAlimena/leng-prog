module Fraction (Fraction, add, sub, mul, divide, hcf) where

type Fraction = (Int, Int)

-- Implement the `add` Function

add :: Fraction -> Fraction -> Fraction
add (a, b) (c, d) = simplify (a * d + c * b, b * d)

-- Implement the `sub` Function

sub :: Fraction -> Fraction -> Fraction
sub (a, b) (c, d) = simplify (a * d - c * b, b * d)

-- Implement the `mul` Function

mul :: Fraction -> Fraction -> Fraction
mul (a, b) (c, d) = simplify (a * c, b * d)

-- Implement the `divide` Function

divide :: Fraction -> Fraction -> Fraction
divide (a, b) (c, d) = simplify (a * d, b * c)

-- Implement the `hcf` Function

hcf :: Int -> Int -> Int
hcf a 0 = a
hcf a b = hcf b (a `mod` b)

-- Implement the `simplify` Function
simplify :: Fraction -> Fraction
simplify (n, d) = let g = hcf n d in (n `div` g, d `div` g)
