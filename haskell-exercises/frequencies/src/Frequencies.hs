module Frequencies  (Frequency, frequencyMap, frequencies, insert, insertionSort) where

import Data.Map(Map)
import qualified Data.Map as Map
import Data.Tuple(swap)

import System.Environment (getArgs)
import System.IO (readFile)

main :: IO ()
main = do
    args <- getArgs
    if null args
        then putStrLn "Error: No file name provided."
        else do
            let fileName = head args
            content <- readFile fileName
            let freqList = frequencies content
            mapM_ print (reverse freqList)

type Frequency = (Int, Char)

frequencies::String -> [Frequency]
frequencies [] = []
frequencies str = insertionSort $ map swap $ Map.toList $ frequencyMap str

frequencyMap::(Ord a) => [a] -> Map a Int
frequencyMap [] = Map.empty
frequencyMap (x:xs) = Map.insert x (counter + 1) (frequencyMap xs)
            where counter = Map.findWithDefault 0 x (frequencyMap xs)

insert::(Ord a) => a -> [a] -> [a]
insert a [] = [a]
insert a xs = [x | x <- xs, x < a] ++ [a] ++ [x | x <- xs, x >= a]

insertionSort :: (Ord a) => [a] -> [a]
insertionSort [] = []
insertionSort [x] = [x]
insertionSort (x:xs) = insert x (insertionSort xs)


