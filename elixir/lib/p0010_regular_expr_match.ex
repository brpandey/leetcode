_ = """
10. Regular Expression Matching
Hard

Given an input string (s) and a pattern (p), implement regular expression matching with support for '.' and '*'.

'.' Matches any single character.
'*' Matches zero or more of the preceding element.

The matching should cover the entire input string (not partial).

Note:

    s could be empty and contains only lowercase letters a-z.
    p could be empty and contains only lowercase letters a-z, and characters like . or *.

Example 1:

Input:
s = "aa"
p = "a"
Output: false
Explanation: "a" does not match the entire string "aa".

Example 2:

Input:
s = "aa"
p = "a*"
Output: true
Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".

Example 3:

Input:
s = "ab"
p = ".*"
Output: true
Explanation: ".*" means "zero or more (*) of any character (.)".

Example 4:

Input:
s = "aab"
p = "c*a*b"
Output: true
Explanation: c can be repeated 0 times, a can be repeated 1 time. Therefore, it matches "aab".

Example 5:

Input:
s = "mississippi"
p = "mis*is*p*."
Output: false


"""

defmodule RegularExprMatch do
  def run(string, pattern) when is_binary(string) and is_binary(pattern) do
    s = string |> String.graphemes()
    p = pattern |> String.graphemes()

    match(true, s, p)
  end

  # Base cases
  # If the past current char didn't match the past current pattern char, then false
  def match(false, _, _), do: false
  # If there's no regular expression left and the string is empty than true
  def match(true, [], []), do: true
  # If there's no regular expression left but the string hasn't terminated than false
  def match(true, _, []), do: false
  # if s is [], and p is X*, p can shrink to zero occurences of X which equal []
  def match(true, [], [_h | ["*" | []]]), do: true

  ##################################################################################
  # Tail recursive cases
  # 1) Asterisk case
  def match(true, [s_head | s_tail] = s, [p_h1 | ["*" | p_t]] = p) do
    # E.g. s = "aab" so s = ["a" | ["a", "b"]]
    # E.g. p = "c*a*b" so p = ["c" | ["*" | [a*b..]]]
    same = s_head == p_h1 or "." == p_h1

    # 1) Shift pattern to char after '*" (0 occurences case)
    # Given s = "aab", p = "c*a*b"
    # 2) Shift string text a char right (1 or more occurences case)
    # Given s = "ab", p = "a*"
    match(true, s, p_t) or match(same, s_tail, p)
  end

  # 2) Default case
  def match(true, [s_head | s_tail], [p_head | p_tail]) do
    same = s_head == p_head or "." == p_head
    # If we don't have an asterisk, assuming same is true we recurse to new lists:
    # where we've shifted the string text and pattern to their next element
    match(same, s_tail, p_tail)
  end
end
