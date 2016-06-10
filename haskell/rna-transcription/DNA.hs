module DNA where

toRNA :: [Char] -> [Char]
toRNA str = map complement str

complement :: Char -> Char
complement 'G' = 'C'
complement 'C' = 'G'
complement 'T' = 'A'
complement 'A' = 'U'
