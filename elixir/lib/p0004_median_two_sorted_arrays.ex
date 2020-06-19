defmodule MedianTwoSortedArrays do
  require Integer

  @moduledoc """
  https://leetcode.com/problems/median-of-two-sorted-arrays/

  There are two sorted arrays nums1 and nums2 of size m and n respectively.
  Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).
  You may assume nums1 and nums2 cannot be both empty.

  Example 1:

  nums1 = [1, 3]
  nums2 = [2]

  The median is 2.0

  Example 2:

  nums1 = [1, 2]
  nums2 = [3, 4]

  The median is (2 + 3)/2 = 2.5
  """

  # https://www.drdobbs.com/parallel/finding-the-median-of-two-sorted-arrays/240169222
  # https://www.youtube.com/watch?v=LPFhl65R7ww

  # Since Elixir doesn't support inf/ negative infinity let's use the limits
  # for 32 bit signed integers here
  @negative_inf (:math.pow(2, 31) - 1) * -1
  @positive_inf :math.pow(2, 31) - 1

  def run(list1, list2)
      when is_list(list1) and is_list(list2) and length(list1) > length(list2) do
    run(list2, list1)
  end

  def run(list1, list2)
      when is_list(list1) and is_list(list2) and length(list1) <= length(list2) do
    len_x = length(list1)
    len_y = length(list2)

    partition(list1, list2, 0, len_x, len_x, len_y)
  end

  def partition(_list1, _list2, low, high, _len_x, _len_y) when low > high,
    do: IO.puts("Error: low can't be greater than high")

  def partition(list1, list2, low, high, len_x, len_y) when low <= high do
    # We use the smallest list - list1 - as the starting point and try good faith to pick a median value
    # and see if it will stick otherwise we will refine our guess with other attempts (represented by the tail recursive calls)

    # Since we are pursuing a more optimal solution than O(n) brute force merge of both lists/arrays,
    # we construct "two virtual halves" of the two arrays hopefully representing a well matched division
    # while the elements reside in their original respective sorted arrays

    # After we choose our partition x, we basically compute whatever partition y value
    # for list2 so that we have a basically balanced set
    # So essentially left half (x + y) is more or less equal right (x + y)

    # We are able to achieve the log(n+m) runtime efficiency by basically halving these values
    # on each iteration or partition tail recursive call.  The direct O(1) indexing via arrays enables that
    partition_x = Kernel.div(low + high, 2)
    partition_y = Kernel.div(len_x + len_y + 1, 2) - partition_x

    # Generate bounding box of four values which represent the median box boundary
    # which are just the corresponding values to the partition value indices

    # List1 (x)
    # In the edge cases where max_left has no set of values we use negative inf as a placeholder
    # and similiarily for min_right
    max_left_x = if partition_x > 0, do: Enum.at(list1, partition_x - 1), else: @negative_inf
    min_right_x = if partition_x == len_x, do: @positive_inf, else: Enum.at(list1, partition_x)

    # List2 (y)
    # In the edge cases where max_left has no set of values we use negative inf as a placeholder
    # and similiarily for min_right
    max_left_y = if partition_y > 0, do: Enum.at(list2, partition_y - 1), else: @negative_inf
    min_right_y = if partition_y == len_y, do: @positive_inf, else: Enum.at(list2, partition_y)

    cond do
      # Case that confirms that our "bounding box" is correctly chosen -- it is cohesive and well stitched
      # Essentially everything in the set of x is less than the set of y
      # The values within the bounding box are exactly in the biggest minimal
      # middle or median set of the two arrays where we can cleanly pick out the correct median value
      max_left_x <= min_right_y and min_right_x >= max_left_y ->
        case Integer.is_even(len_x + len_y) do
          true ->
            # Virtual left half and right half across both arrays
            v_left = Kernel.max(max_left_x, max_left_y)
            v_right = Kernel.min(min_right_x, min_right_y)
            (v_left + v_right) / 2

          false ->
            Kernel.max(max_left_x, max_left_y)
        end

      # Note: in sorted list/array left values are smaller and right values are bigger
      # Note again: We are able to achieve the log(n+m) runtime efficiency by basically halving these low and high values
      # on each iteration or partition tail recursive call.  The direct O(1) indexing via arrays enables that

      # low .....P...... high
      # low ..P..px-1

      # Move left, bounding box comprises median values that are too big (contradicting what a median is), move left and try for smaller values
      # Tail recursive call, which illustrates the binary search our proportional halving of input (low, partition_x - 1)
      max_left_x > min_right_y ->
        partition(list1, list2, low, partition_x - 1, len_x, len_y)

      # Move right, bounding box comprises median values that are too small (contradicting what a median is), move right and try for larger values
      # Tail recursive call, which illustrates the binary search our proportional halving of input (partition_x + 1, high)

      # low .....P...... high
      #        px+1..P.. high

      true ->
        partition(list1, list2, partition_x + 1, high, len_x, len_y)
    end
  end
end

# median is 4
# 0 1 2 3 4 5 6 7 9
# MedianTwoSortedArrays.run([1, 2, 3, 4, 7], [0, 5, 6, 9]) |> IO.inspect(label: "case 1")

# median is 4.5
# 0 1 2 3 4 5 6 7 9 100
# MedianTwoSortedArrays.run([1, 2, 3, 4, 7], [0, 5, 6, 9, 100]) |> IO.inspect(label: "case 2")
