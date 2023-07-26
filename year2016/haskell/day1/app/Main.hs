module Main where

import Data.List (foldl')
import Data.List.Split (splitOn)

main :: IO ()
main = do
    contents <- readFile "../../data/day1_data.txt"
    let splitted = splitOn ", " contents
    let result = foldl' parseDirection ((0,0), (0,1)) splitted
    let distance = manhattanDistance (fst result)
    print distance


parseDirection :: ((Int, Int), (Int, Int)) -> String -> ((Int, Int), (Int, Int))
parseDirection cityGrid input = case splitAt 1 input of
    ("L", rest) ->
        let parsed = read rest
            position = fst cityGrid
            direction = snd cityGrid
            newDirection = (-snd direction, fst direction)
            newPosition = (fst position + fst newDirection * parsed, snd position + snd newDirection * parsed)
        in (newPosition, newDirection)
    ("R", rest) ->
        let parsed = read rest
            position = fst cityGrid
            direction = snd cityGrid
            newDirection = (snd direction, -fst direction)
            newPosition = (fst position + fst newDirection * parsed, snd position + snd newDirection * parsed)
        in (newPosition, newDirection)        
    _ -> ((0,0), (0,0))

manhattanDistance :: (Int, Int) -> Int
manhattanDistance (x, y) = abs x + abs y
