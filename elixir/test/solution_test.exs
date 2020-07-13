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

  # p0004_median_two_sorted_arrays.ex
  test "median two sorted arrays" do
    assert 2 == MedianTwoSortedArrays.run([1, 3], [2])
    assert 2.5 == MedianTwoSortedArrays.run([1, 2], [3, 40])
    assert 4 == MedianTwoSortedArrays.run([1, 2, 3, 4, 7], [0, 5, 6, 9])
    assert 4.5 == MedianTwoSortedArrays.run([1, 2, 3, 4, 7], [0, 5, 6, 9, 100])
  end

  test "longest palindrome" do
    assert "b" == LongestPalindromeSubstring.run("ab")
    assert "aa" == LongestPalindromeSubstring.run("aab")
    assert "aba" == LongestPalindromeSubstring.run("babad")
    assert "bb" == LongestPalindromeSubstring.run("cbbd")
    assert "racecar" == LongestPalindromeSubstring.run("racecars")
  end

  test "zigzag conversion" do
    assert "PAYPALISHIRING" == ZigZagConversion.run("PAYPALISHIRING", 1)
    assert "PYAIHRNAPLSIIG" == ZigZagConversion.run("PAYPALISHIRING", 2)
    assert "PAHNAPLSIIGYIR" == ZigZagConversion.run("PAYPALISHIRING", 3)
    assert "PINALSIGYAHRPI" == ZigZagConversion.run("PAYPALISHIRING", 4)
  end

  test "reverse integer" do
    assert 321 == ReverseInt.run(123)
    assert -321 == ReverseInt.run(-123)
    assert 21 == ReverseInt.run(120)
    assert 8_463_847_412 == ReverseInt.run(floor(:math.pow(2, 31)))
    assert 0 == ReverseInt.run(floor(:math.pow(2, 31) + 1))
  end

  test "atoi" do
    assert 42 == Atoi.run("42")
    assert -42 == Atoi.run("    -42")
    assert 4193 == Atoi.run("4193 with words")
    assert 0 == Atoi.run("words and 987")
    assert -2_147_483_648 == Atoi.run("-91283472332")
  end

  test "integer palindrome" do
    assert true == IntPalindrome.run(121)
    assert false == IntPalindrome.run(-121)
    assert false == IntPalindrome.run(10)
    assert true == IntPalindrome.run(12321)
    assert false == IntPalindrome.run(12351)
    assert true == IntPalindrome.run(1881)
    assert false == IntPalindrome.run(1981)
  end
end
