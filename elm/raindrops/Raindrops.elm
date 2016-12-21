module Raindrops exposing (..)

import String

raindrops : Int -> String
raindrops num =
    let 
        pling = if num % 3 == 0 then "Pling" else ""
        plang = if num % 5 == 0 then "Plang" else ""
        plong = if num % 7 == 0 then "Plong" else ""
        r = pling ++ plang ++ plong
    in
       if (String.isEmpty r) then toString num else r
