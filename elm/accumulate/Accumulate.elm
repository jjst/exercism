module Accumulate exposing (..)

accumulate : (a -> b) -> List a -> List b
accumulate f xs =
  case xs of
    [] -> []
    head :: tail -> (f head) :: (accumulate f tail)
