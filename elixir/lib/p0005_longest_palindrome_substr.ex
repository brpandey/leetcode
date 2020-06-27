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

  def palindrome(array, left, right, len)
      when is_integer(left) and is_integer(right) and is_integer(len) do
    # Functional version of a while loop
    # Generate a lazy stream of palindrome boundary values first and then consume with our while condition
    Stream.iterate({left, right}, fn {l, r} -> {l - 1, r + 1} end)
    |> Enum.reduce_while([], fn {l, r}, acc ->
      # Are the valid indice values a mirror of each other?
      if l >= 0 and r < len and :array.get(l, array) == :array.get(r, array) do
        {:cont, acc}
      else
        {:halt, {r - l - 1, l + 1, r - 1}}
      end
    end)
  end
end
