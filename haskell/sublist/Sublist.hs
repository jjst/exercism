module Sublist where

data Sublist = Equal | Unequal | Sublist | Superlist deriving (Eq, Ord, Show)

sublist :: (Eq a) => [a] -> [a] -> Sublist
sublist a b 
    | lenA == lenB = if a == b then Equal else Unequal
    | lenA < lenB = if any (\xs -> xs == a) $ map (\x -> take lenA . drop x $ b) [0..deltaLength] then Sublist else Unequal
    | lenA > lenB = if (sublist b a == Sublist) then Superlist else Unequal
    where lenA = length a
          lenB = length b
          deltaLength = lenB - lenA
