defmodule Util do
  def fuel_needed(mass) do
    Integer.floor_div(mass, 3) - 2
  end
end

{:ok, input} = File.read("../input/01.txt")

lines = String.split(input)

answer_1 = Enum.sum(Enum.map(lines, fn x -> Util.fuel_needed(String.to_integer(x)) end))

IO.puts(answer_1)
