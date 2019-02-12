module SpaceAge (Planet(..), ageOn) where

data Planet = Mercury
            | Venus
            | Earth
            | Mars
            | Jupiter
            | Saturn
            | Uranus
            | Neptune

ageOn :: Planet -> Float -> Float
ageOn Earth      = (/ 31557600.0)
ageOn Mercury    = (ageOn Earth) . (/ 0.2408467)
ageOn Venus      = (ageOn Earth) . (/ 0.61519726)
ageOn Mars       = (ageOn Earth) . (/ 1.8808158)
ageOn Jupiter    = (ageOn Earth) . (/ 11.862615)
ageOn Saturn     = (ageOn Earth) . (/ 29.447498)
ageOn Uranus     = (ageOn Earth) . (/ 84.016846)
ageOn Neptune    = (ageOn Earth) . (/ 164.79132)
