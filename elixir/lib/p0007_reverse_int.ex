defmodule ReverseInt do
  @big :math.pow(2, 31)
  @small :math.pow(2, 31) * -1

  def run(val) when val > @big or val < @small, do: 0

  def run(val) when is_integer(val) do
    Enum.reduce_while(Stream.cycle([:ok]), {0, val}, fn
      _k, {to, 0} ->
        {:halt, to}

      _k, {to, from} ->
        to = to * 10 + rem(from, 10)
        from = div(from, 10)
        {:cont, {to, from}}
    end)
  end
end
