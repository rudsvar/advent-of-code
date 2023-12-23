import Data.Char (isAlpha, isDigit)
import Data.List (splitAt)
import Debug.Trace (trace)

data Game = Game
  { gameId :: Int,
    sets :: [Set]
  }
  deriving (Show)

data Set = Set
  { red :: Int,
    green :: Int,
    blue :: Int
  }
  deriving (Show)

main :: IO ()
main = interact $ show . sum . map gameId . filter validGame . map parseGame . lines

validGame :: Game -> Bool
validGame g = validSets (sets g)

validSets :: [Set] -> Bool
validSets = all validSet

maxSet :: Set
maxSet = Set {red = 12, green = 13, blue = 14}

validSet :: Set -> Bool
validSet set = red set <= red maxSet && green set <= green maxSet && blue set <= blue maxSet

parseGame :: String -> Game
parseGame s = Game {gameId = read id, sets = parseSets rest}
  where
    withoutGame = dropWhile (not . isDigit) s
    (id, ':' : ' ' : rest) = span isDigit withoutGame

parseSets :: String -> [Set]
parseSets [] = []
parseSets s = map parseSet (sepBy ';' s)

findCount :: String -> [(Int, String)] -> Int
findCount _ [] = 0
findCount target ((count, name) : xs)
  | name == target = count
  | otherwise = findCount target xs

parseSet :: String -> Set
parseSet s = Set {red = red, green = green, blue = blue}
  where
    red = findCount "red" counts
    green = findCount "green" counts
    blue = findCount "blue" counts
    counts = parseCounts s

sepBy :: Char -> String -> [String]
sepBy sep [] = []
sepBy sep s = target : sepBy sep restWithoutSep
  where
    (target, rest) = span (/= sep) s
    restWithoutSep = dropWhile (== sep) rest

parseCounts :: String -> [(Int, String)]
parseCounts s = map (parseCount . trim) (sepBy ',' s)

parseCount :: String -> (Int, String)
parseCount s = (read count, trim rest)
  where
    (count, rest) = span isDigit s

trim :: String -> String
trim = reverse . dropWhile (== ' ') . reverse . dropWhile (== ' ')
