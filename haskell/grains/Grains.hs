module Grains where

square :: Int -> Integer
square x = 2^(x-1)

total :: Integer
total = 2^64 - 1 -- We're summing all powers of 2 up to 2^n which is equivalent to `2^(n+1) - 1`
