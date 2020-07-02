_ = """
6. ZigZag Conversion
Medium

The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)

P   A   H   N
A P L S I I G
Y   I   R

And then read line by line: "PAHNAPLSIIGYIR"

Write the code that will take a string and make this conversion given a number of rows:

string convert(string s, int numRows);

Example 1:

Input: s = "PAYPALISHIRING", numRows = 3
Output: "PAHNAPLSIIGYIR"

Example 2:

Input: s = "PAYPALISHIRING", numRows = 4
Output: "PINALSIGYAHRPI"
Explanation:

P     I    N
A   L S  I G
Y A   H R
P     I
"""

defmodule ZigZagConversion do
  def run(str, n) when n == 1 or n == length(str), do: str

  def run(str, n) when is_binary(str) and is_integer(n) do
    steps = Stream.concat(0..(n - 1), (n - 2)..1) |> Stream.cycle()
    zipped = Stream.zip(String.graphemes(str), steps)

    # Prepend to list if not already there (prepend is faster for lists)
    # Then, combine elements to string form (append is faster for strings, when not using iolists)
    zipped
    |> Enum.reduce(%{}, fn {char, row}, map_acc ->
      Map.update(map_acc, row, [char], &List.flatten([char], &1))
    end)
    |> Enum.reduce("", fn {_k, v}, str_acc ->
      str_acc <> (Enum.reverse(v) |> Enum.join())
    end)
  end
end
