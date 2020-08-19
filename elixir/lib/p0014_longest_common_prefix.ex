defmodule LongestCommonPrefix do
  @moduledoc """
  14. Longest Common Prefix
  Easy

  Write a function to find the longest common prefix string amongst an array of strings.

  If there is no common prefix, return an empty string "".

  Example 1:

  Input: ["flower","flow","flight"]
  Output: "fl"

  Example 2:

  Input: ["dog","racecar","car"]
  Output: ""
  Explanation: There is no common prefix among the input strings.

  Note:

  All given inputs are in lowercase letters a-z.
  """

  def run(strings) when is_list(strings) and is_binary(hd(strings)) do
    list =
      Enum.reduce(strings, [], fn str, acc ->
        gp = String.graphemes(str)
        [gp | acc]
      end)

    # strings -> ["flower","flow","flight"]
    # list -> [["f", "l", "o", "w", "e", "r"], ["f", "l", "o", "w"], ["f", "l", "i"]]

    # l grows smaller as we build up prefix
    # l is the list of lists, we cycle through each of the inner lists dropping the head
    {_, prefix} =
      Enum.reduce_while(Stream.cycle([:ok]), {list, []}, fn _, {l, prefix} ->
        # pick first char in first string of list as supposed common char
        common = List.first(l) |> List.first()

        # construct the shorter list of lists
        new_list =
          Enum.reduce_while(l, [], fn
            [^common | t], new_list ->
              {:cont, [t | new_list]}

            _list, _ ->
              {:halt, []}
          end)

        # Based on the result, either add to the prefix value or
        # abort and return current prefix
        case new_list do
          [] -> {:halt, {[], prefix}}
          nl -> {:cont, {nl, [common | prefix]}}
        end
      end)

    Enum.reverse(prefix) |> Enum.join("")
  end
end
