module Shape where

data Point = Point { x::Double, y:: Double} deriving (Eq, Show)

data Circle    = Circle    Point Double deriving (Eq, Show)
data Rectangle = Rectangle Point Point deriving (Eq, Show)


-- A point from a tuple Pair
point::(Double, Double) -> Point
point (a,b) = Point{x=a, y=b}

-- The origin
origin::Point
origin = point (0,0)

-- Rectangle from a Tuple where (x0 y0) == origin
rectangle::(Double, Double) -> Rectangle
rectangle (a,b) = Rectangle origin Point{x=a, y=b}

base::Rectangle -> Double
base (Rectangle pointA pointB) = (x pointB) - (x pointA)

height::Rectangle -> Double
height (Rectangle pointA pointB) = (y pointB) - (y pointA)

pointA::Rectangle -> Point
pointA (Rectangle pointA _)= pointA

pointB::Rectangle -> Point
pointB (Rectangle _ pointB)= pointB

-- Circle from radius
circle::Double -> Circle
circle a = Circle origin a

-- Class Shift

class Shift a where
   shift::a -> (Double, Double) -> a
   
instance Shift Point where
   shift  pt (a,b) = point (a+(x pt),b+(y pt))
   
instance Shift Rectangle where
   shift rectangle (a,b)= Rectangle (point (a,b)) (point (a+(x (pointB rectangle)),b+(y (pointB rectangle))))
   
instance Shift Circle where
   shift (Circle _ r) (a,b) = Circle (Point{x=a, y=b}) r
   
-- Define the Surface class

class Surface a where
    surface :: a -> Double

instance Surface Rectangle where
     surface rectangle = (base rectangle) * (height rectangle)

instance Surface Circle where
      surface (Circle _ r) = pi * r * r
   
