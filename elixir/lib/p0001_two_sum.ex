defmodule TwoSum do
  @moduledoc """
  https://leetcode.com/problems/two-sum/

  Given an array of integers, return indices of the two numbers such that they add up to a specific target.

  You may assume that each input would have exactly one solution, and you may not use the same element twice.

  Example:

  Given nums = [2, 7, 11, 15], target = 9,

  Because nums[0] + nums[1] = 2 + 7 = 9,
  return [0, 1].
  """

  def run(nums, target)
      when is_list(nums) and is_integer(hd(nums)) and is_integer(target) do
    map = nums |> Enum.with_index() |> Map.new()

    Enum.reduce_while(map, [], fn {k, v}, acc ->
      k2 = target - k

      if k2 > 0 and Map.has_key?(map, k2) do
        v2 = Map.get(map, k2)
        {:halt, [v | [v2 | acc]]}
      else
        {:cont, acc}
      end
    end)
  end
end
