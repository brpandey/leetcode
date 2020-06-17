defmodule AddTwoNumbers do
  @moduledoc """
  https://leetcode.com/problems/add-two-numbers/

  You are given two non-empty linked lists representing two non-negative integers.
  The digits are stored in reverse order and each of their nodes contain a single digit.
  Add the two numbers and return it as a linked list.

  You may assume the two numbers do not contain any leading zero, except the number 0 itself.

  Example:

  Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
  Output: 7 -> 0 -> 8
  Explanation: 342 + 465 = 807.
  """

  def run(list1, list2) when length(list1) < length(list2) do
    pad = pad_helper(list1, list2)
    run(list1 ++ pad, list2)
  end

  def run(list1, list2) when length(list1) > length(list2) do
    pad = pad_helper(list1, list2)
    run(list1, list2 ++ pad)
  end

  def run(list1, list2)
      when is_list(list1) and is_list(list2) and length(list1) == length(list2) do
    zipped = Enum.zip(list1, list2)

    {_, list} =
      Enum.reduce(zipped, {0, []}, fn {v1, v2}, {carry_acc, result_acc} ->
        sum = Integer.mod(v1 + v2 + carry_acc, 10)
        result_acc = [sum | result_acc]
        carry_acc = Kernel.div(v1 + v2 + carry_acc, 10)
        {carry_acc, result_acc}
      end)

    list |> Enum.reverse()
  end

  defp pad_helper(list1, list2) when is_list(list1) and is_list(list2) do
    len1 = length(list1)
    len2 = length(list2)

    pad_size = Kernel.abs(len1 - len2)
    Stream.cycle([0]) |> Enum.take(pad_size)
  end
end
