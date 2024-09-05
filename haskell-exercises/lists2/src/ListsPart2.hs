module ListsPart2 (Bit(..), bitAt, charToBits, bits, queens) where

import Data.Char(ord)  
import Data.Bits(testBit)
  
data Bit = F | T  deriving (Eq, Show, Enum, Read)
type Bits = [Bit]

bitAt :: Int -> Char -> Bit
bitAt n c = if testBit (ord c) (7-n) then T else F

charToBits :: Char -> Bits
charToBits c = [bitAt i c | i <- [0..7]]

bits :: String -> Bits
bits = foldr (\c acc -> charToBits c ++ acc) []

type Solution = [Int]

queens :: Int -> [Solution]
queens n = solve n
  where
    solve 0 = [[]]
    solve k = [q:qs | qs <- solve (k-1), q <- [1..n], safe q qs]

    safe q qs = not (q `elem` qs || sameDiag q qs)
    sameDiag q qs = any (\(colDist, q') -> abs (q - q') == colDist) (zip [1..] qs)