defmodule IntPalindrome do
  # Handle special cases
  def run(val) when val < 0 or (rem(val, 10) == 0 and val != 0), do: false

  def run(val) when is_integer(val) do
    {to, from} =
      Enum.reduce_while(Stream.cycle([:ok]), {0, val}, fn _k, {to, from} ->
        cond do
          from > to ->
            to = to * 10 + rem(from, 10)
            from = div(from, 10)
            {:cont, {to, from}}
          true ->
            {:halt, {to, from}}
        end
      end)
    
    # Compare the two halves of the palindrome
    (from == to) or (from == div(to, 10))
  end
end
