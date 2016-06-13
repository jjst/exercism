module DNA where

toRNA :: String -> String
toRNA = map complement

complement :: Char -> Char
complement 'G' = 'C'
complement 'C' = 'G'
complement 'T' = 'A'
complement 'A' = 'U'
