module Sublist where

data Sublist = Equal | Unequal | Sublist | Superlist deriving (Eq, Ord, Show)

sublist :: (Eq a) => [a] -> [a] -> Sublist
sublist a b 
    | length a == length b = if a == b then Equal else Unequal
    | length a < length b = if elem a $ map (\x -> take lenA . drop x $ b) [0..deltaLength] then Sublist else Unequal
    | length a > length b = if sublist b a == Sublist then Superlist else Unequal
    where deltaLength = length b - length a
