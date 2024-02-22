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
    new_px = px + dx * count
    new_py = py + dy * count
    {new_px, new_py}
  end

  @spec apply_action(String.t(), {{number, number}, {number, number}}) ::
          {{number, number}, {number, number}}
  def apply_action(move, {{px, py}, {dx, dy}}) do
    {dxn, dyn} = rotate(move, {dx, dy})
    {pxn, pyn} = apply_movement(move, {px, py}, {dxn, dyn})
    {{pxn, pyn}, {dxn, dyn}}
  end

  @spec apply_movement_with_set(binary(), {any(), any()}, {any(), any()}, any()) ::
          {:found, {any(), any()}} | {{any(), any()}, {any(), any()}, any()}
  def apply_movement_with_set(move, {px, py}, {dx, dy}, set) do
    count = String.slice(move, 1..-1)
    count = String.to_integer(count)

    case Enum.reduce_while(1..count, {{px, py}, {dx, dy}, set}, fn _, state ->
           apply_move(state)
         end) do
      {:found, x, y} -> {:found, {x, y}}
      {{npx, npy}, {ndx, ndy}, nset} -> {{npx, npy}, {ndx, ndy}, nset}
    end
  end

  @spec apply_move({{number(), number()}, {number(), number()}, MapSet.t(any())}) ::
          {:cont, {{number(), number()}, {number(), number()}, MapSet.t(any())}}
          | {:halt, {:found, number(), number()}}
  def apply_move({{px, py}, {dx, dy}, set}) do
    nx = px + dx
    ny = py + dy

    if MapSet.member?(set, {nx, ny}) do
      {:halt, {:found, nx, ny}}
    else
      {:cont, {{nx, ny}, {dx, dy}, MapSet.put(set, {nx, ny})}}
    end
  end

  @spec apply_action_with_set(binary(), {{any(), any()}, {number(), number()}, any()}) ::
          {:cont, {{any(), any()}, {any(), any()}, any()}} | {:halt, {:found, {any(), any()}}}
  def apply_action_with_set(move, {{px, py}, {dx, dy}, set}) do
    {dxn, dyn} = rotate(move, {dx, dy})

    case apply_movement_with_set(move, {px, py}, {dxn, dyn}, set) do
      {:found, {x, y}} -> {:halt, {:found, {x, y}}}
      {{npx, npy}, {ndx, ndy}, nset} -> {:cont, {{npx, npy}, {ndx, ndy}, nset}}
    end
  end
end
