module Lists (member, union, intersection, difference,
              insert, insertionSort,
              binaryToDecimal, toDecimal, toDec, decimal, firsts) where
  
import Data.Char(digitToInt)  

member:: Int -> [Int] -> Bool
member _ []      = False
member e (x:xs)  = e == x || member e xs


union:: [Int] -> [Int] -> [Int]
union [] ys     = ys
union (x:xs) ys 
  | member x ys = union xs ys
  | otherwise   = x : union xs ys

intersection:: [Int] -> [Int] -> [Int]
intersection xs ys = [x | x <- xs, member x ys]

difference:: [Int] -> [Int] -> [Int]
difference xs ys = [x | x <- xs, not (member x ys)]

insert:: Int -> [Int] -> [Int]
insert a xs = [x | x <- xs, x < a] ++ [a] ++ [x | x <- xs, x >= a]

insertionSort :: [Int] -> [Int]
insertionSort [] = []
insertionSort (x:xs) = insert x (insertionSort xs)

binaryToDecimal :: [Int] -> Int
binaryToDecimal xs = sum [x * 2 ^ i | (x, i) <- zip (reverse xs) [0..]]

reverseList :: [Int] -> [Int]
reverseList [] = []
reverseList (x:xs) = reverseList xs ++ [x]

toDecimal :: Int -> [Int] -> Int
toDecimal base xs = sum [x * base ^ i | (x, i) <- zip (reverse xs) [0..]]

toDec :: Int -> String -> Int
toDec base s = toDecimal base (map digitToInt s)

-- Same as `toDec` But use a list comprehension

decimal::Int -> String -> Int
decimal base s = sum [digitToInt x * base ^ i | (x, i) <- zip (reverse s) [0..]]

--using list comprehension
firsts :: [a] -> [[a]]
firsts xs = [take n xs | n <- [1..length xs]]

--using recursion
firsts :: [a] -> [[a]]
firsts [] = []
firsts xs = firsts (init xs) ++ [xs]

-- Given two String that represents numbers in binary implement the 'binaryAdd' function
-- DO NOT USE a predefined '+' operation

--binaryAdd::String -> String -> String
--binaryAdd  = error "Implement it"