defmodule SolutionTest do
  use ExUnit.Case

  # p0001_two_sum.ex
  test "two_sum" do
    assert [0, 1] == TwoSum.run([2, 7, 11, 15], 9)
    assert [1, 4] == TwoSum.run([7, 1, 10, 22, 14, 6], 15)
    assert [1, 4] == TwoSum.run([7, 1, 10, 22, 14, 6, 8], 15)
  end

  # p0002_add_two_numbers.ex
  test "add_two_numbers" do
    assert [7, 0, 8] == AddTwoNumbers.run([2, 4, 3], [5, 6, 4])
    assert [7, 0, 8, 9] == AddTwoNumbers.run([2, 4, 3], [5, 6, 4, 9])
    assert [7, 0, 8, 7] == AddTwoNumbers.run([2, 4, 3, 7], [5, 6, 4])
  end

  # p0003_longest_substr_no_repeat.ex
  test "longest_substr_no_repeat" do
    assert 3 == LongestSubstrNoRepeat.run("pwwkew")
    assert 3 == LongestSubstrNoRepeat.run("abcabcbb")
    assert 1 == LongestSubstrNoRepeat.run("bbbbb")
  end
end
