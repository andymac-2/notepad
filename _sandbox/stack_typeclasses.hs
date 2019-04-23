import Data.List

class (Foldable s) => Stack s where
    push :: a -> s a -> s a
    pop :: s a -> Maybe (a, s a)

foldrStack :: Stack s => (a -> b -> b) -> b -> s a -> b
foldrStack f z s = case pop s of
    Nothing -> z
    Just (x, s') ->  x `f` (foldrStack f z s')

instance Stack [] where
    push = (:)
    pop = uncons

data Deque a = Deque [a] [a] deriving Show
instance Foldable Deque where
    foldr = foldrStack

instance Stack Deque where
    push y (Deque xs ys) = Deque xs (y: ys)

    pop (Deque [] []) = Nothing
    pop (Deque xs (y: ys)) = Just (y, Deque xs ys)
    pop (Deque xs []) = pop (Deque [] (reverse xs))