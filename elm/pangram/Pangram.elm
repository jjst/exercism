module Pangram exposing (..)

import Char
import Set

asciiLetters = List.range 97 122 |> List.map Char.fromCode |> Set.fromList

isPangram : String -> Bool
isPangram s =
  let
    uniqueCharsInString = 
      s |> String.map Char.toLower 
        |> String.toList 
        |> Set.fromList 
        |> Set.intersect asciiLetters
  in
     uniqueCharsInString == asciiLetters
