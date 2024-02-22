defmodule Day1 do

  @spec manhattan_distance({number, number}) :: number
  def manhattan_distance({x, y}) do
    abs(x) + abs(y)
  end

  @spec rotate(String.t(), {number, number}) :: {number, number}
  def rotate(move, {x, y}) do
    case String.first(move) do
      "R" -> {y, -x}
      "L" -> {-y, x}
    end
  end

  @spec apply_movement(String.t(), {number, number}, {number, number}) :: {number, number}
  def apply_movement(move, {px, py}, {dx, dy}) do
    count = String.slice(move, 1..-1)
    count = String.to_integer(count)
    new_px = px + (dx * count)
    new_py = py + (dy * count)
    {new_px, new_py}
  end

  @spec apply_action(String.t(), {{number, number}, {number, number}}) :: {{number, number}, {number, number}}
  def apply_action(move, {{px, py}, {dx, dy}}) do
    {dxn, dyn} = rotate(move, {dx, dy})
    {pxn, pyn} = apply_movement(move, {px, py}, {dxn, dyn})
    {{pxn, pyn}, {dxn, dyn}}
  end

  @spec apply_action_with_set(
          binary(),
          {{number(), number()}, {number(), number()}, MapSet.t({number(), number()})}
        ) ::
          {:cont, {{number(), number()}, {number(), number()}, MapSet.t({number(), number()})}}
          | {:halt, {{number(), number()}, {number(), number()}, MapSet.t({number(), number()})}}
  def apply_action_with_set(move, {{px, py}, {dx, dy}, set}) do
    {dxn, dyn} = rotate(move, {dx, dy})
    {pxn, pyn} = apply_movement(move, {px, py}, {dxn, dyn})

    if MapSet.member?(set, {pxn, pyn}) do
      {:halt, {{pxn, pyn}, {dxn, dyn}, set}}
    else
      {:cont, {{pxn, pyn}, {dxn, dyn}, MapSet.put(set, {pxn, pyn})}}
    end
  end

end
