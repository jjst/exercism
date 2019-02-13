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
ageOn Earth       = (/ 31557600.0)
ageOn otherPlanet = (ageOn Earth) . (/ orbitalPeriodInEarthYears otherPlanet)
     where
            orbitalPeriodInEarthYears Mercury    = 0.2408467
            orbitalPeriodInEarthYears Venus      = 0.61519726
            orbitalPeriodInEarthYears Earth      = 1
            orbitalPeriodInEarthYears Mars       = 1.8808158
            orbitalPeriodInEarthYears Jupiter    = 11.862615
            orbitalPeriodInEarthYears Saturn     = 29.447498
            orbitalPeriodInEarthYears Uranus     = 84.016846
            orbitalPeriodInEarthYears Neptune    = 164.79132
