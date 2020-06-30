defmodule LongestSubstrNoRepeat do
  @moduledoc """
  Longest Substring Without Repeating Characters

  Given a string, find the length of the longest substring
  without repeating characters

  Example 1:
  Input: "abcabcbb"
  Output: 3
  Explanation: The answer is "abc" with the length of 3.

  Example 2:
  Input: "bbbbb"
  Output: 1
  Explanation: The answer is "b", with the length of 1

  Example 3:
  Input "pwwkew"
  Output: 3
  Explanation: The answer is "wke", with the length of 3.

  Note that the answer must be a substring "pwke" is a
  subsequence and not a substring
  """

  def run(str) when is_binary(str) do
    list = String.codepoints(str)

    # p w w k e w
    {longest, _, _, _} =
      Enum.reduce(list, {%{}, %{}, 0, 0}, fn k, {longest_acc, current_acc, l_size, c_size} ->
        {current_acc, c_size} =
          case Map.has_key?(current_acc, k) do
            # Reset the current map since k is already part of the subsequence map
            # But with k as the new entry
            # and we can't have repeating characters
            true -> {%{k => 0}, 1}
            false -> {Map.put(current_acc, k, c_size), c_size + 1}
          end

        case c_size > l_size do
          true -> {current_acc, current_acc, c_size, c_size}
          false -> {longest_acc, current_acc, l_size, c_size}
        end
      end)

    #    IO.puts("Explanation: The answer is \"#{inspect(longest)}\"")

    Kernel.map_size(longest)
  end
end
