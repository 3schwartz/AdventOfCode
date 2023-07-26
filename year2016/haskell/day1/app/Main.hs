module Main where

import Data.List (foldl')
import Data.List.Split (splitOn)
import qualified Data.Set as S

main :: IO ()
main = do
    contents <- readFile "../../data/day1_data.txt"
    let splitted = splitOn ", " contents
        initialState = stateNew (coordNew 0 0) (coordNew 0 1)

        part1 = foldl' parseDirection initialState splitted
        distance = manhattanDistance (position part1)
    putStrLn $ "Part 1: " ++ show distance
    
    let stateWithSet = stateWithSetNew (stateNew (coordNew 0 0) (coordNew 0 1)) (S.empty)
        part2 = returnIfMultipleVisits stateWithSet splitted
        distancePart2 = manhattanDistance (position (state part2))
    putStrLn $ "Part 2: " ++ show distancePart2

data Coord = Coord { x::Int, y::Int }
    deriving (Show, Eq, Ord)
coordNew :: Int -> Int -> Coord
coordNew x y = Coord { x = x, y = y }

data StateWithSet = StateWithSet { state :: State, visited :: S.Set Coord }
    deriving (Show)
stateWithSetNew :: State -> S.Set Coord -> StateWithSet
stateWithSetNew state visited = StateWithSet { state = state, visited = visited }

getState :: StateWithSet -> State
getState (StateWithSet state _ ) = state

data State = State {position :: Coord, direction :: Coord }
    deriving (Show)

stateNew :: Coord -> Coord -> State
stateNew position direction = State { position = position, direction = direction }

returnIfMultipleVisits :: StateWithSet -> [String] -> StateWithSet
returnIfMultipleVisits stateWithSet [] = stateWithSet
returnIfMultipleVisits (StateWithSet state visited) (input : inputs)
    | S.notMember (position state) visited =
        let updated = S.insert (position state) visited
            (newDirection, parsed) = getMovement (direction state) input
            newPosition = Coord { x = x (position state) + x newDirection
                                , y = y (position state) + y newDirection}
            (newState, found) = move (stateWithSetNew (stateNew newPosition newDirection) updated) (parsed - 1)
        in
            case found of 
                True -> newState
                False -> returnIfMultipleVisits newState inputs
    | otherwise = stateWithSetNew state visited

move :: StateWithSet -> Int -> (StateWithSet, Bool)
move stateWithSet 0 = (stateWithSet, False)
move (StateWithSet state visited) count
    | S.member (position state) visited = ((stateWithSetNew state visited), True) 
    | otherwise =
        let updated = S.insert (position state) visited
            newPosition = Coord { x = x (position state) + x (direction state)
                                , y = y (position state) + y (direction state)}
            newState = stateWithSetNew (stateNew newPosition (direction state)) updated
        in move newState (count - 1)


parseDirection :: State -> String -> State
parseDirection state input =
    let (newDirection, parsed) = getMovement (direction state) input
        newPosition = Coord { x = x (position state) + x newDirection * parsed
                            , y = y (position state) + y newDirection * parsed}
    in State { position = newPosition, direction = newDirection }

getMovement :: Coord -> String -> (Coord, Int)
getMovement direction input = case splitAt 1 input of
    ("L", rest) -> (Coord { x = -y direction, y = x direction}, read rest)
    ("R", rest) -> (Coord { x = y direction, y = -x direction}, read rest)
    _ -> ((coordNew 0 0), 0)

manhattanDistance :: Coord -> Int
manhattanDistance (Coord x y) = abs x + abs y
