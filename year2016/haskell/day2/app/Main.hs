module Main where

main :: IO ()
main = do
    input <- readFile "../data/day2_data.txt"
    let linesList = lines input
    let keys = tail $ scanl processKey (newCoord 1 1) linesList
    let password = concatMap (show . mapKey) keys
    putStrLn $ "Part 1: " ++ password

processKey :: Coord -> String -> Coord
processKey = foldl moveKey

data Coord = Coord { x::Int, y::Int }
    deriving (Show)

newCoord :: Int -> Int -> Coord
newCoord x y = Coord { x = x, y = y }

moveKey :: Coord -> Char -> Coord
moveKey (Coord x y) 'L'
    | x > 0         = Coord (x-1) y
    | otherwise     = Coord x y
moveKey (Coord x y) 'R'
    | x < 2         = Coord (x+1) y
    | otherwise     = Coord x y
moveKey (Coord x y) 'D'
    | y < 2         = Coord x (y+1)
    | otherwise     = Coord x y
moveKey (Coord x y) 'U'
    | y > 0         = Coord x (y-1)
    | otherwise     = Coord x y
moveKey _ direction = error $ "Invalid direction: " ++ [direction]

mapKey :: Coord -> Int
mapKey (Coord 0 0) = 1
mapKey (Coord 1 0) = 2
mapKey (Coord 2 0) = 3
mapKey (Coord 0 1) = 4
mapKey (Coord 1 1) = 5
mapKey (Coord 2 1) = 6
mapKey (Coord 0 2) = 7
mapKey (Coord 1 2) = 8
mapKey (Coord 2 2) = 9
mapKey coord = error $ "Invalid coordinates " ++ show coord
