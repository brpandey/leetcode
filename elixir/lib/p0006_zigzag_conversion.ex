defmodule ZigZagConversion do
  def run(str, n) when n == 1 or n == length(str), do: str

  def run(str, n) when is_binary(str) and is_integer(n) do
    steps = Stream.concat(0..(n - 1), (n - 2)..1) |> Stream.cycle()
    zipped = Stream.zip(String.codepoints(str), steps)

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
