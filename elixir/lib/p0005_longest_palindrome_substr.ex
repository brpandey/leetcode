defmodule LongestPalindromeSubstring do
  @moduledoc """
  Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.

  Example 1:

  Input: "babad"
  Output: "bab"
  Note: "aba" is also a valid answer.

  Example 2:

  Input: "cbbd"
  Output: "bb"
  """

  def run(str) when byte_size(str) == 1, do: str

  def run(str) when is_binary(str) do
    # {len, low index, high index}
    longest = {1, 0, 1}
    length = String.length(str)
    list = String.codepoints(str)
    array = :array.from_list(list)

    {{_len, low, high}, _} =
      Enum.reduce(list, {longest, 0}, fn _c, {acc, i} ->
        # Two palindrome passes
        # For odd length strings e.g. aab, centered at i
        # For even length strings e.g. cbbd, centered at i, i+1
        val1 = palindrome(array, i, i, length)
        val2 = palindrome(array, i, i + 1, length)

        acc = Kernel.max(val1, val2) |> Kernel.max(acc)
        {acc, i + 1}
      end)

    String.slice(str, low..high)
  end

  def palindrome(array, low, high, len) do
    # Functional version of a while loop using composable streams
    # "Generating" only empty lists and a final tuple value for the sequence list
    # Revert l and h back to the prev iteration on final value
    # Including a length value in the final tuple for easier term ordering later
    Stream.unfold({low, high, false}, fn {l, h, stop} ->
      cond do
        # Ends unfold
        stop ->
          nil

        # Are the valid indice values a mirror of each other?
        l >= 0 and h < len and :array.get(l, array) == :array.get(h, array) ->
          {[], {l - 1, h + 1, false}}

        true ->
          {{h - l - 1, l + 1, h - 1}, {nil, nil, true}}
      end
    end)
    |> Stream.filter(fn x -> x != [] end)
    |> Enum.take(len)
    |> List.first()
  end
end
