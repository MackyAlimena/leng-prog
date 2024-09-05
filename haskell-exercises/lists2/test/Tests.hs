import Test.Hspec    (describe, hspec, it, shouldBe)
import ListsPart2 (Bit(..),  charToBits, bits, queens)

main :: IO ()
main = hspec $ do
  describe "charToBits" $ do
    it "A" $ do
      charToBits 'A' `shouldBe` [F,T,F,F,F,F,F,T]
    it "B" $ do
      charToBits 'B' `shouldBe` [F,T,F,F,F,F,T, F]
    it "Char 0" $ do
      charToBits '\0' `shouldBe` [F,F,F,F,F,F,F,F]
      
      
  describe "bits" $ do
    it "AB" $ do
      bits "AB" `shouldBe` [F,T,F,F,F,F,F,T,F,T,F,F,F,F,T,F]
    it "\"\"" $ do
      bits "" `shouldBe` []
      

               