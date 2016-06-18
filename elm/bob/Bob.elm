module Bob exposing (..)
import String exposing (endsWith, toUpper, isEmpty, trim, any)
import Char
import List

uppercaseLetters : List Char
uppercaseLetters = List.map Char.fromCode [65..90]

hey : String -> String
hey s =
   if isEmpty (trim s) then
      "Fine. Be that way!"
   else if any (\c -> List.member c uppercaseLetters) s && s == toUpper s then
      "Whoa, chill out!"
   else if endsWith "?" s then
      "Sure."
   else
      "Whatever."
