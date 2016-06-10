module LeapYear where

isLeapYear :: Int -> Bool
isLeapYear y = (y `rem` 400 == 0) || (y `rem` 100 /= 0 && y `rem` 4 == 0)
