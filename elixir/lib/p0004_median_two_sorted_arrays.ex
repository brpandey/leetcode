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
      when is_list(list1) and is_list(list2) and is_integer(hd(list1)) and is_integer(hd(list2)) do
    m = length(list1)
    n = length(list2)

    # Elixir doesn't support O(1) direct indexing style arrays
    # so leveraging Erlang here (Elixir hashmaps felt too cumbersome here)
    nums1 = :array.from_list(list1)
    nums2 = :array.from_list(list2)

    # the first array passed in should be the smaller one
    if(m > n) do
      partition(nums2, nums1, 0, n, n, m)
    else
      partition(nums1, nums2, 0, m, m, n)
    end
  end

  def partition(_nums1, _nums2, low, high, _m, _n) when low > high,
    do: IO.puts("Error: low can't be greater than high")

  def partition(nums1, nums2, low, high, m, n) when low <= high do
    # We use the smallest array - nums1 - as the starting point and try good faith to pick a median value
    # and see if it will stick otherwise we will refine our guess with other attempts
    # (represented by the tail recursive calls)

    # Since we are pursuing a more optimal solution than O(n) brute force merge of both lists/arrays,
    # we construct "two virtual halves" of the two arrays hopefully representing a well matched division
    # while the elements reside in their original respective sorted arrays

    # After we choose our partition x, we basically compute whatever partition y value
    # for nums2 so that we have a basically balanced set
    # So essentially left half (x + y) is more or less equal right (x + y)

    # We are able to achieve the log(n+m) runtime efficiency by basically halving these partition values
    # on each iteration or partition tail recursive call.  The direct O(1) indexing via arrays enables that
    p_x = Kernel.div(low + high, 2)
    p_y = Kernel.div(m + n + 1, 2) - p_x

    # Generate bounding box of four values which represent the median box boundary
    # which are just the corresponding values to the partition value indices

    # Nums1 (x)
    # In the edge cases where max_left has no set of values we use negative inf as a placeholder
    # and similiarily for min_right
    max_left_x = if p_x > 0, do: :array.get(p_x - 1, nums1), else: @negative_inf
    min_right_x = if p_x == m, do: @positive_inf, else: :array.get(p_x, nums1)

    # Nums2 (y)
    # In the edge cases where max_left has no set of values we use negative inf as a placeholder
    # and similiarily for min_right
    max_left_y = if p_y > 0, do: :array.get(p_y - 1, nums2), else: @negative_inf
    min_right_y = if p_y == n, do: @positive_inf, else: :array.get(p_y, nums2)

    cond do
      # Case that confirms that our "bounding box" is correctly chosen --
      # it is cohesive and well stitched as essentially
      # everything in the set of x is less than the set of y
      # The values within the bounding box are exactly in the biggest minimal
      # middle or median set of the two arrays where we can cleanly pick out
      # the correct median value while avoid further unnecessary iterations
      max_left_x <= min_right_y and min_right_x >= max_left_y ->
        case Integer.is_even(m + n) do
          true ->
            # Virtual left half and right half across both arrays
            v_left = Kernel.max(max_left_x, max_left_y)
            v_right = Kernel.min(min_right_x, min_right_y)
            (v_left + v_right) / 2

          false ->
            Kernel.max(max_left_x, max_left_y)
        end

      # Note: in sorted array left values are smaller and right values are bigger
      # Note again: We are able to achieve the log(n+m) runtime efficiency by basically halving these low and high values
      # on each iteration or partition tail recursive call.  The direct O(1) indexing via arrays enables that

      # low .....P...... high
      # low ..P..px-1

      # Move left, bounding box comprises median values that are too big (contradicting what a median is), move left and try for smaller values
      # Tail recursive call, which illustrates the binary search our proportional halving of input (low, partition_x - 1)
      max_left_x > min_right_y ->
        partition(nums1, nums2, low, p_x - 1, m, n)

      # Move right, bounding box comprises median values that are too small (contradicting what a median is), move right and try for larger values
      # Tail recursive call, which illustrates the binary search our proportional halving of input (partition_x + 1, high)

      # low .....P...... high
      #        px+1..P.. high

      true ->
        partition(nums1, nums2, p_x + 1, high, m, n)
    end
  end
end
