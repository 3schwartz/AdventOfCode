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
parseDirection cityGrid input = do
    let (newDirection, parsed) = getMovement (snd cityGrid) input
    let position = fst cityGrid
    let newPosition = (fst position + fst newDirection * parsed, snd position + snd newDirection * parsed)
    (newPosition, newDirection)

getMovement :: (Int, Int) -> String -> ((Int, Int), Int)
getMovement direction input = case splitAt 1 input of
    ("L", rest) -> ((-snd direction, fst direction), read rest)
    ("R", rest) -> ((snd direction, -fst direction), read rest)
    _ -> ((0,0), 0)    

manhattanDistance :: (Int, Int) -> Int
manhattanDistance (x, y) = abs x + abs y
