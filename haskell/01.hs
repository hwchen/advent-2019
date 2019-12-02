import System.IO

fuel_needed:: Integer -> Integer
fuel_needed mass = (quot mass 3) - 2

fuel_needed_recursive:: Integer -> Integer
fuel_needed_recursive mass = sum . fuel_needed_inner $ [fuel_needed mass]

fuel_needed_inner :: [Integer] -> [Integer]
fuel_needed_inner [] = []
fuel_needed_inner all @ (m:masses) = if n > 0 then fuel_needed_inner(n:all) else all
    where n = fuel_needed m

main = do
    input <- readFile "../input/01.txt"
    print . sum . map (\s -> fuel_needed . read $ s :: Integer) . words $ input
    print . sum . map (\s -> fuel_needed_recursive . read $ s :: Integer) . words $ input
