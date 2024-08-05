module CollatzConjecture (collatz) where

collatz :: Integer -> Maybe Integer
collatz n
  | n <= 0    = Nothing
  | n == 1    = Just 0
  | even n    = case collatz (n `div` 2) of
                  Just steps -> Just (steps + 1)
                  Nothing    -> Nothing
  | otherwise = case collatz (3 * n + 1) of
                  Just steps -> Just (steps + 1)
                  Nothing    -> Nothing

-- or:
-- collatz n
--  | n <= 0    = Nothing
--   | n == 1    = Just 0
--   | even n    = fmap (+1) (collatz (n `div` 2))
--   | otherwise = fmap (+1) (collatz (3 * n + 1))