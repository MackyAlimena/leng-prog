module Trie (Trie(..), left, right, find, decode, toList) where

import Bit

data Trie a = Leaf a | (Trie a) :-: (Trie a) deriving (Eq, Show)

left :: Trie a -> Trie a
left (Leaf _) = error "Left of: Leaf"
left (a :-: _) = a

right :: Trie a -> Trie a
right (Leaf _) = error "Right of: Leaf"
right (_ :-: a) = a

find :: Bits -> Trie a -> a
find [] (Leaf a) = a
find (x:xs) (l :-: r)
    | x == F = find xs l
    | x == T = find xs r
find _ _ = error "Pattern not matched"

decode :: Bits -> Trie Char -> String
decode bits trie = decodeAux bits trie trie

decodeAux :: Bits -> Trie Char -> Trie Char -> String
decodeAux [] (Leaf a) _ = [a]
decodeAux bits (Leaf a) trie = a : decodeAux bits trie trie
decodeAux (x:xs) (l :-: r) trie
    | x == F = decodeAux xs l trie
    | x == T = decodeAux xs r trie
decodeAux _ _ _ = error "Pattern not matched"

toList :: Trie a -> [(a, Bits)]
toList t = toListAux t []

toListAux :: Trie a -> Bits -> [(a, Bits)]
toListAux (Leaf a) bits = [(a, bits)]
toListAux (l :-: r) bits = toListAux l (bits ++ [F]) ++ toListAux r (bits ++ [T])